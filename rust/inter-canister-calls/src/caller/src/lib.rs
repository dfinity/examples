// Some of the imports will only be used in later examples; we list them here for simplicity
use candid::{Nat, Principal};
use ic_cdk::api::time;
use ic_cdk::call::{Call, CallErrorExt, RejectCode};
use ic_cdk::management_canister::{EcdsaCurve, EcdsaKeyId, SignWithEcdsaArgs, SignWithEcdsaResult, cost_sign_with_ecdsa};
use ic_cdk_macros::update;
use sha2::{Digest, Sha256};

// When calling other canisters:
// 1. The simplest is to mark your function as `update`. Then you can always call any public
//    endpoint on any other canister.
// 2. Mark the function as `async`. Then you can use the `Call` API to call other canisters.
// This particular example requires the caller to provide the principal (i.e., ID) of the counter canister.
#[update]
pub async fn call_get_and_set(counter: Principal, new_value: Nat) -> Nat {
    // To make a call, you must provide the principal (i.e., ID) of the canister you're
    // calling, and the method name that you're calling. In this example, require the caller to provide
    // the principal of the counter canister as an argument to `call_get_and_set`.
    // When making a call, you must choose between bounded and unbounded wait calls. These call
    // types have different failure modes that will be explained later.
    let old = Call::unbounded_wait(counter, "get_and_set")
        // `Call` follows the builder pattern; you can customize call options before you finalize
        // the call by issuing the `call()` method. For this example, provide an argument of type that
        // get_and_set expects, a Nat (non-negative integer). The Rust CDK serializes the argument
        // for you using the Candid format, so you just need to provide the Rust value.
        .with_arg(&new_value)
        // Use async/await syntax to obtain the call result. Note that the call is only issued once
        // you await it!
        .await
        // Calls can *always* fail. In this first example it's actually OK to just panic on error,
        // but panicking here is almost always a bad idea due to how the ICP's messaging model works.
        .expect("Failed to get the old value. Bail out")
        // To deserialize the Candid-encoded response, the CDK needs to know the type to expect.
        // You can use the turbofish syntax to specify the type as show below, where we expect a Candid
        // Nat (i.e., a non-negative integer) as the response.
        .candid::<Nat>()
        .expect("Candid decoding failed");
    old
}

#[update]
pub async fn set_then_get(counter: Principal, new_value: Nat) -> Nat {
    Call::unbounded_wait(counter, "set")
        .with_arg(&new_value)
        .await
        // In this particular example it's still OK to panic, as there are no state changes.
        .expect("Failed to set the value. Bailing out");

    let current_value: Nat = Call::unbounded_wait(counter, "get")
        .await
        .expect("Failed to get the current_value value. Bail out")
        .candid()
        .expect("Candid decoding failed");

    // It looks like you should be able to assert:
    // assert!(current_value == new_value);
    // But due to how ICP's messaging model works, this is *NOT* guaranteed to hold!
    current_value
}

#[update]
pub async fn call_increment(counter: Principal) -> Result<(), String> {
    match Call::unbounded_wait(counter, "increment")
        .await {
        // The counter canister successfully responded. Here, it means that the call was successful.
        // A more complicated target than the counter (e.g., a ledger) could also return
        // "user-level" errors that you should handle.
        Ok(_) => Ok(()),
        // A non-clean reject for unbounded-wait calls only occurs if the callee is malfunctioning
        // in some way (including panics). On the simple counter example that we deploy locally, this
        // should not happen.
        Err(e) if !e.is_clean_reject() =>
            Err(format!("Should not happen: the call failed with a non-clean reject: {:?}. We don't know whether the counter increased", e)),
        // Clean errors mean that the call did not execute, and the caller did not change its state.
        Err(e) => {
            Err(format!("The call failed with error {:?}. The counter wasn't increased", e))
        }
    }
}

#[update]
pub async fn call_get(counter: Principal) -> Result<Nat, String> {
    match Call::bounded_wait(counter, "get")
        // The default timeout is 10 seconds. Here it is changed to 1 second.
        .change_timeout(1)
        .await
    {
        Ok(bytes) => bytes
            .candid::<Nat>()
            .map_err(|e| format!("Candid decoding failed: {:?}", e)),
        // A non-clean reject for bounded-wait calls can also happen due to a timeout.
        Err(e) if !e.is_clean_reject() => Err(format!(
            "Getting the value failed with a non-clean reject: {:?}",
            e
        )),
        Err(e) => Err(format!("The call failed with a clean reject {:?}", e)),
    }
}

/// Retries setting the counter to the provided value even if errors appear, until it succeeds,
/// times out, or hits an unrecoverable error.
#[update]
pub async fn stubborn_set(counter: Principal, new_value: Nat) -> Result<(), String> {
    // Let's set a timeout to 10 minutes.
    let timeout = std::time::Duration::from_secs(10 * 60).as_nanos() as u64;
    // Compute the deadline based on the current IC time.
    let deadline = time() + timeout;
    // Try to set the counter to the provided value, retrying where possible.
    loop {
        match Call::bounded_wait(counter, "set")
            .with_arg(&new_value)
            .await
        {
            Ok(_) => return Ok(()),
            // The immediately_retryable() predicate will return false if the call is likely to
            // just fail again if you retry immediately.
            Err(e) if e.is_immediately_retryable() => {
                // Even if you can retry, don't if it is out of time
                if time() > deadline {
                    return Err(format!("Timed out while trying to set the value: {:?}", e));
                } else {
                    continue;
                }
            }
            // If immediate retry isn't possible, bail out. Leave the upstream caller
            // to clean up the mess. For example, they can use the `call_get` endpoint to see whether
            // the set was successful.
            Err(e) => {
                return Err(format!(
                    "Hit an error that we can't retry: {:?}. Bailing out",
                    e
                ))
            }
        }
    }
}

#[update]
pub async fn sign_message(message: String) -> Result<String, String> {
    let message_hash = Sha256::digest(&message).to_vec();

    let request = SignWithEcdsaArgs {
        message_hash,
        // This example does not use the fancier signing features
        derivation_path: vec![],
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            // This is the key name used for local testing; different
            // key names are needed for the mainnet
            name: "dfx_test_key".to_string(),
        },
    };

    let cycles_cost = cost_sign_with_ecdsa(&request).map_err(|e| {
        format!(
            "Failed to compute cycles cost for signing with ECDSA: {:?}",
            e
        )
    })?;

    // Use bounded-wait calls in this example, since the amount attached is
    // fairly low, and losing the attached cycles isn't catastrophic.
    match Call::bounded_wait(Principal::management_canister(), "sign_with_ecdsa")
        .with_arg(&request)
        // Signing with a test key requires 30 billion cycles
        .with_cycles(cycles_cost)
        .await
    {
        Ok(resp) => match resp.candid::<SignWithEcdsaResult>() {
            Ok(signature) => Ok(hex::encode(signature.signature)),
            Err(e) => Err(format!("Error decoding response: {:?}", e)),
        },
        // A SysUnknown reject code only occurs due to a bounded-wait call timing out.
        // It means that no cycles will be refunded, even
        // if the call didn't make it to the callee. Here, this is fine since
        // only a small amount is used.
        Err(ic_cdk::call::CallFailed::CallRejected(e))
        if e.reject_code() == Ok(RejectCode::SysUnknown) =>
            {
                Err(format!(
                    "Got a SysUnknown error while signing message: {:?}; cycles are not refunded",
                    e
                ))
            }
        Err(e) => Err(format!("Error signing message: {:?}", e)),
    }
}
