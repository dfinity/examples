use ic_cdk::{query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::DefaultMemoryImpl;
use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, Memo, TransferArg, TransferError};
use icrc_ledger_types::icrc2::allowance::{Allowance, AllowanceArgs};
use icrc_ledger_types::icrc2::approve::{ApproveArgs, ApproveError};
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
use icrc_ledger_types::icrc3::transactions::{Approve, Burn, Mint, Transaction, Transfer};
use std::cell::RefCell;
use types::*;

mod types;

const MAX_MEMO_SIZE: usize = 32;
const PERMITTED_DRIFT_NANOS: u64 = 60_000_000_000;
const TRANSACTION_WINDOW_NANOS: u64 = 24 * 60 * 60 * 1_000_000_000;

// Error codes
const MEMO_TOO_LONG_ERROR_CODE: usize = 0;

// Create data structures in stable memory so that the data persist across canister upgrades.
const CONFIGURATION_MEMORY_ID: MemoryId = MemoryId::new(1);
const TRANSACTION_LOG_MEMORY_ID: MemoryId = MemoryId::new(2);
thread_local! {
    /// Static memory manager to manage the memory available for stable structures.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Initialize canister state.
    static STATE: RefCell<State> = MEMORY_MANAGER.with(|cell| {
        let mm = cell.borrow();
        let configuration = ConfigCell::init(mm.get(CONFIGURATION_MEMORY_ID), Configuration::default())
            .expect("failed to initialize the config cell");
        let transaction_log = TransactionLog::init(mm.get(TRANSACTION_LOG_MEMORY_ID))
            .expect("failed to initialize the transaction log");
        RefCell::new(State {
            configuration,
            transaction_log,
        })
    });
}

// Convenience method to make read operations simpler
pub fn read_state<R>(f: impl FnOnce(&State) -> R) -> R {
    STATE.with(|cell| f(&cell.borrow()))
}

// Convenience method to make state changes simpler
pub fn mutate_state<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|cell| f(&mut cell.borrow_mut()))
}

// Calculates the current balance of an account
fn balance(account: Account) -> Tokens {
    read_state(|state| {
        let mut balance = 0_usize.into();
        for tx in state.transaction_log.iter() {
            if let Some(mint) = tx.0.mint {
                if mint.to == account {
                    balance += mint.amount;
                }
            }
            if let Some(burn) = tx.0.burn {
                if burn.from == account {
                    balance -= burn.amount;
                }
            }
            if let Some(transfer) = tx.0.transfer {
                if transfer.to == account {
                    balance += transfer.amount.clone();
                }
                if transfer.from == account {
                    balance -= transfer.amount;
                    if let Some(fee) = transfer.fee {
                        balance -= fee;
                    }
                }
            }
            if let Some(approve) = tx.0.approve {
                if let Some(fee) = approve.fee {
                    balance -= fee;
                }
            }
        }
        balance
    })
}

// Calculates the current total supply of tokens
fn total_supply() -> Tokens {
    read_state(|state| {
        let mut supply = 0_usize.into();
        for tx in state.transaction_log.iter() {
            if let Some(mint) = tx.0.mint {
                supply += mint.amount;
            }
            if let Some(burn) = tx.0.burn {
                supply -= burn.amount;
            }
            if let Some(transfer) = tx.0.transfer {
                if let Some(fee) = transfer.fee {
                    supply -= fee;
                }
            }
            if let Some(approve) = tx.0.approve {
                if let Some(fee) = approve.fee {
                    supply -= fee;
                }
            }
        }
        supply
    })
}

// Calculates how much `spender` is allowed to spend from `account` at the moment
fn allowance(account: Account, spender: Account, now: u64) -> Allowance {
    read_state(|state| {
        let mut allowance = 0_usize.into();
        let mut last_approval_expiry = None;
        for tx in state.transaction_log.iter() {
            // Reset expired approval
            if let Some(expires_at) = last_approval_expiry {
                if expires_at < tx.0.timestamp {
                    allowance = 0_usize.into();
                    last_approval_expiry = None;
                }
            }
            // Add pending approval
            if let Some(approve) = tx.0.approve {
                if approve.from == account && approve.spender == spender {
                    allowance = approve.amount;
                    last_approval_expiry = approve.expires_at;
                }
            }
            if let Some(transfer) = tx.0.transfer {
                if transfer.from == account && transfer.spender == Some(spender) {
                    allowance -= transfer.amount;
                    if let Some(fee) = transfer.fee {
                        allowance -= fee;
                    }
                }
            }
        }
        if let Some(expires_at) = last_approval_expiry {
            if expires_at < now {
                allowance = 0_usize.into();
                last_approval_expiry = None;
            }
        }
        Allowance {
            allowance,
            expires_at: last_approval_expiry,
        }
    })
}

