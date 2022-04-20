//! Maintains user signatures and expirations.
use ic_certified_map::{leaf_hash, AsHashTree, Hash, HashTree, RbTree};
use std::borrow::Cow;
use std::collections::BinaryHeap;

#[derive(Default)]
struct Unit;

impl AsHashTree for Unit {
    fn root_hash(&self) -> Hash {
        leaf_hash(&b""[..])
    }
    fn as_hash_tree(&self) -> HashTree<'_> {
        HashTree::Leaf(Cow::from(&b""[..]))
    }
}

#[derive(PartialEq, Eq)]
struct SigExpiration {
    expires_at: u64,
    seed_hash: Hash,
    msg_hash: Hash,
}

impl Ord for SigExpiration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // BinaryHeap is a max heap, but we want expired entries
        // first, hence the inversed order.
        other.expires_at.cmp(&self.expires_at)
    }
}

impl PartialOrd for SigExpiration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Default)]
pub struct SignatureMap {
    certified_map: RbTree<Hash, RbTree<Hash, Unit>>,
    expiration_queue: BinaryHeap<SigExpiration>,
}

impl SignatureMap {
    pub fn put(&mut self, seed: Hash, message: Hash, signature_expires_at: u64) {
        if self.certified_map.get(&seed[..]).is_none() {
            let mut submap = RbTree::new();
            submap.insert(message, Unit);
            self.certified_map.insert(seed, submap);
        } else {
            self.certified_map.modify(&seed[..], |submap| {
                submap.insert(message, Unit);
            });
        }
        self.expiration_queue.push(SigExpiration {
            seed_hash: seed,
            msg_hash: message,
            expires_at: signature_expires_at,
        });
    }

    pub fn delete(&mut self, seed: Hash, message: Hash) {
        let mut is_empty = false;
        self.certified_map.modify(&seed[..], |m| {
            m.delete(&message[..]);
            is_empty = m.is_empty();
        });
        if is_empty {
            self.certified_map.delete(&seed[..]);
        }
    }

    pub fn prune_expired(&mut self, now: u64, max_to_prune: usize) -> usize {
        let mut num_pruned = 0;

        for _step in 0..max_to_prune {
            if let Some(expiration) = self.expiration_queue.peek() {
                if expiration.expires_at > now {
                    return num_pruned;
                }
            }
            if let Some(expiration) = self.expiration_queue.pop() {
                self.delete(expiration.seed_hash, expiration.msg_hash);
            }
            num_pruned += 1;
        }

        num_pruned
    }

    pub fn len(&self) -> usize {
        self.expiration_queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.expiration_queue.is_empty()
    }

    pub fn root_hash(&self) -> Hash {
        self.certified_map.root_hash()
    }

    pub fn witness(&self, seed: Hash, message: Hash) -> Option<HashTree<'_>> {
        self.certified_map.get(&seed[..])?.get(&message[..])?;
        let witness = self
            .certified_map
            .nested_witness(&seed[..], |nested| nested.witness(&message[..]));
        Some(witness)
    }
}

#[cfg(test)]
mod test;
