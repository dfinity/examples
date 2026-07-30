# Native app examples

Examples for integrating native applications (mobile, desktop, game engines) with canisters on the Internet Computer. Each example is self-contained: follow its README to deploy the canister side with `icp-cli` and build the native app.

Prerequisites and dev-container setup are covered in the [repository README](../README.md).

## Examples

- [`unity_ii_deeplink`](unity_ii_deeplink/) — authenticate a Unity mobile app through [Internet Identity](https://docs.internetcomputer.org/guides/authentication/internet-identity) using a browser-based bridge canister and deep-link callbacks, then call a canister with the resulting delegation.

## Security considerations and best practices

If you base your application on one of these examples, we recommend you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on the Internet Computer. The examples provided here may not implement all the best practices.
