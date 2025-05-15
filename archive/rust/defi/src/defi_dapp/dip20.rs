use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::call::Call;

pub struct DIP20 {
    principal: Principal,
}

#[derive(CandidType, Debug, PartialEq, Deserialize)]
pub enum TxError {
    InsufficientBalance,
    InsufficientAllowance,
    Unauthorized,
    LedgerTrap,
    AmountTooSmall,
    BlockUsed,
    ErrorOperationStyle,
    ErrorTo,
    Other,
}
pub type TxReceipt = Result<Nat, TxError>;

#[allow(non_snake_case)]
#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct Metadata {
    pub logo: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub totalSupply: Nat,
    pub owner: Principal,
    pub fee: Nat,
}

impl DIP20 {
    pub fn new(principal: Principal) -> Self {
        DIP20 { principal }
    }

    pub async fn transfer(&self, target: Principal, amount: Nat) -> TxReceipt {
        // let call_result: Result<(TxReceipt,), _> =
        //     ic_cdk::api::call::call(self.principal, "transfer", (target, amount)).await;

        Call::unbounded_wait(self.principal, "transfer")
            .with_args(&(target, amount))
            .await
            .unwrap()
            .candid()
            .unwrap()
    }

    pub async fn transfer_from(
        &self,
        source: Principal,
        target: Principal,
        amount: Nat,
    ) -> TxReceipt {
        Call::unbounded_wait(self.principal, "transferFrom")
            .with_args(&(source, target, amount))
            .await
            .unwrap()
            .candid()
            .unwrap()
    }

    pub async fn allowance(&self, owner: Principal, spender: Principal) -> Nat {
        Call::unbounded_wait(self.principal, "allowance")
            .with_args(&(owner, spender))
            .await
            .unwrap()
            .candid()
            .unwrap()
    }

    pub async fn get_metadata(&self) -> Metadata {
        Call::unbounded_wait(self.principal, "getMetadata")
            .with_args(&())
            .await
            .unwrap()
            .candid()
            .unwrap()
    }
}
