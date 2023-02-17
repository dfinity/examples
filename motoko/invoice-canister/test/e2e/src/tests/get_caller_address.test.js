import { describe, it, expect } from 'vitest';
import { aPriori, SupportedTokens } from '../utils/constants.js';
import {
  nnsFundedSecp256k1Actor as invoiceCanisterInstaller, // "original invoice canister deployer"
  getRandomActor,
} from '../utils/identity.js';
import { toHexStringFromUInt8Array, toHexStringFromICRC1Account } from '../utils/utils.js';

describe('Test get_caller_address Functionality', async () => {
  const expectedICPAddress =
    aPriori.nnsFundedSecp256k1Identity.asInvoicePrincipalSubaccount.icp.asText;
  const expectedICRCAddress =
    aPriori.nnsFundedSecp256k1Identity.asInvoicePrincipalSubaccount.icrc1.asText;
  const invoiceCanisterIdLiteral = aPriori.invoiceCanister.canisterId.principal.asText;
  describe('When Calling get_caller_address to get an #ICP Type Address', async () => {
    it(`should get the account identifer and as encoded text of the caller's creator subaccount | #ICP -> ok`, async () => {
      const originalResult = await invoiceCanisterInstaller.get_caller_address({
        token: SupportedTokens.asVariantUnitObj.ICP,
      });
      expect(originalResult?.ok?.asText).toBeTruthy();
      expect(originalResult?.ok?.asAddress).toBeTruthy();
      const { asAddress, asText } = originalResult.ok;
      expect(asText).toStrictEqual(expectedICPAddress);
      expect(toHexStringFromUInt8Array(asAddress.ICP)).toStrictEqual(expectedICPAddress);
    });
    it(`should reject if the caller not authorized | #ICP -> err #NotAuthorized`, async () => {
      const result = await getRandomActor().get_caller_address({
        token: SupportedTokens.asVariantUnitObj.ICP,
      });
      expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
  describe('When Calling get_caller_address to get an #ICP_nns Type Address', async () => {
    it(`should get the account identifer and as encoded text of the caller's creator subaccount | #ICP_nns -> ok`, async () => {
      const result = await invoiceCanisterInstaller.get_caller_address({
        token: SupportedTokens.asVariantUnitObj.ICP_nns,
      });
      expect(result?.ok?.asText).toBeTruthy();
      expect(result?.ok?.asAddress).toBeTruthy();
      const { asAddress, asText } = result.ok;
      expect(asText).toStrictEqual(expectedICPAddress);
      expect(toHexStringFromUInt8Array(asAddress.ICP_nns)).toStrictEqual(expectedICPAddress);
    });
    it(`should reject if the caller not authorized | #ICP_nns -> err #NotAuthorized`, async () => {
      const result = await getRandomActor().get_caller_address({
        token: SupportedTokens.asVariantUnitObj.ICP_nns,
      });
      expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
  describe('When Calling get_caller_address to get an #ICRC1_ExampleToken Type Address', async () => {
    it(`should get the account and as encoded text of the caller's creator subaccount | #ICRC1_ExampleToken -> ok`, async () => {
      const result = await invoiceCanisterInstaller.get_caller_address({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken,
      });
      expect(result?.ok?.asText).toBeTruthy();
      expect(result?.ok?.asAddress).toBeTruthy();
      const { asAddress, asText } = result.ok;
      expect(asText).toStrictEqual(expectedICRCAddress);
      expect(
        // is the same as asAddress.ICRC1_ExampleToken.owner.toString()
        asAddress[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken].owner.toString(),
      ).toStrictEqual(invoiceCanisterIdLiteral);
      expect(toHexStringFromICRC1Account(asAddress.ICRC1_ExampleToken)).toStrictEqual(
        expectedICRCAddress,
      );
    });
    it(`should reject if the caller not authorized | #ICRC1_ExampleToken -> err #NotAuthorized`, async () => {
      const result = await getRandomActor().get_caller_address({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken,
      });
      expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
  describe('When Calling get_caller_address to get an #ICRC1_ExampleToke2n Type Address', async () => {
    it(`should get the account and as encoded text of the caller's creator subaccount | #ICRC1_ExampleToken2 -> ok`, async () => {
      const result = await invoiceCanisterInstaller.get_caller_address({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2,
      });
      expect(result?.ok?.asText).toBeTruthy();
      expect(result?.ok?.asAddress).toBeTruthy();
      const { asAddress, asText } = result.ok;
      expect(asText).toStrictEqual(expectedICRCAddress);
      expect(
        // is the same as asAddress.ICRC1_ExampleToken2.owner.toString()
        asAddress[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2].owner.toString(),
      ).toStrictEqual(invoiceCanisterIdLiteral);
      expect(toHexStringFromICRC1Account(asAddress.ICRC1_ExampleToken2)).toStrictEqual(
        expectedICRCAddress,
      );
    });
    it(`should reject if the caller not authorized | #ICRC1_ExampleToken2 -> err #NotAuthorized`, async () => {
      const result = await getRandomActor().get_caller_address({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2,
      });
      expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
});
