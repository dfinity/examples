import { $, argv, chalk, fs } from 'zx';
// oneLine to make it fit more neatly into console output.
import { oneLine } from 'common-tags';
import { spawnSync } from 'child_process';

/**  
  Startup script to get a correctly configured local replica up and running. 

  Deploys the following canisters (in this order):
    1) dfx nns installed canisters (for using its nns-ledger canister)
    2) invoice canister
    3) ICP ledger canister from downloaded wasm and did ("Ledger Local Setup" )
    4) ICRC1 token-ledger canister (with archiving) (Dfinity Rosetta repository)
    5) ICRC1 token-ledger canister (with archiving) (Dfinity Rosetta repository)

    The four token-ledger canisters are required for the invoice canister to process 
  transactions of their token type. Their dfx json defined names map to the 
  `SupportedToken` variant entries (see developer docs for more info) as follows:

      nns-ledger                        > #ICP_nns              
      icp_ledger_canister               > #ICP                 
      icrc1_token_ledger_canister_ex1   > #ICRC1_ExampleToken  
      icrc1_token_ledger_canister_ex2   > #ICRC1_ExampleToken2

  Example of calling the script:
    $ node ./clean-startup.mjs --deployForTesting 

    By default, `deployForTesting` is set to false. To limit the console output to 
  explicit calls made to the console logger, the `--quiet` flag can also be passed.

    For all but the `dfx nns` installed ICP ledger, the minting accounts are set 
  for the currently used dfx identity; if the flag `deployForTesting` is passed 
  `nns-funded-secp256k1` will be added as an identity, if necessary, and switched 
  to be the currently used identity.
  
    This identity is one of two that the `dfx nns` installed ICP ledger is initialized 
  with sending funds to in lieu of setting a minting account. If the testing flag 
  is passed, it will also be used to deploy all the canisters (the invoice canister's 
  allowed creators list can only be changed by the canister's installer).
  
    Finally, to prepare for E2E testing funds are sent to the creator subaccount 
  addresses of each token type for the `nns-funded-secp256k1` identity which acts 
  as a balance holder / source of funds in the E2E tests (as opposed to calling the 
  token-ledger canisters explicitly, transfers of funds are facilitated through 
  the invoice canister's own `transfer` method).
  
    Note that after the E2E testing completes, the current identity will be the
  `nns-funded-secp256k1` identity. 

    Also note as `dfx nns install` is used the system wide network configuration
  must used and configured as a system subnet type replica with port 8080 to work
  correctly. A check is performed at the beginning to verify this is set. For
  more information see the `dfx nns` link directly below:

    For more information on dfx nns:
  https://github.com/dfinity/sdk/blob/master/docs/cli-reference/dfx-nns.md

    For more information on "Ledger Local Setup":
  https://internetcomputer.org/docs/current/developer-docs/integrations/ledger/ledger-local-setup/
  
    For more information on ICRC1 token-ledger canister:
  https://github.com/dfinity/ic/tree/master/rs/rosetta-api/icrc1/ledger

    For more information on zx (used to script dfx cli):
  https://github.com/google/zx */

// Predefined constants commonly used.
const constants = {
  nnsFundedSecp256k1Identity: {
    // Name used for the imported identity.
    dfxIdentityName: 'nns-funded-secp256k1',
    principalLiteral: `hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe`,
    creatorSubaccounts: {
    // Specified here as they are used in E2E testing.
      icpAccountIdentifierLiteral:
        '5bea0a832af66531a1c2dda9e4f027f6c31dd943af039ff509ae492841b8b980',
      icpAccountIdentifierBlobLiteral: `[\\ea\\0a\\83*\\f6e1\\a1\\c2\\dd\\a9\\e4\\f0\\27\\f6\\c3\\1d\\d9C\\af\\03\\9f\\f5\\09\\aeI(A\\b8\\b9\\80`,
      icrc1SubaccountBlobLiteral: `\\dc\\f6(N7;lQ\\e0\\9aG&\\1e\\fb\\bfI\\c2\\95\\bdS\\88\\08\\de2\\f8\\bc\\eb\\a3bZl\\60`
    },
  },
  invoiceCanisterId: 'q4eej-kyaaa-aaaaa-aaaha-cai',
  icpLedgerCanisterDfxJsonName: 'icp_ledger_canister',
  icrc1ExampleToken: {
    dfxJsonName: 'icrc1_token_ledger_canister_ex1',
    // Intentionally contrived so no collisions IRL.
    tokenName: 'Internet Computer Random Curency One Example Token',
    tokenSymbol: '_1ICRC1EX',
  },
  icrc1ExampleToken2: {
    dfxJsonName: 'icrc1_token_ledger_canister_ex2',
    // Intentionally contrived so no collisions IRL.
    tokenName: 'Two Internet Computer Random Curency One Example Token',
    tokenSymbol: '_2ICRC1EX',
  },
};

