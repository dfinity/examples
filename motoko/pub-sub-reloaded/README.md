# PubSub Reloaded

This project enhances the original [PubSub example](link-to-original) to provide a clearer demonstration of inter-canister calls on the Internet Computer, specifically showing how functions can be passed as arguments between canisters. While maintaining the simplicity of the original design, this version improves the architecture by:

1. Clearly defining the three key roles in a pub/sub system:

   - Publisher: manages subscriptions and broadcasts messages
   - Subscribers: receive and process messages for their topics of interest
   - Content Creator: generates the content to be published (previously implicit in the original design)

2. Implementing a more intuitive message type: replacing the `Counter` type with a `NewsMessage` type that better represents a real-world pub/sub scenario

3. Supporting multiple subscribers out of the box, with a pre-configured setup that demonstrates how multiple subscribers can receive updates for the same topics

The example try to maintain the original's simplicity while providing a more practical and comprehensive demonstration of pub/sub principles.

## Prerequisites

This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

Begin by opening a terminal window.

## Step 1: Setup the project environment

Navigate into the folder containing the project's files and start a local instance of the Internet Computer with the commands:

```bash
cd examples/motoko/pub-sub
dfx start --background
```

## Step 2: Deploy the canisters:

```bash
dfx deploy
```

## Step 3: Subscribe to the "Apples" topic

```bash
dfx canister call sub init '("Apples")'
```

## Step 4: Publish to the "Apples" topic

```bash
dfx canister call pub publish '(record { "topic" = "Apples"; "value" = 2 })'
```

## Step 5: Receive your subscription

```bash
dfx canister call sub getCount
```

The output should resemble the following:

```bash
(2 : nat)
```

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app, since it makes inter-canister calls:

- [Be aware that state may change during inter-canister calls.](https://internetcomputer.org/docs/current/developer-docs/security/security-best-practices/overview)
- [Only make inter-canister calls to trustworthy canisters.](https://internetcomputer.org/docs/current/developer-docs/security/security-best-practices/overview)
- [Don't panic after await and don't lock shared resources across await boundaries.](https://internetcomputer.org/docs/current/developer-docs/security/security-best-practices/overview)
