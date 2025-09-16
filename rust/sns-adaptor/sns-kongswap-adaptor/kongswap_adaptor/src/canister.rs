use crate::state::storage::{ConfigState, StableTransaction};
use crate::validation::{
    ValidatedDepositRequest, ValidatedTreasuryManagerInit, ValidatedWithdrawRequest,
};
use candid::Principal;
use ic_canister_log::{declare_log_buffer, log};
use ic_cdk::{init, inspect_message, post_upgrade, pre_upgrade, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{Cell as StableCell, DefaultMemoryImpl, Vec as StableVec};
use kongswap_adaptor::agent::ic_cdk_agent::CdkAgent;
use kongswap_adaptor::agent::AbstractAgent;
use kongswap_adaptor::audit::OperationContext;
use kongswap_adaptor::treasury_manager::{
    AuditTrail, AuditTrailRequest, Balances, BalancesRequest, DepositRequest, Error, Operation,
    TreasuryManager, TreasuryManagerArg, TreasuryManagerResult, WithdrawRequest,
};
use lazy_static::lazy_static;
use state::KongSwapAdaptor;
use std::{cell::RefCell, time::Duration};

mod balances;
mod deposit;
mod emit_transaction;
mod kong_api;
mod kong_types;
mod ledger_api;
mod logged_arithmetics;
mod rewards;
mod state;
mod tx_error_codes;
mod validation;
mod withdraw;

const RUN_PERIODIC_TASKS_INTERVAL: Duration = Duration::from_secs(60 * 60); // one hour

pub(crate) type Memory = VirtualMemory<DefaultMemoryImpl>;
pub(crate) type StableAuditTrail = StableVec<StableTransaction, Memory>;
pub(crate) type StableBalances = StableCell<ConfigState, Memory>;

// Canister ID from the mainnet.
// See https://dashboard.internetcomputer.org/canister/2ipq2-uqaaa-aaaar-qailq-cai
lazy_static! {
    static ref KONG_BACKEND_CANISTER_ID: Principal =
        Principal::from_text("2ipq2-uqaaa-aaaar-qailq-cai").unwrap();
    static ref ICP_LEDGER_CANISTER_ID: Principal =
        Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap();
}

const BALANCES_MEMORY_ID: MemoryId = MemoryId::new(0);
const AUDIT_TRAIL_MEMORY_ID: MemoryId = MemoryId::new(1);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static BALANCES: RefCell<StableBalances> =
        MEMORY_MANAGER.with(|memory_manager|
            RefCell::new(
                StableCell::init(
                    memory_manager.borrow().get(BALANCES_MEMORY_ID),
                    ConfigState::default()
                )
                .expect("BALANCES init should not cause errors")
            )
        );

    static AUDIT_TRAIL: RefCell<StableAuditTrail> =
        MEMORY_MANAGER.with(|memory_manager|
            RefCell::new(
                StableVec::init(
                    memory_manager.borrow().get(AUDIT_TRAIL_MEMORY_ID)
                )
                .expect("AUDIT_TRAIL init should not cause errors")
            )
        );

}

fn time_ns() -> u64 {
    ic_cdk::api::time()
}

fn canister_state() -> KongSwapAdaptor<CdkAgent> {
    KongSwapAdaptor::new(
        time_ns,
        CdkAgent::new(),
        ic_cdk::api::canister_self(),
        &BALANCES,
        &AUDIT_TRAIL,
    )
}

/// Ensures that only the canister itself or its controllers can call the method.
/// The canister itself is allowed to call the method, as it can be used
/// in order to commit the canister state.
fn check_access() {
    let caller = ic_cdk::api::msg_caller();

    if caller == ic_cdk::api::canister_self() {
        return;
    }

    if ic_cdk::api::is_controller(&caller) {
        return;
    }

    ic_cdk::trap("Only a controller can call this method.");
}

// Inspect the ingress messages in the pre-consensus phase and reject unauthorized access early.
#[inspect_message]
fn inspect_message() {
    let method_name = ic_cdk::api::msg_method_name();

    // Queries can be called by anyone, even if they are called as updates.
    if !["balances", "audit_trail"].contains(&method_name.as_str()) {
        check_access();
    }

    ic_cdk::api::accept_message();
}

