import { $, chalk } from "zx";
import { spawnSync } from "child_process"

/*
* Script to spin up fresh replica with:
*   -nns-ledger installed by dfx nns command
*   -invoice canister deployed and generations created
*
* Call examples:
* 
*  >> node ./install-local.mjs 
*
*  Note:
*    This script prepares for E2E testing by importing an identity from test/e2e/src/utils/ident-1.pem which
*    is one of the two identities the nns-ledger is initialized with sending ~1000000 ICP to; after importing
*    into dfx identity, funds are transfered to the identity of the balance holder used in the E2E testing,
*    and then it's removed from the dfx identity list to keep things clean. E2E tests pass funds between
*    subaccounts without having to invoke the nns-ledger canister (for now). 
*       
*  !!! If networks.json isn't configured properly this script will fail, see readme for more details...
*/

console.info(chalk.bgBlue("restarting clean to reset nns-ledger"))
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

await $`dfx identity import invoice_nns-funded-Secp256k1 test/e2e/src/utils/ident-1.pem --disable-encryption`
await $`dfx identity use invoice_nns-funded-Secp256k1`
// note this account identifier corresponds to the balance holder's principal-subaccount of the invoice canister
await $`dfx ledger transfer 675d3f2043c6bf5d642454cf0890b467673f0bacfd1c85882c0d650d4c6d2abb --icp 1000 --memo 0`
await $`dfx identity use default`
// to keep things clean, dev can always import later if needed
await $`dfx identity remove invoice_nns-funded-Secp256k1`

try {
    await $`dfx canister create invoice`
    await $`dfx build invoice`
  } catch (error) {
    const { _stderr } = error;
    // to be certain it's expected, for more info on this error see: https://github.com/dfinity/sdk/pull/2687
    if (_stderr.includes(`examples/motoko/invoice-canister/.dfx/local/canisters/idl/ryjl3-tyaaa-aaaaa-aaaba-cai.did. This may produce errors during the build.`)
        && _stderr.includes(`examples/motoko/invoice-canister/src/invoice/ICPLedger.mo:1.1-1.36: import error [M0009]`) 
        && _stderr.includes(`WARN: Failed to copy canister candid from`)
        && _stderr.includes(`examples/motoko/invoice-canister/.dfx/local/canisters/idl/ryjl3-tyaaa-aaaaa-aaaba-cai.did" does not exist`)
    ) {
        console.warn(chalk.yellow(`Error due to import of nns-ledger is expected first time invoice canister is freshly built: should deploy successfully this time...`));
    } else {
        // was something else abort! 
        throw new Error(error)
    }
}

await $`dfx generate invoice`
await $`dfx deploy invoice`

console.info(chalk.bgBlue("\nInvoice canister ready...\n"))