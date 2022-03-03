import { DBSchema, openDB } from 'idb';

interface KeyStorage extends DBSchema {
  keys: {
    key: string;
    value: CryptoKey;
  };
}

const db = openDB<KeyStorage>('crypto-store', 1, {
  upgrade(db) {
    db.createObjectStore('keys');
  },
});

export async function loadKey(key) {
  return (await db).get('keys', key);
}
export async function storeKey(key, val) {
  return (await db).put('keys', val, key);
}
export async function clearKeys() {
  return (await db).clear('keys');
}
