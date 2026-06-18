#!/usr/bin/env bash
set -e

echo "=== Test 1/3: public_key() returns a hex-encoded public key ==="
result=$(icp canister call backend public_key '()') && \
  echo "$result" && \
  echo "$result" | grep -q 'public_key_hex' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2/3: sign() returns a hex-encoded signature ==="
result=$(icp canister call backend sign '("hello world")') && \
  echo "$result" && \
  echo "$result" | grep -q 'signature_hex' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3/3: signature verifies cryptographically (secp256k1) ==="
npm install --silent

export LC_ALL=C

function get_text_in_double_quotes() {
    printf '%s' "$(echo "$1" | sed -e 's/^[^"]*"//' -e 's/".*//g')"
}

message="hello world"
echo "message=$message"

signature_hex=$(get_text_in_double_quotes "$(icp canister call backend sign "(\"$message\")" | grep signature_hex)")
echo "signature_hex=$signature_hex"

public_key_hex=$(get_text_in_double_quotes "$(icp canister call backend public_key '()' | grep public_key_hex)")
echo "public_key_hex=$public_key_hex"

node <<END
const secp256k1 = require("secp256k1");
const crypto = require("crypto");

const signature = new Uint8Array(Buffer.from("${signature_hex}", "hex"));
const public_key = new Uint8Array(Buffer.from("${public_key_hex}", "hex"));
const message_hash = new Uint8Array(crypto.createHash("sha256").update("${message}", "utf-8").digest());

const verified = secp256k1.ecdsaVerify(signature, message_hash, public_key);
console.log("verified =", verified);
if (!verified) process.exit(1);
END
echo "PASS"
