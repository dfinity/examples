use crate::{
    balances::{Party, ValidatedBalances},
    log_err,
    logged_arithmetics::{logged_saturating_add, logged_saturating_sub},
    state::storage::{ConfigState, StableTransaction},
    validation::ValidatedAsset,
    StableAuditTrail, StableBalances,
};
use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;
use kongswap_adaptor::{
    agent::AbstractAgent,
    audit::OperationContext,
    treasury_manager::{AuditTrail, Error, Operation, Transaction, TreasuryManagerOperation},
};
// use sns_treasury_manager::{Error, Operation, TreasuryManagerOperation};
use std::{cell::RefCell, thread::LocalKey};
// use treasury_manager::{AuditTrail, Transaction};

pub(crate) mod storage;

const NS_IN_SECOND: u64 = 1_000_000_000;

pub const MAX_LOCK_DURATION_NS: u64 = 45 * 60 * NS_IN_SECOND; // 45 minutes

pub(crate) struct KongSwapAdaptor<A: AbstractAgent> {
    time_ns: fn() -> u64,
    pub agent: A,
    pub id: Principal,
    balances: &'static LocalKey<RefCell<StableBalances>>,
    audit_trail: &'static LocalKey<RefCell<StableAuditTrail>>,
}

impl<A: AbstractAgent> KongSwapAdaptor<A> {
    pub fn new(
        time_ns: fn() -> u64,
        agent: A,
        id: Principal,
        balances: &'static LocalKey<RefCell<StableBalances>>,
        audit_trail: &'static LocalKey<RefCell<StableAuditTrail>>,
    ) -> Self {
        KongSwapAdaptor {
            time_ns,
            agent,
            id,
            balances,
            audit_trail,
        }
    }

    pub fn time_ns(&self) -> u64 {
        (self.time_ns)()
    }

    /// Initializes the canister state with the given assets and their owner accounts.
    /// This should only be called once, at canister initialization.
    /// If called multiple times, it will log an error and do nothing.
    pub fn initialize(
        &self,
        asset_0: ValidatedAsset,
        asset_1: ValidatedAsset,
        owner_account: Principal,
    ) {
        self.balances.with_borrow_mut(|cell| {
            if let ConfigState::Initialized(balances) = cell.get() {
                log_err(&format!(
                    "Cannot initialize balances: already initialized at timestamp {}",
                    balances.timestamp_ns
                ));
            }

            // On each ledger, use the main account and no subaccount for managing the assets.
            let manager_account = Account {
                owner: self.id,
                subaccount: None,
            };

            let timestamp_ns = self.time_ns();

            let validated_balances = ValidatedBalances::new(
                timestamp_ns,
                asset_0,
                asset_1,
                owner_account.into(),
                manager_account,
            );

            if let Err(err) = cell.set(ConfigState::Initialized(validated_balances)) {
                log_err(&format!("Failed to initialize balances: {:?}", err));
            }
        });
    }

    /// Applies a function to the mutable reference of the balances,
    /// if the canister has been initialized.
    pub fn with_balances_mut<F>(&self, f: F)
    where
        F: FnOnce(&mut ValidatedBalances),
    {
        self.balances.with_borrow_mut(|cell| {
            let ConfigState::Initialized(balances) = cell.get() else {
                return;
            };

            let mut mutable_balances = balances.clone();
            f(&mut mutable_balances);

            if let Err(err) = cell.set(ConfigState::Initialized(mutable_balances)) {
                log_err(&format!("Failed to update balances: {:?}", err));
            }
        })
    }

    /// Returns a copy of the balances.
    ///
    /// Only safe to call after the canister has been initialized.
    pub fn get_cached_balances(&self) -> ValidatedBalances {
        self.balances.with_borrow(|cell| {
            let ConfigState::Initialized(balances) = cell.get() else {
                ic_cdk::trap("BUG: Balances should be initialized");
            };

            balances.clone()
        })
    }

