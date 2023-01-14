import { Secp256k1KeyIdentity } from "@dfinity/identity";
import {
  createActor as createSellerActor,
  canisterId as sellerCanister,
} from "../../declarations/seller";
import {
  createActor as createInvoiceActor,
  canisterId as invoiceCanister,
} from "../../declarations/invoice";

// Identity from empty seed - do not use for production
const seed = new Uint8Array(32);
for (let i = 0; i < 32; i++) {
  seed[i] = i;
}
export const identity = Secp256k1KeyIdentity.generate(seed);

export const sellerActor = createSellerActor(sellerCanister, {
  actorOptions: { identity },
});
export const invoiceActor = createInvoiceActor(invoiceCanister, {
  actorOptions: { identity },
});
