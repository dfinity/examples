#!/usr/bin/env bash
set -e

echo "=== Test 1/5: public_key returns hex for bip340secp256k1 ==="
result=$(icp canister call backend public_key '(variant { bip340secp256k1 })') && \
  echo "$result" && \
  echo "$result" | grep -q 'public_key_hex' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2/5: public_key returns hex for ed25519 ==="
result=$(icp canister call backend public_key '(variant { ed25519 })') && \
  echo "$result" && \
  echo "$result" | grep -q 'public_key_hex' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3/5: sign returns signature_hex for bip340secp256k1 ==="
result=$(icp canister call backend sign '("hello world of BIP340-secp256k1!", variant { bip340secp256k1 }, null)') && \
  echo "$result" && \
  echo "$result" | grep -q 'signature_hex' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4/5: sign returns signature_hex for ed25519 ==="
result=$(icp canister call backend sign '("hello world", variant { ed25519 }, null)') && \
  echo "$result" && \
  echo "$result" | grep -q 'signature_hex' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 5/5: all three signature types verify cryptographically ==="
npm install --silent

set -euo pipefail
export LC_ALL=C

function get_text_in_double_quotes() {
    echo -n "$1" | sed -e 's/^[^"]*"//' -e 's/"//g' -e 's/;//g'
}

# BIP340 requires the message to be exactly 32 bytes; ed25519 accepts any length.
# Pass a 32-byte ASCII string so the same value works for all three algorithms.
message="hello world of BIP340-secp256k1!"

echo "message=$message (${#message} bytes)"

# --- ed25519 ---
echo "--- ed25519 ---"
ed25519_sig_hex=$(get_text_in_double_quotes "$(icp canister call backend sign "(\"${message}\" ,(variant { ed25519 }), null)" | grep signature_hex)")
echo "ed25519_signature_hex=$ed25519_sig_hex"
ed25519_pubkey_hex=$(get_text_in_double_quotes "$(icp canister call backend public_key '(variant { ed25519 })' | grep public_key_hex)")
echo "ed25519_public_key_hex=$ed25519_pubkey_hex"

node <<END
const ed = require('@noble/ed25519');
const crypto = require('crypto');
// Wire up Node.js built-in SHA512 so the sync verify() works without @noble/hashes
ed.etc.sha512Sync = (...m) => new Uint8Array(crypto.createHash('sha512').update(ed.etc.concatBytes(...m)).digest());
const sig    = Buffer.from("${ed25519_sig_hex}", "hex");
const pubkey = Buffer.from("${ed25519_pubkey_hex}", "hex");
const msg    = Buffer.from("${message}", "utf8");
const ok = ed.verify(sig, msg, pubkey);
console.log("ed25519 verified:", ok);
if (!ok) { console.error("ed25519 verification FAILED"); process.exit(1); }
END

# --- bip340secp256k1 ---
echo "--- bip340secp256k1 ---"
bip340_sig_hex=$(get_text_in_double_quotes "$(icp canister call backend sign "(\"${message}\" ,(variant { bip340secp256k1 }), null)" | grep signature_hex)")
echo "bip340_signature_hex=$bip340_sig_hex"
bip340_pubkey_hex=$(get_text_in_double_quotes "$(icp canister call backend public_key '(variant { bip340secp256k1 })' | grep public_key_hex)")
echo "bip340_public_key_hex=$bip340_pubkey_hex"

node <<END
const ecc = require('tiny-secp256k1');
const sig    = Buffer.from("${bip340_sig_hex}", "hex");
const pubkey = Buffer.from("${bip340_pubkey_hex}".substring(2), "hex");  // drop 02/03 prefix
const msg    = Buffer.from("${message}", "utf8");  // 32 bytes
const ok = ecc.verifySchnorr(msg, pubkey, sig);
console.log("bip340 verified:", ok);
if (!ok) { console.error("bip340 verification FAILED"); process.exit(1); }
END

# --- bip341 (tweaked key) ---
echo "--- bip341 ---"
tweak_hex="012345678901234567890123456789012345678901234567890123456789abcd"
bip341_sig_hex=$(get_text_in_double_quotes "$(icp canister call backend sign "(\"${message}\" ,(variant { bip340secp256k1 }), opt \"${tweak_hex}\")" | grep signature_hex)")
echo "bip341_signature_hex=$bip341_sig_hex"

node <<END
const { tweakKey } = require('bitcoinjs-lib/src/payments/bip341.js');
const bitcoin = require('bitcoinjs-lib');
const ecc     = require('tiny-secp256k1');
bitcoin.initEccLib(ecc);

const sig    = Buffer.from("${bip341_sig_hex}", "hex");
const tweak  = Buffer.from("${tweak_hex}", "hex");
const pubkey = Buffer.from("${bip340_pubkey_hex}".substring(2), "hex");
const msg    = Buffer.from("${message}", "utf8");
const tweaked = tweakKey(pubkey, tweak).x;
const ok = ecc.verifySchnorr(msg, tweaked, sig);
console.log("bip341 verified:", ok);
if (!ok) { console.error("bip341 verification FAILED"); process.exit(1); }
END

echo "PASS"
echo "successfully validated all 3 signature types"