declare_log_buffer!(name = LOG, capacity = 100);

fn log_err(msg: &str) {
    log(&format!("Error: {}", msg));
}

fn log(msg: &str) {
    let msg = format!("[KongSwapAdaptor] {}", msg);

    if cfg!(target_arch = "wasm32") {
        ic_cdk::api::debug_print(&msg);
    } else {
        println!("{}", msg);
    }

    log!(LOG, "{}", msg);
}

impl<A: AbstractAgent> TreasuryManager for KongSwapAdaptor<A> {
    /// Withdraws the specified amounts of the managed assets to the specified accounts.
    /// The state lock must be held upon calling this method.
    async fn withdraw(&mut self, request: WithdrawRequest) -> TreasuryManagerResult {
        self.check_state_lock()?;

        // We refresh the external custodian balances, as it could
        // be unknown to the treasury manager, whether an external
        // trader has swapped their tokens on the pool and consequently
        // changes the balances or not.
        self.refresh_balances().await;

        let (ledger_0, ledger_1) = self.ledgers();

        let (default_owner_0, default_owner_1) = self.owner_accounts();

        let ValidatedWithdrawRequest {
            withdraw_account_0,
            withdraw_account_1,
        } = (
            ledger_0,
            ledger_1,
            default_owner_0,
            default_owner_1,
            request,
        )
            .try_into()
            .map_err(|err: String| vec![Error::new_precondition(err)])?;

        let mut context = OperationContext::new(Operation::Withdraw);

        let returned_amounts = self
            .withdraw_impl(&mut context, withdraw_account_0, withdraw_account_1)
            .await
            .map(Balances::from)?;

        self.finalize_audit_trail_transaction(context);

        Ok(returned_amounts)
    }

    /// Deposits the specified amounts of the managed assets from the specified accounts.
    /// The state lock must be held upon calling this method.
    async fn deposit(&mut self, request: DepositRequest) -> TreasuryManagerResult {
        self.check_state_lock()?;

        let ValidatedDepositRequest {
            allowance_0,
            allowance_1,
        } = request
            .try_into()
            .map_err(|err: String| vec![Error::new_precondition(err)])?;

        self.validate_deposit_args(allowance_0, allowance_1)
            .map_err(|err| vec![err])?;

        let mut context = OperationContext::new(Operation::Deposit);

        let deposited_amounts = self
            .deposit_impl(&mut context, allowance_0, allowance_1)
            .await
            .map(Balances::from)?;

        self.finalize_audit_trail_transaction(context);

        Ok(deposited_amounts)
    }

    fn audit_trail(&self, _request: AuditTrailRequest) -> AuditTrail {
        self.get_audit_trail()
    }

    fn balances(&self, _request: BalancesRequest) -> TreasuryManagerResult {
        Ok(Balances::from(self.get_cached_balances()))
    }

    async fn refresh_balances(&mut self) {
        // This operation _can_ and _should_ be lock-free.
        //
        // I. It should be lock free, as periodic tasks should not block deposits and withdrawals.
        // II. It can be lock free, as it does not interfere with any other operations in a bad way.
        //
        // This is how this operation modifies the state:
        // 0. It appends to the audit_trail.
        // 1. It is the sole operation that modifies the external_custodian balances.
        // 2. It does not modify any other part of the state.

        let mut context = OperationContext::new(Operation::Balances);

        let result = self.refresh_balances_impl(&mut context).await;

        if let Err(err) = result {
            log_err(&format!("refresh_balances failed: {:?}", err));
        }

        self.finalize_audit_trail_transaction(context);
    }

