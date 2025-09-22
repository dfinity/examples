use crate::{
    balances::{ValidatedBalanceBook, ValidatedBalances},
    ICP_LEDGER_CANISTER_ID,
};
use candid::{CandidType, Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;
use kongswap_adaptor::treasury_manager::{
    Allowance, Asset, Balance, BalanceBook, Balances, DepositRequest, TreasuryManagerInit,
    WithdrawRequest,
};
use maplit::btreemap;
use serde::Deserialize;
use std::str::FromStr;

pub const MAX_SYMBOL_BYTES: usize = 10;

pub(crate) struct ValidatedTreasuryManagerInit {
    pub asset_0: ValidatedAsset,
    pub asset_1: ValidatedAsset,
}

/// This function validates that the provided assets are suitable for use with the KongSwapAdaptor.
/// This is done by checking that:
/// 1. Exactly two assets are provided.
/// 2. One of the assets represents ICP tokens (asset_1).
/// 3. The other asset dis our SNS token (asset_0).
/// 4. The ledger_canister_id of the ICP asset is the ICP ledger canister ID.
/// 5. The ledger_canister_id of the SNS token asset is NOT the ICP ledger canister ID.
pub(crate) fn validate_assets(
    mut assets: Vec<Asset>,
) -> Result<(ValidatedAsset, ValidatedAsset), String> {
    let mut problems = vec![];

    let form_error = |err: &str| -> Result<(ValidatedAsset, ValidatedAsset), String> {
        Err(format!("Invalid assets: {}", err))
    };

    let Some(Asset::Token {
        symbol: symbol_1,
        ledger_canister_id: ledger_canister_id_1,
        ledger_fee_decimals: ledger_fee_decimals_1,
    }) = assets.pop()
    else {
        return form_error("KongSwapAdaptor requires some assets.");
    };

    let Some(Asset::Token {
        symbol: symbol_0,
        ledger_canister_id: ledger_canister_id_0,
        ledger_fee_decimals: ledger_fee_decimals_0,
    }) = assets.pop()
    else {
        return form_error(&format!(
            "KongSwapAdaptor requires two assets (got {}).",
            assets.len()
        ));
    };

    if !assets.is_empty() {
        problems.push(format!(
            "KongSwapAdaptor requires exactly two assets (got {}).",
            assets.len()
        ));
    }

    if symbol_0 == "ICP" {
        problems.push("asset_0 must NOT represent ICP tokens.".to_string());
    }

    if symbol_1 != "ICP" {
        problems.push("asset_1 must represent ICP tokens.".to_string());
    }

    if ledger_canister_id_0 == *ICP_LEDGER_CANISTER_ID {
        problems.push("asset_0 ledger must NOT be the ICP ledger.".to_string());
    }

    if ledger_canister_id_1 != *ICP_LEDGER_CANISTER_ID {
        problems.push("asset_1 ledger must be the ICP ledger.".to_string());
    }

    if !problems.is_empty() {
        return form_error(&format!("\n  - {}", problems.join("  - \n")));
    }

    let asset_0 = ValidatedAsset::try_from(Asset::Token {
        symbol: symbol_0,
        ledger_canister_id: ledger_canister_id_0,
        ledger_fee_decimals: ledger_fee_decimals_0,
    })
    .map_err(|err| format!("Failed to validate asset_0: {}", err))?;

    let asset_1 = ValidatedAsset::try_from(Asset::Token {
        symbol: symbol_1,
        ledger_canister_id: ledger_canister_id_1,
        ledger_fee_decimals: ledger_fee_decimals_1,
    })
    .map_err(|err| format!("Failed to validate asset_1: {}", err))?;

    Ok((asset_0, asset_1))
}

/// Validates that exactly two allowances are provided, and that their assets are valid.
/// Returns the validated allowances if successful, or an error message if validation fails.
/// An allowance is valid if:
/// 1. Its asset is valid (see `ValidatedAsset`).
/// 2. Its amount_decimals can be converted to u64.
/// 3. Its owner_account is valid (see `account_into_icrc1_account`).
pub(crate) fn validate_allowances(
    mut allowances: Vec<Allowance>,
) -> Result<(ValidatedAllowance, ValidatedAllowance), String> {
    let Some(allowance_1) = allowances.pop() else {
        return Err("KongSwapAdaptor requires some allowances.".to_string());
    };

    let Some(allowance_0) = allowances.pop() else {
        return Err(format!(
            "KongSwapAdaptor requires two allowances (got {}).",
            allowances.len()
        ));
    };

    let mut problems = vec![];

    if !allowances.is_empty() {
        problems.push(format!(
            "KongSwapAdaptor requires exactly two allowances (got {}).",
            allowances.len()
        ));
    }

    let allowance_0 = ValidatedAllowance::try_from(allowance_0)
        .map_err(|err| format!("Failed to validate allowance_0: {}", err))?;

    let allowance_1 = ValidatedAllowance::try_from(allowance_1)
        .map_err(|err| format!("Failed to validate allowance_1: {}", err))?;

    if !problems.is_empty() {
        let problems = problems.join("  - \n");
        return Err(format!("Invalid allowances:\n - {}", problems));
    }

    Ok((allowance_0, allowance_1))
}

impl TryFrom<Allowance> for ValidatedAllowance {
    type Error = String;

    fn try_from(allowance: Allowance) -> Result<Self, Self::Error> {
        let Allowance {
            asset,
            amount_decimals,
            owner_account,
        } = allowance;

        let mut problems = vec![];

        let asset = match ValidatedAsset::try_from(asset) {
            Ok(asset) => Some(asset),
            Err(err) => {
                problems.push(err);
                None
            }
        };

        let amount_decimals = match decode_nat_to_u64(amount_decimals) {
            Ok(amount_decimals) => Some(amount_decimals),
            Err(err) => {
                problems.push(err);
                None
            }
        };

        if !problems.is_empty() {
            let problems = problems.join("  - \n");
            return Err(format!("Invalid allowance:\n - {}", problems));
        }

        let asset = asset.unwrap();
        let amount_decimals = amount_decimals.unwrap();
        let owner_account = account_into_icrc1_account(&owner_account);

        Ok(Self {
            asset,
            amount_decimals,
            owner_account,
        })
    }
}

impl TryFrom<Asset> for ValidatedAsset {
    type Error = String;

    fn try_from(value: Asset) -> Result<Self, Self::Error> {
        let Asset::Token {
            symbol,
            ledger_canister_id,
            ledger_fee_decimals,
        } = value;

        let symbol = ValidatedSymbol::try_from(symbol.as_str())
            .map_err(|err| format!("Failed to validate asset symbol: {}", err))?;

        let ledger_fee_decimals = decode_nat_to_u64(ledger_fee_decimals)
            .map_err(|err| format!("Failed to validate asset ledger fee_decimals: {}", err))?;

        Ok(Self::Token {
            symbol,
            ledger_canister_id,
            ledger_fee_decimals,
        })
    }
}

impl TryFrom<TreasuryManagerInit> for ValidatedTreasuryManagerInit {
    type Error = String;

    fn try_from(init: TreasuryManagerInit) -> Result<Self, Self::Error> {
        let TreasuryManagerInit { assets } = init;

        let (asset_0, asset_1) = validate_assets(assets)
            .map_err(|err| format!("Failed to validate TreasuryManagerInit: {}", err))?;

        Ok(Self { asset_0, asset_1 })
    }
}

#[derive(CandidType, Clone, Copy, Debug, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum ValidatedAsset {
    Token {
        symbol: ValidatedSymbol,
        ledger_canister_id: Principal,
        ledger_fee_decimals: u64,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct ValidatedAllowance {
    pub asset: ValidatedAsset,
    pub amount_decimals: u64,
    pub owner_account: Account,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct ValidatedDepositRequest {
    pub allowance_0: ValidatedAllowance,
    pub allowance_1: ValidatedAllowance,
}

impl TryFrom<DepositRequest> for ValidatedDepositRequest {
    type Error = String;

    fn try_from(value: DepositRequest) -> Result<Self, Self::Error> {
        let DepositRequest { allowances } = value;

        let (allowance_0, allowance_1) = validate_allowances(allowances)
            .map_err(|err| format!("Failed to validate DepositRequest: {}", err))?;

        Ok(Self {
            allowance_0,
            allowance_1,
        })
    }
}

pub(crate) struct ValidatedWithdrawRequest {
    pub withdraw_account_0: Account,
    pub withdraw_account_1: Account,
}

pub(crate) fn account_into_icrc1_account(
    account: &kongswap_adaptor::treasury_manager::Account,
) -> Account {
    Account {
        owner: account.owner,
        subaccount: account.subaccount,
    }
}

pub(crate) fn icrc1_account_into_account(
    account: &Account,
) -> kongswap_adaptor::treasury_manager::Account {
    kongswap_adaptor::treasury_manager::Account {
        owner: account.owner,
        subaccount: account.subaccount,
    }
}

impl TryFrom<(Principal, Principal, Account, Account, WithdrawRequest)>
    for ValidatedWithdrawRequest
{
    type Error = String;

    fn try_from(
        value: (Principal, Principal, Account, Account, WithdrawRequest),
    ) -> Result<Self, Self::Error> {
        let mut errors = vec![];

        let (
            ledger_0,
            ledger_1,
            default_withdraw_account_0,
            default_withdraw_account_1,
            WithdrawRequest { withdraw_accounts },
        ) = value;

        let (withdraw_account_0, withdraw_account_1) =
            if let Some(ledger_to_account) = withdraw_accounts {
                if ledger_to_account.get(&ledger_0).is_none() {
                    errors.push(format!(
                        "Withdraw account for ledger {} not found.",
                        ledger_0
                    ));
                }

                if ledger_to_account.get(&ledger_1).is_none() {
                    errors.push(format!(
                        "Withdraw account for ledger {} not found.",
                        ledger_1
                    ));
                }

                if !errors.is_empty() {
                    return Err(errors.join(", "));
                }

                let withdraw_account_0 = ledger_to_account.get(&ledger_0).unwrap();
                let withdraw_account_1 = ledger_to_account.get(&ledger_1).unwrap();

                (
                    account_into_icrc1_account(withdraw_account_0),
                    account_into_icrc1_account(withdraw_account_1),
                )
            } else {
                (default_withdraw_account_0, default_withdraw_account_1)
            };

        Ok(Self {
            withdraw_account_0,
            withdraw_account_1,
        })
    }
}

// (symbol, ledger_canister_id, ledger_fee_decimals)
impl TryFrom<(String, String, u64)> for ValidatedAsset {
    type Error = String;

    fn try_from(value: (String, String, u64)) -> Result<Self, Self::Error> {
        let (symbol, ledger_canister_id, ledger_fee_decimals) = value;

        let symbol = ValidatedSymbol::try_from(symbol)?;

        let ledger_canister_id = Principal::from_str(&ledger_canister_id).map_err(|_| {
            format!(
                "Cannot interpret second component as a principal: {}",
                ledger_canister_id
            )
        })?;

        Ok(Self::Token {
            symbol,
            ledger_canister_id,
            ledger_fee_decimals,
        })
    }
}

fn take_bytes(input: &str) -> [u8; MAX_SYMBOL_BYTES] {
    let mut result = [0u8; MAX_SYMBOL_BYTES];
    let bytes = input.as_bytes();

    let copy_len = std::cmp::min(bytes.len(), MAX_SYMBOL_BYTES);
    result[..copy_len].copy_from_slice(&bytes[..copy_len]);

    result
}

fn is_valid_symbol_character(b: &u8) -> bool {
    *b == 0 || b.is_ascii_graphic()
}

#[derive(CandidType, Clone, Copy, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ValidatedSymbol {
    /// An Ascii string of up to MAX_SYMBOL_BYTES, e.g., "CHAT" or "ICP".
    /// Stored as a fixed-size byte array, so the whole `Asset` type can derive `Copy`.
    /// Can be created from
    repr: [u8; MAX_SYMBOL_BYTES],
}

impl TryFrom<[u8; 10]> for ValidatedSymbol {
    type Error = String;

    fn try_from(value: [u8; 10]) -> Result<Self, Self::Error> {
        // Check that the symbol is valid ASCII.
        if !value.iter().all(is_valid_symbol_character) {
            return Err(format!("Symbol must be ASCII and graphic; got {:?}", value));
        }

        Ok(ValidatedSymbol { repr: value })
    }
}

impl TryFrom<&str> for ValidatedSymbol {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > MAX_SYMBOL_BYTES {
            return Err(format!(
                "Symbol must not exceed {} bytes or characters, got {} bytes.",
                MAX_SYMBOL_BYTES,
                value.len()
            ));
        }

        let bytes = take_bytes(&value);

        let symbol = Self::try_from(bytes)?;

        Ok(symbol)
    }
}

impl TryFrom<String> for ValidatedSymbol {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

fn bytes_to_string(bytes: &[u8]) -> String {
    // Find the first null byte (if any)
    let null_pos = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());

    // Convert only ASCII characters
    bytes[..null_pos].iter().map(|&c| c as char).collect()
}

impl std::fmt::Display for ValidatedSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol_str = bytes_to_string(&self.repr);
        write!(f, "{}", symbol_str)
    }
}

