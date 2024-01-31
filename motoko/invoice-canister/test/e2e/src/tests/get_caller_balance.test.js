import { describe, it, expect } from 'vitest';
import { SupportedTokens } from '../utils/constants.js';
import {
  nnsFundedSecp256k1Actor as invoiceCanisterInstaller, // "original invoice canister deployer"
  getRandomActor,
  getActorByIdentity,
  getRandomIdentity,
} from '../utils/identity.js';

describe('Test get_caller_balance Functionality', async () => {
  // Use added allowed creator to verify balance calls works
  // as invoice canister installer also acts as balance holder.
  // Ie specifically check the balance of the newly added identity is 0.
  const authorizedCallerIdentity = getRandomIdentity();
  const result = await invoiceCanisterInstaller.add_allowed_creator({
    who: authorizedCallerIdentity.getPrincipal(),
  });
  if (!result.ok) {
    throw new Error(
      'Could not add allowed creator aborting get_caller_balance tests\nresult was ' +
        JSON.stringify(result),
    );
  }
  const authorizedCaller = getActorByIdentity(authorizedCallerIdentity);
  describe('When Caller Wants their #ICP Type Balance', async () => {
    it(`should get the balance of the invoice canister installer's creator subaccount | #ICP -> ok`, async () => {
      // Since the deployer is also the balance holder, balance could be variable.
      const result = await invoiceCanisterInstaller.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP,
      });
      expect(result?.ok).toBeTruthy();
    });
    it(`should correctly get the balance of an authorized caller's creator subaccount | #ICP -> ok`, async () => {
      const result = await authorizedCaller.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP,
      });
      expect(result?.ok?.balance).toStrictEqual({
        [SupportedTokens.asVariantTagLiteral.ICP]: {
          e8s: 0n,
        },
      });
    });
    it(`should reject if the caller not authorized | #ICP -> err #NotAuthorized`, async () => {
      const result = await getRandomActor().get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP,
      });
      expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
  describe('When Caller Wants their #ICP_nns Type Balance', async () => {
    it(`should get the balance of the invoice canister installer's creator subaccount | #ICP_nns -> ok`, async () => {
      // Since the deployer is also the balance holder, balance could be variable.
      const result = await invoiceCanisterInstaller.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP_nns,
      });
      expect(result?.ok).toBeTruthy();
    });
    it(`should correctly get the balance of an authorized caller's creator subaccount | #ICP_nns -> ok`, async () => {
      const result = await authorizedCaller.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP_nns,
      });
      expect(result?.ok?.balance).toStrictEqual({
        [SupportedTokens.asVariantTagLiteral.ICP_nns]: {
          e8s: 0n,
        },
      });
    });
    it(`should reject if the caller not authorized | #ICP_nns -> err #NotAuthorized`, async () => {
      const result = await getRandomActor().get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP_nns,
      });
      expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
  describe('When Caller Wants their #ICRC1_ExampleToken Type Balance', async () => {
    it(`should get the balance of the invoice canister installer's creator subaccount | #ICRC1_ExampleToken -> ok`, async () => {
      const result = await invoiceCanisterInstaller.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken,
      });
      expect(result?.ok).toBeTruthy();
    });
    it(`should correctly get the balance of an authorized caller's creator subaccount | #ICRC1_ExampleToken -> ok`, async () => {
      const result = await authorizedCaller.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken,
      });
      expect(result?.ok?.balance).toStrictEqual({
        [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: 0n,
      });
    });
    it(`should reject if the caller not authorized | #ICRC1_ExampleToken -> err #NotAuthorized`, async () => {
      const result = await getRandomActor().get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken,
      });
      expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
  describe('When Caller Wants their #ICRC1_ExampleToken2 Type Balance', async () => {
    it(`should get the balance of the invoice canister installer's creator subaccount | #ICRC1_ExampleToken2 -> ok`, async () => {
      const result = await invoiceCanisterInstaller.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2,
      });
      expect(result?.ok).toBeTruthy();
    });
    it(`should correctly get the balance of an authorized caller's creator subaccount | #ICRC1_ExampleToken2 -> ok`, async () => {
      const result = await authorizedCaller.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2,
      });
      expect(result?.ok?.balance).toStrictEqual({
        [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: 0n,
      });
    });
    it(`should reject if the caller not authorized | #ICRC1_ExampleToken2 -> err #NotAuthorized`, async () => {
      const result = await getRandomActor().get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2,
      });
      expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
});
