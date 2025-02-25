//! A demo of a very bare-bones bitcoin "wallet".
//!
//! The wallet here showcases how bitcoin addresses can be be computed
//! and how bitcoin transactions can be signed. It is missing several
//! pieces that any production-grade wallet would have, including:
//!
//! * Support for address types that aren't P2PKH, P2TR script spend, or P2TR
//!   key spend with *untweaked* key.
//! * Caching spent UTXOs so that they are not reused in future transactions.
//! * Option to set the fee.

mod common;
pub mod p2pkh;
pub mod p2tr;
pub mod p2tr_key_only;
