/// Implements functions to interact with the ledger canisters to get balances
/// and return remaining assets to the owner accounts.
use crate::{
    balances::Party,
    state::KongSwapAdaptor,
    tx_error_codes::TransactionErrorCodes,
    validation::{decode_nat_to_u64, ValidatedAsset},
};
use candid::Nat;
use icrc_ledger_types::icrc1::{
    account::Account,
    transfer::{Memo, TransferArg},
};
use kongswap_adaptor::treasury_manager::{Error, ErrorKind};
use kongswap_adaptor::{agent::AbstractAgent, audit::OperationContext};

impl<A: AbstractAgent> KongSwapAdaptor<A> {
    async fn get_ledger_balance_decimals(
        &mut self,
        context: &mut OperationContext,
        asset: ValidatedAsset,
    ) -> Result<u64, Error> {
        let ledger_canister_id = asset.ledger_canister_id();

        let request = Account {
            owner: self.id,
            subaccount: None,
        };

        let human_readable = format!(
            "Calling {}.icrc1_balance_of to get the remaining balance of {}.",
            ledger_canister_id,
            asset.symbol(),
        );

        let balance_decimals = self
            .emit_transaction(
                context.next_operation(),
                ledger_canister_id,
                request,
                human_readable,
            )
            .await?;

        let balance_decimals = decode_nat_to_u64(balance_decimals).map_err(|error| Error {
            code: u64::from(TransactionErrorCodes::PostConditionCode),
            message: error.clone(),
            kind: ErrorKind::Postcondition {},
        })?;

        Ok(balance_decimals)
    }

    pub(crate) async fn get_ledger_balances(
        &mut self,
        context: &mut OperationContext,
    ) -> Result<(u64, u64), Vec<Error>> {
        let (asset_0, asset_1) = self.assets();

        // TODO: These calls could be parallelized.
        let balance_0_decimals = self.get_ledger_balance_decimals(context, asset_0).await;
        let balance_1_decimals = self.get_ledger_balance_decimals(context, asset_1).await;

        match (balance_0_decimals, balance_1_decimals) {
            (Ok(balance_0), Ok(balance_1)) => Ok((balance_0, balance_1)),
            (Err(err), Ok(_)) | (Ok(_), Err(err)) => Err(vec![err]),
            (Err(err_1), Err(err_2)) => Err(vec![err_1, err_2]),
        }
    }

    async fn return_remaining_assets_to_owner_impl(
        &mut self,
        context: &mut OperationContext,
        asset: ValidatedAsset,
        amount_decimals: u64,
        withdraw_account: Account,
    ) -> Result<(), Error> {
        if amount_decimals == 0 {
            return Ok(());
        }

        let ledger_canister_id = asset.ledger_canister_id();

        let human_readable = format!(
            "Calling {}.icrc1_transfer to return {} {} from KongSwapAdaptor to {}.",
            ledger_canister_id,
            amount_decimals,
            asset.symbol(),
            withdraw_account,
        );

        let operation = context.next_operation();

        let request = TransferArg {
            from_subaccount: None,
            to: withdraw_account,
            fee: Some(Nat::from(asset.ledger_fee_decimals())),
            created_at_time: Some(self.time_ns()),
            memo: Some(Memo::from(Vec::<u8>::from(operation))),
            amount: Nat::from(amount_decimals - asset.ledger_fee_decimals()),
        };

        self.emit_transaction(operation, ledger_canister_id, request, human_readable)
            .await?;

        self.move_asset(
            asset,
            amount_decimals,
            Party::TreasuryManager,
            Party::TreasuryOwner,
        );

        Ok(())
    }

    pub(crate) async fn return_remaining_assets_to_owner(
        &mut self,
        context: &mut OperationContext,
        withdraw_account_0: Account,
        withdraw_account_1: Account,
    ) -> Result<(), Vec<Error>> {
        let (asset_0, asset_1) = self.assets();

        // Take into account that the ledger fee required for returning the assets.
        let (return_amount_0_decimals, return_amount_1_decimals) =
            self.get_ledger_balances(context).await?;

        let mut withdraw_errors = vec![];

        for (asset, amount_decimals, withdraw_account) in [
            (asset_0, return_amount_0_decimals, withdraw_account_0),
            (asset_1, return_amount_1_decimals, withdraw_account_1),
        ] {
            let result = self
                .return_remaining_assets_to_owner_impl(
                    context,
                    asset,
                    amount_decimals,
                    withdraw_account,
                )
                .await;

            if let Err(err) = result {
                withdraw_errors.push(err);
            }
        }

        if !withdraw_errors.is_empty() {
            return Err(withdraw_errors);
        }

        Ok(())
    }
}
