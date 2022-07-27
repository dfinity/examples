# Bitcoin Wallet Example Frontend

This project uses Svelte with minimal dependencies.

## Local development

There are several options to run the project locally.

### Develop the frontend and connect to the backend canisters

```
npm run dev
```

This command will use the local `bitcoin_wallet` canister, and will use the local Internet Identity canister for authentication.

### Develop just the frontend without dfx

To run just the Svelte app without depending on any canisters, run:

```
USE_MOCK_API=true USE_PROD_II=true npm run dev
```

This command will use a dummy API wrapper, and will use Internet Identity on the IC for authentication. Use this command if you're only interested in developing the frontend app.
