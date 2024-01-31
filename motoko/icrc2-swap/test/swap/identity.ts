import { Ed25519KeyIdentity } from "@dfinity/identity";

// Minter identity which holds the ICP and initial tokens on the local network
//
// This key is not a secret. Only use it for testing! It is from:
// https://internetcomputer.org/docs/current/references/cli-reference/dfx-nns/#example-accessing-icp-on-the-command-line
const minterPrivateKey = Buffer.from(
  "N3HB8Hh2PrWqhWH2Qqgr1vbU9T3gb1zgdBD8ZOdlQnVS7zC/nkEqaT1kSuvo4i3ldHWSkQZdw5I4LU5jOsDC6Q==",
  "base64",
);
export const minter = Ed25519KeyIdentity.fromSecretKey(minterPrivateKey);

// Randomly generate a new test account each run so there are no collisions,
// and our tests are forced to be robust.
export function newIdentity(): Ed25519KeyIdentity {
  return Ed25519KeyIdentity.generate();
}
