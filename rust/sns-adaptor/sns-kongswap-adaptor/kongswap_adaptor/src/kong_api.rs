use crate::{
    kong_types::{
        kong_lp_balance_to_decimals, AddTokenArgs, UserBalanceLPReply, UserBalancesArgs,
        UserBalancesReply,
    },
    log_err, KongSwapAdaptor, KONG_BACKEND_CANISTER_ID,
};
use candid::{Nat, Principal};
use kongswap_adaptor::treasury_manager::Error;
use kongswap_adaptor::{agent::AbstractAgent, audit::OperationContext};

impl<A: AbstractAgent> KongSwapAdaptor<A> {
    pub fn lp_token(&self) -> String {
        let (asset_0, asset_1) = self.assets();
        format!("{}_{}", asset_0.symbol(), asset_1.symbol())
    }

    /// When adding a new token to KongSwap, if the token already exists,
    /// the canister returns an error. This function calls the canister
    /// to add the token, and ignores the error if the token already exists.
    /// For any other error, it returns the error.
    pub async fn maybe_add_token(
        &mut self,
        context: &mut OperationContext,
        ledger_canister_id: Principal,
    ) -> Result<(), Error> {
        let token = format!("IC.{}", ledger_canister_id);

        let human_readable = format!(
            "Calling KongSwapBackend.add_token to attempt to add {}.",
            token
        );

        let request = AddTokenArgs {
            token: token.clone(),
        };

        let response = self
            .emit_transaction(
                context.next_operation(),
                *KONG_BACKEND_CANISTER_ID,
                request,
                human_readable,
            )
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(Error { message, .. }) if message == format!("Token {} already exists", token) => {
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    /// When spinning up a pool with token pair (A, B), the LP token is named "A_B".
    /// This function queries the KongSwap backend canister to get the LP token balance
    /// for the LP token named after the two assets managed by this adaptor.
    pub async fn lp_balance(&mut self, context: &mut OperationContext) -> Nat {
        let request = UserBalancesArgs {
            principal_id: self.id.to_string(),
        };

        let human_readable =
            "Calling KongSwapBackend.user_balances to get LP balances.".to_string();

        let result = self
            .emit_transaction(
                context.next_operation(),
                *KONG_BACKEND_CANISTER_ID,
                request,
                human_readable,
            )
            .await;

        let replies = match result {
            Ok(replies) => replies,
            Err(err) => {
                log_err(&format!(
                    "Failed to call KongSwapBackend.user_balances to get LP balance for {}: {}. \
                     Defaulting to 0.",
                    self.lp_token(),
                    err.message
                ));
                return Nat::from(0_u8);
            }
        };

        let lp_balance = replies.into_iter().find_map(
            |UserBalancesReply::LP(UserBalanceLPReply {
                 symbol, balance, ..
             })| {
                if symbol == self.lp_token() {
                    Some(kong_lp_balance_to_decimals(balance))
                } else {
                    None
                }
            },
        );

        if let Some(lp_balance) = lp_balance {
            lp_balance
        } else {
            log_err(&format!(
                "Failed to get LP balance for {}. Defaulting to 0.",
                self.lp_token(),
            ));
            Nat::from(0_u8)
        }
    }
}