    async fn issue_rewards(&mut self) {
        // This operation _can_ and _should_ be lock-free.
        //
        // I. It should be lock free, as periodic tasks should not block deposits and withdrawals.
        //    (If in the future issuing rewards would change the balances, this operation should
        //     probably be made blocking).
        // II. It can be lock free, as it does not interfere with any other operations in a bad way.
        //
        // This is how this operation modifies the state:
        // 0. It appends to the audit_trail.
        // 1. It is the sole operation that modifies the metadata of managed assets.
        // 2. It also modifies the balances of the following parties:
        //    - (Incrementing) Treasury owner balance.
        //    - (Decrementing) Treasury manager balance.
        //    - (Increment) fee collector balance.
        //    The order in which these increments and decrements happen is not very important,
        //    and thus it is okay that this order is not enforced by the code.

        let mut context = OperationContext::new(Operation::IssueReward);

        if let Err(err) = self.refresh_ledger_metadata(&mut context).await {
            log_err(&format!("Failed to refresh ledger metadata: {:?}", err));
        }

        if let Err(err) = self.issue_rewards_impl(&mut context).await {
            log_err(&format!("issue_rewards failed: {:?}", err));
        }

        self.finalize_audit_trail_transaction(context);
    }
}

/// Deposits the specified amounts of the managed assets from the specified accounts.
#[update]
async fn deposit(request: DepositRequest) -> TreasuryManagerResult {
    check_access();

    log("deposit.");

    let result = canister_state().deposit(request).await?;

    Ok(result)
}

/// Withdraws the specified amounts of the managed assets to the specified accounts.
#[update]
async fn withdraw(request: WithdrawRequest) -> TreasuryManagerResult {
    check_access();

    log("withdraw.");

    let result = canister_state().withdraw(request).await?;

    Ok(result)
}

/// Returns the cached balances of the managed assets.
#[query]
fn balances(request: BalancesRequest) -> TreasuryManagerResult {
    canister_state().balances(request)
}

/// Returns the audit trail of operations.
#[query]
fn audit_trail(request: AuditTrailRequest) -> AuditTrail {
    canister_state().audit_trail(request)
}

async fn run_periodic_tasks() {
    log("run_periodic_tasks.");

    let mut kong_adaptor = canister_state();

    // Now
    // 1. Refresh ledger metadata
    // 2. Issue rewards
    // 3. Refresh balances

    // Before
    // 1. Refresh balances
    //    - Refresh ledger metadata
    // 2. Issue rewards
    //   - Refresh balances

    kong_adaptor.issue_rewards().await;

    kong_adaptor.refresh_balances().await;
}

// @todo init_prodic_tasks is currently disabled. Enable it once
// we make the first deposit.
fn init_periodic_tasks() {
    let _new_timer_id = ic_cdk_timers::set_timer_interval(RUN_PERIODIC_TASKS_INTERVAL, || {
        ic_cdk::futures::spawn(run_periodic_tasks())
    });
}

/// When initializing the canister, we don't expect any token transfers to have happened.
/// We soelely set up the canister state by setting the managed assets and their owners.
#[init]
async fn canister_init(arg: TreasuryManagerArg) {
    log("init...");

    let TreasuryManagerArg::Init(init) = arg else {
        ic_cdk::trap("Expected TreasuryManagerArg::Init on canister install.");
    };

    let ValidatedTreasuryManagerInit { asset_0, asset_1 } = init
        .try_into()
        .expect("Failed to validate TreasuryManagerInit.");

    canister_state().initialize(asset_0, asset_1, ic_cdk::api::msg_caller());
}

#[pre_upgrade]
fn canister_pre_upgrade() {
    log("pre_upgrade.");
}

#[post_upgrade]
fn canister_post_upgrade(arg: TreasuryManagerArg) {
    log("post_upgrade.");

    let TreasuryManagerArg::Upgrade(_upgrade) = arg else {
        ic_cdk::trap("Expected TreasuryManagerArg::Upgrade on canister upgrade.");
    };

    init_periodic_tasks();
}

/// Used in order to commit the canister state, which requires an inter-canister call.
/// Otherwise, a trap could discard the state mutations, complicating recovery.
/// See: https://internetcomputer.org/docs/building-apps/security/inter-canister-calls#journaling
#[update(hidden = true)]
fn commit_state() {
    check_access();
}

fn candid_service() -> String {
    candid::export_service!();
    __export_service()
}

fn main() {
    candid::export_service!();
    println!("{}", candid_service());
}
