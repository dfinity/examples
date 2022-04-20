//! This module is a port of Bernstein's CubeHash1632 "simple"
//! implementation in C:
//! https://github.com/floodyberry/supercop/blob/master/crypto_hash/cubehash1632/simple/cubehash.c
use std::num::Wrapping;

pub const HASH_BYTES: usize = 32;

const CUBEHASH_ROUNDS: u16 = 16;
const CUBEHASH_BLOCKBYTES: u16 = 32;

// CubeHash 160+16/32+160-256

#[inline]
fn rotate(a: Wrapping<u32>, b: usize) -> Wrapping<u32> {
    (a << b) | (a >> (32 - b))
}

fn transform(state: &mut [Wrapping<u32>; 32]) {
    let mut y: [Wrapping<u32>; 16] = [Wrapping(0); 16];

    for _round in 0..CUBEHASH_ROUNDS {
        for i in 0..16 {
            state[i + 16] += state[i];
        }
        for i in 0..16 {
            y[i ^ 8] = state[i];
        }
        for i in 0..16 {
            state[i] = rotate(y[i], 7);
        }
        for i in 0..16 {
            state[i] ^= state[i + 16];
        }
        for i in 0..16 {
            y[i ^ 2] = state[i + 16];
        }
        for i in 0..16 {
            state[i + 16] = y[i];
        }
        for i in 0..16 {
            state[i + 16] += state[i];
        }
        for i in 0..16 {
            y[i ^ 4] = state[i];
        }
        for i in 0..16 {
            state[i] = rotate(y[i], 11);
        }
        for i in 0..16 {
            state[i] ^= state[i + 16];
        }
        for i in 0..16 {
            y[i ^ 1] = state[i + 16];
        }
        for i in 0..16 {
            state[i + 16] = y[i];
        }
    }
}

pub struct CubeHash {
    state: [Wrapping<u32>; 32],
    pos: u16,
}

impl CubeHash {
    /// Constructs a new CubeHash state that produces a hash of the
    /// specified length.
    pub fn new() -> Self {
        let mut state_t = [Wrapping(0); 32];
        state_t[0] = Wrapping(HASH_BYTES as u32);
        state_t[1] = Wrapping(CUBEHASH_BLOCKBYTES as u32);
        state_t[2] = Wrapping(CUBEHASH_ROUNDS as u32);

        for _i in 0..10 {
            transform(&mut state_t);
        }

        let state: [Wrapping<u32>; 32] = [
            Wrapping(0xea2bd4b4),
            Wrapping(0xccd6f29f),
            Wrapping(0x63117e71),
            Wrapping(0x35481eae),
            Wrapping(0x22512d5b),
            Wrapping(0xe5d94e63),
            Wrapping(0x7e624131),
            Wrapping(0xf4cc12be),
            Wrapping(0xc2d0b696),
            Wrapping(0x42af2070),
            Wrapping(0xd0720c35),
            Wrapping(0x3361da8c),
            Wrapping(0x28cceca4),
            Wrapping(0x8ef8ad83),
            Wrapping(0x4680ac00),
            Wrapping(0x40e5fbab),
            Wrapping(0xd89041c3),
            Wrapping(0x6107fbd5),
            Wrapping(0x6c859d41),
            Wrapping(0xf0b26679),
            Wrapping(0x09392549),
            Wrapping(0x5fa25603),
            Wrapping(0x65c892fd),
            Wrapping(0x93cb6285),
            Wrapping(0x2af2b5ae),
            Wrapping(0x9e4b4e60),
            Wrapping(0x774abfdd),
            Wrapping(0x85254725),
            Wrapping(0x15815aeb),
            Wrapping(0x4ab6aad6),
            Wrapping(0x9cdaf8af),
            Wrapping(0xd6032c0a),
        ];
        assert_eq!(state_t, state);
        Self { state, pos: 0 }
    }

    pub fn update(&mut self, data: &[u8]) {
        for b in data.iter() {
            let u = Wrapping(*b as u32) << (8 * (self.pos % 4) as usize);
            self.state[(self.pos / 4) as usize] ^= u;
            self.pos += 1;
            if self.pos == CUBEHASH_BLOCKBYTES {
                transform(&mut self.state);
                self.pos = 0;
            }
        }
    }

    pub fn finalize(mut self) -> [u8; HASH_BYTES] {
        let u = Wrapping(128u32) << (8 * (self.pos % 4) as usize);
        self.state[(self.pos / 4) as usize] ^= u;
        transform(&mut self.state);
        self.state[31] ^= Wrapping(1);
        for _ in 0..10 {
            transform(&mut self.state);
        }

        let mut buf: [u8; HASH_BYTES] = [0; HASH_BYTES];
        for i in 0..HASH_BYTES {
            buf[i] = (self.state[i / 4] >> (8 * (i % 4))).0 as u8;
        }
        buf
    }
}

#[cfg(test)]
mod test;
