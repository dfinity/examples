# Rust examples

Canonical examples for building canisters on the Internet Computer with [Rust](https://docs.internetcomputer.org/languages/rust). Each example is self-contained: follow its README to deploy it locally with `icp-cli` and run its tests. Most examples have a Motoko counterpart in [`../motoko/`](../motoko/) implementing the same Candid interface.

**New to ICP?** Start with [`hello_world`](hello_world/) — a full-stack canister with a frontend and the structural template for all examples. Then try [`who_am_i`](who_am_i/) for user authentication with [Internet Identity](https://docs.internetcomputer.org/guides/authentication/internet-identity).

Prerequisites and dev-container setup are covered in the [repository README](../README.md).

## Getting started

- [`hello_world`](hello_world/) — full-stack "Hello, world!" with a Rust backend and a Vite frontend.
- [`backend_only`](backend_only/) — the minimal canister: a single `greet` query function, no frontend.
- [`backend_wasm64`](backend_wasm64/) — the same minimal canister compiled for the Wasm64 target.
- [`who_am_i`](who_am_i/) — sign in with Internet Identity and see the principal your app receives.

## Full-stack applications

- [`daily_planner`](daily_planner/) — monthly calendar with notes and tasks; fetches historic facts via HTTPS outcalls.
- [`flying_ninja`](flying_ninja/) — 2D side-scroller game with an on-chain leaderboard.
- [`photo_gallery`](photo_gallery/) — upload and view photos served via the HTTP gateway with cacheable responses.
- [`llm_chatbot`](llm_chatbot/) — chat with a large language model from a canister.

## Tokens and payments

- [`icp_transfer`](icp_transfer/) — hold ICP in a canister and send it to other accounts via the ICP ledger.
- [`receiving-icp`](receiving-icp/) — generate account identifiers and check balances to receive ICP.
- [`exchange-rates`](exchange-rates/) — query crypto and fiat exchange rates from the Exchange Rate Canister (XRC).
- [`stake_neuron_from_cli`](stake_neuron_from_cli/) — stake ICP into an NNS neuron from a Rust CLI binary using `ic-agent`.

## Chain fusion and signing

- [`basic_bitcoin`](basic_bitcoin/) — send and receive Bitcoin (P2PKH, P2WPKH, P2TR addresses).
- [`basic_ethereum`](basic_ethereum/) — send and receive Ether using threshold ECDSA.
- [`evm_block_explorer`](evm_block_explorer/) — fetch block data from Ethereum and other EVM chains via the EVM RPC canister.
- [`threshold-ecdsa`](threshold-ecdsa/) — a threshold ECDSA signing oracle.
- [`threshold-schnorr`](threshold-schnorr/) — a threshold Schnorr (BIP340/BIP341, Ed25519) signing oracle.
- [`x509`](x509/) — issue and verify X.509 certificates with threshold signatures.

## HTTPS outcalls

- [`send_http_get`](send_http_get/) — make a `GET` request to an external API from a canister.
- [`send_http_post`](send_http_post/) — make a `POST` request to an external API from a canister.

## Inter-canister patterns

- [`inter-canister-calls`](inter-canister-calls/) — bounded-wait and unbounded-wait calls, retries, and attaching cycles.
- [`parallel_calls`](parallel_calls/) — parallel vs. sequential inter-canister calls.
- [`composite_query`](composite_query/) — query functions that call other canisters' queries.
- [`guards`](guards/) — how guard functions interact with asynchronous code.
- [`candid_type_generation`](candid_type_generation/) — generate Rust types from an external canister's `.did` file.

## Canister operations and system features

- [`canister-info`](canister-info/) — retrieve metadata about any canister via the management canister.
- [`canister-snapshots`](canister-snapshots/) — take and restore canister snapshots.
- [`canister-snapshot-download`](canister-snapshot-download/) — download and upload canister snapshots.
- [`canister_logs`](canister_logs/) — canister logging, traps, panics, and error handling.
- [`low_wasm_memory`](low_wasm_memory/) — react to low Wasm memory with the `on_low_wasm_memory` hook.
- [`periodic_tasks`](periodic_tasks/) — timers and heartbeats for scheduled execution.
- [`performance_counters`](performance_counters/) — measure the work a canister performs.
- [`query_stats`](query_stats/) — read a canister's own query statistics.

## Compute and AI

- [`image-classification`](image-classification/) — run an ONNX machine-learning model inside a canister.
- [`face-recognition`](face-recognition/) — face detection and recognition with the Tract ONNX engine.
- [`qrcode`](qrcode/) — long-running image processing in a single message execution.
- [`simd`](simd/) — WebAssembly SIMD acceleration for compute-heavy workloads.

## Testing

- [`unit_testable_rust_canister`](unit_testable_rust_canister/) — structure a canister for comprehensive unit testing.

## Encryption with vetKeys

The [`vetkeys/`](vetkeys/) directory contains examples for [vetKeys](https://docs.internetcomputer.org/concepts/vetkeys), from the low-level API to complete applications: [`basic_vetkd`](vetkeys/basic_vetkd/), [`basic_ibe`](vetkeys/basic_ibe/), [`basic_timelock_ibe`](vetkeys/basic_timelock_ibe/), [`basic_bls_signing`](vetkeys/basic_bls_signing/), [`password_manager`](vetkeys/password_manager/), [`password_manager_with_metadata`](vetkeys/password_manager_with_metadata/), and [`encrypted_notes_app_vetkd`](vetkeys/encrypted_notes_app_vetkd/).

## Security considerations and best practices

If you base your application on one of these examples, we recommend you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on the Internet Computer. The examples provided here may not implement all the best practices.
