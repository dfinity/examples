# Motoko examples

Canonical examples for building canisters on the Internet Computer with [Motoko](https://docs.internetcomputer.org/languages/motoko). Each example is self-contained: follow its README to deploy it locally with `icp-cli` and run its tests. Most examples have a Rust counterpart in [`../rust/`](../rust/) implementing the same Candid interface.

**New to ICP?** Start with [`hello_world`](hello_world/) — a full-stack canister with a frontend and the structural template for all examples. Then try [`who_am_i`](who_am_i/) for user authentication with [Internet Identity](https://docs.internetcomputer.org/guides/authentication/internet-identity).

Prerequisites and dev-container setup are covered in the [repository README](../README.md).

## Getting started

- [`hello_world`](hello_world/) — full-stack "Hello, world!" with a Motoko backend and a Vite frontend.
- [`backend_only`](backend_only/) — the minimal canister: a single `greet` query function, no frontend.
- [`who_am_i`](who_am_i/) — sign in with Internet Identity and see the principal your app receives.

## Full-stack applications

- [`daily_planner`](daily_planner/) — monthly calendar with notes and tasks; fetches historic facts via HTTPS outcalls.
- [`superheroes`](superheroes/) — CRUD application with a React frontend.
- [`filevault`](filevault/) — upload, download, and delete files stored per Internet Identity principal.
- [`flying_ninja`](flying_ninja/) — 2D side-scroller game with an on-chain leaderboard.
- [`ic-pos`](ic-pos/) — point-of-sale app accepting ICRC-1 token payments via QR codes.
- [`llm_chatbot`](llm_chatbot/) — chat with a large language model from a canister.

## Tokens and payments

- [`icp_transfer`](icp_transfer/) — hold ICP in a canister and send it to other accounts via the ICP ledger.
- [`icrc2-swap`](icrc2-swap/) — safe inter-canister call patterns for ICRC-2 token swaps.

## Chain fusion and signing

- [`basic_bitcoin`](basic_bitcoin/) — send and receive Bitcoin using threshold ECDSA and Schnorr signatures.
- [`threshold-ecdsa`](threshold-ecdsa/) — a threshold ECDSA signing oracle.
- [`threshold-schnorr`](threshold-schnorr/) — a threshold Schnorr (BIP340/BIP341, Ed25519) signing oracle.
- [`evm_block_explorer`](evm_block_explorer/) — fetch block data from Ethereum and other EVM chains via the EVM RPC canister.

## HTTPS outcalls

- [`send_http_get`](send_http_get/) — make a `GET` request to an external API from a canister.
- [`send_http_post`](send_http_post/) — make a `POST` request to an external API from a canister.

## Inter-canister patterns

- [`parallel_calls`](parallel_calls/) — parallel vs. sequential inter-canister calls.
- [`pub-sub`](pub-sub/) — publisher/subscriber messaging using shared function references as callbacks.
- [`composite_query`](composite_query/) — query functions that call other canisters' queries.
- [`canister_factory`](canister_factory/) — create canisters dynamically via actor classes and the management canister.

## System features

- [`hello_cycles`](hello_cycles/) — the fundamental cycle management operations.
- [`canister_logs`](canister_logs/) — canister logging with `Debug.print` and trap records.
- [`low_wasm_memory`](low_wasm_memory/) — react to low Wasm memory with the `lowmemory` system hook.
- [`query_stats`](query_stats/) — read a canister's own query statistics.
- [`cert-var`](cert-var/) — certified variables: cryptographically verifiable query responses.
- [`random_maze`](random_maze/) — cryptographic randomness from the network's random beacon.

## Encryption with vetKeys

The [`vetkeys/`](vetkeys/) directory contains examples for [vetKeys](https://docs.internetcomputer.org/concepts/vetkeys), from the low-level API to complete applications: [`basic_vetkd`](vetkeys/basic_vetkd/), [`basic_ibe`](vetkeys/basic_ibe/), [`basic_bls_signing`](vetkeys/basic_bls_signing/), [`password_manager`](vetkeys/password_manager/), [`password_manager_with_metadata`](vetkeys/password_manager_with_metadata/), and [`encrypted_notes_app_vetkd`](vetkeys/encrypted_notes_app_vetkd/).

## Security considerations and best practices

If you base your application on one of these examples, we recommend you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on the Internet Computer. The examples provided here may not implement all the best practices.
