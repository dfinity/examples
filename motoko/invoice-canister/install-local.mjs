import { $, argv, chalk } from "zx";
import { spawnSync } from "child_process"

// add test to verify networks.json is configured to use nns ledger? 
// this script will error out if not networks.json is not configured
// as dfx nns install will fail

/*
* Script to spin up fresh replica with:
*   -nns-ledger installed by dfx nns command
*   -invoice canister deployed with or without passed delegatedAdministrator 
*     >> to specify a delegatedAministrator use the flag --deployWDelegation 
*           by default is set as the Ed25519KeyIdentity the nns-ledger initializes with funds
*           or *TODO* --deployWDelegation=<principal> to pass a principal of your own choosing.
*
* Call examples:
* 
*  >> node ./install-local.mjs 
*  >> node ./install-local.mjs --deployWDelegation
*
*  Note:
*  Currently in this script the other Secp256k1KeyIdentity the nns-ledger is initialized sending
*   funds to is added to dfx identity list to transfer funds to the balance holder identity used
*   in the E2E tests, and then removed to keep things clean; note this logic will be moved to a 
*   pretest specific script in the future.
*   (*TODO* add a flag to optionally install this Secp256k1KeyIdentity identity for the user)
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
try {
    await $`dfx canister create invoice`
    await $`dfx build invoice`
  } catch (error) {
    const { _stderr } = error;
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

if (argv.deployWDelegation) {
// the principal "jg6qm-uw64t-m6ppo-oluwn-ogr5j-dc5pm-lgy2p-eh6px-hebcd-5v73i-nqe" is that of the Ed25519KeyIdentity 
// the nns-ledger is initiallized sending ~10000000 ICP to it; is also used as the delegated adminstrator in E2E tests who 
// besides the original deployer of the canister has the power to add and remove principals from the allowed creators list 
    console.info(chalk.bgBlue("\ndeploying invoice canister with delegated administrator\n"))
    await $`dfx deploy invoice --argument '( opt principal"jg6qm-uw64t-m6ppo-oluwn-ogr5j-dc5pm-lgy2p-eh6px-hebcd-5v73i-nqe" )'`
} else {
    await $`dfx deploy invoice`
}

// todo // todo // todo // todo // todo // todo // todo // todo // todo // todo // todo // todo // todo move these into pretest
// can use Ed25519KeyIdentity js above when move into tests, will still include the ident-1.pem file for development

// ident-1.pem is one of two identities nns ledger initiallized by sending ICP funds to, going to remove after transfering
// funds to E2E test account identity, balanceHolder. E2E tests are atm streamlined so that ICP is transfered back and forth
// between subaccounts controlled by the invoice canister, as opposed to explicitly invoking the ICP ledger canister
await $`dfx identity import invoice-nns-initiallized-icp-funded-identity test/e2e/src/utils/ident-1.pem --disable-encryption`
await $`dfx identity use invoice-nns-initiallized-icp-funded-identity`
// note this account identifier corresponds to the balance holder's principal-subaccount of the invoice canister
await $`dfx ledger transfer 675d3f2043c6bf5d642454cf0890b467673f0bacfd1c85882c0d650d4c6d2abb --icp 1000 --memo 0`
await $`dfx identity use default`
// to keep things clean, dev can always import later if needed
await $`dfx identity remove invoice-nns-initiallized-icp-funded-identity`

await $`dfx generate invoice`

console.info(chalk.bgBlue("\nInvoice canister ready...\n"))