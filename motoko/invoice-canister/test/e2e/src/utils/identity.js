import { Secp256k1KeyIdentity } from '@dfinity/identity-secp256k1';
import { Principal } from '@dfinity/principal';
import { createActor } from '../declarations/invoice/index.js';
import fetch from 'isomorphic-fetch';
import pemfile from 'pem-file';
import { aPriori } from './constants';

// This is only valid because the invoice canister id MUST be the 
// same expected value for these tests to work based on how
// the addressing computations are defined. Normally this should be
// imported as an enviromental variable or parsed from ids.json in /.dfx. 
const canisterId = aPriori.invoiceCanister.canisterId.principal.asText;

// Dfx nns install requires this port. 
const host = 'http://127.0.0.1:8080';

// Secp256k1KeyIdentity that the NNS ICP ledger deployed by
// `dfx nns install` is initialized sending funds to; has principal:
//    hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe
// Is used as the invoice canister installer and balance holder in E2E testing.
const getNNSICPLedgerInitializedFundedSecp256k1KeyIdentity = () => {
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

const nnsFundedSecp256k1Identity = getNNSICPLedgerInitializedFundedSecp256k1KeyIdentity();

const getActor = (identity = Secp256k1KeyIdentity.generate()) => {
  return createActor(canisterId, { agentOptions: { identity, fetch, host } });
};

const nnsFundedSecp256k1Actor = getActor(nnsFundedSecp256k1Identity);

const anonymousActor = getActor(null);
const anonymousPrincipal = Principal.anonymous();

const getRandomActor = () => getActor();
const getActorByIdentity = i => getActor(i);
const getRandomIdentity = () => Secp256k1KeyIdentity.generate();

export {
  nnsFundedSecp256k1Actor,
  nnsFundedSecp256k1Identity,
  anonymousActor,
  anonymousPrincipal,
  getRandomActor,
  getActorByIdentity,
  getRandomIdentity,
};
