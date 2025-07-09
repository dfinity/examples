# Counter

This example demonstrates a counter application. It uses an orthogonally persistent counter variable to store an arbitrary precision natural number that represents the current value of the counter.

By using the Motoko keyword stable when declaring the counter variable, the value of this variable will automatically be preserved whenever your canister code is upgraded. Without the stable keyword, a variable is deemed flexible, and its value is reinitialized on every canister upgrade, i.e. whenever new code is deployed to the canister.

The application provides an interface that exposes the following methods:

- `set`: Sets the value of the counter.
- `inc`: Increments the value of the counter.
- `get`: Gets the value of the counter.

## Deploying from ICP Ninja

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/motoko/counter)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Deploy the project to your local environment:

```
dfx start --background --clean && dfx deploy
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.
