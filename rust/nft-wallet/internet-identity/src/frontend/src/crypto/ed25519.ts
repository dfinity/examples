import { Ed25519KeyIdentity } from "@dfinity/identity";
import { mnemonicToSeedSync } from "bip39";

// A constant used for xor-ing derived paths to make them hardened.
const HARDENED = 0x80000000;

/**
 * Create an Ed25519 according to SLIP 0010:
 * https://github.com/satoshilabs/slips/blob/master/slip-0010.md
 *
 * The derivation path is an array that is always interpreted as a hardened path.
 * e.g. to generate m/44'/223’/0’/0’/0' the derivation path should be [44, 223, 0, 0, 0]
 */
export async function fromSeedWithSlip0010(
  masterSeed: Uint8Array,
  derivationPath: number[] = []
): Promise<Ed25519KeyIdentity> {
  let [slipSeed, chainCode] = await generateMasterKey(masterSeed);

  for (let i = 0; i < derivationPath.length; i++) {
    [slipSeed, chainCode] = await derive(
      slipSeed,
      chainCode,
      derivationPath[i] | HARDENED
    );
  }

  return Ed25519KeyIdentity.generate(slipSeed);
}

/**
 * Create an Ed25519 based on a mnemonic phrase according to SLIP 0010:
 * https://github.com/satoshilabs/slips/blob/master/slip-0010.md
 *
 * NOTE: This method derives an identity even if the mnemonic is invalid. It's
 * the responsibility of the caller to validate the mnemonic before calling this method.
 *
 * @param mnemonic A BIP-39 mnemonic.
 * @param derivationPath an array that is always interpreted as a hardened path.
 * e.g. to generate m/44'/223’/0’/0’/0' the derivation path should be [44, 223, 0, 0, 0]
 * @param skipValidation if true, validation checks on the mnemonics are skipped.
 */
export async function fromMnemonicWithoutValidation(
  mnemonic: string,
  derivationPath: number[] = []
): Promise<Ed25519KeyIdentity> {
  const seed = mnemonicToSeedSync(mnemonic);
  return fromSeedWithSlip0010(seed, derivationPath);
}

async function generateMasterKey(
  seed: Uint8Array
): Promise<[Uint8Array, Uint8Array]> {
  const data = new TextEncoder().encode("ed25519 seed");
  const key = await window.crypto.subtle.importKey(
    "raw",
    data,
    {
      name: "HMAC",
      hash: { name: "SHA-512" },
    },
    false,
    ["sign"]
  );
  const h = await window.crypto.subtle.sign("HMAC", key, seed);
  const slipSeed = new Uint8Array(h.slice(0, 32));
  const chainCode = new Uint8Array(h.slice(32));
  return [slipSeed, chainCode];
}

async function derive(
  parentKey: Uint8Array,
  parentChaincode: Uint8Array,
  i: number
): Promise<[Uint8Array, Uint8Array]> {
  // From the spec: Data = 0x00 || ser256(kpar) || ser32(i)
  const data = new Uint8Array([0, ...parentKey, ...toBigEndianArray(i)]);
  const key = await window.crypto.subtle.importKey(
    "raw",
    parentChaincode,
    {
      name: "HMAC",
      hash: { name: "SHA-512" },
    },
    false,
    ["sign"]
  );

  const h = await window.crypto.subtle.sign("HMAC", key, data.buffer);
  const slipSeed = new Uint8Array(h.slice(0, 32));
  const chainCode = new Uint8Array(h.slice(32));
  return [slipSeed, chainCode];
}

// Converts a 32-bit unsigned integer to a big endian byte array.
function toBigEndianArray(n: number): Uint8Array {
  const byteArray = new Uint8Array([0, 0, 0, 0]);
  for (let i = byteArray.length - 1; i >= 0; i--) {
    const byte = n & 0xff;
    byteArray[i] = byte;
    n = (n - byte) / 256;
  }
  return byteArray;
}
