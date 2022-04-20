const CUBEHASH_ROUNDS = 16; // parameter r in the spec.
const HASH_BIT_LENGTH = 256; // parameter h in the spec.
const CUBEHASH_BLOCKBYTES = 32; // parameter b in the spec.

interface HashState {
  hashbitlen: number;
  pos: number /* number of bits read into x from current block */;
  x: Uint32Array;
}

function ROTATE(a: number, b: number) {
  return (a << b) | (a >>> (32 - b));
}

function init(): HashState {
  const state = {
    hashbitlen: HASH_BIT_LENGTH,
    pos: 0,
    x: new Uint32Array(32),
  };

  state.x[0] = HASH_BIT_LENGTH / 8;
  state.x[1] = CUBEHASH_BLOCKBYTES;
  state.x[2] = CUBEHASH_ROUNDS;

  // Apply initial 10 * CUBEHASH_ROUNDS
  for (let i = 0; i < 10; i++) transform(state);

  return state;
}

function transform(state: HashState) {
  const y = new Uint32Array(16);

  let i;
  for (let r = 0; r < CUBEHASH_ROUNDS; ++r) {
    for (i = 0; i < 16; ++i) state.x[i + 16] += state.x[i];
    for (i = 0; i < 16; ++i) y[i ^ 8] = state.x[i];
    for (i = 0; i < 16; ++i) state.x[i] = ROTATE(y[i], 7);
    for (i = 0; i < 16; ++i) state.x[i] ^= state.x[i + 16];
    for (i = 0; i < 16; ++i) y[i ^ 2] = state.x[i + 16];
    for (i = 0; i < 16; ++i) state.x[i + 16] = y[i];
    for (i = 0; i < 16; ++i) state.x[i + 16] += state.x[i];
    for (i = 0; i < 16; ++i) y[i ^ 4] = state.x[i];
    for (i = 0; i < 16; ++i) state.x[i] = ROTATE(y[i], 11);
    for (i = 0; i < 16; ++i) state.x[i] ^= state.x[i + 16];
    for (i = 0; i < 16; ++i) y[i ^ 1] = state.x[i + 16];
    for (i = 0; i < 16; ++i) state.x[i + 16] = y[i];
  }
}

function update(state: HashState, data: Uint8Array) {
  let databitlen = data.length * 8;
  let idx = 0;
  while (databitlen >= 8) {
    let u = data[idx];
    u <<= 8 * (Math.floor(state.pos / 8) % 4);
    const j = Math.floor(state.pos / 32);
    const val = state.x[j];
    state.x[j] = val ^ u;
    idx += 1;
    databitlen -= 8;
    state.pos += 8;
    if (state.pos === 8 * CUBEHASH_BLOCKBYTES) {
      transform(state);
      state.pos = 0;
    }
  }
  if (databitlen > 0) {
    let u = data[idx];
    u <<= 8 * (Math.floor(state.pos / 8) % 4);
    state.x[Math.floor(state.pos / 32)] ^= u;
    state.pos += databitlen;
  }
}

function final(state: HashState): Uint8Array {
  let u = 128 >>> state.pos % 8;
  u <<= 8 * (Math.floor(state.pos / 8) % 4);
  state.x[Math.floor(state.pos / 32)] ^= u;
  transform(state);
  state.x[31] ^= 1;
  for (let i = 0; i < 10; i++) transform(state);
  const hashval = new Uint8Array(state.hashbitlen / 8);
  for (let i = 0; i < state.hashbitlen / 8; ++i) {
    hashval[i] = state.x[Math.floor(i / 4)] >>> (8 * (i % 4));
  }
  return hashval;
}

export default function (data: Uint8Array): Uint8Array {
  const state = init();
  update(state, data);
  return final(state);
}
