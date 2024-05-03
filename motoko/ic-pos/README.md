---
keywords: [advanced, motoko, bitcoin, pos, point of sale, ckbtc]
---

# IC-POS

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/ic-pos)

![](./media/header.png)


IC-POS is an experimental app to demonstrate a real-world use case for [ckBTC](https://internetcomputer.org/ckbtc/) on the Internet Computer. It is a simple Point of Sale app that allows users to accept ckBTC payments.

The Internet Computer [integrates directly with the Bitcoin network](https://internetcomputer.org/docs/current/developer-docs/integrations/bitcoin/). This allows canisters on the Internet Computer to receive, hold, and send Bitcoin, all directly with transactions on the Bitcoin network. Chain-key Bitcoin (ckBTC) is an ICRC-1-compliant token that is backed 1:1 by Bitcoin and held 100% on the ICP mainnet.

For deeper understanding of the ICP < > BTC integration, see the IC wiki article on [Bitcoin integration](https://wiki.internetcomputer.org/wiki/Bitcoin_Integration).

## Features

- **Create store**: Users log in with their Internet Identity and configure the store with a name and other settings.
- **Charge customer**: Users can charge a customer by entering an amount. This will generate and display a QR code for the customer to scan and pay. QR code follows the [ICRC-22](https://github.com/dfinity/ICRC/issues/22) standard.
- **Send tokens**: Users can send ckBTC tokens to other users.
- **Receive notifications**: Users can choose to receive notifications by email or SMS when a payment is received. This uses the [HTTP Outcall](https://internetcomputer.org/docs/current/developer-docs/integrations/https-outcalls/) feature of the Internet Computer.
- **Transaction history**: Users can view a list of transactions made to the store.

## Try it!

IC-POS is deployed on the Internet Computer. You can try it out here:

https://hngac-6aaaa-aaaal-qb6tq-cai.icp0.io/

https://github.com/kristoferlund/ic-pos/assets/9698363/f67d9952-3ee1-4876-a5e5-6ed0e29bae9d

## Architecture

### Backend

The backend is written in Motoko and consists of one canister, `icpos`. It exposes four public methods:

- `getMerchant` - returns the store configuration for a given principal.
- `updateMerchant` - updates the store configuration for a given principal.
- `setCourierApiKey` - sets the Courier API key used to send email and SMS notifications. Only the canister controller can call this method.
- `setLedgerId` - sets the ledger ID to monitor for transactions. Only the canister controller can call this method.
- `getLogs` - The canister maintains a debug log that can be fetched using this method.

In addition to the public methods, the canister uses a [timer](https://internetcomputer.org/docs/current/motoko/main/timers/) to monitor ledger transactions. When a new transaction is found that matches a configured store – depending on the store settings – the canister will send a notification either by email or SMS.

### Frontend

The frontend is written in Typescript/Vite/React/TailwindCSS. Users authenticate using the Internet Identity to access their store. The first time a user logs in, a store is created for them.

The frontend interacts with the following IC canisters:

- `icpos` - to fetch and update store configuration.
- `ckbtc ledger` - to send ckBTC to other users.
- `ckbtc index` - to fetch transaction history.
- `internet identity` - to authenticate users.

# Local deployment

## Prerequisites

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Install [Node.js](https://nodejs.org/en/).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

### Step 1: Start a local instance of the replica:

```bash
dfx start --clean --background
```

### Step 2: Deploy the Internet Identity canister:

Integration with the [Internet Identity](https://internetcomputer.org/internet-identity/) allows store owners to securely setup and manage their store. The Internet Identity canister is already deployed on the IC mainnet. For local development, you need to deploy it to your local instance of the IC.

```bash
dfx deploy --network local internet_identity
```

### Step 3: Save the current principal as a variable:

The principal will be used when deploying the ledger canister.

```bash
export OWNER=$(dfx identity get-principal)
```

### Step 3: Deploy the ckBTC ledger canister:

The responsibilities of the ledger canister is to keep track of token balances and handle token transfers.

The ckBTC ledger canister is already deployed on the IC mainnet. ckBTC implements the [ICRC-1](https://internetcomputer.org/docs/current/developer-docs/integrations/icrc-1/) token standard. For local development, we deploy the ledger for an ICRC-1 token mimicking the mainnet setup.

Take a moment to read the details of the call we are making below. Not only are we deploying the ledger canister, we are also:

- Deploying the canister to the same canister ID as the mainnet ledger canister. This is to make it easier to switch between local and mainnet deployments.
- Naming the token `Local ckBTC / LCKBTC`
- Setting the owner principal to the principal we saved in the previous step.
- Minting 100_000_000_000 tokens to the owner principal.
- Setting the transfer fee to 10 LCKBTC.

```bash
dfx deploy --network local --specified-id mxzaz-hqaaa-aaaar-qaada-cai icrc1_ledger --argument '
  (variant {
    Init = record {
      token_name = "Local ckBTC";
      token_symbol = "LCKBTC";
      minting_account = record {
        owner = principal "'${OWNER}'";
      };
      initial_balances = vec {
        record {
          record {
            owner = principal "'${OWNER}'";
          };
          100_000_000_000;
        };
      };
      metadata = vec {};
      transfer_fee = 10;
      archive_options = record {
        trigger_threshold = 2000;
        num_blocks_to_archive = 1000;
        controller_id = principal "'${OWNER}'";
      }
    }
  })
'
```

### Step 4: Deploy the index canister:

The index canister syncs the ledger transactions and indexes them by account.

```bash
dfx deploy --network local icrc1_index --argument '
  record {
   ledger_id = (principal "mxzaz-hqaaa-aaaar-qaada-cai");
  }
'
```

### Step 5: Deploy the icpos canister:

The icpos canister manages the store configuration and sends notifications when a payment is received.

The `--argument '(0)'` argument is used to initialize the canister with `startBlock` set to 0. This is used to tell the canister to start monitoring the ledger from block 0. When deploying to the IC mainnet, this should be set to the current block height to prevent the canister from processing old transactions.

```bash
dfx deploy --network local icpos --argument '(0)'
```

### Step 6: Configure the icpos canister:

ic-pos uses [Courier](https://courier.com/) to send email and SMS notifications. If you want to enable notifications, you need to sign up for a Courier account and and create and API key. Then issue the following command:

```bash
dfx canister --network local call icpos setCourierApiKey "pk_prod_..."
```

### Step 7: Build and run the frontend:

Run npm to install dependencies and start the frontend. The frontend will be available at http://localhost:5173.

```bash
npm install
npm run dev
```

Why don't we deploy the frontend as a local canister? Vite uses lazy loading of modules. This does not work when deploying to a local canister. When deploying to the IC mainnet, this is not an issue. Also, running using `npm run dev` allows for hot reloading of the frontend code when making changes.

### Step 8: Make a transfer!

Now that everything is up and running, you can make a transfer to your newly created store.

Transfers made from the owner principal will not trigger notifications in the UI since they are regarded as `mint` transactions. To test notifications, you need to make a transfer from another principal.

The easiest way to do this is to create two stores using two different Internet Identity accounts, using two different web browsers. Then, transfer some tokens from one store to the other.

#### 8.1: Create the first store and supply it with some tokens:

Log in to the frontend using the Internet Identity. Configure the store and navigate to the `Receive` page. Click on the principal pill to copy the address to your clipboard. Then, using the `dfx` command, mint some tokens from your owner principal to the store principal.

```bash
dfx canister --network local call icrc1_ledger icrc1_transfer '
  (record {
    to=(record {
      owner=(principal "[STORE PRINCIPAL 1 HERE]")
    });
    amount=100_000
  })
'
```

#### 8.2: Create the second store:

Log in to the frontend using **a new Internet Identity on another web browser**. Configure the store and navigate to the `Receive` page. Click on the principal pill to copy the address to your clipboard.

Now, go back to the first browser/store, navigate to the `Send` page, and transfer some tokens to the second store.

If everything is working, you should see a notification in the second store.

🎉

## Possible improvements

- Login state is not persisted. Reloading the app will log the user out. This should be done using `localStorage` or `sessionStorage`.
- Show more information about transactions. A transaction detail page.
- Show a confirmation dialog after the user clicks on the `Send` button.

## Known issues

-

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Author

- [kristofer@fmckl.se](mailto:kristofer@fmckl.se)
- Twitter: [@kristoferlund](https://twitter.com/kristoferlund)
- Discord: kristoferkristofer

## License

[MIT](https://github.com/dfinity/examples/blob/master/motoko/ic-pos/LICENSE)
