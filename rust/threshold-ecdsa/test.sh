#!/usr/bin/env bash
set -euo pipefail
export LC_ALL=C

function get_text_in_double_quotes() {
    printf '%s' "$(echo "$1" | sed -e 's/^[^"]*"//' -e 's/".*//g')"
}

test -z "${1:-}" && echo "USAGE: $0 <message to sign and verify>" && exit 1

message="$1"
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
