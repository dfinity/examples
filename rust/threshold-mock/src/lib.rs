use candid::{CandidType, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::cell::RefCell;
use std::time::Duration;

type Memory = VirtualMemory<DefaultMemoryImpl>;

pub mod ecdsa;
pub mod schnorr;
pub mod vetkd;

thread_local! {
    static RNG: RefCell<Option<ChaCha20Rng>> = const { RefCell::new(None) };
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    static CALL_COUNTS: RefCell<StableBTreeMap<String, u64, Memory>> = RefCell::new(
            StableBTreeMap::init(
                MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(1))),
            )
        );
}

const SEEDING_INTERVAL: Duration = Duration::from_secs(3600);

#[ic_cdk::init]
fn init() {
    // Initialize randomness during canister install or reinstall
    schedule_seeding(Duration::ZERO);
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    // Initialize randomness after a canister upgrade
    schedule_seeding(Duration::ZERO);
}

fn schedule_seeding(duration: Duration) {
    ic_cdk_timers::set_timer(duration, || {
        ic_cdk::spawn(async {
            seed_randomness().await;
            // Schedule reseeding on a timer with duration SEEDING_INTERVAL
            schedule_seeding(SEEDING_INTERVAL);
        })
    });
}

async fn seed_randomness() {
    let (seed,): ([u8; 32],) = ic_cdk::call(Principal::management_canister(), "raw_rand", ())
        .await
        .expect("Failed to call the management canister's raw_rand");
    RNG.with_borrow_mut(|rng| *rng = Some(ChaCha20Rng::from_seed(seed)));
    ic_cdk::println!("RNG (re-)seeded");
}

async fn with_rng<T>(fn_with_rng: impl FnOnce(&mut ChaCha20Rng) -> T) -> T {
    RNG.with_borrow_mut(|option_rng| fn_with_rng(option_rng.as_mut().expect("missing RNG")))
}

fn ensure_derivation_path_is_valid(derivation_path: &[Vec<u8>]) {
    if derivation_path.len() > 255 {
        ic_cdk::trap("derivation path too long")
    }
}

fn inc_call_count(method_name: String) {
    CALL_COUNTS.with_borrow_mut(|call_counts| {
        let previous_call_count = call_counts.get(&method_name).unwrap_or_default();
        call_counts.insert(method_name, previous_call_count + 1);
    });
}

#[derive(CandidType)]
pub struct CallCountsResult {
    pub call_counts: Vec<CallCount>,
}

#[derive(CandidType)]
pub struct CallCount {
    pub method_name: String,
    pub call_count: u64,
}

#[ic_cdk::query]
async fn call_counts() -> CallCountsResult {
    CALL_COUNTS.with_borrow(|call_counts| {
        let call_counts: Vec<_> = call_counts
            .iter()
            .map(|(method_name, call_count)| CallCount {
                method_name,
                call_count,
            })
            .collect();
        CallCountsResult { call_counts }
    })
}

// In the following, we register a custom getrandom implementation because
// otherwise getrandom (which is an indirect dependency) fails to compile.
// This is necessary because getrandom by default fails to compile for the
// wasm32-unknown-unknown target (which is required for deploying a canister).
// Our custom implementation always fails, which is sufficient here because
// we always provide randomness explicitly in API calls (via an RNG that is
// seeded from the IC's `raw_rand`) and never request randomness from the
// environment.
getrandom::register_custom_getrandom!(always_fail);
pub fn always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}
