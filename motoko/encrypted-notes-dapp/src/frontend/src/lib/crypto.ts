import Base64 from 'crypto-js/enc-base64';
import AES from 'crypto-js/aes';
import SHA256 from 'crypto-js/sha256';

import { v4 as uuidv4 } from 'uuid';
import type { BackendActor } from './actor';
import { clearKeys, loadKey, storeKey } from './keyStorage';

export class CryptoService {
  constructor(private actor: BackendActor) {
    const deviceAlias = window.localStorage.getItem('deviceAlias');
    if (deviceAlias) {
      this.deviceAlias = deviceAlias;
    } else {
      this.deviceAlias = uuidv4();
      window.localStorage.setItem('deviceAlias', this.deviceAlias);
    }
    console.log('deviceAlias: ' + this.deviceAlias);
  }

  // Symmetric AES key, used to encrypt and decrypt the notes stored in the dapp
  private secretKey: CryptoKey | null = null;
  private secret: string | null = null;
  // Public key associated with logged in device. Used to decrypt the symmetric secretKey,
  // which is stored by the dapp encrypted with the the publicKey
  private privateKey: CryptoKey | null = null;
  // Private key associated with logged in device. Used to encrypt the symmetric secretKey 
  // for each device associated with the current principal, to be stored by the dapp in encrypted form
  private publicKey: CryptoKey | null = null;
  private publicKeyBase64: string | null = null;
  public readonly deviceAlias: string;
  private intervalHandler: number | null = null;

  /**
   * 1. Fetch this browser's public and private key pair. If no keypair exists, one will be generated and stored in localStorage.
   * 2. Register this browser (i.e. "device") with the public key generated in step 1.
   */
  public async init(): Promise<boolean> {
    this.publicKey = await loadKey('public');
    this.privateKey = await loadKey('private');

    if (!this.publicKey || !this.privateKey) {
      await this.initializeKeys();
    }
    const exportedPublicKey = await window.crypto.subtle.exportKey(
      'spki',
      this.publicKey
    );
    this.publicKeyBase64 = window.btoa(CryptoService.ab2str(exportedPublicKey));

    await this.actor.register_device(this.deviceAlias, this.publicKeyBase64);

    // Check if this is this user's very first device
    const isSeeded = await this.actor.is_seeded();
    if (!isSeeded) {
      // This is the first device for this user
      console.log('Not seeded -> generate seed');
      const secret = await this.generateSeed(this.publicKey);
      await this.wrapAndUploadSeed(secret, this.publicKey);
      console.log('seed uploaded');

      if (this.intervalHandler === null) {
        this.intervalHandler = window.setInterval(
          () => this.synchronize(),
          5000
        );
      }

      return true;
    } else {
      // Not the first device
      return await this.syncSeed(this.publicKeyBase64);
    }
  }

  public async pollForSeed() {
    return await this.syncSeed(this.publicKeyBase64);
  }

  public logout() {
    if (this.intervalHandler !== null) {
      window.clearInterval(this.intervalHandler);
      this.intervalHandler = null;
    }
    this.privateKey = null;
    this.publicKey = null;
    this.secretKey = null;
    this.secret = null;
  }

  public async clearDevice() {
    await clearKeys();
    localStorage.removeItem('draft');
    localStorage.removeItem('deviceAlias');
  }

  private async synchronize() {
    console.log('Synchronizing keys...');
    const secretKey = this.secretKey;
    if (secretKey === null) {
      throw new Error('null secret key');
    }

    const keys = await this.actor.get_unsynced_pubkeys();
    const ciphertexts: Array<[string, string]> = [];
    for (const key of keys) {
      const publicKey = await window.crypto.subtle.importKey(
        'spki',
        CryptoService.str2ab(window.atob(key)),
        {
          name: 'RSA-OAEP',
          hash: { name: 'SHA-256' },
        },
        true,
        ['wrapKey']
      );
      const [exportedKey, ciphertext] = await CryptoService.wrapSecret(
        publicKey,
        secretKey
      );
      ciphertexts.push([exportedKey, ciphertext]);
    }

    await this.actor.submit_ciphertexts(ciphertexts);
  }

  private async syncSeed(publicKey: string): Promise<boolean> {
    const maybeWrappedSecret = await this.actor.get_ciphertext(publicKey);
    if ('ok' in maybeWrappedSecret) {
      console.log('existing device && already seeded -> loading seed');
      await this.unwrapSeed(maybeWrappedSecret.ok);
      if (this.intervalHandler === null) {
        this.intervalHandler = window.setInterval(
          () => this.synchronize(),
          5000
        );
      }
      return true;
    }
    return false;
  }

  public isInitialized() {
    return this.secret !== null && this.publicKey !== null;
  }

