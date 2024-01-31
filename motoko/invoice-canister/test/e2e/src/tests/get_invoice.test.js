import { describe, it, expect } from 'vitest';
import { SupportedTokens } from '../utils/constants.js';
import {
  nnsFundedSecp256k1Actor as invoiceCanisterInstaller, // "original invoice canister deployer"
  getRandomActor,
  getActorByIdentity,
  getRandomIdentity,
} from '../utils/identity.js';
import { getTestCreateInvoiceArgs } from '../utils/utils.js';

describe('Test get_invoice Functionality', async () => {
  // Caller to be added to get permissions list.
  const getPermittedIdentity = getRandomIdentity();
  const getPermittedActor = getActorByIdentity(getPermittedIdentity);
  describe('Test Token-Non Specific #err Results From get_invoice', () => {
    it('should reject if no invoice exists for given id and caller authorized | -> err #NotFound', async () => {
      const allowedCreatorIdentity = getRandomIdentity();
      const addResult = await invoiceCanisterInstaller.add_allowed_creator({
        who: allowedCreatorIdentity.getPrincipal(),
      });
      expect(addResult?.ok).toBeTruthy();
      const allowedCreator = getActorByIdentity(allowedCreatorIdentity);
      const getResult = await allowedCreator.get_invoice({ id: "invalidId" });
      expect(getResult?.err?.kind).toStrictEqual({ NotFound: null });
    });
    it('should reject if caller not authorized | -> #NotAuthorized', async () => {
      const getResult = await getRandomActor().get_invoice({ id: "invalidId" });
      expect(getResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
  describe('Test Token Specific #ok Results Returned From get_invoice', () => {
    describe('Test get_invoice Functionality for Invoices using #ICP Type', async () => {
      const createResult = await invoiceCanisterInstaller.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP,
          getPermissions: [getPermittedIdentity.getPrincipal()],
        }),
      );
      expect(createResult?.ok?.invoice).toBeTruthy();
      const { invoice } = createResult.ok;
      const { id } = createResult.ok.invoice;
      it('should correctly get an existing invoice by given id | #ICP -> ok (caller = creator)', async () => {
        const getResult = await invoiceCanisterInstaller.get_invoice({ id });
        expect(getResult?.ok?.invoice).toStrictEqual(invoice);
      });
      it('should correctly get an existing invoice by given id | #ICP -> ok (caller on get permissions)', async () => {
        const getResult = await getPermittedActor.get_invoice({ id });
        expect(getResult?.ok?.invoice).toStrictEqual(invoice);
      });
    });
    describe('Test get_invoice Functionality for Invoices using #ICP_nns Type', async () => {
      const createResult = await invoiceCanisterInstaller.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
          getPermissions: [getPermittedIdentity.getPrincipal()],
        }),
      );
      expect(createResult?.ok?.invoice).toBeTruthy();
      const { invoice } = createResult.ok;
      const { id } = createResult.ok.invoice;
      it('should correctly get an existing invoice by given id | #ICP_nns -> ok (caller = creator)', async () => {
        const getResult = await invoiceCanisterInstaller.get_invoice({ id });
        expect(getResult?.ok?.invoice).toStrictEqual(invoice);
      });
      it('should correctly get an existing invoice by given id | #ICP_nns -> ok (caller on get permissions)', async () => {
        const getResult = await getPermittedActor.get_invoice({ id });
        expect(getResult?.ok?.invoice).toStrictEqual(invoice);
      });
    });
    describe('Test get_invoice Functionality for Invoices using #ICRC1_ExampleToken Type', async () => {
      const createResult = await invoiceCanisterInstaller.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
          getPermissions: [getPermittedIdentity.getPrincipal()],
        }),
      );
      expect(createResult?.ok?.invoice).toBeTruthy();
      const { invoice } = createResult.ok;
      const { id } = createResult.ok.invoice;
      it('should correctly get an existing invoice by given id | #ICRC1_ExampleToken -> ok (caller = creator)', async () => {
        const getResult = await invoiceCanisterInstaller.get_invoice({ id });
        expect(getResult?.ok?.invoice).toStrictEqual(invoice);
      });
      it('should correctly get an existing invoice by given id | #ICRC1_ExampleToken -> ok (caller on get permissions)', async () => {
        const getResult = await getPermittedActor.get_invoice({ id });
        expect(getResult?.ok?.invoice).toStrictEqual(invoice);
      });
    });
    describe('Test get_invoice Functionality for Invoices using #ICRC1_ExampleToken2 Type', async () => {
      const createResult = await invoiceCanisterInstaller.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
          getPermissions: [getPermittedIdentity.getPrincipal()],
        }),
      );
      expect(createResult?.ok?.invoice).toBeTruthy();
      const { invoice } = createResult.ok;
      const { id } = createResult.ok.invoice;
      it('should correctly get an existing invoice by given id | #ICRC1_ExampleToken2 -> ok (caller = creator)', async () => {
        const getResult = await invoiceCanisterInstaller.get_invoice({ id });
        expect(getResult?.ok?.invoice).toStrictEqual(invoice);
      });
      it('should correctly get an existing invoice by given id | #ICRC1_ExampleToken2 -> ok (caller on get permissions)', async () => {
        const getResult = await getPermittedActor.get_invoice({ id });
        expect(getResult?.ok?.invoice).toStrictEqual(invoice);
      });
    });
  });
});
