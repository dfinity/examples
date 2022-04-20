import { entropyToMnemonic, wordlists, validateMnemonic } from "bip39";
import { toHexString } from "@dfinity/identity/lib/cjs/buffer";

/**
 * @returns A random BIP39 mnemonic with 256 bits of entropy.
 */
export function generate(): string {
  const entropy = new Uint32Array(8);
  crypto.getRandomValues(entropy);
  return entropyToMnemonic(toHexString(entropy.buffer), wordlists.english);
}

/**
 * @returns true if the mnemonic is valid, false otherwise.
 */
export function validate(mnemonic: string): boolean {
  return validateMnemonic(mnemonic);
}
