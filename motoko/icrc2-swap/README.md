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

### Step 2: Create our user accounts

```bash
export OWNER=$(dfx identity get-principal)

dfx identity new user_a
export USER_A=$(dfx identity get-principal --identity user_a)

dfx identity new user_b
export USER_B=$(dfx identity get-principal --identity user_b)
```

### Step 2: Deploy two tokens

Deploy token a:

```bash
dfx deploy --network local token_a --argument '
  (variant {
    Init = record {
      token_name = "Token A";
      token_symbol = "A";
      minting_account = record {
        owner = principal "'${OWNER}'";
      };
      initial_balances = vec {
        record {
          record {
            owner = principal "'${USER_A}'";
          };
          100_000_000_000;
        };
      };
      metadata = vec {};
      transfer_fee = 10_000;
      archive_options = record {
        trigger_threshold = 2000;
        num_blocks_to_archive = 1000;
        controller_id = principal "'${OWNER}'";
      };
      feature_flags = opt record {
        icrc2 = true;
      };
    }
  })
'
```

Deploy token b:

```bash
dfx deploy --network local token_b --argument '
  (variant {
    Init = record {
      token_name = "Token B";
      token_symbol = "B";
      minting_account = record {
        owner = principal "'${OWNER}'";
      };
      initial_balances = vec {
        record {
          record {
            owner = principal "'${USER_B}'";
          };
          100_000_000_000;
        };
      };
      metadata = vec {};
      transfer_fee = 10_000;
      archive_options = record {
        trigger_threshold = 2000;
        num_blocks_to_archive = 1000;
        controller_id = principal "'${OWNER}'";
      };
      feature_flags = opt record {
        icrc2 = true;
      };
    }
  })
'
```
### Step 3: Deploy the swap canister

The swap canister accepts deposits, and performs the swap.

```bash
export TOKEN_A=$(dfx canister id --network local token_a)
export TOKEN_B=$(dfx canister id --network local token_b)

dfx deploy --network local swap --argument '
  record {
    token_a = (principal "'${TOKEN_A}'");
    token_b = (principal "'${TOKEN_B}'");
  }
'

export SWAP=$(dfx canister id --network local swap)
```

### Step 4: Approve & deposit tokens

Before we can swap the tokens, they must be transferred to the swap canister.
With ICRC-2, this is a two-step process. First we approve the transfer:

```bash
# Approve user B to deposit 1.00000000 of token b, and 0.0001 extra for the
# transfer fee
dfx canister --network local call --identity user_a token_a icrc2_approve '
  record {
    amount = 100_010_000;
    spender = record {
      owner = principal "'${SWAP}'";
    };
  }
'

# Approve user B to deposit 1.00000000 of token b, and 0.0001 extra for the
# transfer fee
dfx canister --network local call --identity user_b token_b icrc2_approve '
  record {
    amount = 100_010_000;
    spender = record {
      owner = principal "'${SWAP}'";
    };
  }
'
```

Then we can perform the deposit to transfer the tokens from our wallet to the swap canister:

TODO: Explain e8s a bit here

```bash
# Deposit User A's tokens
dfx canister --network local call --identity user_a swap deposit 'record {
  token = principal "'${TOKEN_A}'";
  from = record {
    owner = principal "'${USER_A}'";
  };
  amount = 100_000_000;
}'

# Deposit User B's tokens
dfx canister --network local call --identity user_b swap deposit 'record {
  token = principal "'${TOKEN_B}'";
  from = record {
    owner = principal "'${USER_B}'";
  };
  amount = 100_000_000;
}'
```

### Step 5: Perform a swap

```bash
dfx canister --network local call swap swap 'record {
  user_a = principal "'${USER_A}'";
  user_b = principal "'${USER_B}'";
}'
```

We can check the deposited balances with:

```bash
dfx canister --network local call swap balances
```

That should show us that now user b holds token a, and user a holds token b in
the swap contract.


### Step 6: Withdraw tokens

After the swap, our balandes in the swap canister will have been updated, and we
can withdraw our newly received tokens into our wallet.

```bash
# Withdraw user a's token b balance (1.00000000), minus the 0.0001 transfer fee
dfx canister --network local call --identity user_a swap withdraw 'record {
  token = principal "'${TOKEN_B}'";
  to = record {
    owner = principal "'${USER_A}'";
  };
  amount = 99_990_000;
}'
```

```bash
# Withdraw user b's token a balance (1.00000000), minus the 0.0001 transfer fee
dfx canister --network local call --identity user_b swap withdraw 'record {
  token = principal "'${TOKEN_A}'";
  to = record {
    owner = principal "'${USER_B}'";
  };
  amount = 99_990_000;
}'
```

### Step 7: Check token balances

```bash
# Check user a's token a balance. They should now have 998.99980000 A
dfx canister --network local call token_a icrc1_balance_of 'record {
  owner = principal "'${USER_A}'";
}'

# Check user b's token a balance, They should now have 0.99990000 A.
dfx canister --network local call token_a icrc1_balance_of 'record {
  owner = principal "'${USER_A}'";
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