// Zx escapes its input and this interferes with some dfx commands;
// (see https://github.com/google/zx/issues/164 fmi). Until a better
// quote escaping function for zx for dfx can be made available, can
// use this to run any dfx commands as is (template literal composed
// for example) without zx's escaper and then reset to using its own.
const dfxRaw = async cmd => {
  const escaping = $.quote;
  $.quote = (...all) => all;
  const res = await $`${cmd}`;
  $.quote = escaping;
  return res;
};

// Starts dfx clean in the background.
const restart_dfx_clean_in_background = async () => {
  console.info(chalk.cyan('restarting dfx clean in background'));
  await $`dfx stop`;
  spawnSync('dfx', ['start', '--clean', '--background'], {
    detached: true,
    stdio: 'ignore',
  });
};

// Confirm system wide networks.json is configured correctly for `dfx nns install`.
const check_correct_system_network_config = async () => {
  try {
    const networks = `${await $`dfx info networks-json-path`}`.trim();
    const { local } = JSON.parse(fs.readFileSync(networks, 'utf8'));
    return (local.bind === '127.0.0.1:8080' && local?.replica?.subnet_type === 'system');
  } catch (e) {
    // If for some reason, reading and parsing networks.json throws, definitely return false. 
    return false;
  }
}

// Add the identity initialized with funds by `dfx nns install` that
// will be used to deploy the invoice ledger during E2E testing.
const use_nnsFundedSecp256k1Identity_and_add_if_needed = async () => {
  console.info(
    chalk.cyan(
      `adding the dfx nns ICP ledger's funded secp256k1 identity if not already added to switch current identity to`,
    ),
  );
  const { stdout } = await $`dfx identity list`;
  if (stdout.split('\n').includes(constants.nnsFundedSecp256k1Identity.dfxIdentityName)) {
    // Already includes proceed.
  } else {
    await $`dfx identity import ${constants.nnsFundedSecp256k1Identity.dfxIdentityName} test/nnsFundedSecp256k1.pem --disable-encryption`;
  }
  // Use the correct identity to run E2E testing.
  await $`dfx identity use ${constants.nnsFundedSecp256k1Identity.dfxIdentityName}`;
};

// Deploys the ICP ledger and swaps its did as per 'Ledger Local Setup' dev doc instructions.
const deploy_icp_ledger_from_downloaded_wasm_and_did = async () => {
  console.info(
    chalk.cyan(
      'deploying ICP ledger from downloaded wasm and swapping its public and private did files',
    ),
  );
  // This will also work if an ICP ledger canister's "remote" key is used 
  // in dfx json as long as both steps of the swap are performed.
  const swap_icp_ledger_did_and_write_out_dfx_json = ({ which }) => {
    const pub = `"candid": "src/token-ledger-canisters/icp/ledger.public.did"`;
    const prv = `"candid": "src/token-ledger-canisters/icp/ledger.private.did"`;
    const dfxJsonLines = fs.readFileSync('dfx.json', 'utf8');
    if ((which.prv && dfxJsonLines.includes(prv)) || (which.pub && dfxJsonLines.includes(pub))) {
      // Is already correct.
    }
    fs.writeFileSync(
      'dfx.json',
      fs
        .readFileSync('dfx.json', 'utf8')
        .split('\n')
        .map(l =>
          l.includes(pub) ? l.replace(pub, prv) : l.includes(prv) ? l.replace(prv, pub) : l,
        )
        .join('\n'),
    );
  };
  const mintingAccount = `${await $`dfx ledger account-id`}`.trim();
  const cliDeployLiteral = oneLine`dfx deploy ${constants.icpLedgerCanisterDfxJsonName} 
    --argument '(record {
        minting_account = "${mintingAccount}"; 
        initial_values = vec {}; 
        send_whitelist = vec {}
      }
  )'`;

  // Set the did to the private version in dfx json (1st step).
  swap_icp_ledger_did_and_write_out_dfx_json({ which: { prv: {} } });
  // Deploy the ICP ledger.
  await dfxRaw(cliDeployLiteral);
  // Set the did to the public version in dfx json (2nd step).
  swap_icp_ledger_did_and_write_out_dfx_json({ which: { pub: {} } });
};