    /// Returns the two managed assets.
    /// The first asset is always the SNS token, and the second asset is always ICP.
    /// This ordering is important for DEX operations.
    pub fn assets(&self) -> (ValidatedAsset, ValidatedAsset) {
        let validated_balances = self.get_cached_balances();
        (validated_balances.asset_0, validated_balances.asset_1)
    }

    /// Returns the owner accounts for the two managed assets,
    /// set at canister initialization.
    pub fn owner_accounts(&self) -> (Account, Account) {
        let validated_balances = self.get_cached_balances();
        (
            validated_balances.asset_0_balance.treasury_owner.account,
            validated_balances.asset_1_balance.treasury_owner.account,
        )
    }

    /// Returns the ledger canister IDs for the two managed assets.
    pub fn ledgers(&self) -> (Principal, Principal) {
        let balances = self.get_cached_balances();
        (
            balances.asset_0.ledger_canister_id(),
            balances.asset_1.ledger_canister_id(),
        )
    }

    /// Charges the approval fee for the given asset from the manager balance.
    pub fn charge_fee(&mut self, asset: ValidatedAsset) {
        self.with_balances_mut(|validated_balances| validated_balances.charge_approval_fee(asset));
    }

    /// Returns the asset corresponding to the given ledger canister ID,
    /// or None if the canister ID does not match either managed asset.
    /// As this is called for valid canister IDs only, this should never return None.
    pub fn get_asset_for_ledger(&self, canister_id: &String) -> Option<ValidatedAsset> {
        let (asset_0, asset_1) = self.assets();
        if asset_0.ledger_canister_id().to_string() == *canister_id {
            Some(asset_0)
        } else if asset_1.ledger_canister_id().to_string() == *canister_id {
            Some(asset_1)
        } else {
            None
        }
    }

    /// It moves the given amount of the given asset from `from` to `to` in the
    /// corresponding balance book.
    pub fn move_asset(&mut self, asset: ValidatedAsset, amount: u64, from: Party, to: Party) {
        self.with_balances_mut(|validated_balances| {
            validated_balances.move_asset(asset, from, to, amount)
        });
    }

    /// Adds the given amount to the manager balance of the given asset.
    /// This is called when a deposit is initiated, to reflect the incoming
    /// transfer to the manager account.
    pub fn add_manager_balance(&mut self, asset: ValidatedAsset, amount: u64) {
        self.with_balances_mut(|validated_balances| {
            validated_balances.add_manager_balance(asset, amount)
        });
    }

    /// Finds discrepancies in the balances after a transfer.
    /// If a discrepancy is found, it is recorded in the suspense.
    /// This is used to detect unexpected fees or abnormal behavior of the DEX.
    pub fn find_discrepency(
        &mut self,
        asset: ValidatedAsset,
        balance_before: u64,
        balance_after: u64,
        transferred_amount: u64,
        is_deposit: bool,
    ) {
        self.with_balances_mut(|validated_balances| {
            if is_deposit {
                validated_balances.find_deposit_discrepency(
                    asset,
                    balance_before,
                    balance_after,
                    transferred_amount,
                );
            } else {
                validated_balances.find_withdraw_discrepency(
                    asset,
                    balance_before,
                    balance_after,
                    transferred_amount,
                );
            }
        });
    }

