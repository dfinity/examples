# Security checklist (Motoko)

These notes summarize how this example's **Motoko** backend addresses the security considerations most relevant to a vetKeys app. They follow the broader [IC security best practices](https://docs.internetcomputer.org/guides/security/overview/), which are more exhaustive — a production app should still perform its own review.

## Authentication

- Every note operation requires authentication and rejects the anonymous principal (`assert not Principal.isAnonymous(caller)`).

## Consensus

- The public API has no `query` methods; `getNotes` is an update call, so results go through consensus and cannot be forged by a single malicious node.

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

- Only encrypted data is stored on the canister.
- Per-user limits bound how much any caller can store (max notes, note size, shares).
- State is retained across upgrades. *Future:* back large state with stable memory directly and version it for long-term upgradeability.

## Asset certification

- The frontend is served by the asset canister with certified HTTP responses.
