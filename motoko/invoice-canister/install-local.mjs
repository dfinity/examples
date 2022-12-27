import { $ } from "zx";

// add test to verify networks.json is configured to use nns ledger? 
// this script will error out if not networks.json is not configured
// as dfx nns install will fail

// zx spawn doesn't handle detached, unref of spawned child processes 
// -> https://github.com/google/zx/discussions/371. could unwrap
// but simpler to use lower level blocking api atm
import { spawnSync } from "child_process"

await $`dfx stop`

spawnSync(
  'dfx',
  ['start', '--clean', '--background'],
  { 
    detached: true,
    stdio: 'ignore'
  }
)

await $`dfx nns install`

// ident-1.pem is one of two identities nns ledger initiallized by sending ICP funds to, going to remove after transfering
// funds to E2E test account identity, balanceHolder. E2E tests are atm simplified so that ICP is transfered back and forth
// between subaccounts controlled by the invoice canister, as opposed to explicitly invoking the ICP ledger canister
await $`dfx identity import invoice-nns-initiallized-icp-funded-identity test/e2e/src/utils/ident-1.pem --disable-encryption`
await $`dfx identity use invoice-nns-initiallized-icp-funded-identity`
// note this account identifier corresponds to the balance holder's principal-subaccount of the invoice canister
await $`dfx ledger transfer 675d3f2043c6bf5d642454cf0890b467673f0bacfd1c85882c0d650d4c6d2abb --icp 1000 --memo 0`

await $`dfx identity use default`
// to keep things clean, dev can always import later if needed
await $`dfx identity remove invoice-nns-initiallized-icp-funded-identity`

try {
  await $`dfx deploy invoice`
} catch (expected) {
  // this error is related to that the nns-ledger canister id is not available to dfx when building the invoice
  // the first time the invoice canister is created and built after dfx start --clean; as there's no defined canister id 
  // in /.dfx/canister_ids.json for the ledger canister, the other option is to not include it as a dependency in dfx.json 
  // and import in ICPLedger.mo and instead add a Motoko typing reference to instantiate it directly in Motoko as an actor 
  // by its local canister id as provided by the dfx nns commands
  console.log("error related to importing nns-ledger is expected, atm not sure the reason this happens")
}

// now deploying invoice will succeed
await $`dfx deploy invoice`

// and finally generate declarations for E2E testing
await $`dfx generate invoice`
