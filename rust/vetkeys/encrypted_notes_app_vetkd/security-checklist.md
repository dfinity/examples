# Security checklist (Rust)

These notes summarize how this example's **Rust** backend addresses the security considerations most relevant to a vetKeys app. They follow the broader [IC security best practices](https://docs.internetcomputer.org/guides/security/overview/), which are more exhaustive — a production app should still perform its own review.

## Authentication

- Every note operation requires authentication and rejects the anonymous principal (the `caller()` helper traps for the anonymous principal).

## Consensus

- The public API has no `query` methods; `get_notes` is an update call, so results go through consensus and cannot be forged by a single malicious node.

## Input validation

- Public methods validate their arguments (per-user note limits, note-size limits, authorization checks) and trap on invalid input.

## End-to-end encryption (frontend)

- Notes are encrypted in the browser; the canister only ever stores ciphertext.
- Encryption uses a fresh random IV per message (no deterministic encryption).
- Derived keys are stored as non-extractable `CryptoKey`s in IndexedDB.
- *Future:* shorten the Internet Identity delegation lifetime for a security-sensitive app, and rotate encryption keys periodically.

## vetKD and inter-canister calls

- The backend calls only the trusted vetKD **management canister** (`aaaaa-aa`) to derive keys, and does not mutate shared state after an `await`, avoiding reentrancy hazards.

## Canister storage and upgrades

- State is stored directly in stable memory via `ic-stable-structures` (`StableBTreeMap`), so it is upgrade-safe by construction (no serialization step that could time out on large data).
- Only encrypted data is stored on the canister.
- Per-user limits bound how much any caller can store (max notes, note size, shares).

## Rust-specific

- No `unsafe` code.
- Arithmetic is written to avoid integer overflow.

## Asset certification

- The frontend is served by the asset canister with certified HTTP responses.
