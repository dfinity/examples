#!/usr/bin/env bash
export LC_ALL=C
function get_text_in_double_quotes() {
    printf "$(echo "$1" | sed -e 's/^[^"]*"//' -e 's/".*//g')"
}

test -z "$1" && echo "USAGE: $0 <message to sign and verify>" && exit 1

message="$1"
echo message="$message"

signature_hex=$(get_text_in_double_quotes "$(dfx canister call ecdsa_example_rust sign "$message" | grep signature)")
echo signature_hex="$signature_hex"

public_key_hex=$(get_text_in_double_quotes "$(dfx canister call ecdsa_example_rust public_key | grep public_key)")
echo public_key_hex="$public_key_hex"

verification_result="$(dfx canister call ecdsa_example_rust verify "(\"$signature_hex\", \"$message\", \"$public_key_hex\")" | sed -e 's/.*is_signature_valid = \(.*\) } }.*/\1/')"
echo verification_result="$verification_result"