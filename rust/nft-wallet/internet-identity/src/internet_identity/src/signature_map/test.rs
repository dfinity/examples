use super::*;
use ic_certified_map::Hash;
use sha2::{Digest, Sha256};

fn hash_bytes(value: impl AsRef<[u8]>) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(value.as_ref());
    hasher.finalize().into()
}

fn seed(x: u64) -> Hash {
    hash_bytes(x.to_be_bytes())
}

fn message(x: u64) -> Hash {
    hash_bytes(x.to_le_bytes())
}

#[test]
fn test_signature_lookup() {
    let mut map = SignatureMap::default();
    map.put(seed(1), message(1), 10);
    assert_eq!(
        map.witness(seed(1), message(1))
            .expect("failed to get a witness")
            .reconstruct(),
        map.root_hash()
    );
    assert!(map.witness(seed(1), message(2)).is_none());
    assert!(map.witness(seed(2), message(1)).is_none());

    map.delete(seed(1), message(1));
    assert!(map.witness(seed(1), message(1)).is_none());
}

#[test]
fn test_signature_expiration() {
    let mut map = SignatureMap::default();

    map.put(seed(1), message(1), 10);
    map.put(seed(1), message(2), 20);
    map.put(seed(2), message(1), 15);
    map.put(seed(2), message(2), 25);

    assert_eq!(2, map.prune_expired(/*time now*/ 19, /*max_to_prune*/ 10));
    assert!(map.witness(seed(1), message(1)).is_none());
    assert!(map.witness(seed(2), message(1)).is_none());

    assert!(map.witness(seed(1), message(2)).is_some());
    assert!(map.witness(seed(2), message(2)).is_some());
}

#[test]
fn test_signature_expiration_limit() {
    let mut map = SignatureMap::default();

    for i in 0..10 {
        map.put(seed(i), message(i), 10 * i);
    }

    assert_eq!(5, map.prune_expired(/*time now*/ 100, /*max_to_prune*/ 5));

    for i in 0..5 {
        assert!(map.witness(seed(i), message(i)).is_none());
    }
    for i in 5..10 {
        assert!(map.witness(seed(i), message(i)).is_some());
    }
}

#[test]
fn test_random_modifications() {
    use rand::prelude::*;

    let mut map = SignatureMap::default();
    let mut rng = rand::thread_rng();
    let window_size = 5;

    let mut pairs = Vec::new();

    for round in 1..100 {
        let n_seeds = rng.gen_range(0..5);
        for _i in 0..n_seeds {
            let mut seed_hash = Hash::default();
            rng.fill_bytes(&mut seed_hash);

            let n_messages = rng.gen_range(0..5);
            for _k in 0..n_messages {
                let mut message_hash = Hash::default();
                rng.fill_bytes(&mut message_hash);

                pairs.push((seed_hash.clone(), message_hash.clone()));
                map.put(seed_hash.clone(), message_hash, round);
            }
        }

        map.prune_expired(round.saturating_sub(window_size), 1000);

        for (k, v) in pairs.iter() {
            if let Some(witness) = map.witness(*k, *v) {
                assert_eq!(
                    witness.reconstruct(),
                    map.root_hash(),
                    "produced a bad witness: {:?}",
                    witness
                );
            }
        }
    }
}
