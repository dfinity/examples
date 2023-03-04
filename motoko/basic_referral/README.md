# Basic referral feature for IC Sustainations DAO

Welcome to our Basic referral feature for Sustainations DAO project and to the internet computer development community.

💚 SUSTAINATIONS is a global private community of change-makers, founders, farmers, and builders who work together to write a greener future for our community and the Earth.

## Prerequisites

Verify the following before running this demo:

* Install DFX 0.12.0 `DFX_VERSION=0.12.0 sh -ci "$(curl -fsSL https://smartcontracts.org/install.sh)"`
* Install vessel: https://github.com/dfinity/vessel

* To run the test scripts, you need to download [ic-repl](https://github.com/chenyan2002/ic-repl/releases).

## Demo

1. Start a local internet computer.

   ```text
   dfx start
   ```

1. Open a new terminal window.

1. Create test identities

   ```text
   $ dfx identity new Alice --disable-encryption; dfx identity use Alice; export ALICE=$(dfx identity get-principal);
   $ dfx identity new Bob --disable-encryption; dfx identity use Bob; export BOB=$(dfx identity get-principal);
   ```

1. Deploy canisters

    ```text
   $ ./scripts/deploy_local.sh
   $ export BASIC_REFERRAL=$(dfx canister id basic_referral);
   ```

1. Run the `ic-repl` test script.

   ```text
   ic-repl scripts/referral.test.sh
   ```