  private async initializeKeys() {
    console.log('Local store does not exists, generating keys');
    const keypair = await crypto.subtle.generateKey(
      {
        name: 'RSA-OAEP',
        // Consider using a 4096-bit key for systems that require long-term security
        modulusLength: 2048,
        publicExponent: new Uint8Array([1, 0, 1]),
        hash: 'SHA-256',
      },
      false,
      ['encrypt', 'decrypt', 'wrapKey', 'unwrapKey']
    );
    await storeKey('public', keypair.publicKey);
    await storeKey('private', keypair.privateKey);
    this.publicKey = keypair.publicKey;
    this.privateKey = keypair.privateKey;
  }

  private static ab2str(buf: ArrayBuffer) {
    return String.fromCharCode.apply(null, Array.from(new Uint8Array(buf)));
  }

  private static str2ab(str: string) {
    const buf = new ArrayBuffer(str.length);
    const bufView = new Uint8Array(buf);
    for (let i = 0, strLen = str.length; i < strLen; i++) {
      bufView[i] = str.charCodeAt(i);
    }
    return buf;
  }

  private async generateSeed(publicKey: CryptoKey | null): Promise<CryptoKey> {
    if (publicKey === null) {
      throw new Error('null public key');
    }
    const secret: CryptoKey = await window.crypto.subtle.generateKey(
      {
        name: 'AES-GCM',
        length: 256,
      },
      true,
      ['encrypt', 'decrypt']
    );
    this.secretKey = secret;
    // keep for local usage
    const exported_secret = await window.crypto.subtle.exportKey('raw', secret);
    this.secret = window.btoa(CryptoService.ab2str(exported_secret));
    console.log('Shared secret generated');
    return secret;
  }

  private async wrapAndUploadSeed(
    secret: CryptoKey,
    publicKey: CryptoKey | null
  ) {
    const [exported_publicAsBase64, wrappedSecretBase64] =
      await CryptoService.wrapSecret(publicKey, secret);
    await this.actor.seed(exported_publicAsBase64, wrappedSecretBase64);
  }

  private static async wrapSecret(
    publicKey: CryptoKey | null,
    secret: CryptoKey
  ) {
    if (publicKey === null) {
      throw new Error('null public key');
    }

    const exported = await window.crypto.subtle.exportKey('spki', publicKey);
    const exported_publicAsBase64 = window.btoa(CryptoService.ab2str(exported));
    // Wrap key for own pubkey
    const wrapped = await window.crypto.subtle.wrapKey(
      'raw',
      secret,
      publicKey,
      { name: 'RSA-OAEP' }
    );
    const wrappedSecretBase64 = window.btoa(CryptoService.ab2str(wrapped));
    return [exported_publicAsBase64, wrappedSecretBase64];
  }

  private async unwrapSeed(wrappedSecret: string) {
    if (this.privateKey === null) {
      throw new Error('null private key');
    }
    const unwrappedSecret = await window.crypto.subtle.unwrapKey(
      'raw',
      CryptoService.str2ab(window.atob(wrappedSecret)),
      this.privateKey,
      {
        name: 'RSA-OAEP',
      },
      {
        name: 'AES-GCM',
        length: 256,
      },
      true,
      ['encrypt', 'decrypt']
    );
    this.secretKey = unwrappedSecret;
    const buffer = await window.crypto.subtle.exportKey('raw', unwrappedSecret);
    this.secret = window.btoa(CryptoService.ab2str(buffer));
    console.log('Shared secret unwrapped');
  }

  // The function encrypts data with the shared secretKey.
  public async encrypt(data: string) {
    if (this.secretKey === null) {
          throw new Error('null shared secret!');
    }
    const data_encoded = Uint8Array.from([...data].map(ch => ch.charCodeAt(0))).buffer
    // The iv must never be reused with a given key.
    const iv = window.crypto.getRandomValues(new Uint8Array(12));
    const ciphertext = await window.crypto.subtle.encrypt(
                     {
                       name: "AES-GCM",
                       iv: iv
                     },
                     this.secretKey,
                     data_encoded
                   );

    const iv_decoded = String.fromCharCode(...new Uint8Array(iv));
    const cipher_decoded = String.fromCharCode(...new Uint8Array(ciphertext));
    return iv_decoded + cipher_decoded;
  }

  // The function decrypts the given input data.
  public async decrypt(data: string) {
        if (this.secretKey === null) {
            throw new Error('null shared secret!');
        }
        if (data.length < 13) {
            throw new Error('wrong encoding, too short to contain iv');
        }
        const iv_decoded = data.slice(0,12);
        const cipher_decoded = data.slice(12);
        const iv_encoded = Uint8Array.from([...iv_decoded].map(ch => ch.charCodeAt(0))).buffer;
        const ciphertext_encoded = Uint8Array.from([...cipher_decoded].map(ch => ch.charCodeAt(0))).buffer;

        let decrypted_data_encoded = await window.crypto.subtle.decrypt(
                        {
                          name: "AES-GCM",
                          iv: iv_encoded
                        },
                        this.secretKey,
                        ciphertext_encoded
                      );
        const decrypted_data_decoded = String.fromCharCode(...new Uint8Array(decrypted_data_encoded));
        return decrypted_data_decoded;
  }
}
