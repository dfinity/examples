#!/usr/bin/env bash
export LC_ALL=C
function get_text_in_double_quotes() {
    echo -n "$1" | sed -e 's/^[^"]*"//' -e 's/"//g' -e 's/;//g'
}

function test_impl() {
    test -z "$1" && echo "USAGE: $0 <message to sign and verify>" && exit 1

    message="$1"
    echo message="$message"

#     This should be uncommented when dfx deploys an Ed25519 key

#     ed25519_sign_cmd="dfx canister call schnorr_example_motoko sign '(\"${message}\" ,(variant { ed25519 }))'"
#     ed25519_sig_raw_output="$(eval ${ed25519_sign_cmd} | grep signature_hex)"
#     ed25519_sig_hex="$(get_text_in_double_quotes "${ed25519_sig_raw_output}")"
#     echo ed25519_signature_hex="$ed25519_sig_hex"

#     ed25519_public_key_raw_output="$(dfx canister call schnorr_example_motoko public_key '(variant { ed25519 })' | grep public_key_hex)"
#     ed25519_public_key_hex="$(get_text_in_double_quotes "${ed25519_public_key_raw_output}")"
#     echo ed25519_public_key_hex="$ed25519_public_key_hex"

#     node <<END
#     import('@noble/curves/ed25519').then((ed25519) => { verify(ed25519.ed25519); })
#     .catch((err) => { console.log(err) });

#     function verify(ed25519) {
#         const sig = '${ed25519_sig_hex}';
#         const pubkey = '${ed25519_public_key_hex}';
#         const msg = Uint8Array.from(Buffer.from("${message}", 'utf8'));

#         console.log(ed25519.verify(sig, msg, pubkey));
#     }
# END

    bip340_sign_cmd="dfx canister call schnorr_example_motoko sign '(\"${message}\" ,(variant { bip340secp256k1 }))'"
    bip340_sig_raw_output="$(eval ${bip340_sign_cmd} | grep signature_hex)"
    bip340_sig_hex=$(get_text_in_double_quotes "${bip340_sig_raw_output}")
    echo bip340_signature_hex="${bip340_sig_hex}"

    bip340_public_key_raw_output="$(dfx canister call schnorr_example_motoko public_key 'variant { bip340secp256k1 }' | grep public_key_hex)"
    bip340_public_key_hex=$(get_text_in_double_quotes "${bip340_public_key_raw_output}")
    echo bip340_public_key_hex="${bip340_public_key_hex}"

    node <<END
    import('@noble/curves/secp256k1').then((bip340) => { verify(bip340.schnorr); })
    .catch((err) => { console.log(err) });

    function verify(bip340) {
        const sig = '${bip340_sig_hex}';
        const pubkey = '${bip340_public_key_hex}'.substring(2);
        const msg = Uint8Array.from(Buffer.from("${message}", 'utf8'));

        console.log(bip340.verify(sig, msg, pubkey));
    }
END
}

test_output=$(test_impl "$1")
echo $test_output
if echo $test_output | grep "true"; then
    echo "successfully validated signatures"
    exit 0
else
    echo "failed to validate signatures"
    exit 1
fi
