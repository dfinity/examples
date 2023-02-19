import { describe, it, expect } from 'vitest';
import { aPriori, SupportedTokens } from '../utils/constants.js';
import { getRandomIdentity, anonymousActor } from '../utils/identity.js';
import { getTestCreateInvoiceArgs } from '../utils/utils.js';

describe('Test Anonymous Principal is Disallowed as a Caller', async () => {
  it('should reject anonymous caller | create_invoice (all token kinds) -> err #NotAuthorized', async () => {
    // Check for all supported token types just to show it is indeed not dependent on the token.
    let result = await anonymousActor.create_invoice(
      getTestCreateInvoiceArgs({
        whichToken: SupportedTokens.asVariantTagLiteral.ICP,
      }),
    );
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    result = await anonymousActor.create_invoice(
      getTestCreateInvoiceArgs({
        whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
      }),
    );
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    result = await anonymousActor.create_invoice(
      getTestCreateInvoiceArgs({
        whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
      }),
    );
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    result = await anonymousActor.create_invoice(
      getTestCreateInvoiceArgs({
        whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
      }),
    );
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
  });
  it('should reject anonymous caller | add_allowed_creator -> err #NotAuthorized', async () => {
    const result = await anonymousActor.add_allowed_creator({
      who: getRandomIdentity().getPrincipal(),
    });
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
  });
  it('should reject anonymous caller | remove_allowed_creator -> err #NotAuthorized', async () => {
    const result = await anonymousActor.remove_allowed_creator({
      who: getRandomIdentity().getPrincipal(),
    });
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
  });
  it('should reject anonymous caller | get_allowed_creators_list -> err #NotAuthorized', async () => {
    const result = await anonymousActor.get_allowed_creators_list();
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
  });
  it('should reject anonymous caller | get_invoice -> err #NotAuthorized', async () => {
    const result = await anonymousActor.get_invoice({ id: "6GNGGRXAKGTXG070DV4GW2JKCJ" });
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
  });
  it('should reject anonymous caller | get_caller_balance -> err #NotAuthorized', async () => {
    let result = await anonymousActor.get_caller_balance({
      token: SupportedTokens.asVariantUnitObj.ICP,
    });
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    result = await anonymousActor.get_caller_balance({
      token: SupportedTokens.asVariantUnitObj.ICP_nns,
    });
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    result = await anonymousActor.get_caller_balance({
      token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken,
    });
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    result = await anonymousActor.get_caller_balance({
      token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2,
    });
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
  });
  it('should reject anonymous caller | get_caller_address (any token type) -> err #NotAuthorized', async () => {
    let result = await anonymousActor.get_caller_address({
      token: SupportedTokens.asVariantUnitObj.ICP,
    });
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    result = await anonymousActor.get_caller_address({
      token: SupportedTokens.asVariantUnitObj.ICP_nns,
    });
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    result = await anonymousActor.get_caller_address({
      token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken,
    });
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    result = await anonymousActor.get_caller_address({
      token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2,
    });
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
  });
  it('should reject anonymous caller | verify_invoice -> err #NotAuthorized', async () => {
    const result = await anonymousActor.verify_invoice({ id: "6GNGGRXAKGTXG070DV4GW2JKCJ" });
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
  });
  it('should reject anonymous caller | transfer (all token kinds) -> err #NotAuthorized', async () => {
    let result = await anonymousActor.transfer({
      tokenAmount: {
        [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: 1000000n },
      },
      destination: {
        HumanReadable: aPriori.nnsFundedSecp256k1Identity.asDefaultSubaccount.icp.asText,
      },
    });
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    result = await anonymousActor.transfer({
      tokenAmount: {
        [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: 1000000n },
      },
      destination: {
        HumanReadable: aPriori.nnsFundedSecp256k1Identity.asDefaultSubaccount.icp.asText,
      },
    });
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    result = await anonymousActor.transfer({
      tokenAmount: {
        [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: 1000000n,
      },
      destination: {
        HumanReadable: aPriori.nnsFundedSecp256k1Identity.asDefaultSubaccount.icrc1.asText,
      },
    });
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    result = await anonymousActor.transfer({
      tokenAmount: {
        [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: 1000000n,
      },
      destination: {
        HumanReadable: aPriori.nnsFundedSecp256k1Identity.asDefaultSubaccount.icrc1.asText,
      },
    });
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
  });
  it('should reject anonymous caller | to_other_address_format -> err #NotAuthorized', async () => {
    const result = await anonymousActor.to_other_address_format({
      token: [],
      address: [],
    });
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
  });
  it('should reject anonymous caller | recover_invoice_subaccount_balance -> err #NotAuthorized', async () => {
    const result = await anonymousActor.recover_invoice_subaccount_balance({
      id: "6GNGGRXAKGTXG070DV4GW2JKCJ",
      destination: {
        HumanReadable: aPriori.nnsFundedSecp256k1Identity.asDefaultSubaccount.icp.asText,
      },
    });
    expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
  });
});