// Makes sure `created_at_time` is in the span of allowed timestamps
fn validate_created_at_time(created_at_time: Option<u64>, now: u64) -> Result<(), TransferError> {
    if let Some(tx_time) = created_at_time {
        if tx_time > now && now - tx_time > TRANSACTION_WINDOW_NANOS + PERMITTED_DRIFT_NANOS {
            return Err(TransferError::CreatedInFuture { ledger_time: now });
        }
        if tx_time < now && now - tx_time > TRANSACTION_WINDOW_NANOS + PERMITTED_DRIFT_NANOS {
            return Err(TransferError::TooOld);
        }
    }
    Ok(())
}

// Makes sure the memo fits the requirements
fn validate_memo(memo: Option<&Memo>) -> Result<(), TransferError> {
    if let Some(memo) = memo {
        if memo.0.len() > MAX_MEMO_SIZE {
            return Err(TransferError::GenericError {
                error_code: MEMO_TOO_LONG_ERROR_CODE.into(),
                message: "Memo too long".into(),
            });
        }
    }
    Ok(())
}

// Appends a validated transaction to the transaction log
fn record_tx(tx: &StorableTransaction) -> BlockIndex {
    mutate_state(|state| {
        let idx = state.transaction_log.len();
        state
            .transaction_log
            .push(tx)
            .expect("Failed to grow transaction log.");
        idx.into()
    })
}

// Tries to find a transaction in the transaction log
fn find_tx(tx: &TxInfo) -> Option<BlockIndex> {
    read_state(|state| {
        for (i, candidate_tx) in state.transaction_log.iter().enumerate() {
            if tx.is_approval {
                if let Some(approve) = candidate_tx.0.approve {
                    if tx.from == approve.from
                        && tx.spender == Some(approve.spender)
                        && tx.amount == approve.amount
                        && tx.expected_allowance == approve.expected_allowance
                        && tx.expires_at == approve.expires_at
                        && tx.memo == approve.memo
                        && tx.created_at_time == approve.created_at_time
                    {
                        return Some(i.into());
                    }
                }
            } else {
                if let Some(burn) = candidate_tx.0.burn {
                    if tx.to == state.configuration.get().minting_account
                        && tx.from == burn.from
                        && tx.amount == burn.amount
                        && tx.spender == burn.spender
                        && tx.memo == burn.memo
                        && tx.created_at_time == burn.created_at_time
                    {
                        return Some(i.into());
                    }
                }
                if let Some(mint) = candidate_tx.0.mint {
                    if Some(tx.from) == state.configuration.get().minting_account
                        && tx.to == Some(mint.to)
                        && tx.amount == mint.amount
                        && tx.memo == mint.memo
                        && tx.created_at_time == mint.created_at_time
                    {
                        return Some(i.into());
                    }
                }
                if let Some(transfer) = candidate_tx.0.transfer {
                    if tx.from == transfer.from
                        && tx.to == Some(transfer.to)
                        && tx.amount == transfer.amount
                        && tx.spender == transfer.spender
                        && tx.memo == transfer.memo
                        && tx.created_at_time == transfer.created_at_time
                    {
                        return Some(i.into());
                    }
                }
            }
        }
        None
    })
}

