import type { BackendActor } from './actor';
import { get, set } from 'idb-keyval';
import * as vetkd from "@icp-sdk/vetkeys";

export class CryptoService {
  private keyMaterialCache = new Map<string, vetkd.DerivedKeyMaterial>();

  constructor(private actor: BackendActor) {}

  public async encryptWithNoteKey(note_id: bigint, owner: string, data: string): Promise<string> {
    const keyMaterial = await this.fetchNoteKeyMaterial(note_id, owner);
    const associatedData = buildInput(note_id, owner);
    const encrypted = await keyMaterial.encryptMessage(data, "note-key", associatedData);
    return String.fromCharCode(...encrypted);
  }

  public async decryptWithNoteKey(note_id: bigint, owner: string, data: string): Promise<string> {
    const keyMaterial = await this.fetchNoteKeyMaterial(note_id, owner);
    const associatedData = buildInput(note_id, owner);
    const ciphertext = Uint8Array.from([...data].map(ch => ch.charCodeAt(0)));
    const decrypted = await keyMaterial.decryptMessage(ciphertext, "note-key", associatedData);
    return String.fromCharCode(...decrypted);
  }

  private async fetchNoteKeyMaterial(note_id: bigint, owner: string): Promise<vetkd.DerivedKeyMaterial> {
    const cacheKey = `${note_id}_${owner}`;

    // 1. In-memory cache (fastest, session-scoped)
    const memCached = this.keyMaterialCache.get(cacheKey);
    if (memCached) return memCached;

    // 2. IndexedDB cache (persisted across sessions via getCryptoKey/fromCryptoKey)
    const storedCryptoKey: CryptoKey | undefined = await get([note_id.toString(), owner]);
    if (storedCryptoKey) {
      const keyMaterial = await vetkd.DerivedKeyMaterial.fromCryptoKey(storedCryptoKey);
      this.keyMaterialCache.set(cacheKey, keyMaterial);
      return keyMaterial;
    }

    // 3. Fetch from canister (first access)
    const tsk = vetkd.TransportSecretKey.random();
    const ek_bytes_hex = await this.actor.encrypted_symmetric_key_for_note(note_id, tsk.publicKeyBytes());
    const encryptedVetKey = vetkd.EncryptedVetKey.deserialize(hex_decode(ek_bytes_hex));
    const pk_bytes_hex = await this.actor.symmetric_key_verification_key_for_note();
    const dpk = vetkd.DerivedPublicKey.deserialize(hex_decode(pk_bytes_hex));
    const input = buildInput(note_id, owner);
    const vetKey = encryptedVetKey.decryptAndVerify(tsk, dpk, input);
    const keyMaterial = await vetKey.asDerivedKeyMaterial();

    // Store the underlying non-extractable CryptoKey in IndexedDB (same pattern as EncryptedMaps)
    await set([note_id.toString(), owner], keyMaterial.getCryptoKey());
    this.keyMaterialCache.set(cacheKey, keyMaterial);
    return keyMaterial;
  }
}

function buildInput(note_id: bigint, owner: string): Uint8Array {
  const note_id_bytes = bigintTo128BitBigEndianUint8Array(note_id);
  const owner_utf8 = new TextEncoder().encode(owner);
  const input = new Uint8Array(note_id_bytes.length + owner_utf8.length);
  input.set(note_id_bytes);
  input.set(owner_utf8, note_id_bytes.length);
  return input;
}

const hex_decode = (hexString: string) =>
  Uint8Array.from(hexString.match(/.{1,2}/g)!.map((byte) => parseInt(byte, 16)));

function bigintTo128BitBigEndianUint8Array(bn: bigint): Uint8Array {
  var hex = BigInt(bn).toString(16);
  while (hex.length < 32) { hex = '0' + hex; }
  var len = hex.length / 2;
  var u8 = new Uint8Array(len);
  for (var i = 0, j = 0; i < len; i++, j += 2) {
    u8[i] = parseInt(hex.slice(j, j + 2), 16);
  }
  return u8;
}
