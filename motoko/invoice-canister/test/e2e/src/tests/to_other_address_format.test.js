import { describe, it, expect } from 'vitest';
import { aPriori, SupportedTokens } from '../utils/constants.js';
import {
  nnsFundedSecp256k1Actor as invoiceCanisterInstaller, // "original invoice canister deployer"
  nnsFundedSecp256k1Identity as installerPrincipal,
  getRandomActor,
} from '../utils/identity.js';
import {
  toHexStringFromUInt8Array,
  toHexStringFromICRC1Account,
  principalFromText,
} from '../utils/utils.js';

describe('Test to_other_address_format Functionality', async () => {
  // Assign some locally scoped so can be changed as needed.
  const authorizedCaller = invoiceCanisterInstaller;
  const authorizedCallerPrincipal = installerPrincipal.getPrincipal();

  const expectedDefaultSubaccountAccountIdentifierText =
    aPriori.nnsFundedSecp256k1Identity.asDefaultSubaccount.icp.asText;

  const expectedDefaultSubaccountAccountText =
    aPriori.nnsFundedSecp256k1Identity.asDefaultSubaccount.icrc1.asText;

  // Used for verifying ICRC1 account addressing with non trivial subaccount.
  // (borrowed from Motoko unit tests, note the knownCanisterId is different
  // from the canisterId of the invoice cansiter used in E2e testing).
  const knownCanisterId = principalFromText(aPriori.invoiceCanister.canisterId.principal.asText);
  const knownSubaccount = aPriori.invoiceCanister.icrc1Subaccount.subaccount.asText;
  const knownIRC1AccountText = aPriori.invoiceCanister.icrc1Subaccount.asText;
  describe('Test #ok Results Returned From to_other_address_format', async () => {
    /*
    Note: Each 'describe' token specific #ok result 'it' tests are:
      1) encode the address
      2) decode the address
      3) compute the default subaccount address
    */
    describe('Test to_other_address_format Functionality for ICP Token Type Addressing', async () => {
      it('should correctly encode an account identifier into text', async () => {
        const toOtherResult = await authorizedCaller.to_other_address_format({
          token: [],
          address: [
            {
              CanisterExpected: {
                [SupportedTokens.asVariantTagLiteral.ICP]: Uint8Array.from(
                  Buffer.from(expectedDefaultSubaccountAccountIdentifierText, 'hex'),
                ),
              },
            },
          ],
        });
        expect(toOtherResult?.ok?.asText).toBeTruthy();
        expect(toOtherResult?.ok?.asAddress).toBeTruthy();
        const { asText, asAddress } = toOtherResult.ok;
        expect(asText).toStrictEqual(expectedDefaultSubaccountAccountIdentifierText);
        // is same as = asAddress.ICP;
        const accountIdentifier = asAddress[SupportedTokens.asVariantTagLiteral.ICP];
        expect(toHexStringFromUInt8Array(accountIdentifier)).toStrictEqual(
          expectedDefaultSubaccountAccountIdentifierText,
        );
      });
      it('should correctly decode acceptable text into an account identifier', async () => {
        const toOtherResult = await authorizedCaller.to_other_address_format({
          token: [SupportedTokens.asVariantUnitObj.ICP],
          address: [
            {
              HumanReadable: expectedDefaultSubaccountAccountIdentifierText,
            },
          ],
        });
        expect(toOtherResult?.ok?.asText).toBeTruthy();
        expect(toOtherResult?.ok?.asAddress).toBeTruthy();
        const { asText, asAddress } = toOtherResult.ok;
        expect(asText).toStrictEqual(expectedDefaultSubaccountAccountIdentifierText);
        // is same as = asAddress.ICP;
        const accountIdentifier = asAddress[SupportedTokens.asVariantTagLiteral.ICP];
        expect(toHexStringFromUInt8Array(accountIdentifier)).toStrictEqual(
          expectedDefaultSubaccountAccountIdentifierText,
        );
      });
      it('should correctly compute the default subaccount account identifier from a principal', async () => {
        const toOtherResult = await authorizedCaller.to_other_address_format({
          token: [SupportedTokens.asVariantUnitObj.ICP],
          address: [],
        });
        expect(toOtherResult?.ok?.asText).toBeTruthy();
        expect(toOtherResult?.ok?.asAddress).toBeTruthy();
        const { asText, asAddress } = toOtherResult.ok;
        expect(asText).toStrictEqual(expectedDefaultSubaccountAccountIdentifierText);
        // is same as = asAddress.ICP;
        const accountIdentifier = asAddress[SupportedTokens.asVariantTagLiteral.ICP];
        expect(toHexStringFromUInt8Array(accountIdentifier)).toStrictEqual(
          expectedDefaultSubaccountAccountIdentifierText,
        );
      });
    });
    describe('Test to_other_address_format Functionality for ICP_nns Token Type Addressing', async () => {
      it('should correctly encode an account identifier into text', async () => {
        const toOtherResult = await authorizedCaller.to_other_address_format({
          token: [],
          address: [
            {
              CanisterExpected: {
                [SupportedTokens.asVariantTagLiteral.ICP_nns]: Uint8Array.from(
                  Buffer.from(expectedDefaultSubaccountAccountIdentifierText, 'hex'),
                ),
              },
            },
          ],
        });
        expect(toOtherResult?.ok?.asText).toBeTruthy();
        expect(toOtherResult?.ok?.asAddress).toBeTruthy();
        const { asText, asAddress } = toOtherResult.ok;
        expect(asText).toStrictEqual(expectedDefaultSubaccountAccountIdentifierText);
        // is same as = asAddress.ICP_nns;
        const accountIdentifier = asAddress[SupportedTokens.asVariantTagLiteral.ICP_nns];
        expect(toHexStringFromUInt8Array(accountIdentifier)).toStrictEqual(
          expectedDefaultSubaccountAccountIdentifierText,
        );
      });
      it('should correctly decode acceptable text into an account identifier', async () => {
        const toOtherResult = await authorizedCaller.to_other_address_format({
          token: [SupportedTokens.asVariantUnitObj.ICP_nns],
          address: [
            {
              HumanReadable: expectedDefaultSubaccountAccountIdentifierText,
            },
          ],
        });
        expect(toOtherResult?.ok?.asText).toBeTruthy();
        expect(toOtherResult?.ok?.asAddress).toBeTruthy();
        const { asText, asAddress } = toOtherResult.ok;
        expect(asText).toStrictEqual(expectedDefaultSubaccountAccountIdentifierText);
        // is same as = asAddress.ICP_nns;
        const accountIdentifier = asAddress[SupportedTokens.asVariantTagLiteral.ICP_nns];
        expect(toHexStringFromUInt8Array(accountIdentifier)).toStrictEqual(
          expectedDefaultSubaccountAccountIdentifierText,
        );
      });
      it('should correctly compute the default subaccount account identifier from a principal', async () => {
        const toOtherResult = await authorizedCaller.to_other_address_format({
          token: [SupportedTokens.asVariantUnitObj.ICP_nns],
          address: [],
        });
        expect(toOtherResult?.ok?.asText).toBeTruthy();
        expect(toOtherResult?.ok?.asAddress).toBeTruthy();
        const { asText, asAddress } = toOtherResult.ok;
        expect(asText).toStrictEqual(expectedDefaultSubaccountAccountIdentifierText);
        // is same as = asAddress.ICP_nns;
        const accountIdentifier = asAddress[SupportedTokens.asVariantTagLiteral.ICP_nns];
        expect(toHexStringFromUInt8Array(accountIdentifier)).toStrictEqual(
          expectedDefaultSubaccountAccountIdentifierText,
        );
      });
    });
    describe('Test to_other_address_format Functionality for ICRC1_ExampleToken Type Addressing', async () => {
      it('should correctly encode an icrc1 account into text', async () => {
        const toOtherResult = await authorizedCaller.to_other_address_format({
          token: [],
          address: [
            {
              CanisterExpected: {
                [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: {
                  owner: authorizedCallerPrincipal,
                  subaccount: [],
                },
              },
            },
          ],
        });
        expect(toOtherResult?.ok?.asText).toBeTruthy();
        expect(toOtherResult?.ok?.asAddress).toBeTruthy();
        const { asText, asAddress } = toOtherResult.ok;
        expect(asText).toStrictEqual(expectedDefaultSubaccountAccountText);
        // is same as = asAddress.ICRC1_ExampleToken;
        const account = asAddress[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken];
        const { owner, subaccount } = account;
        expect(owner.toString()).toStrictEqual(authorizedCallerPrincipal.toString());
        expect(subaccount).toStrictEqual([]);
        expect(toHexStringFromICRC1Account(account)).toStrictEqual(
          expectedDefaultSubaccountAccountText,
        );
      });
      it('should correctly encode an icrc1 account with non-trivial subaccount into text', async () => {
        const toOtherResult = await authorizedCaller.to_other_address_format({
          token: [],
          address: [
            {
              CanisterExpected: {
                [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: {
                  owner: knownCanisterId,
                  subaccount: [Buffer.from(knownSubaccount, 'hex')],
                },
              },
            },
          ],
        });
        expect(toOtherResult?.ok?.asText).toBeTruthy();
        expect(toOtherResult?.ok?.asAddress).toBeTruthy();
        const { asText, asAddress } = toOtherResult.ok;
        expect(asText).toStrictEqual(knownIRC1AccountText);
        // is same as = asAddress.ICRC1_ExampleToken;
        const account = asAddress[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken];
        const { owner, subaccount } = account;
        expect(owner.toString()).toStrictEqual(knownCanisterId.toString());
        expect(toHexStringFromUInt8Array(subaccount[0])).toStrictEqual(knownSubaccount);
        expect(toHexStringFromICRC1Account(account)).toStrictEqual(knownIRC1AccountText);
      });
      it('should correctly decode acceptable text into an icrc1 account', async () => {
        const toOtherResult = await authorizedCaller.to_other_address_format({
          token: [SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken],
          address: [
            {
              HumanReadable: expectedDefaultSubaccountAccountText,
            },
          ],
        });
        expect(toOtherResult?.ok?.asText).toBeTruthy();
        expect(toOtherResult?.ok?.asAddress).toBeTruthy();
        const { asText, asAddress } = toOtherResult.ok;
        expect(asText).toStrictEqual(expectedDefaultSubaccountAccountText);
        // is same as = asAddress.ICRC1_ExampleToken;
        const account = asAddress[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken];
        const { owner, subaccount } = account;
        expect(owner.toString()).toStrictEqual(authorizedCallerPrincipal.toString());
        expect(subaccount).toStrictEqual([]);
        expect(toHexStringFromICRC1Account(account)).toStrictEqual(
          expectedDefaultSubaccountAccountText,
        );
      });
      it('should correctly compute the default subaccount icrc1 account from a principal', async () => {
        const toOtherResult = await authorizedCaller.to_other_address_format({
          token: [SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken],
          address: [],
        });
        expect(toOtherResult?.ok?.asText).toBeTruthy();
        expect(toOtherResult?.ok?.asAddress).toBeTruthy();
        const { asText, asAddress } = toOtherResult.ok;
        expect(asText).toStrictEqual(expectedDefaultSubaccountAccountText);
        // is same as = asAddress.ICRC1_ExampleToken;
        const account = asAddress[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken];
        const { owner, subaccount } = account;
        expect(owner.toString()).toStrictEqual(authorizedCallerPrincipal.toString());
        expect(subaccount).toStrictEqual([]);
        expect(toHexStringFromICRC1Account(account)).toStrictEqual(
          expectedDefaultSubaccountAccountText,
        );
      });
    });
    describe('Test to_other_address_format Functionality for ICRC1_ExampleToken2 Type Addressing', async () => {
      it('should correctly encode an icrc1 account to text', async () => {
        const toOtherResult = await authorizedCaller.to_other_address_format({
          token: [],
          address: [
            {
              CanisterExpected: {
                [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: {
                  owner: authorizedCallerPrincipal,
                  subaccount: [],
                },
              },
            },
          ],
        });
        expect(toOtherResult?.ok?.asText).toBeTruthy();
        expect(toOtherResult?.ok?.asAddress).toBeTruthy();
        const { asText, asAddress } = toOtherResult.ok;
        expect(asText).toStrictEqual(expectedDefaultSubaccountAccountText);
        // is same as = asAddress.ICRC1_ExampleToken;
        const account = asAddress[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2];
        const { owner, subaccount } = account;
        expect(owner.toString()).toStrictEqual(authorizedCallerPrincipal.toString());
        expect(subaccount).toStrictEqual([]);
        expect(toHexStringFromICRC1Account(account)).toStrictEqual(
          expectedDefaultSubaccountAccountText,
        );
      });
      it('should correctly encode an icrc1 account with non-trivial subaccount into text', async () => {
        const toOtherResult = await authorizedCaller.to_other_address_format({
          token: [],
          address: [
            {
              CanisterExpected: {
                [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: {
                  owner: knownCanisterId,
                  subaccount: [Buffer.from(knownSubaccount, 'hex')],
                },
              },
            },
          ],
        });
        expect(toOtherResult?.ok?.asText).toBeTruthy();
        expect(toOtherResult?.ok?.asAddress).toBeTruthy();
        const { asText, asAddress } = toOtherResult.ok;
        expect(asText).toStrictEqual(knownIRC1AccountText);
        // is same as = asAddress.ICRC1_ExampleToken;
        const account = asAddress[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2];
        const { owner, subaccount } = account;
        expect(owner.toString()).toStrictEqual(knownCanisterId.toString());
        expect(toHexStringFromUInt8Array(subaccount[0])).toStrictEqual(knownSubaccount);
        expect(toHexStringFromICRC1Account(account)).toStrictEqual(knownIRC1AccountText);
      });
      it('should correctly decode acceptable text into an icrc1 account', async () => {
        const toOtherResult = await authorizedCaller.to_other_address_format({
          token: [SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2],
          address: [
            {
              HumanReadable: expectedDefaultSubaccountAccountText,
            },
          ],
        });
        expect(toOtherResult?.ok?.asText).toBeTruthy();
        expect(toOtherResult?.ok?.asAddress).toBeTruthy();
        const { asText, asAddress } = toOtherResult.ok;
        expect(asText).toStrictEqual(expectedDefaultSubaccountAccountText);
        // is same as = asAddress.ICRC1_ExampleToken2;
        const account = asAddress[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2];
        const { owner, subaccount } = account;
        expect(owner.toString()).toStrictEqual(authorizedCallerPrincipal.toString());
        expect(subaccount).toStrictEqual([]);
        expect(toHexStringFromICRC1Account(account)).toStrictEqual(
          expectedDefaultSubaccountAccountText,
        );
      });
      it('should correctly compute the default subaccount icrc1 account from a principal', async () => {
        const toOtherResult = await authorizedCaller.to_other_address_format({
          token: [SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2],
          address: [],
        });
        expect(toOtherResult?.ok?.asText).toBeTruthy();
        expect(toOtherResult?.ok?.asAddress).toBeTruthy();
        const { asText, asAddress } = toOtherResult.ok;
        expect(asText).toStrictEqual(expectedDefaultSubaccountAccountText);
        // is same as = asAddress.ICRC1_ExampleToken2;
        const account = asAddress[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2];
        const { owner, subaccount } = account;
        expect(owner.toString()).toStrictEqual(authorizedCallerPrincipal.toString());
        expect(subaccount).toStrictEqual([]);
        expect(toHexStringFromICRC1Account(account)).toStrictEqual(
          expectedDefaultSubaccountAccountText,
        );
      });
    });
  });
  describe('Test #err Results Returned From to_other_address_format', async () => {
    it('should reject and return err kind #NotAuthorized if caller not authorized', async () => {
      // err kind #NotAuthorized
      const toOtherResult = await getRandomActor().to_other_address_format({
        token: [],
        address: [],
      });
      expect(toOtherResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
    describe('If Missing which Token Type to Convert Address Type of | -> #err kind #MissingTokenType', async () => {
      it('should reject when calling for default subaccount address but missing given token type', async () => {
        // When calling without token type to convert caller's principal to address of.
        const toOtherResult = await authorizedCaller.to_other_address_format({
          token: [],
          address: [],
        });
        expect(toOtherResult?.err?.kind).toStrictEqual({ MissingTokenType: null });
      });
      it('should reject when given acceptable encoded address text but missing token type', async () => {
        // When calling with acceptable text but no token type to convert it address form to.
        const toOtherResult = await authorizedCaller.to_other_address_format({
          token: [],
          address: [
            {
              HumanReadable: expectedDefaultSubaccountAccountText,
            },
          ],
        });
        expect(toOtherResult?.err?.kind).toStrictEqual({ MissingTokenType: null });
      });
    });
    describe('If Given an Invalid Destination | #err kind #InvalidDestination', async () => {
      describe('Such as Invalid Text to Decode | Text -> #err kind #InvalidDestination', async () => {
        it('should reject if given invalid text to be decoded into an account identifier | #ICP', async () => {
          const toOtherResult = await authorizedCaller.to_other_address_format({
            token: [SupportedTokens.asVariantUnitObj.ICP],
            address: [
              {
                HumanReadable: 'Ø⸎⸖𝛀𝛚ༀ≟',
              },
            ],
          });
          expect(toOtherResult?.err?.kind).toStrictEqual({ InvalidDestination: null });
        });
        it('should reject if given invalid text to be decoded into an account identifier | #ICP_nns', async () => {
          const toOtherResult = await authorizedCaller.to_other_address_format({
            token: [SupportedTokens.asVariantUnitObj.ICP],
            address: [
              {
                HumanReadable: '⸎⸖𝛀𝛚ༀ≟Ø',
              },
            ],
          });
          expect(toOtherResult?.err?.kind).toStrictEqual({ InvalidDestination: null });
        });
        it('should reject if given invalid text to be decoded into an icrc1 account | #ICRC1_ExampleToken', async () => {
          const toOtherResult = await authorizedCaller.to_other_address_format({
            token: [SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken],
            address: [
              {
                HumanReadable: '⸖𝛀𝛚ༀ≟Ø⸎',
              },
            ],
          });
          expect(toOtherResult?.err?.kind).toStrictEqual({ InvalidDestination: null });
        });
        it('should reject if given invalid text to be decoded into an icrc1 account | #ICRC1_ExampleToken2', async () => {
          const toOtherResult = await authorizedCaller.to_other_address_format({
            token: [SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2],
            address: [
              {
                HumanReadable: '𝛀𝛚ༀ≟Ø⸎⸖',
              },
            ],
          });
          expect(toOtherResult?.err?.kind).toStrictEqual({ InvalidDestination: null });
        });
        it('should reject if given text to decode into icrc1 account is just a reserved principal | #ICRC1_ExampleToken', async () => {
          const toOtherResult = await authorizedCaller.to_other_address_format({
            token: [SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken],
            address: [
              {
                HumanReadable: 'ddhvl-oibai-bqibi-ga6xx-6',
              },
            ],
          });
          expect(toOtherResult?.err?.kind).toStrictEqual({ InvalidDestination: null });
        });
        it('should reject if given text to decode into icrc1 account is just a reserved principal | #ICRC1_ExampleToken2', async () => {
          const toOtherResult = await authorizedCaller.to_other_address_format({
            token: [SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2],
            address: [
              {
                HumanReadable: 'ddhvl-oibai-bqibi-ga6xx-6',
              },
            ],
          });
          expect(toOtherResult?.err?.kind).toStrictEqual({ InvalidDestination: null });
        });
      });
      describe('Such as an Invalid Address to Encode | Address -> #err kind #InvalidDestination', async () => {
        it('should reject if given an invalid account identifier | #ICP', async () => {
          const toOtherResult = await authorizedCaller.to_other_address_format({
            token: [],
            address: [
              {
                CanisterExpected: {
                  [SupportedTokens.asVariantTagLiteral.ICP]: new Uint8Array(1, 3, 3),
                },
              },
            ],
          });
          expect(toOtherResult?.err?.kind).toStrictEqual({ InvalidDestination: null });
        });
        it('should reject if given an invalid account identifier | #ICP_nns', async () => {
          const toOtherResult = await authorizedCaller.to_other_address_format({
            token: [],
            address: [
              {
                CanisterExpected: {
                  [SupportedTokens.asVariantTagLiteral.ICP_nns]: new Uint8Array(),
                },
              },
            ],
          });
          expect(toOtherResult?.err?.kind).toStrictEqual({ InvalidDestination: null });
        });
        it('should reject if given an invalid icrc1 account | #ICRC1_ExampleToken', async () => {
          const toOtherResult = await authorizedCaller.to_other_address_format({
            token: [],
            address: [
              {
                CanisterExpected: {
                  [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: {
                    owner: authorizedCallerPrincipal,
                    subaccount: [new Uint8Array()],
                  },
                },
              },
            ],
          });
          expect(toOtherResult?.err?.kind).toStrictEqual({ InvalidDestination: null });
        });
        it('should reject if given an invalid icrc1 account | #ICRC1_ExampleToken2', async () => {
          const toOtherResult = await authorizedCaller.to_other_address_format({
            token: [],
            address: [
              {
                CanisterExpected: {
                  [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: {
                    owner: authorizedCallerPrincipal,
                    subaccount: [new Uint8Array(9, 9, 9, 9, 9, 9, 9, 9)],
                  },
                },
              },
            ],
          });
          expect(toOtherResult?.err?.kind).toStrictEqual({ InvalidDestination: null });
        });
      });
    });
  });
});