// Turns TxInfo into a validated transaction
fn classify_tx(tx: TxInfo, now: u64) -> Result<StorableTransaction, TransferError> {
    // Deduplication only happens if `created_at_time` is set
    if tx.created_at_time.is_some() {
        if let Some(duplicate_of) = find_tx(&tx) {
            return Err(TransferError::Duplicate { duplicate_of });
        }
    }
    if let Some(specified_fee) = tx.fee {
        let expected_fee = read_state(|state| state.configuration.get().transfer_fee.clone());
        if specified_fee != expected_fee {
            return Err(TransferError::BadFee { expected_fee });
        }
    }
    if tx.is_approval {
        return Ok(StorableTransaction(Transaction {
            kind: "approve".to_string(),
            mint: None,
            burn: None,
            transfer: None,
            approve: Some(Approve {
                from: tx.from,
                spender: tx.spender.expect("Bug: failed to forward spender"),
                amount: tx.amount,
                expected_allowance: tx.expected_allowance,
                expires_at: tx.expires_at,
                memo: tx.memo,
                fee: Some(read_state(|state| {
                    state.configuration.get().transfer_fee.clone()
                })),
                created_at_time: tx.created_at_time,
            }),
            timestamp: now,
        }));
    } else if let Some(minter) =
        read_state(|state| state.configuration.get().minting_account.clone())
    {
        if Some(tx.from) == Some(minter) {
            return Ok(StorableTransaction(Transaction {
                kind: "mint".to_string(),
                mint: Some(Mint {
                    amount: tx.amount,
                    to: tx.to.expect("Bug: failed to forward mint receiver"),
                    memo: tx.memo,
                    created_at_time: tx.created_at_time,
                }),
                burn: None,
                transfer: None,
                approve: None,
                timestamp: now,
            }));
        } else if tx.to == Some(minter) {
            let transfer_fee = read_state(|state| state.configuration.get().transfer_fee.clone());
            if tx.amount < transfer_fee {
                return Err(TransferError::BadBurn {
                    min_burn_amount: transfer_fee,
                });
            }
            let balance = balance(tx.from);
            if balance < tx.amount.clone() + transfer_fee {
                return Err(TransferError::InsufficientFunds { balance });
            }
            return Ok(StorableTransaction(Transaction {
                kind: "burn".to_string(),
                mint: None,
                burn: Some(Burn {
                    amount: tx.amount,
                    from: tx.from,
                    spender: tx.spender,
                    memo: tx.memo,
                    created_at_time: tx.created_at_time,
                }),
                transfer: None,
                approve: None,
                timestamp: now,
            }));
        }
    }
    let balance = balance(tx.from);
    if balance
        < tx.amount.clone() + read_state(|state| state.configuration.get().transfer_fee.clone())
    {
        return Err(TransferError::InsufficientFunds { balance });
    }
    Ok(StorableTransaction(Transaction {
        kind: "transfer".to_string(),
        mint: None,
        burn: None,
        transfer: Some(Transfer {
            amount: tx.amount,
            from: tx.from,
            to: tx.to.expect("Bug: failed to forward transfer receiver"),
            spender: tx.spender,
            memo: tx.memo,
            fee: Some(read_state(|state| {
                state.configuration.get().transfer_fee.clone()
            })),
            created_at_time: tx.created_at_time,
        }),
        approve: None,
        timestamp: now,
    }))
}

// Runs validity checks and records the transaction if it is valid
fn apply_tx(tx: TxInfo) -> Result<BlockIndex, TransferError> {
    validate_memo(tx.memo.as_ref())?;
    let now = ic_cdk::api::time();
    validate_created_at_time(tx.created_at_time, now)?;
    let transaction = classify_tx(tx, now)?;
    Ok(record_tx(&transaction))
}

#[update]
fn icrc1_transfer(arg: TransferArg) -> Result<BlockIndex, TransferError> {
    let from = Account {
        owner: ic_cdk::api::caller(),
        subaccount: arg.from_subaccount,
    };
    let tx = TxInfo {
        from,
        to: Some(arg.to),
        amount: arg.amount,
        spender: None,
        memo: arg.memo,
        fee: arg.fee,
        created_at_time: arg.created_at_time,
        expected_allowance: None,
        expires_at: None,
        is_approval: false,
    };
    apply_tx(tx)
}

#[query]
fn icrc1_balance_of(account: Account) -> Tokens {
    balance(account)
}

#[query]
fn icrc1_total_supply() -> Tokens {
    total_supply()
}

#[query]
fn icrc1_minting_account() -> Option<Account> {
    read_state(|state| state.configuration.get().minting_account.clone())
}

#[query]
fn icrc1_name() -> String {
    read_state(|state| state.configuration.get().token_name.clone())
}

#[query]
fn icrc1_token_symbol() -> String {
    read_state(|state| state.configuration.get().token_symbol.clone())
}

#[query]
fn icrc1_decimals() -> u8 {
    read_state(|state| state.configuration.get().decimals)
}

#[query]
fn icrc1_fee() -> Tokens {
    read_state(|state| state.configuration.get().transfer_fee.clone())
}

#[query]
fn icrc1_metadata() -> Vec<(String, MetadataValue)> {
    vec![
        ("icrc1:name".to_string(), MetadataValue::Text(icrc1_name())),
        (
            "icrc1:symbol".to_string(),
            MetadataValue::Text(icrc1_token_symbol()),
        ),
        (
            "icrc1:decimals".to_string(),
            MetadataValue::Nat(icrc1_decimals().into()),
        ),
        ("icrc1:fee".to_string(), MetadataValue::Nat(icrc1_fee())),
        (
            "icrc1:logo".to_string(),
            MetadataValue::Text(read_state(|state| {
                state.configuration.get().token_logo.clone()
            })),
        ),
    ]
}

#[query]
fn icrc1_supported_standards() -> Vec<SupportedStandard> {
    vec![
        SupportedStandard {
            name: "ICRC-1".to_string(),
            url: "https://github.com/dfinity/ICRC-1/tree/main/standards/ICRC-1".to_string(),
        },
        SupportedStandard {
            name: "ICRC-2".to_string(),
            url: "https://github.com/dfinity/ICRC-1/tree/main/standards/ICRC-2".to_string(),
        },
    ]
}

