use candid::Principal;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::cell::RefCell;
use std::time::Duration;

pub mod ecdsa;
pub mod schnorr;
pub mod vetkd;

thread_local! {
    static RNG: RefCell<Option<ChaCha20Rng>> = const { RefCell::new(None) };
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

// In the following, we register a custom getrandom implementation because
// otherwise getrandom (which is a dependency of k256) fails to compile.
// This is necessary because getrandom by default fails to compile for the
// wasm32-unknown-unknown target (which is required for deploying a canister).
// Our custom implementation always fails, which is sufficient here because
// we only use the k256 crate for verifying secp256k1 signatures, and such
// signature verification does not require any randomness.
getrandom::register_custom_getrandom!(always_fail);
pub fn always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}