impl std::fmt::Debug for ValidatedSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol_str = bytes_to_string(&self.repr);
        write!(f, "{}", symbol_str)
    }
}

impl ValidatedAsset {
    pub fn symbol(&self) -> String {
        match self {
            Self::Token { symbol, .. } => symbol.to_string(),
        }
    }

    pub fn set_symbol(&mut self, new_symbol: ValidatedSymbol) -> bool {
        match self {
            Self::Token { ref mut symbol, .. } => {
                if symbol == &new_symbol {
                    false
                } else {
                    *symbol = new_symbol;
                    true
                }
            }
        }
    }

    pub fn ledger_fee_decimals(&self) -> u64 {
        match self {
            Self::Token {
                ledger_fee_decimals,
                ..
            } => *ledger_fee_decimals,
        }
    }

    pub fn set_ledger_fee_decimals(&mut self, new_fee_decimals: u64) -> bool {
        match self {
            Self::Token {
                ref mut ledger_fee_decimals,
                ..
            } => {
                if ledger_fee_decimals == &new_fee_decimals {
                    false
                } else {
                    *ledger_fee_decimals = new_fee_decimals;
                    true
                }
            }
        }
    }

    pub fn ledger_canister_id(&self) -> Principal {
        match self {
            Self::Token {
                ledger_canister_id, ..
            } => *ledger_canister_id,
        }
    }
}