const deploy_icrc1_token_canister = async (
  currentIdentityPrincipal,
  { dfxJsonName, tokenName, tokenSymbol },
) => {
  console.info(
    chalk.cyan(
      `deploying ICRC1 token canister with name ${tokenName} with ${currentIdentityPrincipal} as minting principal`,
    ),
  );
  const deploymentLiteral = oneLine`dfx deploy ${dfxJsonName} --argument '( 
    record { 
      token_symbol = "${tokenSymbol}";
      token_name =  "${tokenName}";
      minting_account = record { owner = principal"${currentIdentityPrincipal}" };
      transfer_fee = 10_000;
      metadata = vec {};
      initial_balances = vec { }; 
      archive_options = record {
        num_blocks_to_archive = 2000;
        trigger_threshold = 1000;
        controller_id = principal"${currentIdentityPrincipal}";
      };
    }
  )'`;
  await dfxRaw(deploymentLiteral);
};

// Transfers funds to each token's address creator subaccount of the nnsfundedsecp256k1Identity.
// This identity is used in E2E testing as a balance holder to source funds; the addresses 
// of these subaccounts are the same as returned from the nnsfundedsecp256k1Identity calling 
// the invoice's `get_caller_address` method.
const disburse_funds_to_nnsFundedSecp256k1Identity_creator_subaccounts = async () => {
  console.info(
    chalk.cyan(`disbursing funds to principal subaccounts of invoice canister for E2E testing`),
  );
  // Also note that as the two ICP ledger canisters (as well as the two ICRC1 token-ledger canisters) 
  // share the same addressing computations so the same account identifier/account is used in either case.

  // First the `dfx nns` installed ICP ledger (`#ICP_nns` in invoice canister Motoko code).
  const principalSubaccount =
    constants.nnsFundedSecp256k1Identity.creatorSubaccounts.icpAccountIdentifierLiteral;
  await dfxRaw(`dfx ledger transfer ${principalSubaccount} --memo 123 --e8s 100000000000000`);

  // For the following three transfers, since identity is also minting account no fee.

  // Next ICP ledger installed by downloaded wasm and did (`#ICP` in invoice canister Motoko code).
  let transferLiteral = oneLine`dfx canister call ${constants.icpLedgerCanisterDfxJsonName} transfer '(
    record { 
      memo = 1;
      fee = record { 
        e8s = 0 
      };
      amount = record { 
        e8s = 100000000000000 
      };
      to = blob "${constants.nnsFundedSecp256k1Identity.creatorSubaccounts.icpAccountIdentifierBlobLiteral}"
    }
  )'`;
  await dfxRaw(transferLiteral);

  // Next first ICRC1 token-ledger canister (`#ICRC1_ExampleToken1` in invoice canister's Motoko code).
  transferLiteral = oneLine`dfx canister call ${constants.icrc1ExampleToken.dfxJsonName} icrc1_transfer '(
    record { 
        to = record {  
          owner = principal"${constants.invoiceCanisterId}";  
          subaccount = opt blob"${constants.nnsFundedSecp256k1Identity.creatorSubaccounts.icrc1SubaccountBlobLiteral}"; 
        };
        amount = 100000000000;
      }
  )'`;
  await dfxRaw(transferLiteral);

  // Next second ICRC1 token-ledger canister (`#ICRC1_ExampleToken2` in invoice's canister Motoko code).
  transferLiteral = oneLine`dfx canister call ${constants.icrc1ExampleToken2.dfxJsonName} icrc1_transfer '(
    record { 
        to = record {  
          owner = principal"${constants.invoiceCanisterId}";  
          subaccount = opt blob"${constants.nnsFundedSecp256k1Identity.creatorSubaccounts.icrc1SubaccountBlobLiteral}"; 
        };
        amount = 100000000000;
      }
  )'`;
  await dfxRaw(transferLiteral);
};


// As subaccount addresses are dependent on the value of caller's principal, E2E will not 
// work if there was a change in how the @dfinity/identity computes its principals.
const first_e2e_precheck = ({ testing, currentIdentityPrincipal }) => {
  if (testing) {
    if (currentIdentityPrincipal !== constants.nnsFundedSecp256k1Identity.principalLiteral) {
      throw new Error(
        "Mismatch between expected principal and current secp256k1Identity's principal\n"
        + "indicates breaking change for E2E tests. Aborting...",
      );
    }
  }
}