    /// Returns the audit trail.
    fn with_audit_trail<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&StableAuditTrail) -> R,
    {
        self.audit_trail.with_borrow(|audit_trail| f(audit_trail))
    }

    fn with_audit_trail_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut StableAuditTrail) -> R,
    {
        self.audit_trail
            .with_borrow_mut(|audit_trail| f(audit_trail))
    }

    /// Returns the index of the pushed transaction in the audit trail, or None if the transaction
    /// could not be pushed.
    pub fn push_audit_trail_transaction(&self, transaction: StableTransaction) -> Option<u64> {
        self.with_audit_trail_mut(|audit_trail| {
            let index = audit_trail.len();
            if let Err(err) = audit_trail.push(&transaction) {
                log_err(&format!(
                    "Cannot push transaction to audit trail: {}\ntransaction: {:?}",
                    err, transaction
                ));
                None
            } else {
                Some(index)
            }
        })
    }

    /// Updates the transaction at the given index in the audit trail.
    pub fn set_audit_trail_transaction_result(&self, index: u64, transaction: StableTransaction) {
        self.with_audit_trail_mut(|audit_trail| {
            if index < audit_trail.len() {
                audit_trail.set(index, &transaction);
            } else {
                log_err(&format!(
                    "BUG: Invalid index {} for audit trail. Audit trail length: {}",
                    index,
                    audit_trail.len(),
                ));
            }
        });
    }

    /// This function finalizes the audit trail by marking the last transaction with the same
    /// operation as finalized. If no such transaction exists, it logs an error.
    /// If the last transaction is already finalized, it does nothing.
    pub fn finalize_audit_trail_transaction(&self, context: OperationContext) {
        let index_transaction = self.with_audit_trail(|audit_trail| {
            let num_transactions = audit_trail.len();
            audit_trail
                .iter()
                .rev()
                .enumerate()
                .find_map(|(rev_index, transaction)| {
                    let transaction_operation = transaction.operation;

                    if transaction_operation.operation == context.operation
                        && !transaction_operation.step.is_final
                    {
                        let rev_index: u64 = match rev_index.try_into() {
                            Ok(index) => index,
                            Err(err) => {
                                log_err(&format!(
                                    "BUG: cannot convert usize {} to u64: {}",
                                    rev_index, err
                                ));
                                return None;
                            }
                        };
                        let index = logged_saturating_sub(
                            num_transactions,
                            logged_saturating_add(rev_index, 1),
                        );

                        Some((index, transaction.clone()))
                    } else {
                        None
                    }
                })
        });

        let Some((index, mut transaction)) = index_transaction else {
            log_err(&format!(
                "Audit trail does not have an {} operation that could be finalized. \
                     Operation context: {:?}",
                context.operation.name(),
                context,
            ));
            return;
        };

        transaction.operation.step.is_final = true;

        self.set_audit_trail_transaction_result(index, transaction);
    }

    /// It checks the balances before and after a withdrawal from the DEX,
    /// and updates the internal state accordingly.
    /// If there are any claim IDs returned by the DEX, it returns an error.
    fn get_remaining_lock_duration_ns(&self) -> Option<u64> {
        let now_ns = self.time_ns();

        fn is_locking_transaction(treasury_manager_operation: &TreasuryManagerOperation) -> bool {
            [Operation::Deposit, Operation::Withdraw]
                .contains(&treasury_manager_operation.operation)
        }

        let AuditTrail { transactions } = self.get_audit_trail();
        let Some(transaction) = transactions
            .iter()
            .rev()
            .find(|transaction| is_locking_transaction(&transaction.treasury_manager_operation))
        else {
            return None;
        };

        if transaction.treasury_manager_operation.step.is_final {
            return None;
        }

        let acquired_timestamp_ns = transaction.timestamp_ns;
        let expiry_timestamp_ns =
            logged_saturating_add(acquired_timestamp_ns, MAX_LOCK_DURATION_NS);

        if now_ns > expiry_timestamp_ns {
            log_err(&format!("Transaction lock expired: {:?}", transaction));
            return None;
        }

        Some(logged_saturating_sub(expiry_timestamp_ns, now_ns))
    }

    /// Checks if the last transaction has been finalized, or if its lock has expired.
    pub fn check_state_lock(&self) -> Result<(), Vec<Error>> {
        if let Some(remaining_lock_duration_ns) = self.get_remaining_lock_duration_ns() {
            return Err(vec![Error::new_temporarily_unavailable(format!(
                "Canister state is locked. Please try again in {} seconds.",
                remaining_lock_duration_ns / NS_IN_SECOND
            ))]);
        }
        Ok(())
    }

    pub fn get_audit_trail(&self) -> AuditTrail {
        let transactions = self
            .audit_trail
            .with_borrow(|audit_trail| audit_trail.iter().map(Transaction::from).collect());

        AuditTrail { transactions }
    }
}
