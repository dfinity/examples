import { Secp256k1KeyIdentity } from "@dfinity/identity-secp256k1";
import { Ed25519KeyIdentity } from '@dfinity/identity';

import pemfile from 'pem-file';
import {
    createActor as createSellerActor,
    canisterId as sellerCanister
} from "../../declarations/seller";
import {
    createActor as createInvoiceActor,
    canisterId as invoiceCanister
} from "../../declarations/invoice";

// These are well known identities used by the `dfx nns` command. 
// Do not use in production. One is the Ed25519KeyIdentity and the other
// is the Secp256k1KeyIdentity. Typically you'd use the Ed25519KeyIdentity
// in node/js environments, but since the Secp256k1KeyIdentity was already
// added in JS for E2E and two identities are needed, they are used here:

// Copied from E2E testing (hence the inline key content)
// Note that `deposit_free_money` does not authenticate the caller, 
// so any identity could be used for the invoice canister's actor.
const nnsFundedSecp256k1Identity = () => {
  return Secp256k1KeyIdentity.fromSecretKey(
    pemfile
      .decode(
        // Included as a literal for relative import convenience.
        // Same as contents of the nnsFundedSecp256k1.pem file.
        `
  -----BEGIN EC PRIVATE KEY-----
  MHQCAQEEICJxApEbuZznKFpV+VKACRK30i6+7u5Z13/DOl18cIC+oAcGBSuBBAAK
  oUQDQgAEPas6Iag4TUx+Uop+3NhE6s3FlayFtbwdhRVjvOar0kPTfE/N8N6btRnd
  74ly5xXEBNSXiENyxhEuzOZrIWMCNQ==
  -----END EC PRIVATE KEY-----
  `.replace(/(\n)\s+/g, '$1'),
    // replace(<regex>) makes template literal multiline to be ok for pemfile.
      ) 
      .slice(7, 39),
  );
};

// Copied from https://github.com/dfinity/sdk/blob/master/docs/cli-reference/dfx-nns.md#examples-1
const base64ToUInt8Array = (base64String) => {
  return Buffer.from(base64String, 'base64')
};

const publicKey = "Uu8wv55BKmk9ZErr6OIt5XR1kpEGXcOSOC1OYzrAwuk=";
const privateKey =
  "N3HB8Hh2PrWqhWH2Qqgr1vbU9T3gb1zgdBD8ZOdlQnVS7zC/nkEqaT1kSuvo4i3ldHWSkQZdw5I4LU5jOsDC6Q==";
const nnsFundedEd25519KeyIdentity = Ed25519KeyIdentity.fromKeyPair(
  base64ToUInt8Array(publicKey),
  base64ToUInt8Array(privateKey)
);

export const sellerActor = createSellerActor(sellerCanister, {
    actorOptions: { nnsFundedEd25519KeyIdentity },
});
export const invoiceActor = createInvoiceActor(invoiceCanister, {
    actorOptions: { nnsFundedSecp256k1Identity },
});

  