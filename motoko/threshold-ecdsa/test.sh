function tohex() {
    printf "$(echo "$1" | sed -e 's/^[^"]*"//' -e 's/".*//g' -e 's/%/%%/g' -e 's/\\/\\x/g')" | od -An -tx1 -w64 | sed -e 's/ //g'
}

test -z "$1" && echo "USAGE: $0 <message to sign and verify>" && exit 1

sha256=$(echo "$1" | shasum -a 256 | sed -e 's/ .*//g')
echo sha256="$sha256"

public_key=$(tohex "$(dfx canister call ecdsa_example_motoko public_key | grep public_key)")
echo public_key="$public_key"

args="(blob \"$(echo $sha256 | sed -e 's/\(..\)/\\\1/g')\")"
signature=$(tohex "$(dfx canister call ecdsa_example_motoko sign "$args" | grep signature)")
echo signature=$signature

node <<END
let { ecdsaVerify } = require("secp256k1");
let public_key = new Uint8Array(Buffer.from("${public_key}", "hex"));
let hash = new Uint8Array(Buffer.from("${sha256}", "hex"));
let signature = new Uint8Array(Buffer.from("${signature}", "hex"));
let verified = ecdsaVerify(signature, hash, public_key);
console.log("verified = ", verified)
END
