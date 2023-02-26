// #MaxInvoicesCreated is a benchmark test not included in this test suite.

import { describe, it, expect } from 'vitest';
import { SupportedTokens } from '../utils/constants.js';
import {
  nnsFundedSecp256k1Actor as invoiceCanisterInstaller, // "original invoice canister deployer"
  nnsFundedSecp256k1Identity as installerIdentity,
  getRandomActor,
  getActorByIdentity,
  getRandomIdentity,
} from '../utils/identity.js';
import { getTestCreateInvoiceArgs } from '../utils/utils.js';

describe('Test create_invoice Functionality', async () => {
  // Adding a new identity to confirm non-installer can create invoices.
  const installerPrincipal = installerIdentity.getPrincipal();
  const invoiceCreatorIdentity = getRandomIdentity();
  const invoiceCreatorPrincipal = invoiceCreatorIdentity.getPrincipal();
  const result = await invoiceCanisterInstaller.add_allowed_creator({
    who: invoiceCreatorPrincipal,
  });
  // Confirm identity was added as allowed creator;
  expect(result?.ok).toBeTruthy();
  const invoiceCreator = getActorByIdentity(invoiceCreatorIdentity);
  const billableAmountDue = 500000n;

  describe('Test Token Specific #ok Results Returned From create_invoice', () => {
    describe('Test create_invoice Functionality for Invoices using #ICP Type', async () => {
      it('should correctly create an invoice | #ICP -> ok', async () => {
        const createResult = await invoiceCanisterInstaller.create_invoice(
          getTestCreateInvoiceArgs({
            whichToken: SupportedTokens.asVariantTagLiteral.ICP,
            amountDue: billableAmountDue,
          }),
        );
        expect(createResult?.ok?.invoice).toBeTruthy();
        // Note to self: use snapshots in the future.
        const { amountDue, creator, token, paid, amountPaid, tokenVerbose } =
          createResult.ok.invoice;
        expect(billableAmountDue).toStrictEqual(amountDue);
        expect(creator).toStrictEqual(installerPrincipal);
        expect(token).toStrictEqual(SupportedTokens.asVariantUnitObj.ICP);
        // The above line is same as expect(token).toStrictEqual({ ICP: null });
        expect(!paid).toBe(true);
        expect(amountPaid).toStrictEqual(0n);
        // Testing "critical" verbose token details.
        expect(tokenVerbose?.fee).toStrictEqual(10000n);
        expect(tokenVerbose?.name.includes('Internet Computer Protocol')).toBe(true);
        expect(`${tokenVerbose?.decimals}`).toStrictEqual('8');
      });
      it('should correctly create an invoice with permission | #ICP -> ok', async () => {
        // Populate permissions array to test.
        let addToGet = [],
          addToVerify = [];
        for (let i = 0; i < ~~(Math.random() * 256); ++i) {
          addToGet.push(getRandomIdentity().getPrincipal());
        }
        for (let i = 0; i < ~~(Math.random() * 256); ++i) {
          addToVerify.push(getRandomIdentity().getPrincipal());
        }
        const createResult = await invoiceCanisterInstaller.create_invoice(
          getTestCreateInvoiceArgs({
            whichToken: SupportedTokens.asVariantTagLiteral.ICP,
            amountDue: billableAmountDue,
            getPermissions: addToGet,
            verifyPermissions: addToVerify,
          }),
        );
        expect(createResult?.ok?.invoice).toBeTruthy();
        const { amountDue, creator, token, paid, amountPaid, tokenVerbose, permissions } =
          createResult.ok.invoice;
        expect(billableAmountDue).toStrictEqual(amountDue);
        expect(creator).toStrictEqual(installerPrincipal);
        expect(token).toStrictEqual(SupportedTokens.asVariantUnitObj.ICP);
        expect(!paid).toBe(true);
        expect(amountPaid).toStrictEqual(0n);
        expect(tokenVerbose?.fee).toStrictEqual(10000n);
        expect(tokenVerbose?.name.includes('Internet Computer Protocol')).toBe(true);
        expect(`${tokenVerbose?.decimals}`).toStrictEqual('8');
        expect(permissions).toBeTruthy();
        const { canGet, canVerify } = permissions[0];
        // Compare equality using prinicipals sorted as strings.
        const canGetP = canGet.map(p => p.toString()).sort();
        const canVerifyP = canVerify.map(p => p.toString()).sort();
        addToGet = addToGet.map(p => p.toString()).sort();
        addToVerify = addToVerify.map(p => p.toString()).sort();
        expect(canGetP).toStrictEqual(addToGet);
        expect(canVerifyP).toStrictEqual(addToVerify);
      });
      it('should correctly create an invoice with details | #ICP -> ok', async () => {
        const descrip =
          'Nature is an infinite sphere of which the center is everywhere and the circumference nowhere';
        const met = { Seller: 'Savvy ICP Enthusiast' };
        const createResult = await invoiceCanisterInstaller.create_invoice(
          getTestCreateInvoiceArgs({
            whichToken: SupportedTokens.asVariantTagLiteral.ICP,
            amountDue: billableAmountDue,
            description: descrip,
            meta: met,
          }),
        );
        expect(createResult?.ok?.invoice).toBeTruthy();
        const { amountDue, creator, token, paid, amountPaid, tokenVerbose, details } =
          createResult.ok.invoice;
        expect(billableAmountDue).toStrictEqual(amountDue);
        expect(creator).toStrictEqual(installerPrincipal);
        expect(token).toStrictEqual(SupportedTokens.asVariantUnitObj.ICP);
        expect(!paid).toBe(true);
        expect(amountPaid).toStrictEqual(0n);
        expect(tokenVerbose?.fee).toStrictEqual(10000n);
        expect(tokenVerbose?.name.includes('Internet Computer Protocol')).toBe(true);
        expect(`${tokenVerbose?.decimals}`).toStrictEqual('8');
        expect(details).toBeTruthy();
        const { description, meta } = details[0];
        expect(description).toStrictEqual(descrip);
        expect(JSON.parse(Buffer.from(meta).toString('utf8'))).toStrictEqual(met);
      });
      it('should allow a caller on allowed creators list to correctly create an invoice | #ICP -> ok', async () => {
        const createResult = await invoiceCreator.create_invoice(
          getTestCreateInvoiceArgs({
            whichToken: SupportedTokens.asVariantTagLiteral.ICP,
            amountDue: billableAmountDue,
          }),
        );
        expect(createResult?.ok?.invoice).toBeTruthy();
        const { amountDue, creator, token, paid, amountPaid, tokenVerbose } =
          createResult.ok.invoice;
        expect(billableAmountDue).toStrictEqual(amountDue);
        expect(creator).toStrictEqual(invoiceCreatorPrincipal);
        expect(token).toStrictEqual(SupportedTokens.asVariantUnitObj.ICP);
        expect(!paid).toBe(true);
        expect(amountPaid).toStrictEqual(0n);
        expect(tokenVerbose?.fee).toStrictEqual(10000n);
        expect(tokenVerbose?.name.includes('Internet Computer Protocol')).toBe(true);
        expect(`${tokenVerbose?.decimals}`).toStrictEqual('8');
      });
    });
    describe('Test create_invoice Functionality for Invoices using #ICP Type', async () => {
      it('should correctly create an invoice | #ICP -> ok', async () => {
        const createResult = await invoiceCanisterInstaller.create_invoice(
          getTestCreateInvoiceArgs({
            whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
            amountDue: billableAmountDue,
          }),
        );
        expect(createResult?.ok?.invoice).toBeTruthy();
        const { amountDue, creator, token, paid, amountPaid, tokenVerbose } =
          createResult.ok.invoice;
        expect(billableAmountDue).toStrictEqual(amountDue);
        expect(creator).toStrictEqual(installerPrincipal);
        expect(token).toStrictEqual(SupportedTokens.asVariantUnitObj.ICP_nns);
        expect(!paid).toBe(true);
        expect(amountPaid).toStrictEqual(0n);
        expect(tokenVerbose?.fee).toStrictEqual(10000n);
        expect(tokenVerbose?.name.includes('Internet Computer Protocol')).toBe(true);
        expect(`${tokenVerbose?.decimals}`).toStrictEqual('8');
      });
      it('should correctly create an invoice with permission | #ICP -> ok', async () => {
        let addToGet = [],
          addToVerify = [];
        for (let i = 0; i < ~~(Math.random() * 256); ++i) {
          addToGet.push(getRandomIdentity().getPrincipal());
        }
        for (let i = 0; i < ~~(Math.random() * 256); ++i) {
          addToVerify.push(getRandomIdentity().getPrincipal());
        }
        const createResult = await invoiceCanisterInstaller.create_invoice(
          getTestCreateInvoiceArgs({
            whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
            amountDue: billableAmountDue,
            getPermissions: addToGet,
            verifyPermissions: addToVerify,
          }),
        );
        expect(createResult?.ok?.invoice).toBeTruthy();
        const { amountDue, creator, token, paid, amountPaid, tokenVerbose, permissions } =
          createResult.ok.invoice;
        expect(billableAmountDue).toStrictEqual(amountDue);
        expect(creator).toStrictEqual(installerPrincipal);
        expect(token).toStrictEqual(SupportedTokens.asVariantUnitObj.ICP_nns);
        expect(!paid).toBe(true);
        expect(amountPaid).toStrictEqual(0n);
        expect(tokenVerbose?.fee).toStrictEqual(10000n);
        expect(tokenVerbose?.name.includes('Internet Computer Protocol')).toBe(true);
        expect(`${tokenVerbose?.decimals}`).toStrictEqual('8');
        expect(permissions).toBeTruthy();
        const { canGet, canVerify } = permissions[0];
        const canGetP = canGet.map(p => p.toString()).sort();
        const canVerifyP = canVerify.map(p => p.toString()).sort();
        addToGet = addToGet.map(p => p.toString()).sort();
        addToVerify = addToVerify.map(p => p.toString()).sort();
        expect(canGetP).toStrictEqual(addToGet);
        expect(canVerifyP).toStrictEqual(addToVerify);
      });
      it('should correctly create an invoice with details | #ICP -> ok', async () => {
        const descrip =
          'Nature is an infinite sphere of which the center is everywhere and the circumference nowhere';
        const met = { Seller: 'Savvy ICP Enthusiast' };
        const createResult = await invoiceCanisterInstaller.create_invoice(
          getTestCreateInvoiceArgs({
            whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
            amountDue: billableAmountDue,
            description: descrip,
            meta: met,
          }),
        );
        expect(createResult?.ok?.invoice).toBeTruthy();
        const { amountDue, creator, token, paid, amountPaid, tokenVerbose, details } =
          createResult.ok.invoice;
        expect(billableAmountDue).toStrictEqual(amountDue);
        expect(creator).toStrictEqual(installerPrincipal);
        expect(token).toStrictEqual(SupportedTokens.asVariantUnitObj.ICP_nns);
        expect(!paid).toBe(true);
        expect(amountPaid).toStrictEqual(0n);
        expect(tokenVerbose?.fee).toStrictEqual(10000n);
        expect(tokenVerbose?.name.includes('Internet Computer Protocol')).toBe(true);
        expect(`${tokenVerbose?.decimals}`).toStrictEqual('8');
        expect(details).toBeTruthy();
        const { description, meta } = details[0];
        expect(description).toStrictEqual(descrip);
        expect(JSON.parse(Buffer.from(meta).toString('utf8'))).toStrictEqual(met);
      });
      it('should allow a caller on allowed creators list to correctly create an invoice | #ICP -> ok', async () => {
        const createResult = await invoiceCreator.create_invoice(
          getTestCreateInvoiceArgs({
            whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
            amountDue: billableAmountDue,
          }),
        );
        expect(createResult?.ok?.invoice).toBeTruthy();
        const { amountDue, creator, token, paid, amountPaid, tokenVerbose } =
          createResult.ok.invoice;
        expect(billableAmountDue).toStrictEqual(amountDue);
        expect(creator).toStrictEqual(invoiceCreatorPrincipal);
        expect(token).toStrictEqual(SupportedTokens.asVariantUnitObj.ICP_nns);
        expect(!paid).toBe(true);
        expect(amountPaid).toStrictEqual(0n);
        expect(tokenVerbose?.fee).toStrictEqual(10000n);
        expect(tokenVerbose?.name.includes('Internet Computer Protocol')).toBe(true);
        expect(`${tokenVerbose?.decimals}`).toStrictEqual('8');
      });
    });
    describe('Test create_invoice Functionality for Invoices using #ICRC1_ExampleToken Type', async () => {
      it('should correctly create an invoice | #ICRC1_ExampleToken -> ok', async () => {
        const createResult = await invoiceCanisterInstaller.create_invoice(
          getTestCreateInvoiceArgs({
            whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
            amountDue: billableAmountDue,
          }),
        );
        expect(createResult?.ok?.invoice).toBeTruthy();
        const { amountDue, creator, token, paid, amountPaid, tokenVerbose } =
          createResult.ok.invoice;
        expect(billableAmountDue).toStrictEqual(amountDue);
        expect(creator).toStrictEqual(installerPrincipal);
        expect(token).toStrictEqual(SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken);
        expect(!paid).toBe(true);
        expect(amountPaid).toStrictEqual(0n);
        expect(tokenVerbose?.fee).toStrictEqual(10000n);
        expect(
          tokenVerbose?.name.includes('Internet Computer Random Currency One Example Token'),
        ).toBe(true);
        expect(`${tokenVerbose?.decimals}`).toStrictEqual('8');
      });
      it('should correctly create an invoice with permission | #ICRC1_ExampleToken -> ok', async () => {
        let addToGet = [],
          addToVerify = [];
        for (let i = 0; i < ~~(Math.random() * 256); ++i) {
          addToGet.push(getRandomIdentity().getPrincipal());
        }
        for (let i = 0; i < ~~(Math.random() * 256); ++i) {
          addToVerify.push(getRandomIdentity().getPrincipal());
        }
        const createResult = await invoiceCanisterInstaller.create_invoice(
          getTestCreateInvoiceArgs({
            whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
            amountDue: billableAmountDue,
            getPermissions: addToGet,
            verifyPermissions: addToVerify,
          }),
        );
        expect(createResult?.ok?.invoice).toBeTruthy();
        const { amountDue, creator, token, paid, amountPaid, tokenVerbose, permissions } =
          createResult.ok.invoice;
        expect(billableAmountDue).toStrictEqual(amountDue);
        expect(creator).toStrictEqual(installerPrincipal);
        expect(token).toStrictEqual(SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken);
        expect(!paid).toBe(true);
        expect(amountPaid).toStrictEqual(0n);
        expect(tokenVerbose?.fee).toStrictEqual(10000n);
        expect(
          tokenVerbose?.name.includes('Internet Computer Random Currency One Example Token'),
        ).toBe(true);
        expect(`${tokenVerbose?.decimals}`).toStrictEqual('8');
        expect(permissions).toBeTruthy();
        const { canGet, canVerify } = permissions[0];
        const canGetP = canGet.map(p => p.toString()).sort();
        const canVerifyP = canVerify.map(p => p.toString()).sort();
        addToGet = addToGet.map(p => p.toString()).sort();
        addToVerify = addToVerify.map(p => p.toString()).sort();
        expect(canGetP).toStrictEqual(addToGet);
        expect(canVerifyP).toStrictEqual(addToVerify);
      });
      it('should correctly create an invoice with details | #ICRC1_ExampleToken -> ok', async () => {
        const descrip =
          'Nature is an infinite sphere of which the center is everywhere and the circumference nowhere';
        const met = { Seller: 'Savvy ICP Enthusiast' };
        const createResult = await invoiceCanisterInstaller.create_invoice(
          getTestCreateInvoiceArgs({
            whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
            amountDue: billableAmountDue,
            description: descrip,
            meta: met,
          }),
        );
        expect(createResult?.ok?.invoice).toBeTruthy();
        const { amountDue, creator, token, paid, amountPaid, tokenVerbose, details } =
          createResult.ok.invoice;
        expect(billableAmountDue).toStrictEqual(amountDue);
        expect(creator).toStrictEqual(installerPrincipal);
        expect(token).toStrictEqual(SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken);
        expect(!paid).toBe(true);
        expect(amountPaid).toStrictEqual(0n);
        expect(tokenVerbose?.fee).toStrictEqual(10000n);
        expect(
          tokenVerbose?.name.includes('Internet Computer Random Currency One Example Token'),
        ).toBe(true);
        expect(`${tokenVerbose?.decimals}`).toStrictEqual('8');
        expect(details).toBeTruthy();
        const { description, meta } = details[0];
        expect(description).toStrictEqual(descrip);
        expect(JSON.parse(Buffer.from(meta).toString('utf8'))).toStrictEqual(met);
      });
      it('should allow a caller on allowed creators list to correctly create an invoice | #ICRC1_ExampleToken -> ok', async () => {
        const createResult = await invoiceCreator.create_invoice(
          getTestCreateInvoiceArgs({
            whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
            amountDue: billableAmountDue,
          }),
        );
        expect(createResult?.ok?.invoice).toBeTruthy();
        const { amountDue, creator, token, paid, amountPaid, tokenVerbose } =
          createResult.ok.invoice;
        expect(billableAmountDue).toStrictEqual(amountDue);
        expect(creator).toStrictEqual(invoiceCreatorPrincipal);
        expect(token).toStrictEqual(SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken);
        expect(!paid).toBe(true);
        expect(amountPaid).toStrictEqual(0n);
        expect(tokenVerbose?.fee).toStrictEqual(10000n);
        expect(
          tokenVerbose?.name.includes('Internet Computer Random Currency One Example Token'),
        ).toBe(true);
        expect(`${tokenVerbose?.decimals}`).toStrictEqual('8');
      });
    });
    describe('Test create_invoice Functionality for Invoices using #ICRC1_ExampleToken2 Type', async () => {
      it('should correctly create an invoice | #ICRC1_ExampleToken2 -> ok', async () => {
        const createResult = await invoiceCanisterInstaller.create_invoice(
          getTestCreateInvoiceArgs({
            whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
            amountDue: billableAmountDue,
          }),
        );
        expect(createResult?.ok?.invoice).toBeTruthy();
        const { amountDue, creator, token, paid, amountPaid, tokenVerbose } =
          createResult.ok.invoice;
        expect(billableAmountDue).toStrictEqual(amountDue);
        expect(creator).toStrictEqual(installerPrincipal);
        expect(token).toStrictEqual(SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2);
        expect(!paid).toBe(true);
        expect(amountPaid).toStrictEqual(0n);
        expect(tokenVerbose?.fee).toStrictEqual(10000n);
        expect(
          tokenVerbose?.name.includes('Internet Computer Random Currency One Example Token'),
        ).toBe(true);
        expect(`${tokenVerbose?.decimals}`).toStrictEqual('8');
      });
      it('should correctly create an invoice with permission | #ICRC1_ExampleToken2 -> ok', async () => {
        let addToGet = [],
          addToVerify = [];
        for (let i = 0; i < ~~(Math.random() * 256); ++i) {
          addToGet.push(getRandomIdentity().getPrincipal());
        }
        for (let i = 0; i < ~~(Math.random() * 256); ++i) {
          addToVerify.push(getRandomIdentity().getPrincipal());
        }
        const createResult = await invoiceCanisterInstaller.create_invoice(
          getTestCreateInvoiceArgs({
            whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
            amountDue: billableAmountDue,
            getPermissions: addToGet,
            verifyPermissions: addToVerify,
          }),
        );
        expect(createResult?.ok?.invoice).toBeTruthy();
        const { amountDue, creator, token, paid, amountPaid, tokenVerbose, permissions } =
          createResult.ok.invoice;
        expect(billableAmountDue).toStrictEqual(amountDue);
        expect(creator).toStrictEqual(installerPrincipal);
        expect(token).toStrictEqual(SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2);
        expect(!paid).toBe(true);
        expect(amountPaid).toStrictEqual(0n);
        expect(tokenVerbose?.fee).toStrictEqual(10000n);
        expect(
          tokenVerbose?.name.includes('Internet Computer Random Currency One Example Token'),
        ).toBe(true);
        expect(`${tokenVerbose?.decimals}`).toStrictEqual('8');
        expect(permissions).toBeTruthy();
        const { canGet, canVerify } = permissions[0];
        const canGetP = canGet.map(p => p.toString()).sort();
        const canVerifyP = canVerify.map(p => p.toString()).sort();
        addToGet = addToGet.map(p => p.toString()).sort();
        addToVerify = addToVerify.map(p => p.toString()).sort();
        expect(canGetP).toStrictEqual(addToGet);
        expect(canVerifyP).toStrictEqual(addToVerify);
      });
      it('should correctly create an invoice with details | #ICRC1_ExampleToken2 -> ok', async () => {
        const descrip =
          'Nature is an infinite sphere of which the center is everywhere and the circumference nowhere';
        const met = { Seller: 'Savvy ICP Enthusiast' };
        const createResult = await invoiceCanisterInstaller.create_invoice(
          getTestCreateInvoiceArgs({
            whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
            amountDue: billableAmountDue,
            description: descrip,
            meta: met,
          }),
        );
        expect(createResult?.ok?.invoice).toBeTruthy();
        const { amountDue, creator, token, paid, amountPaid, tokenVerbose, details } =
          createResult.ok.invoice;
        expect(billableAmountDue).toStrictEqual(amountDue);
        expect(creator).toStrictEqual(installerPrincipal);
        expect(token).toStrictEqual(SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2);
        expect(!paid).toBe(true);
        expect(amountPaid).toStrictEqual(0n);
        expect(tokenVerbose?.fee).toStrictEqual(10000n);
        expect(
          tokenVerbose?.name.includes('Internet Computer Random Currency One Example Token'),
        ).toBe(true);
        expect(`${tokenVerbose?.decimals}`).toStrictEqual('8');
        expect(details).toBeTruthy();
        const { description, meta } = details[0];
        expect(description).toStrictEqual(descrip);
        expect(JSON.parse(Buffer.from(meta).toString('utf8'))).toStrictEqual(met);
      });
      it('should allow a caller on allowed creators list to correctly create an invoice | #ICRC1_ExampleToken2 -> ok', async () => {
        const createResult = await invoiceCreator.create_invoice(
          getTestCreateInvoiceArgs({
            whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
            amountDue: billableAmountDue,
          }),
        );
        expect(createResult?.ok?.invoice).toBeTruthy();
        const { amountDue, creator, token, paid, amountPaid, tokenVerbose } =
          createResult.ok.invoice;
        expect(billableAmountDue).toStrictEqual(amountDue);
        expect(creator).toStrictEqual(invoiceCreatorPrincipal);
        expect(token).toStrictEqual(SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2);
        expect(!paid).toBe(true);
        expect(amountPaid).toStrictEqual(0n);
        expect(tokenVerbose?.fee).toStrictEqual(10000n);
        expect(
          tokenVerbose?.name.includes('Internet Computer Random Currency One Example Token'),
        ).toBe(true);
        expect(`${tokenVerbose?.decimals}`).toStrictEqual('8');
      });
    });
  });
  describe('Test #err Results Returned from create_invoice', async () => {
    it('should reject if given meta too large | -> err #MetaTooLarge', async () => {
      const createResult = await invoiceCanisterInstaller.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
          amountDue: billableAmountDue,
          meta: new Array(32_001).fill(0),
        }),
      );
      expect(createResult?.err?.kind).toStrictEqual({ MetaTooLarge: null });
    });
    it('should reject if given description too large | -> err #DescriptionTooLarge', async () => {
      const createResult = await invoiceCanisterInstaller.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
          amountDue: billableAmountDue,
          description:
            'This entire globe, this star, not being subject to death, and dissolution and annihilation being impossible anywhere in Nature, from time to time renews itself by changing and altering all its parts. There is no absolute up or down, as Aristotle taught; no absolute position in space; but the position of a body is relative to that of other bodies. Everywhere there is incessant relative change in position throughout the universe, and the observer is always at the centre of things.',
        }),
      );
      expect(createResult?.err?.kind).toStrictEqual({ DescriptionTooLarge: null });
    });
    it('should reject if given too many principals for verify permissions list | -> err #TooManyPermissions', async () => {
      const tooManyPermission = [];
      for (let i = 0; i < 257; ++i) {
        tooManyPermission.push(getRandomIdentity().getPrincipal());
      }
      let createResult = await invoiceCanisterInstaller.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
          amountDue: billableAmountDue,
          verifyPermissions: tooManyPermission,
        }),
      );
      expect(createResult?.err?.kind).toStrictEqual({ TooManyPermissions: null });
    });
    it('should reject if given too many principals for get permissions list | -> err #TooManyPermissions', async () => {
      const tooManyPermission = [];
      for (let i = 0; i < 257; ++i) {
        tooManyPermission.push(getRandomIdentity().getPrincipal());
      }
      const createResult = await invoiceCanisterInstaller.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP,
          amountDue: billableAmountDue,
          getPermissions: tooManyPermission,
        }),
      );
      expect(createResult?.err?.kind).toStrictEqual({ TooManyPermissions: null });
    });
    it('should reject if given amount due less than transfer fee for each token type | -> err #InsufficientAmountDue', async () => {
      let createResult = await invoiceCanisterInstaller.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP,
          amountDue: 10n,
        }),
      );
      expect(createResult?.err?.kind).toStrictEqual({ InsufficientAmountDue: null });
      createResult = await invoiceCanisterInstaller.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
          amountDue: 10n,
        }),
      );
      expect(createResult?.err?.kind).toStrictEqual({ InsufficientAmountDue: null });
      createResult = await invoiceCanisterInstaller.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
          amountDue: 10n,
        }),
      );
      expect(createResult?.err?.kind).toStrictEqual({ InsufficientAmountDue: null });
      createResult = await invoiceCanisterInstaller.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
          amountDue: 10n,
        }),
      );
      expect(createResult?.err?.kind).toStrictEqual({ InsufficientAmountDue: null });
    });
    it('should reject for creating invoice of each token type if caller not authorized | -> err #NotAuthorized', async () => {
      let createResult = await getRandomActor().create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP,
        }),
      );
      expect(createResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
      createResult = await getRandomActor().create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
        }),
      );
      expect(createResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
      createResult = await getRandomActor().create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
        }),
      );
      expect(createResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
      createResult = await getRandomActor().create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
        }),
      );
      expect(createResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
});
