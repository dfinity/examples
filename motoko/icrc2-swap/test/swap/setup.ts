import * as fs from "fs";
import * as path from "path";
import { IDL } from "@dfinity/candid";
import { AccountIdentifier } from "@dfinity/nns";

import { init as tokenInit } from "../../src/declarations/token_a/token_a.did.js";

import { minter, tester } from "./identity";
import { ledger, deploy } from "./agent";

export default async function setup(globalConfig: any, projectConfig: any) {
  console.log(globalConfig.testPathPattern);
  console.log(projectConfig.cache);

  // Fund the test account with ICP
  // const to = AccountIdentifier.fromPrincipal({ principal: tester.getPrincipal() });
  // const amount = BigInt(10_000_000_000)
  // const balance = await ledger(minter).accountBalance({ accountIdentifier: to });
  // if (balance < amount) {
  //   await ledger(minter).transfer({ to, amount });
  // }

  // Deploy the token a canister
  // const token_a_arg = tokenInit({ IDL }); // TODO: Set the actual args here somehow, and encode it.
  // const token_wasm = fs.readFileSync(path.join(__dirname, "../../.dfx/local/canisters/token_a/token_a.wasm.gz"));
  // const token_a = deploy(minter, token_wasm, token_a_arg);

  // Deploy the swap canister
};
