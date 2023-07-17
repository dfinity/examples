# send_http_post

The purpose of this dapp is to give developers a minimal dapp that uses the IC's HTTPS outcalls feature to make a `POST` request.

This demo goes in hand with the [developer documentation on HTTPS outcalls](https://internetcomputer.org/docs/current/developer-docs/integrations/https-outcalls/https-outcalls-post).

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd send_http_post_motoko
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```
