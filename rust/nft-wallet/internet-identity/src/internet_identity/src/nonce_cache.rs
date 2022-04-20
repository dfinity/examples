//! A cache of recently seen nonces.
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

type Timestamp = u64;

/// Data structure containing the list of recently seen proof of work
/// nonces.
#[derive(Default)]
pub struct NonceCache {
    known_nonces: HashSet<(Timestamp, u64)>,
    expiration_queue: BinaryHeap<(Reverse<Timestamp>, u64)>,
}

impl NonceCache {
    /// Adds the specified combination of timestamp and nonce to this
    /// cache.
    pub fn add(&mut self, ts: Timestamp, nonce: u64) {
        if self.known_nonces.insert((ts, nonce)) {
            self.expiration_queue.push((Reverse(ts), nonce));
        }
    }

    /// Prunes all the entries older from the specified expiry
    /// timestamp from the cache.
    pub fn prune_expired(&mut self, expiry: Timestamp) {
        while let Some((Reverse(ts), nonce)) = self.expiration_queue.peek() {
            if *ts <= expiry {
                let entry = (*ts, *nonce);
                self.expiration_queue.pop();
                self.known_nonces.remove(&entry);
            } else {
                return;
            }
        }
    }

    /// Returns true if the specified combination of timestamp and
    /// nonce is in this cache.
    pub fn contains(&self, ts: Timestamp, nonce: u64) -> bool {
        self.known_nonces.contains(&(ts, nonce))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_prune_expired_entries() {
        let mut cache = NonceCache::default();
        cache.add(10, 0);
        cache.add(15, 1);
        cache.add(20, 2);

        assert!(cache.contains(10, 0));
        assert!(cache.contains(15, 1));
        assert!(cache.contains(20, 2));
        assert!(!cache.contains(15, 0));

        cache.prune_expired(15);

        assert!(!cache.contains(10, 0));
        assert!(!cache.contains(15, 1));
        assert!(cache.contains(20, 2));
    }
}
