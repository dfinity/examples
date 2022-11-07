import { $ } from "zx";

// await $.spawn(
//   `set -e

//     dfx stop
//     dfx start --background --clean`
// );

const hasMinter = (await $`dfx identity list`).stdout.includes("minter");
console.log(hasMinter);
let MINT_ACC;

// Setup minter and set MINT_ACC
if (!hasMinter) {
  await $`dfx identity new minter --disable-encryption
    dfx identity use minter`;
  MINT_ACC = await $`dfx ledger account-id`;
} else {
  await $`dfx identity use minter`;
  MINT_ACC = await $`dfx ledger account-id`;
}

// Switch back to default
await $`dfx identity use default`;

const LEDGER_ACC = (await $`dfx ledger account-id`).stdout;
const TEST_ACC =
  "cd60093cef12e11d7b8e791448023348103855f682041e93f7d0be451f48118b";

await $`
# Use private api for install
rm src/ledger/ledger.did
cp src/ledger/ledger.private.did src/ledger/ledger.did`;

try {


  await $`dfx deploy ledger --argument '(record {minting_account = "'${MINT_ACC}'"; initial_values = vec { record { "'${LEDGER_ACC}'"; record { e8s=100_000_000_000 } }; record { "'${TEST_ACC}'"; record { e8s=100_000_000_000 } }; }; send_whitelist = vec {}})'`;

  // dfx deploy ledger --argument '(record {minting_account = "'${MINT_ACC}'"; initial_values = vec { record { "'${LEDGER_ACC}'"; record { e8s=100_000_000_000 } }; record { "'${TEST_ACC}'"; record { e8s=100_000_000_000 } }; }; send_whitelist = vec {}})'

  // # Replace with public api
  await $`
rm src/ledger/ledger.did
cp src/ledger/ledger.public.did src/ledger/ledger.did

dfx deploy invoice
`;
} catch (error) {
  console.error(error);
}