import { Ed25519PublicKey } from "@dfinity/identity";
import * as ed25519 from "./ed25519";
import { fromHexString } from "@dfinity/identity/lib/cjs/buffer";

type TestVector = {
  seed: string;
  privateKey: string;
  publicKey: string;
  derivationPath?: number[];
};

// Test vectors consist of taken from
// https://github.com/satoshilabs/slips/blob/master/slip-0010.md
// The public key vectors contained a leading 0-byte for no obvious reason.
// These were removed.
const testVectorsSLIP10 = [
  {
    seed: "000102030405060708090a0b0c0d0e0f",
    privateKey:
      "2b4be7f19ee27bbf30c667b642d5f4aa69fd169872f8fc3059c08ebae2eb19e7",
    publicKey:
      "a4b2856bfec510abab89753fac1ac0e1112364e7d250545963f135f2a33188ed",
  },
  {
    seed: "000102030405060708090a0b0c0d0e0f",
    privateKey:
      "68e0fe46dfb67e368c75379acec591dad19df3cde26e63b93a8e704f1dade7a3",
    publicKey:
      "8c8a13df77a28f3445213a0f432fde644acaa215fc72dcdf300d5efaa85d350c",
    derivationPath: [0],
  },
  {
    seed: "000102030405060708090a0b0c0d0e0f",
    privateKey:
      "b1d0bad404bf35da785a64ca1ac54b2617211d2777696fbffaf208f746ae84f2",
    publicKey:
      "1932a5270f335bed617d5b935c80aedb1a35bd9fc1e31acafd5372c30f5c1187",
    derivationPath: [0, 1],
  },
  {
    seed: "000102030405060708090a0b0c0d0e0f",
    privateKey:
      "92a5b23c0b8a99e37d07df3fb9966917f5d06e02ddbd909c7e184371463e9fc9",
    publicKey:
      "ae98736566d30ed0e9d2f4486a64bc95740d89c7db33f52121f8ea8f76ff0fc1",
    derivationPath: [0, 1, 2],
  },
  {
    seed: "000102030405060708090a0b0c0d0e0f",
    privateKey:
      "30d1dc7e5fc04c31219ab25a27ae00b50f6fd66622f6e9c913253d6511d1e662",
    publicKey:
      "8abae2d66361c879b900d204ad2cc4984fa2aa344dd7ddc46007329ac76c429c",
    derivationPath: [0, 1, 2, 2],
  },
  {
    seed: "000102030405060708090a0b0c0d0e0f",
    privateKey:
      "8f94d394a8e8fd6b1bc2f3f49f5c47e385281d5c17e65324b0f62483e37e8793",
    publicKey:
      "3c24da049451555d51a7014a37337aa4e12d41e485abccfa46b47dfb2af54b7a",
    derivationPath: [0, 1, 2, 2, 1000000000],
  },
];

test("derive Ed25519 via SLIP 0010", async () => {
  await Promise.all(
    testVectorsSLIP10.map(async (testVector: TestVector, i) => {
      const seedBlob = fromHexString(testVector.seed);
      const expectedPrivateKey = fromHexString(testVector.privateKey);
      const expectedPublicKey = fromHexString(testVector.publicKey);

      let identity = await ed25519.fromSeedWithSlip0010(
        new Uint8Array(seedBlob),
        testVector.derivationPath
      );

      const keyPair = identity.getKeyPair();
      expect(keyPair.secretKey.slice(0, 32)).toEqual(
        new Uint8Array(expectedPrivateKey)
      );
      expect(keyPair.publicKey.toDer()).toEqual(
        Ed25519PublicKey.fromRaw(expectedPublicKey).toDer()
      );
    })
  );
});

test("Can derive identity from invalid mnemonic", async () => {
  await expect(
    ed25519.fromMnemonicWithoutValidation("")
  ).resolves.not.toThrow();
  await expect(
    ed25519.fromMnemonicWithoutValidation("g4rb4g3")
  ).resolves.not.toThrow();
  await expect(
    ed25519.fromMnemonicWithoutValidation("basket actual")
  ).resolves.not.toThrow();
});
