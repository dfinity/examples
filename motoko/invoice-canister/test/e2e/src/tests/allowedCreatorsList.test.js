import { describe, it, expect } from 'vitest';
import { SupportedTokens } from '../utils/constants.js';
import {
  nnsFundedSecp256k1Actor as invoiceCanisterInstaller, // "original invoice canister deployer"
  getRandomActor,
  getActorByIdentity,
  getRandomIdentity,
  anonymousPrincipal,
} from '../utils/identity.js';
import { getTestCreateInvoiceArgs } from '../utils/utils.js';

describe('Test Functionality of Allowed Creators List', async () => {
  // Clear existing list of allowed creators before any tests in this suite.
  const existingAllowedCreators = await invoiceCanisterInstaller.get_allowed_creators_list();
  expect(existingAllowedCreators?.ok?.allowed).toBeTruthy();
  for (let p of existingAllowedCreators.ok.allowed) {
    await invoiceCanisterInstaller.remove_allowed_creator({ who: p });
  }
  // Verify allowed creators list is empty before starting tests.
  const checkResult = await invoiceCanisterInstaller.get_allowed_creators_list();
  expect(checkResult?.ok?.allowed?.length).toEqual(0);

  describe('Test add_allowed_creator Functionality', async () => {
    // err kind MaxAllowed case is in benchmark testing.
    it('should allow invoice canister deployer to add a non-anonymous principal to allowed creators list and that works correctly | -> ok', async () => {
      // Generate an identity to get the principal to add.
      const aRandomId = getRandomIdentity();
      // Confirm the identity as actor is not yet authorized...
      // (to create an invoice)
      const anActor = getActorByIdentity(aRandomId);
      let createResult = await anActor.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
        }),
      );
      expect(createResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
      // (or to get their balance)
      let balanceResult = await anActor.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP_nns,
      });
      expect(balanceResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
      // (assuming other methods will be similar moving on)...
      // Add them to the allowed creators list.
      const aPrincipal = aRandomId.getPrincipal();
      const result = await invoiceCanisterInstaller.add_allowed_creator({ who: aPrincipal });
      // Check that it resulted ok.
      expect(result?.ok).toBeTruthy();
      // Check the principal can create an invoice.
      createResult = await anActor.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
        }),
      );
      // Confirm it was created ok by this identity's principal.
      expect(createResult?.ok?.invoice).toBeTruthy();
      const { creator } = createResult.ok.invoice;
      expect(creator).toStrictEqual(aPrincipal);
      // Confirm caller can now also get their balance.
      balanceResult = await anActor.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP_nns,
      });
      expect(balanceResult?.ok?.balance).toStrictEqual({
        [SupportedTokens.asVariantTagLiteral.ICP_nns]: {
          e8s: 0n,
        },
      });
    });
    it('should reject if principal to add already on the list | -> err #AlreadyAdded', async () => {
      // Generate an identity to get the principal to add.
      const aRandomId = getRandomIdentity();
      // Add them to the allowed creators list.
      const aPrincipal = aRandomId.getPrincipal();
      let result = await invoiceCanisterInstaller.add_allowed_creator({ who: aPrincipal });
      // Check that it resulted ok.
      expect(result?.ok).toBeTruthy();
      // Confirm trying to add them again returns an err.
      result = await invoiceCanisterInstaller.add_allowed_creator({ who: aPrincipal });
      expect(result?.err?.kind).toStrictEqual({ AlreadyAdded: null });
    });
    it('should reject if principal to add is anonymous | -> err #AnonymousIneligible ', async () => {
      const result = await invoiceCanisterInstaller.add_allowed_creator({
        who: anonymousPrincipal,
      });
      expect(result?.err?.kind).toStrictEqual({ AnonymousIneligible: null });
    });
    it('should reject if caller unauthorized | -> err #NotAuthorized', async () => {
      const result = await getRandomActor().add_allowed_creator({
        who: getRandomIdentity().getPrincipal(),
      });
      expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
  describe('Test remove_allowed_creator Functionality', async () => {
    it('should allow invoice canister deployer to remove a principal from the allowed creators list | -> ok', async () => {
      // Generate an identity to get the principal to add and then remove.
      const aRandomId = getRandomIdentity();
      // Add them to the allowed creators list.
      const aPrincipal = aRandomId.getPrincipal();
      let result = await invoiceCanisterInstaller.add_allowed_creator({ who: aPrincipal });
      // Check that it resulted ok.
      expect(result?.ok).toBeTruthy();
      // Confirm they can be removed ok.
      result = await invoiceCanisterInstaller.remove_allowed_creator({ who: aPrincipal });
      expect(result?.ok?.message).toBeTruthy();
      // Double check that principal is not on the list.
      const checkResult = await invoiceCanisterInstaller.get_allowed_creators_list();
      expect(checkResult?.ok?.allowed).toBeTruthy();
      expect(checkResult.ok.allowed).not.toContain(aPrincipal);
    });
    it('should reject if principal to remove not on the list | -> err #NotFound', async () => {
      // Generate an identity to get the principal to add, remove and then check error if trying to remove again.
      const aRandomId = getRandomIdentity();
      // Add them to the allowed creators list.
      const aPrincipal = aRandomId.getPrincipal();
      let result = await invoiceCanisterInstaller.add_allowed_creator({ who: aPrincipal });
      // Check they were added.
      expect(result?.ok).toBeTruthy();
      result = await invoiceCanisterInstaller.remove_allowed_creator({ who: aPrincipal });
      // Check they removed.
      expect(result?.ok?.message).toBeTruthy();
      // Confirm err if trying to remove same again.
      result = await invoiceCanisterInstaller.remove_allowed_creator({ who: aPrincipal });
      expect(result?.err?.kind).toStrictEqual({ NotFound: null });
    });
    it('should reject if caller unauthorized | -> #NotAuthorized', async () => {
      const result = await getRandomActor().remove_allowed_creator({
        who: getRandomIdentity().getPrincipal(),
      });
      expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
  describe('Test get_allowed_creators_list Functionality', async () => {
    it(
      'should get the list of allowed creators for the invoice canister deployer correctly | -> ok',
      async () => {
        const existingListResult = await invoiceCanisterInstaller.get_allowed_creators_list();
        // Check that it resulted ok.
        expect(existingListResult?.ok?.allowed).toBeTruthy();
        // Remove any on the list.
        for (let p of existingListResult.ok.allowed) {
          const removedResult = await invoiceCanisterInstaller.remove_allowed_creator({ who: p });
          expect(removedResult?.ok?.message).toBeTruthy();
        }
        const added = [];
        for (let i = 0; i < 10 /*~~(Math.random() * 10)*/; ++i) {
          const aPrincipalToAdd = getRandomIdentity().getPrincipal();
          added.push(aPrincipalToAdd.toString());
          const result = await invoiceCanisterInstaller.add_allowed_creator({
            who: aPrincipalToAdd,
          });
          expect(result?.ok?.message).toBeTruthy();
        }
        const expectedResult = await invoiceCanisterInstaller.get_allowed_creators_list();
        // Check that it resulted ok with the same list of principals.
        expect(expectedResult?.ok?.allowed).toBeTruthy();
        let returnedResultList = expectedResult.ok.allowed.map(p => p.toString()).sort();
        expect(returnedResultList).toStrictEqual(added.sort());
      },
      // This one may take longer than usual, so extend test timeout.
      { timeout: 60000 },
    );
    it('should reject if caller not authorized (ie not the canister installer) | -> #NotAuthorized', async () => {
      // not authorized caller in this case also just means anyone not the invoice canister installer
      const result = await getRandomActor().get_allowed_creators_list();
      expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
});
