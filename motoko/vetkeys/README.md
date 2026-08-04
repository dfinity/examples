# vetKeys examples (Motoko)

Examples for [vetKeys](https://docs.internetcomputer.org/concepts/vetkeys) — verifiably encrypted threshold key derivation, the Internet Computer's primitive for on-chain encryption. Each example is self-contained: follow its README to deploy it locally with `icp-cli`. Rust counterparts live in [`../../rust/vetkeys/`](../../rust/vetkeys/).

**Start with** [`basic_vetkd`](basic_vetkd/) to understand the raw API, then move to the SDK-based examples.

## Basics

- [`basic_vetkd`](basic_vetkd/) — the raw VetKD management canister API: symmetric key derivation and IBE without any SDK abstraction.
- [`basic_ibe`](basic_ibe/) — secure messaging with Identity-Based Encryption, using Internet Identity principals as encryption keys.
- [`basic_bls_signing`](basic_bls_signing/) — a threshold BLS signing service where users can only sign for their own principal.

## Applications

- [`password_manager`](password_manager/) — a decentralized password manager built on Encrypted Maps, with shareable vaults.
- [`password_manager_with_metadata`](password_manager_with_metadata/) — extends the password manager with unencrypted metadata alongside encrypted passwords.
- [`encrypted_notes_app_vetkd`](encrypted_notes_app_vetkd/) — end-to-end encrypted note-taking with multi-device access and note sharing.

## Security considerations and best practices

If you base your application on one of these examples, we recommend you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on the Internet Computer. The examples provided here may not implement all the best practices.