#[update]
fn icrc2_approve(arg: ApproveArgs) -> Result<BlockIndex, ApproveError> {
    validate_memo(arg.memo.as_ref()).map_err(to_approve_error)?;
    let approver_account = Account {
        owner: ic_cdk::api::caller(),
        subaccount: arg.from_subaccount,
    };
    let now = ic_cdk::api::time();
    if let Some(expected_allowance) = arg.expected_allowance.as_ref() {
        let current_allowance = allowance(approver_account, arg.spender, now).allowance;
        if current_allowance != *expected_allowance {
            return Err(ApproveError::AllowanceChanged { current_allowance });
        }
    }
    let tx = TxInfo {
        from: approver_account,
        to: None,
        amount: arg.amount,
        spender: Some(arg.spender),
        memo: arg.memo,
        fee: arg.fee,
        created_at_time: arg.created_at_time,
        expected_allowance: arg.expected_allowance,
        expires_at: arg.expires_at,
        is_approval: true,
    };
    apply_tx(tx).map_err(to_approve_error)
}

#[update]
fn icrc2_transfer_from(arg: TransferFromArgs) -> Result<BlockIndex, TransferFromError> {
    if ic_cdk::api::caller() == arg.from.owner {
        return icrc1_transfer(TransferArg {
            to: arg.to,
            from_subaccount: arg.from.subaccount,
            amount: arg.amount,
            fee: arg.fee,
            memo: arg.memo,
            created_at_time: arg.created_at_time,
        })
        .map_err(to_transfer_from_error);
    }
    validate_memo(arg.memo.as_ref()).map_err(to_transfer_from_error)?;
    let spender = Account {
        owner: ic_cdk::api::caller(),
        subaccount: arg.spender_subaccount,
    };
    let now = ic_cdk::api::time();
    let allowance = allowance(arg.from, spender, now);
    let transfer_fee = read_state(|state| state.configuration.get().transfer_fee.clone());
    if allowance.allowance < arg.amount.clone() + transfer_fee {
        return Err(TransferFromError::InsufficientAllowance {
            allowance: allowance.allowance,
        });
    }
    let tx = TxInfo {
        from: arg.from,
        to: Some(arg.to),
        amount: arg.amount,
        spender: Some(spender),
        memo: arg.memo,
        fee: arg.fee,
        created_at_time: arg.created_at_time,
        expected_allowance: None,
        expires_at: None,
        is_approval: false,
    };
    apply_tx(tx).map_err(to_transfer_from_error)
}

#[query]
fn icrc2_allowance(arg: AllowanceArgs) -> Allowance {
    let now = ic_cdk::api::time();
    allowance(arg.account, arg.spender, now)
}

#[update]
fn create_token(args: CreateTokenArgs) -> Result<String, String> {
    let minting_account = Account {
        owner: ic_cdk::api::caller(),
        subaccount: None,
    };
    let init_tx = StorableTransaction(Transaction {
        kind: "initial mint".to_string(),
        mint: Some(Mint {
            amount: args.initial_supply,
            to: minting_account,
            memo: None,
            created_at_time: None,
        }),
        burn: None,
        transfer: None,
        approve: None,
        timestamp: ic_cdk::api::time(),
    });
    record_tx(&init_tx);
    mutate_state(|state| {
        state
            .configuration
            .set(Configuration {
                token_name: args.token_name,
                token_symbol: args.token_symbol,
                token_logo: args.token_logo,
                transfer_fee: 10_000_usize.into(),
                decimals: 8,
                minting_account: Some(minting_account),
                token_created: true,
            })
            .map_err(|_| "Failed to set initial config".to_string())?;
        Ok("Token created".to_string())
    })
}

#[query]
fn token_created() -> bool {
    read_state(|state| state.configuration.get().token_created)
}

#[update]
fn delete_token() -> Result<String, String> {
    if !token_created() {
        return Err("Token not created".to_string());
    };

    if ic_cdk::api::caller()
        != read_state(|state| {
            state
                .configuration
                .get()
                .minting_account
                .clone()
                .unwrap()
                .owner
        })
    {
        return Err("Caller is not the token creator".to_string());
    };

    // Reset STATE
    STATE.with_borrow_mut(|state| {
        state.configuration.set(Configuration::default()).unwrap();

        let mem = MEMORY_MANAGER.with(|cell| cell.borrow_mut().get(TRANSACTION_LOG_MEMORY_ID));
        state.transaction_log = TransactionLog::new(mem).unwrap();
    });
    Ok("Token deleted".to_string())
}

// Export the interface for the smart contract.
ic_cdk::export_candid!();
