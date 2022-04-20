// run this with
// npx ts-node pow.ts 000400000000000001 0
// (parameters are canister id and timestamp)

import proofOfWork from "./src/frontend/src/crypto/pow"
import { Principal } from "@dfinity/principal";

const timestamp = BigInt(process.argv[3]);
const canisterId = Principal.fromText(process.argv[2]);

console.log("Canister id:", canisterId.toText());
console.log("Timestamp:", timestamp);

const pow = proofOfWork(timestamp, canisterId);

console.log("Nonce:", pow.nonce);