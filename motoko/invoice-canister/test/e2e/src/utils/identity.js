//const Principal = require("@dfinity/principal");
import { Principal } from "@dfinity/principal";

const Identity = require("@dfinity/identity");
const { Secp256k1KeyIdentity, Ed25519KeyIdentity } = Identity;

const sha256 = require("sha256");
const fs = require("fs");
const Path = require("path");

const localCanisterIds = require("../../../../.dfx/local/canister_ids.json");
const canisterId = localCanisterIds.invoice.local;
const host = "http://127.0.0.1:8080";

const fetch = require("isomorphic-fetch");

const declarations = require("../declarations/invoice");
const { createActor } = declarations;

const parseIdentity = (keyPath) => {
  const rawKey = fs
    .readFileSync(Path.join(__dirname, keyPath))
    .toString()
    .replace("-----BEGIN EC PRIVATE KEY-----", "")
    .replace("-----END EC PRIVATE KEY-----", "")
    .trim();
  const rawBuffer = Uint8Array.from(rawKey).buffer;
  const privKey = Uint8Array.from(sha256(rawBuffer, { asBytes: true }));
  // Initialize an identity from the secret key
  return Secp256k1KeyIdentity.fromSecretKey(Uint8Array.from(privKey).buffer);
};

const defaultIdentity = parseIdentity("test-ec-secp256k1-priv-key.pem");
// Account that will receive a large balance of ICP for testing from install.sh
const balanceHolderIdentity = parseIdentity(
  "test-ec-secp256k1-priv-key-balanceholder.pem"
);


const getNNSLedgerInitializedFundedEd25519KeyIdentity = () => {
  // should have funds initialized in the nns-ledger
  // has principal: jg6qm-uw64t-m6ppo-oluwn-ogr5j-dc5pm-lgy2p-eh6px-hebcd-5v73i-nqe
  // has invoice default subaccount accountId: e157a3ffdd20d2551634a4bb42feb948b353221da411c191b62085eea314b4ee
  // and has ICP default subaccount accountId: 5b315d2f6702cb3a27d826161797d7b2c2e131cd312aece51d4d5574d1247087
  const publicKey = "Uu8wv55BKmk9ZErr6OIt5XR1kpEGXcOSOC1OYzrAwuk=";
  const privateKey ="N3HB8Hh2PrWqhWH2Qqgr1vbU9T3gb1zgdBD8ZOdlQnVS7zC/nkEqaT1kSuvo4i3ldHWSkQZdw5I4LU5jOsDC6Q==";
  const base64ToUInt8Array = (base64String) => Buffer.from(base64String, 'base64');
  return Ed25519KeyIdentity.fromKeyPair(base64ToUInt8Array(publicKey),base64ToUInt8Array(privateKey));
}
const delegatedAdminIdentity = getNNSLedgerInitializedFundedEd25519KeyIdentity();

const getActor = (identity = Secp256k1KeyIdentity.generate()) => {
  return createActor(canisterId, { agentOptions: { identity, fetch, host } })
}

const defaultActor = getActor(defaultIdentity);
const balanceHolder = getActor(balanceHolderIdentity);
const delegatedAdministrator = getActor(delegatedAdminIdentity);
const anonymousActor = getActor(null);
const anonymousPrincipal = Principal.anonymous();

const getRandomActor = () => getActor();
const getActorByPrincipal = (p) => getActor(p);
const getRandomPrincipal = () => Secp256k1KeyIdentity.generate().getPrincipal();

module.exports = {
  defaultActor,
  defaultIdentity,
  balanceHolder,
  balanceHolderIdentity,
  delegatedAdministrator,
  anonymousActor,
  getRandomActor,
  getActorByPrincipal,
  getRandomPrincipal,
  anonymousPrincipal
};