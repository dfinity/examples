import cubeHash from "./cubehash";
import { Principal } from "@dfinity/principal";
import bigUintLE from "biguintle";
import {
  ProofOfWork,
  Timestamp,
} from "../../generated/internet_identity_types";

const DIFFICULTY = 2; // Number of leading bytes that must equal zero in the hash.
const DOMAIN = "ic-proof-of-work";
const NONCE_OFFSET =
  DOMAIN.length + 1 /* domain + prefix */ + 8; /* timestamp length */

/**
 * Compute a ProofOfWork (PoW).
 *
 * @param timestamp The timestamp at which the PoW is valid.
 * @param canisterId The principal of the II canister to be included in the signature.
 * @returns
 */
export default function (
  timestamp: Timestamp,
  canisterId: Principal
): ProofOfWork {
  console.time("PoW");
  // Start from a random nonce.
  let nonce = BigInt(Math.floor(Math.random() * Number.MAX_SAFE_INTEGER));

  const canisterIdBlob = canisterId.toUint8Array();
  const message = Buffer.concat([
    Buffer.from([DOMAIN.length]),
    Buffer.from(DOMAIN),
    toLeBytes(timestamp),
    toLeBytes(nonce),
    Buffer.from([canisterIdBlob.length]),
    canisterIdBlob,
  ]);

  // Keep incrementing the nonce until we find a hash that checks.
  // eslint-disable-next-line
  while (true) {
    const hash = cubeHash(message);
    if (hashOk(hash)) {
      break;
    }

    // Hash doesn't check. Increment nonce and update the message.
    nonce += BigInt(1);
    const nonce_encoded = toLeBytes(nonce);
    for (let i = 0; i < nonce_encoded.length; i++) {
      message[NONCE_OFFSET + i] = nonce_encoded[i];
    }
  }

  console.timeEnd("PoW");
  return {
    timestamp: timestamp,
    nonce: nonce,
  };
}

function toLeBytes(num: BigInt): Buffer {
  const b = Buffer.alloc(8);
  bigUintLE.encode(num, b);
  return b;
}

// Returns true if the hash passes the set level of difficulty.
function hashOk(hash: Uint8Array): boolean {
  for (let i = 0; i < DIFFICULTY; i++) {
    if (hash[i] != 0) {
      return false;
    }
  }
  return true;
}
