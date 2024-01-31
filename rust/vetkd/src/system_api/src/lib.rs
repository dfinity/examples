use ic_cdk::update;
use ic_crypto_internal_bls12_381_type::{G2Affine, Scalar};
use ic_crypto_internal_bls12_381_vetkd::{
    DerivationPath, DerivedPublicKey, EncryptedKey, EncryptedKeyShare, TransportPublicKey,
    TransportPublicKeyDeserializationError,
};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::cell::RefCell;
use types::{
    VetKDCurve, VetKDEncryptedKeyReply, VetKDEncryptedKeyRequest, VetKDKeyId, VetKDPublicKeyReply,
    VetKDPublicKeyRequest,
};

mod types;

const ENCRYPTED_KEY_CYCLE_COSTS: u64 = 0;

/// DISCLAIMER: This canister here provides an *unsafe* example implementation
/// of a [proposed](https://github.com/dfinity/interface-spec/pull/158) vetKD
/// system API for demonstration purposes. Because of this, in the following
/// we hard-code a randomly generated master secret key. In case vetKD will be
/// integrated into the Internet Computer protocol, then such a key would be
/// created in a secure manner with distributed key generation so that the key
/// never exists in combined form anywyere and nodes can use it only collectively.
const MASTER_SK_HEX: &str = "718c36cd1dcf3501fd04bbe24c3bb9eedfd066d2420e794dd9342cf71d04176f";

lazy_static::lazy_static! {
    static ref MASTER_SK: Scalar = Scalar::deserialize(
        &hex::decode(MASTER_SK_HEX).expect("failed to hex-decode")
    ).expect("failed to deserialize Scalar");
    static ref MASTER_PK: G2Affine = G2Affine::from(G2Affine::generator() * &*MASTER_SK);
}

thread_local! {
    static RNG: RefCell<Option<ChaCha20Rng>> = RefCell::new(None);
}

#[update]
async fn vetkd_public_key(request: VetKDPublicKeyRequest) -> VetKDPublicKeyReply {
    ensure_bls12_381_test_key_1(request.key_id);
    ensure_derivation_path_is_valid(&request.derivation_path);
    let derivation_path = {
        let canister_id = request.canister_id.unwrap_or_else(ic_cdk::caller);
        DerivationPath::new(canister_id.as_slice(), &request.derivation_path)
    };
    let derived_public_key = DerivedPublicKey::compute_derived_key(&MASTER_PK, &derivation_path);
    VetKDPublicKeyReply {
        public_key: derived_public_key.serialize().to_vec(),
    }
}

#[update]
async fn vetkd_encrypted_key(request: VetKDEncryptedKeyRequest) -> VetKDEncryptedKeyReply {
    ensure_call_is_paid(ENCRYPTED_KEY_CYCLE_COSTS);
    ensure_bls12_381_test_key_1(request.key_id);
    ensure_derivation_path_is_valid(&request.public_key_derivation_path);
    let derivation_path = DerivationPath::new(
        ic_cdk::caller().as_slice(),
        &request.public_key_derivation_path,
    );
    let tpk =
        TransportPublicKey::deserialize(&request.encryption_public_key).unwrap_or_else(
            |e| match e {
                TransportPublicKeyDeserializationError::InvalidPublicKey => {
                    ic_cdk::trap("invalid encryption public key")
                }
            },
        );
    let eks = with_rng(|rng| {
        EncryptedKeyShare::create(
            rng,
            &MASTER_PK,
            &MASTER_SK,
            &tpk,
            &derivation_path,
            &request.derivation_id,
        )
    })
    .await;
    let ek = EncryptedKey::combine(
        &vec![(0, MASTER_PK.clone(), eks)],
        1,
        &MASTER_PK,
        &tpk,
        &derivation_path,
        &request.derivation_id,
    )
    .unwrap_or_else(|_e| ic_cdk::trap("bad key share"));

    VetKDEncryptedKeyReply {
        encrypted_key: ek.serialize().to_vec(),
    }
}

fn ensure_bls12_381_test_key_1(key_id: VetKDKeyId) {
    if key_id.curve != VetKDCurve::Bls12_381 {
        ic_cdk::trap("unsupported key ID curve");
    }
    if key_id.name.as_str() != "test_key_1" {
        ic_cdk::trap("unsupported key ID name");
    }
}

fn ensure_derivation_path_is_valid(derivation_path: &Vec<Vec<u8>>) {
    if derivation_path.len() > 255 {
        ic_cdk::trap("derivation path too long")
    }
}

fn ensure_call_is_paid(cycles: u64) {
    if ic_cdk::api::call::msg_cycles_accept(cycles) < cycles {
        ic_cdk::trap("insufficient cycles");
    }
}

/// Uses an RNG from the canister's state that is seeded _once_ from a system call to `raw_rand`.
/// IMPORTANT: this technique cannot generally be considered secure/safe because malicious nodes
/// could then predict randomness. However, in this particular case here it is OK to use the
/// cached RNG because the entire canister is anyway _unsafe_ and only for demonstration purposes.
async fn with_rng<T>(fn_with_rng: impl FnOnce(&mut ChaCha20Rng) -> T) -> T {
    let rng_initialized = RNG.with(|option_rng| match &*option_rng.borrow() {
        Some(_rng) => true,
        None => false,
    });
    if !rng_initialized {
        let (raw_rand,): (Vec<u8>,) = ic_cdk::api::management_canister::main::raw_rand()
            .await
            .unwrap_or_else(|_e| ic_cdk::trap("call to raw_rand failed"));
        let raw_rand_32_bytes: [u8; 32] = raw_rand
            .try_into()
            .unwrap_or_else(|_e| panic!("raw_rand not 32 bytes"));
        let rng = ChaCha20Rng::from_seed(raw_rand_32_bytes);
        RNG.with(|option_rng| {
            // Initialize the RNG only if it wasn't initialized before.
            // Note that this is necessary because the RNG state is accessed
            // both before (read) and after (write) the async inter-canister
            // call to raw_rand, which leads to the code being executed in
            // different messages. See
            // https://internetcomputer.org/docs/current/developer-docs/security/
            // for more details regarding _canister development security best
            // practices_.
            option_rng.borrow_mut().get_or_insert(rng);
        });
        ic_cdk::println!("RNG initialized");
    }
    RNG.with(|option_rng| fn_with_rng(option_rng.borrow_mut().as_mut().expect("missing RNG")))
}