pub(crate) fn decode_nat_to_u64(value: Nat) -> Result<u64, String> {
    let u64_digit_components = value.0.to_u64_digits();

    match &u64_digit_components[..] {
        [] => Ok(0),
        [val] => Ok(*val),
        vals => Err(format!(
            "Error parsing a Nat value `{:?}` to u64: expected a unique u64 value, got {:?}.",
            &value,
            vals.len(),
        )),
    }
}

impl From<ValidatedAsset> for Asset {
    fn from(value: ValidatedAsset) -> Self {
        let ValidatedAsset::Token {
            symbol,
            ledger_canister_id,
            ledger_fee_decimals,
        } = value;

        let symbol = symbol.to_string();
        let ledger_fee_decimals = Nat::from(ledger_fee_decimals);

        Self::Token {
            symbol,
            ledger_canister_id,
            ledger_fee_decimals,
        }
    }
}

impl From<ValidatedAllowance> for Allowance {
    fn from(value: ValidatedAllowance) -> Self {
        let ValidatedAllowance {
            asset,
            amount_decimals,
            owner_account,
        } = value;

        let asset = Asset::from(asset);
        let amount_decimals = Nat::from(amount_decimals);
        let owner_account = icrc1_account_into_account(&owner_account);

        Allowance {
            asset,
            amount_decimals,
            owner_account,
        }
    }
}