// As subaccount addresses are dependent on the value of invoice canister's id, E2E will not 
// work if the deployed invoice canister id has 'somehow' changed from its expected value.
const second_e2e_precheck = async ({ testing }) => {
  if (testing) {
    const invoiceCanisterId = `${await $`dfx canister id invoice`}`.trim();
    if (invoiceCanisterId !== constants.invoiceCanisterId) {
      throw new Error(
        'Mismatch between expected canister id and current invoice canister id\n'
        +'indicates breaking change for E2E tests. Aborting...',
      );
    }
  }
}

// Prints out formatted into console canister id info.
const printCanistersInfo = async () => {
  const dfxnnsIcpLedger = `ryjl3-tyaaa-aaaaa-aaaba-cai`;
  const invoiceCanister = `${await $`dfx canister id invoice`}`.trim();
  const dldWasmIcpLedger =
    `${await $`dfx canister id ${constants.icpLedgerCanisterDfxJsonName}`}`.trim();
  const icrc1CanisterEx1 =
    `${await $`dfx canister id ${constants.icrc1ExampleToken.dfxJsonName}`}`.trim();
  const icrc1CanisterEx2 =
    `${await $`dfx canister id ${constants.icrc1ExampleToken2.dfxJsonName}`}`.trim();
  console.info(chalk.green('\ndeployed canister ids:'));
  console.info(chalk.green(`  ⊱ invoice                         ${invoiceCanister}`));
  console.info(
    chalk.green(`  ⊱ nns-ledger                      ${dfxnnsIcpLedger} (#ICP_nns)`),
  );
  console.info(
    chalk.green(
      `  ⊱ ${constants.icpLedgerCanisterDfxJsonName}             ${dldWasmIcpLedger} (#ICP)`,
    ),
  );
  console.info(
    chalk.green(
      `  ⊱ ${constants.icrc1ExampleToken.dfxJsonName} ${icrc1CanisterEx1} (#ICRC1_ExampleToken)`,
    ),
  );
  console.info(
    chalk.green(
      `  ⊱ ${constants.icrc1ExampleToken2.dfxJsonName} ${icrc1CanisterEx2} (#ICRC1_ExampleToken2)`,
    ),
  );
};

const run = async (testing = false) => {
  console.info(
    chalk.cyan(
      `spinning up a local replica for${testing ? ' testing ' : ''}the invoice canister...`,
    ),
  );
  await restart_dfx_clean_in_background();

  if (!(await check_correct_system_network_config())) {
    console.info(
      chalk.red(
        `system wide networks.json does not exist or is not configured for running dfx nns installed canisters\nAborting...`,
      ),
    );
    return;
  }
  
  if (testing) {
    // Add of need and switch to the identity `dfx nns install` 
    // initially sends one hundred quadrillion ICP e8s to.
    await use_nnsFundedSecp256k1Identity_and_add_if_needed();
  }
  
  const currentIdentityPrincipal = `${await $`dfx identity get-principal`}`.trim();
  first_e2e_precheck({ testing, currentIdentityPrincipal });
  
  console.info(chalk.cyan(`running dfx nns install`));
  // `dfx nns install` must be run first after clean dfx start.
  // Deploys token-ledger canister that maps to #ICP_nns.
  await $`dfx nns install`;

  console.info(chalk.cyan(`deploying invoice canister`));
  // To keep invoice canister id consistent, deploy it first after `dfx nns install` is done.
  await $`dfx deploy invoice`;
  await second_e2e_precheck({ testing });

  // Deploys token-ledger canister that maps to #ICP.
  await deploy_icp_ledger_from_downloaded_wasm_and_did(currentIdentityPrincipal);
  // Deploys token-ledger canister that maps to #ICRC1ExampleToken.
  await deploy_icrc1_token_canister(currentIdentityPrincipal, constants.icrc1ExampleToken);
  // Deploys token-ledger canister that maps to #ICRC1ExampleToken2.
  await deploy_icrc1_token_canister(currentIdentityPrincipal, constants.icrc1ExampleToken2);

  if (testing) {
    await disburse_funds_to_nnsFundedSecp256k1Identity_creator_subaccounts();
  }
  if (!$.verbose) {
    // Console output canister ids since zx not being verbose will not display it.
    await printCanistersInfo();
  }
  // All done.
  console.info(chalk.cyan(`\nall canisters deployed and ready to be called...`));
};

// Not using the zx shebang so handle setting verbosity manually.
$.verbose = argv.quiet !== true;

run(argv.deployForTesting === true);
