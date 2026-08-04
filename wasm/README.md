# WebAssembly examples

Examples of canisters written directly in WebAssembly, without a higher-level language or CDK. They show what a canister looks like at the lowest level — useful for understanding the [Internet Computer interface specification](https://docs.internetcomputer.org/references/ic-interface-spec) and for building canisters from languages that compile to Wasm.

Prerequisites and dev-container setup are covered in the [repository README](../README.md).

## Examples

- [`counter`](counter/) — a counter canister hand-written in WebAssembly Text Format (WAT); the compiled module is only 389 bytes.

## Security considerations and best practices

If you base your application on one of these examples, we recommend you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on the Internet Computer. The examples provided here may not implement all the best practices.
