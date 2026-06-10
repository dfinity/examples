#!/usr/bin/env bash
set -euo pipefail
export LC_ALL=C

function get_text_in_double_quotes() {
    echo -n "$1" | sed -e 's/^[^"]*"//' -e 's/"//g' -e 's/;//g'
}

function test_impl() {
    test -z "${1:-}" && echo "USAGE: $0 <message to sign and verify>" && exit 1

    local message="$1"
    echo "message=$message"

    local ed25519_sign_cmd="icp canister call backend sign '(\"${message}\" ,(variant { ed25519 }), null)'"
    local ed25519_sig_raw_output
    ed25519_sig_raw_output="$(eval "${ed25519_sign_cmd}" | grep signature_hex)"
    local ed25519_sig_hex
    ed25519_sig_hex="$(get_text_in_double_quotes "${ed25519_sig_raw_output}")"
    echo "ed25519_signature_hex=$ed25519_sig_hex"

    local ed25519_public_key_raw_output
    ed25519_public_key_raw_output="$(icp canister call backend public_key '(variant { ed25519 })' | grep public_key_hex)"
    local ed25519_public_key_hex
    ed25519_public_key_hex="$(get_text_in_double_quotes "${ed25519_public_key_raw_output}")"
    echo "ed25519_public_key_hex=$ed25519_public_key_hex"

    node <<END
    async function run() {
        try {
            const ed25519 = await import('@noble/ed25519');
            const sha512 = await import('@noble/hashes/sha512');
            ed25519.etc.sha512Sync = (...m) => sha512.sha512(ed25519.etc.concatBytes(...m));

            const sig = '${ed25519_sig_hex}';
            const pubkey = '${ed25519_public_key_hex}';
            const msg = Uint8Array.from(Buffer.from("${message}", 'utf8'));
            console.log(await ed25519.verify(sig, msg, pubkey));
        } catch(err) {
            console.error(err);
            process.exit(1);
        }
    }
    run();
END

    local bip340_sign_cmd="icp canister call backend sign '(\"${message}\" ,(variant { bip340secp256k1 }), null)'"
    local bip340_sig_raw_output
    bip340_sig_raw_output="$(eval "${bip340_sign_cmd}" | grep signature_hex)"
    local bip340_sig_hex
    bip340_sig_hex="$(get_text_in_double_quotes "${bip340_sig_raw_output}")"
    echo "bip340_signature_hex=$bip340_sig_hex"

    local bip340_public_key_raw_output
    bip340_public_key_raw_output="$(icp canister call backend public_key '(variant { bip340secp256k1 })' | grep public_key_hex)"
    local bip340_public_key_hex
    bip340_public_key_hex="$(get_text_in_double_quotes "${bip340_public_key_raw_output}")"
    echo "bip340_public_key_hex=$bip340_public_key_hex"

    node <<END
    async function run() {
        try {
            const ecc = await import('tiny-secp256k1');
            const sig = Buffer.from('${bip340_sig_hex}', 'hex');
            const pubkey = Buffer.from('${bip340_public_key_hex}'.substring(2), 'hex');
            const msg = Buffer.from("${message}", 'utf8');
            console.log(ecc.verifySchnorr(msg, pubkey, sig));
        } catch(err) {
            console.error(err);
            process.exit(1);
        }
    }
    run();
END

    local bip341_tweak_hex="012345678901234567890123456789012345678901234567890123456789abcd"
    local bip341_sign_cmd="icp canister call backend sign '(\"${message}\" ,(variant { bip340secp256k1 }), opt \"${bip341_tweak_hex}\")'"
    local bip341_sig_raw_output
    bip341_sig_raw_output="$(eval "${bip341_sign_cmd}" | grep signature_hex)"
    local bip341_sig_hex
    bip341_sig_hex="$(get_text_in_double_quotes "${bip341_sig_raw_output}")"
    echo "bip341_signature_hex=$bip341_sig_hex"

    node <<END
    async function run() {
        try {
            const { tweakKey } = await import('bitcoinjs-lib/src/payments/bip341.js');
            const bitcoin = await import('bitcoinjs-lib');
            const ecc = await import('tiny-secp256k1');
            bitcoin.initEccLib(ecc);

            const sig = Buffer.from('${bip341_sig_hex}', 'hex');
            const tweak = Buffer.from('${bip341_tweak_hex}', 'hex');
            const pubkey = Buffer.from('${bip340_public_key_hex}'.substring(2), 'hex');
            const msg = Buffer.from("${message}", 'utf8');
            const tweaked_pubkey = tweakKey(pubkey, tweak).x;
            console.log(ecc.verifySchnorr(msg, tweaked_pubkey, sig));
        } catch(err) {
            console.error(err);
            process.exit(1);
        }
    }
    run();
END
}

test_output=$(test_impl "$1")
echo "$test_output"
NUM_SUCCESS_VALIDATIONS=$(echo "$test_output" | grep -c "^true$" || true)
# Expects 3 true outputs: 1) ed25519, 2) bip340, 3) bip341
if [ "$NUM_SUCCESS_VALIDATIONS" -eq 3 ]; then
    echo "successfully validated all 3 signature types"
    exit 0
else
    echo "failed: only $NUM_SUCCESS_VALIDATIONS/3 signature types validated"
    exit 1
fi