#[derive(CandidType, Clone, Debug, Deserialize, PartialEq)]
pub struct ValidatedBalance {
    pub amount_decimals: u64,
    pub account: Account,
}

impl From<ValidatedBalance> for Balance {
    fn from(value: ValidatedBalance) -> Self {
        Self {
            amount_decimals: Nat::from(value.amount_decimals),
            account: Some(kongswap_adaptor::treasury_manager::Account {
                owner: value.account.owner,
                subaccount: value.account.subaccount,
            }),
            name: None,
        }
    }
}

impl TryFrom<Balance> for ValidatedBalance {
    type Error = String;
    fn try_from(value: Balance) -> Result<Self, Self::Error> {
        let mut errors = vec![];

        let amount_decimals_result = decode_nat_to_u64(value.amount_decimals.clone());
        if amount_decimals_result.is_err() {
            errors.push(format!(
                "Failed to convert amount {} to u64",
                value.amount_decimals
            ));
        };

        let icrc1_account = value
            .account
            .map(|account| account_into_icrc1_account(&account));

        if icrc1_account.is_none() {
            errors.push(format!("Owner account of the balance is not set"));
        };

        if value.name.is_none() {
            errors.push(format!("Name is not set"));
        };

        if !errors.is_empty() {
            return Err(errors.join(", "));
        }

        Ok(Self {
            amount_decimals: amount_decimals_result.unwrap(),
            account: icrc1_account.unwrap(),
        })
    }
}

impl From<ValidatedBalanceBook> for BalanceBook {
    fn from(value: ValidatedBalanceBook) -> Self {
        Self {
            treasury_owner: Some(value.treasury_owner.clone().into()),
            treasury_manager: Some(value.treasury_manager.clone().into()),
            external_custodian: Some(Balance {
                amount_decimals: Nat::from(value.external),
                account: None,
                name: None,
            }),
            fee_collector: Some(Balance {
                amount_decimals: Nat::from(value.fee_collector),
                account: None,
                name: None,
            }),
            payees: Some(Balance {
                amount_decimals: Nat::from(value.spendings),
                account: None,
                name: None,
            }),
            payers: Some(Balance {
                amount_decimals: Nat::from(value.earnings),
                account: None,
                name: None,
            }),
            suspense: Some(Balance {
                amount_decimals: Nat::from(value.suspense),
                account: None,
                name: None,
            }),
        }
    }
}

impl From<ValidatedBalances> for Balances {
    fn from(value: ValidatedBalances) -> Self {
        let asset_to_balances = Some(btreemap! {
            Asset::from(value.asset_0) => BalanceBook::from(value.asset_0_balance),
            Asset::from(value.asset_1) => BalanceBook::from(value.asset_1_balance),
        });

        Self {
            timestamp_ns: value.timestamp_ns,
            asset_to_balances,
        }
    }
}
