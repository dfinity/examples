# ICRC-2 Swap

ICRC-2 Swap is simple canister demonstrating how to safely work with ICRC-2
tokens. It handles depositing, swapping, and withdrawing ICRC-2 tokens.

The asynchronous nature of developing on the Internet Computer presents some
unique challenges, which mean the design patterns for inter-canister calls are
different from other synchronous blockchains.

## Features

- **Deposit Tokens**: Users can deposit tokens into the contract to be ready for
  swapping.
- **Swap Tokens**: Users can swap the tokens for each other. This is implemented
  in a very simple naive 1:1 manner. The point is just to demonstrate some
  minimal behavior.
- **Withdraw Tokens**: Users can send withdraw the resulting tokens after
  swapping.

## Try it!

ICRC2-Swap is deployed on the Internet Computer. You can try it out here:

https://???.icp0.io/

# Local deployment

## Prerequisites

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Install [Node.js](https://nodejs.org/en/).

### Step 1: Start a local instance of the Internet Computer

```bash
dfx start --clean --background
```

### Step 2: Deploy two tokens

<!-- TODO: Deploy two ICRC-2 tokens here --!>

### Step 3: Deploy the swap canister

The swap canister accepts deposits, and performs the swap.

```bash
dfx deploy --network local swap --argument '
  record {
   token_a = (principal "mxzaz-hqaaa-aaaar-qaada-cai");
   token_b = (principal "mxzaz-hqaaa-aaaar-qaada-cai");
  }
'
```

### Step 4: Approve & deposit tokens

Before we can swap the tokens, they must be transferred to the swap canister.
With ICRC-2, this is a two-step process. First we approve the transfer:

```bash
dfx canister --network local call token_a icrc2_approve 'record {}'
```

Then we can perform the deposit to transfer the tokens from our wallet to the swap canister:

<!-- TODO: Explain e8s a bit here --!>

```bash
dfx canister --network local call swap deposit 'record {
   token = (principal "mxzaz-hqaaa-aaaar-qaada-cai");
   amount = 100_000_000;
}'
```

### Step 5: Perform a swap

```bash
dfx canister --network local call swap swap 'record {
}'
```

### Step 6: Withdraw tokens

After the swap, our balandes in the swap canister will have been updated, and we
can withdraw our newly received tokens into our wallet.

```bash
dfx canister --network local call swap withdraw 'record {
   token = (principal "mxzaz-hqaaa-aaaar-qaada-cai");
   amount = 100_000_000;
}'
```

### Step 7: Check token balances

```bash
dfx canister --network local call token_a icrc1_balance 'record {
}'
```

If everything is working, you should see a your dfx wallet balances reflected in
the token balances.

🎉

## Possible Improvements

- Keep a history of deposits/withdrawaps/swaps.
- Add a frontend.

## Known issues

- Any DeFi on the Internet Computer is experimental. It is a constantly evolving
  space, with unknown attacks, and should be treated as such.
- Due to the nature of inter-canister messaging on the IC, it is possible for
  malicious token canisters to cause this swap contract to deadlock. It should
  only be used with trusted token canisters.
- Currently, there are no limits on the state size of this canister. This could
  allow malicious users to spam the canister, bloating the size until it runs
  out of space. However, the only way to increase the size is to call `deposit`,
  which would cost tokens. For a real canister, you should calculate the maximum
  size of your canister, limit it to a reasonable amount, and monitor the
  current size to know when to re-architect.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Author

- [0xAegir@protonmail.com](mailto:0xAegir@protonmail.com)
- Twitter: [@0xAegir](https://twitter.com/0xAegir)

## License

[MIT](LICENSE)
