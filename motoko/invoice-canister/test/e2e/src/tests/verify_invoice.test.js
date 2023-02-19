// Note each it test uses its own invoice to test against.

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

describe('Test verify_invoice Functionality', async () => {
  const installerPrincipal = installerIdentity.getPrincipal();
  // Adding a new identity to create the invoices to verify
  // subaccount to subaccount transfer upon successfuly verification.
  const invoiceCreatorIdentity = getRandomIdentity();
  const result = await invoiceCanisterInstaller.add_allowed_creator({
    who: invoiceCreatorIdentity.getPrincipal(),
  });
  if (!result.ok) {
    throw new Error(
      'Could not add allowed creator aborting verify_invoice tests\ncall result was ' +
        JSON.stringify(result),
    );
  }
  const invoiceCreator = getActorByIdentity(invoiceCreatorIdentity);
  // Same used for all tokens in this project.
  const transferFeeAmount = 10000n;
  // Billable amount used to create invoices.
  const billedAmountDue = 1000000n;
  // Half the amount to test incomplete payment.
  const halfAmount = 500000n;
  // Amount transferred to simulate partial payment.
  const halfAmountPlusTransferFee = halfAmount + transferFeeAmount;
  // Amount transferred to complete payment.
  const payableTransferAmount = billedAmountDue + transferFeeAmount;
  // Amount transferred to invoice creator's principal subaccount
  // after invoice is successfully verified paid.
  const amountCredited = billedAmountDue - transferFeeAmount;
  describe('Test Token-Non Specific #err Results Return From verify_invoice', () => {
    // Confirm No Invoice Found for invoice ids that have not been created.
    it('should reject and return err kind #NotFound if no invoice for given id exists and caller authorized', async () => {
      const verifyResult = await invoiceCanisterInstaller.verify_invoice({ id: "invalidId" });
      expect(verifyResult?.err?.kind).toStrictEqual({ NotFound: null });
    });

    // Confirm if the caller is not authorized (not the original invoice canister deployer or on the
    // allowed creators list) they cannot determine if an invoice exists for a given id.
    it('should reject and return err kind #NotAuthorized if no invoice for given id exists and caller not authorized', async () => {
      const verifyResult = await getRandomActor().verify_invoice({ id: "invalidId" });
      expect(verifyResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
  describe('Test verify_invoice Functionality for #ICP Type', () => {
    it('should reject if zero amount has been paid | #ICP -> err #Unpaid', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP,
          amountDue: billedAmountDue,
          verifyPermissions: [installerPrincipal],
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id } = createInvoiceResult.ok.invoice;

      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({ Unpaid: null });
      // Also returned for someone on verify permissions list.
      verifyResult = await invoiceCanisterInstaller.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({ Unpaid: null });
    });
    it('should reject invoice balance has only been partially paid | #ICP -> err #IncompletePayment', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP,
          amountDue: billedAmountDue,
          verifyPermissions: [installerPrincipal],
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Transfer half the amount to pay the amount due.
      const transferResult = await invoiceCanisterInstaller.transfer({
        tokenAmount: {
          // is just equal to { ICP: { e8s: amount }}
          [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: halfAmountPlusTransferFee },
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm the transaction went through.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm incomplete payment with partial amount.
      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({
        IncompletePayment: {
          partialAmountPaid: {
            [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: halfAmount },
          },
        },
      });
      // Also returned for someone on verify permissions list.
      verifyResult = await invoiceCanisterInstaller.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({
        IncompletePayment: {
          partialAmountPaid: {
            [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: halfAmount },
          },
        },
      });
    });
    it('should correctly mark & transfer proceeds to creator if amount due is paid | #ICP -> ok #VerifiedPaid', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP,
          amountDue: billedAmountDue,
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Get the invoice creator's principal subaccount balance.
      const balanceBeforeResult = await invoiceCreator.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP,
      });
      // Confirm the balance call returned ok.
      expect(balanceBeforeResult?.ok).toBeTruthy();
      const balanceBefore = balanceBeforeResult.ok.balance.ICP.e8s;
      // Transfer the amount to pay the invoice balance amount due.
      const transferResult = await invoiceCanisterInstaller.transfer({
        tokenAmount: {
          // is just equal to { ICP: { e8s: amount }}
          [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: payableTransferAmount },
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm invoice updated as paid.
      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.ok?.VerifiedPaid?.invoice).toBeTruthy();
      let { paid, amountPaid, amountDue, verifiedPaidAtTime } =
        verifyResult.ok.VerifiedPaid.invoice;
      expect(amountPaid >= amountDue).toBe(true);
      expect(paid).toBe(true);
      expect(verifiedPaidAtTime).toBeTruthy();
      // Confirm proceeds transferred to invoice creator's principal subaccount.
      const balanceAfterResult = await invoiceCreator.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP,
      });
      expect(balanceAfterResult?.ok).toBeTruthy();
      const balanceAfter = balanceAfterResult.ok.balance.ICP.e8s;
      expect(balanceAfter >= balanceBefore + amountCredited);
    });
    it('should return already verified if invoice already verified | #ICP -> ok #VerifiedAlready', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP,
          amountDue: billedAmountDue,
          verifyPermissions: [installerPrincipal],
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Transfer the amount to pay the invoice balance amount due.
      const transferResult = await invoiceCanisterInstaller.transfer({
        tokenAmount: {
          // is just equal to { ICP: { e8s: amount }}
          [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: payableTransferAmount },
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm invoice updated as paid.
      const verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.ok?.VerifiedPaid?.invoice).toBeTruthy();
      let { paid, amountPaid, amountDue, verifiedPaidAtTime } =
        verifyResult.ok.VerifiedPaid.invoice;
      const invoiceFirstTime = verifyResult.ok.VerifiedPaid.invoice;
      expect(amountPaid >= amountDue).toBe(true);
      expect(paid).toBe(true);
      expect(verifiedPaidAtTime).toBeTruthy();
      // Confirm invoice already verified paid returns already verified paid if verified again.
      const verifyAgainResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyAgainResult?.ok?.VerifiedAlready?.invoice).toBeTruthy();
      let {
        paid: againPaid,
        amountPaid: againAmountPaid,
        amountDue: againAmount,
        verifiedPaidAtTime: againAtTime,
      } = verifyAgainResult.ok.VerifiedAlready.invoice;
      const invoiceSecondTime = verifyAgainResult.ok.VerifiedAlready.invoice;
      expect(againAmountPaid >= againAmount).toBe(true);
      expect(againPaid).toBe(true);
      expect(againAtTime).toBeTruthy();
      expect(invoiceFirstTime).toStrictEqual(invoiceSecondTime);
    });
    it('should allow for someone on verify permissions list to verify | #ICP -> ok (caller on verify permissions)', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP,
          amountDue: billedAmountDue,
          verifyPermissions: [installerPrincipal],
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Transfer the amount to pay the invoice balance amount due.
      const transferResult = await invoiceCanisterInstaller.transfer({
        tokenAmount: {
          // is just equal to { ICP: { e8s: amount }}
          [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: payableTransferAmount },
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm invoice updated as paid for principal on verify permissions list.
      const verifyResult = await invoiceCanisterInstaller.verify_invoice({ id });
      expect(verifyResult?.ok?.VerifiedPaid?.invoice).toBeTruthy();
      let { paid, amountPaid, amountDue, verifiedPaidAtTime } =
        verifyResult.ok.VerifiedPaid.invoice;
      expect(amountPaid >= amountDue).toBe(true);
      expect(paid).toBe(true);
      expect(verifiedPaidAtTime).toBeTruthy();
    });
    it('should reject if unauthorized caller | #ICP -> err #NotAuthorized', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP,
          amountDue: billedAmountDue,
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id } = createInvoiceResult.ok.invoice;
      // Confirm random unauthorized caller can't verify_invoice.
      let randomVerifyResult = await getRandomActor().verify_invoice({ id });
      expect(randomVerifyResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
      // Confirm the invoice canister deployer isn't a special case if not added to verify permissions list.
      randomVerifyResult = await invoiceCanisterInstaller.verify_invoice({ id });
      expect(randomVerifyResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
      // Confirm the invoice creator isn't a special case either.
      const anotherCreateResult = await invoiceCanisterInstaller.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP,
          amountDue: billedAmountDue,
        }),
      );
      expect(anotherCreateResult?.ok?.invoice).toBeTruthy();
      randomVerifyResult = await invoiceCreator.verify_invoice({
        id: anotherCreateResult.ok.invoice.id,
      });
      expect(randomVerifyResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
      // Or another random authorized caller.
      randomVerifyResult = await getRandomActor().verify_invoice({
        id: anotherCreateResult.ok.invoice.id,
      });
      expect(randomVerifyResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
  describe('Test verify_invoice Functionality for #ICP_nns Type', () => {
    it('should reject if zero amount has been paid | #ICP_nns -> err #Unpaid', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
          amountDue: billedAmountDue,
          verifyPermissions: [installerPrincipal],
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id } = createInvoiceResult.ok.invoice;

      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({ Unpaid: null });
      // Also returned for someone on verify permissions list.
      verifyResult = await invoiceCanisterInstaller.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({ Unpaid: null });
    });
    it('should reject invoice balance has only been partially paid | #ICP_nns -> err #IncompletePayment', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
          amountDue: billedAmountDue,
          verifyPermissions: [installerPrincipal],
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Transfer half the amount to pay the amount due.
      const transferResult = await invoiceCanisterInstaller.transfer({
        tokenAmount: {
          // is just equal to { ICP_nns: { e8s: amount }}
          [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: halfAmountPlusTransferFee },
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm the transaction went through.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm incomplet payment with partial amount.
      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({
        IncompletePayment: {
          partialAmountPaid: {
            [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: halfAmount },
          },
        },
      });
      // Also returned for someone on verify permissions list.
      verifyResult = await invoiceCanisterInstaller.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({
        IncompletePayment: {
          partialAmountPaid: {
            [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: halfAmount },
          },
        },
      });
    });
    it('should correctly mark & transfer proceeds to creator if amount due is paid | #ICP_nns -> ok #VerifiedPaid', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
          amountDue: billedAmountDue,
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Get the invoice creator's principal subaccount balance.
      const balanceBeforeResult = await invoiceCreator.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP_nns,
      });
      // Confirm the balance call returned ok.
      expect(balanceBeforeResult?.ok).toBeTruthy();
      const balanceBefore = balanceBeforeResult.ok.balance.ICP_nns.e8s;
      // Transfer the amount to pay the invoice balance amount due.
      const transferResult = await invoiceCanisterInstaller.transfer({
        tokenAmount: {
          // is just equal to { ICP_nns: { e8s: amount }}
          [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: payableTransferAmount },
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm invoice updated as paid.
      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.ok?.VerifiedPaid?.invoice).toBeTruthy();
      let { paid, amountPaid, amountDue, verifiedPaidAtTime } =
        verifyResult.ok.VerifiedPaid.invoice;
      expect(amountPaid >= amountDue).toBe(true);
      expect(paid).toBe(true);
      expect(verifiedPaidAtTime).toBeTruthy();
      // Confirm proceeds transferred to invoice creator's principal subaccount.
      const balanceAfterResult = await invoiceCreator.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP_nns,
      });
      expect(balanceAfterResult?.ok).toBeTruthy();
      const balanceAfter = balanceAfterResult.ok.balance.ICP_nns.e8s;
      expect(balanceAfter >= balanceBefore + amountCredited);
    });
    it('should return already verified if invoice already verified | #ICP_nns -> ok #VerifiedAlready', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
          amountDue: billedAmountDue,
          verifyPermissions: [installerPrincipal],
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Transfer the amount to pay the invoice balance amount due.
      const transferResult = await invoiceCanisterInstaller.transfer({
        tokenAmount: {
          // is just equal to { ICP_nns: { e8s: amount }}
          [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: payableTransferAmount },
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm invoice updated as paid.
      const verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.ok?.VerifiedPaid?.invoice).toBeTruthy();
      let { paid, amountPaid, amountDue, verifiedPaidAtTime } =
        verifyResult.ok.VerifiedPaid.invoice;
      const invoiceFirstTime = verifyResult.ok.VerifiedPaid.invoice;
      expect(amountPaid >= amountDue).toBe(true);
      expect(paid).toBe(true);
      expect(verifiedPaidAtTime).toBeTruthy();
      // Confirm invoice already verified paid returns already verified paid if verified again.
      const verifyAgainResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyAgainResult?.ok?.VerifiedAlready?.invoice).toBeTruthy();
      let {
        paid: againPaid,
        amountPaid: againAmountPaid,
        amountDue: againAmount,
        verifiedPaidAtTime: againAtTime,
      } = verifyAgainResult.ok.VerifiedAlready.invoice;
      const invoiceSecondTime = verifyAgainResult.ok.VerifiedAlready.invoice;
      expect(againAmountPaid >= againAmount).toBe(true);
      expect(againPaid).toBe(true);
      expect(againAtTime).toBeTruthy();
      expect(invoiceFirstTime).toStrictEqual(invoiceSecondTime);
    });
    it('should allow for someone on verify permissions list to verify | #ICP_nns -> ok (caller on verify permissions)', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
          amountDue: billedAmountDue,
          verifyPermissions: [installerPrincipal],
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Transfer the amount to pay the invoice balance amount due.
      const transferResult = await invoiceCanisterInstaller.transfer({
        tokenAmount: {
          // is just equal to { ICP_nns: { e8s: amount }}
          [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: payableTransferAmount },
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm invoice updated as paid for principal on verify permissions list.
      const verifyResult = await invoiceCanisterInstaller.verify_invoice({ id });
      expect(verifyResult?.ok?.VerifiedPaid?.invoice).toBeTruthy();
      let { paid, amountPaid, amountDue, verifiedPaidAtTime } =
        verifyResult.ok.VerifiedPaid.invoice;
      expect(amountPaid >= amountDue).toBe(true);
      expect(paid).toBe(true);
      expect(verifiedPaidAtTime).toBeTruthy();
    });
    it('should reject if unauthorized caller | #ICP_nns -> err #NotAuthorized', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
          amountDue: billedAmountDue,
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id } = createInvoiceResult.ok.invoice;
      // Confirm random unauthorized caller can't verify_invoice.
      let randomVerifyResult = await getRandomActor().verify_invoice({ id });
      expect(randomVerifyResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
      // Confirm the invoice canister deployer isn't a special case if not added to verify permissions list.
      randomVerifyResult = await invoiceCanisterInstaller.verify_invoice({ id });
      expect(randomVerifyResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
      // Confirm the invoice creator isn't a special case either.
      const anotherCreateResult = await invoiceCanisterInstaller.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
          amountDue: billedAmountDue,
        }),
      );
      expect(anotherCreateResult?.ok?.invoice).toBeTruthy();
      randomVerifyResult = await invoiceCreator.verify_invoice({
        id: anotherCreateResult.ok.invoice.id,
      });
      expect(randomVerifyResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
      // Or another random authorized caller.
      randomVerifyResult = await getRandomActor().verify_invoice({
        id: anotherCreateResult.ok.invoice.id,
      });
      expect(randomVerifyResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
  describe('Test verify_invoice Functionality for #ICRC1_ExampleToken Type', () => {
    it('should reject if zero amount has been paid | #ICRC1_ExampleToken -> err #Unpaid', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
          amountDue: billedAmountDue,
          verifyPermissions: [installerPrincipal],
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id } = createInvoiceResult.ok.invoice;

      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({ Unpaid: null });
      // Also returned for someone on verify permissions list.
      verifyResult = await invoiceCanisterInstaller.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({ Unpaid: null });
    });
    it('should reject invoice balance has only been partially paid | #ICRC1_ExampleToken -> err #IncompletePayment', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
          amountDue: billedAmountDue,
          verifyPermissions: [installerPrincipal],
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Transfer half the amount to pay the amount due.
      const transferResult = await invoiceCanisterInstaller.transfer({
        tokenAmount: {
          // is just equal to { ICRC1_ExampleToken: amount }
          [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: halfAmountPlusTransferFee,
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm the transaction went through.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm incomplet payment with partial amount.
      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({
        IncompletePayment: {
          partialAmountPaid: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: halfAmount,
          },
        },
      });
      // Also returned for someone on verify permissions list.
      verifyResult = await invoiceCanisterInstaller.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({
        IncompletePayment: {
          partialAmountPaid: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: halfAmount,
          },
        },
      });
    });
    it('should correctly mark & transfer proceeds to creator if amount due is paid | #ICRC1_ExampleToken -> ok #VerifiedPaid', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
          amountDue: billedAmountDue,
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Get the invoice creator's principal subaccount balance.
      const balanceBeforeResult = await invoiceCreator.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken,
      });
      // Confirm the balance call returned ok.
      expect(balanceBeforeResult?.ok).toBeTruthy();
      const balanceBefore = balanceBeforeResult.ok.balance.ICRC1_ExampleToken;
      // Transfer the amount to pay the invoice balance amount due.
      const transferResult = await invoiceCanisterInstaller.transfer({
        tokenAmount: {
          // is just equal to { ICRC1_ExampleToken: amount }
          [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: payableTransferAmount,
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm invoice updated as paid.
      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.ok?.VerifiedPaid?.invoice).toBeTruthy();
      let { paid, amountPaid, amountDue, verifiedPaidAtTime } =
        verifyResult.ok.VerifiedPaid.invoice;
      expect(amountPaid >= amountDue).toBe(true);
      expect(paid).toBe(true);
      expect(verifiedPaidAtTime).toBeTruthy();
      // Confirm proceeds transferred to invoice creator's principal subaccount.
      const balanceAfterResult = await invoiceCreator.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken,
      });
      expect(balanceAfterResult?.ok).toBeTruthy();
      const balanceAfter = balanceAfterResult.ok.balance.ICRC1_ExampleToken;
      expect(balanceAfter >= balanceBefore + amountCredited);
    });
    it('should return already verified if invoice already verified | #ICRC1_ExampleToken -> ok #VerifiedAlready', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
          amountDue: billedAmountDue,
          verifyPermissions: [installerPrincipal],
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Transfer the amount to pay the invoice balance amount due.
      const transferResult = await invoiceCanisterInstaller.transfer({
        tokenAmount: {
          // is just equal to { ICRC1_ExampleToken: amount }
          [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: payableTransferAmount,
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm invoice updated as paid.
      const verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.ok?.VerifiedPaid?.invoice).toBeTruthy();
      let { paid, amountPaid, amountDue, verifiedPaidAtTime } =
        verifyResult.ok.VerifiedPaid.invoice;
      const invoiceFirstTime = verifyResult.ok.VerifiedPaid.invoice;
      expect(amountPaid >= amountDue).toBe(true);
      expect(paid).toBe(true);
      expect(verifiedPaidAtTime).toBeTruthy();
      // Confirm invoice already verified paid returns already verified paid if verified again.
      const verifyAgainResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyAgainResult?.ok?.VerifiedAlready?.invoice).toBeTruthy();
      let {
        paid: againPaid,
        amountPaid: againAmountPaid,
        amountDue: againAmount,
        verifiedPaidAtTime: againAtTime,
      } = verifyAgainResult.ok.VerifiedAlready.invoice;
      const invoiceSecondTime = verifyAgainResult.ok.VerifiedAlready.invoice;
      expect(againAmountPaid >= againAmount).toBe(true);
      expect(againPaid).toBe(true);
      expect(againAtTime).toBeTruthy();
      expect(invoiceFirstTime).toStrictEqual(invoiceSecondTime);
    });
    it('should allow for someone on verify permissions list to verify | #ICRC1_ExampleToken -> ok (caller on verify permissions)', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
          amountDue: billedAmountDue,
          verifyPermissions: [installerPrincipal],
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Transfer the amount to pay the invoice balance amount due.
      const transferResult = await invoiceCanisterInstaller.transfer({
        tokenAmount: {
          // is just equal to { ICRC1_ExampleToken: amount }
          [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: payableTransferAmount,
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm invoice updated as paid for principal on verify permissions list.
      const verifyResult = await invoiceCanisterInstaller.verify_invoice({ id });
      expect(verifyResult?.ok?.VerifiedPaid?.invoice).toBeTruthy();
      let { paid, amountPaid, amountDue, verifiedPaidAtTime } =
        verifyResult.ok.VerifiedPaid.invoice;
      expect(amountPaid >= amountDue).toBe(true);
      expect(paid).toBe(true);
      expect(verifiedPaidAtTime).toBeTruthy();
    });
    it('should reject if unauthorized caller | #ICRC1_ExampleToken -> err #NotAuthorized', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
          amountDue: billedAmountDue,
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id } = createInvoiceResult.ok.invoice;
      // Confirm random unauthorized caller can't verify_invoice.
      let randomVerifyResult = await getRandomActor().verify_invoice({ id });
      expect(randomVerifyResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
      // Confirm the invoice canister deployer isn't a special case if not added to verify permissions list.
      randomVerifyResult = await invoiceCanisterInstaller.verify_invoice({ id });
      expect(randomVerifyResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
      // Confirm the invoice creator isn't a special case either.
      const anotherCreateResult = await invoiceCanisterInstaller.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
          amountDue: billedAmountDue,
        }),
      );
      expect(anotherCreateResult?.ok?.invoice).toBeTruthy();
      randomVerifyResult = await invoiceCreator.verify_invoice({
        id: anotherCreateResult.ok.invoice.id,
      });
      expect(randomVerifyResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
      // Or another random authorized caller.
      randomVerifyResult = await getRandomActor().verify_invoice({
        id: anotherCreateResult.ok.invoice.id,
      });
      expect(randomVerifyResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
  describe('Test verify_invoice Functionality for #ICRC1_ExampleToken2 Type', () => {
    it('should reject if zero amount has been paid | #ICRC1_ExampleToken2 -> err #Unpaid', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
          amountDue: billedAmountDue,
          verifyPermissions: [installerPrincipal],
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id } = createInvoiceResult.ok.invoice;

      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({ Unpaid: null });
      // Also returned for someone on verify permissions list.
      verifyResult = await invoiceCanisterInstaller.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({ Unpaid: null });
    });
    it('should reject invoice balance has only been partially paid | #ICRC1_ExampleToken2 -> err #IncompletePayment', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
          amountDue: billedAmountDue,
          verifyPermissions: [installerPrincipal],
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Transfer half the amount to pay the amount due.
      const transferResult = await invoiceCanisterInstaller.transfer({
        tokenAmount: {
          // is just equal to { ICRC1_ExampleToken2: amount }
          [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: halfAmountPlusTransferFee,
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm the transaction went through.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm incomplet payment with partial amount.
      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({
        IncompletePayment: {
          partialAmountPaid: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: halfAmount,
          },
        },
      });
      // Also returned for someone on verify permissions list.
      verifyResult = await invoiceCanisterInstaller.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({
        IncompletePayment: {
          partialAmountPaid: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: halfAmount,
          },
        },
      });
    });
    it('should correctly mark & transfer proceeds to creator if amount due is paid | #ICRC1_ExampleToken2 -> ok #VerifiedPaid', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
          amountDue: billedAmountDue,
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Get the invoice creator's principal subaccount balance.
      const balanceBeforeResult = await invoiceCreator.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2,
      });
      // Confirm the balance call returned ok.
      expect(balanceBeforeResult?.ok).toBeTruthy();
      const balanceBefore = balanceBeforeResult.ok.balance.ICRC1_ExampleToken2;
      // Transfer the amount to pay the invoice balance amount due.
      const transferResult = await invoiceCanisterInstaller.transfer({
        tokenAmount: {
          // is just equal to { ICRC1_ExampleToken2: amount }
          [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: payableTransferAmount,
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm invoice updated as paid.
      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.ok?.VerifiedPaid?.invoice).toBeTruthy();
      let { paid, amountPaid, amountDue, verifiedPaidAtTime } =
        verifyResult.ok.VerifiedPaid.invoice;
      expect(amountPaid >= amountDue).toBe(true);
      expect(paid).toBe(true);
      expect(verifiedPaidAtTime).toBeTruthy();
      // Confirm proceeds transferred to invoice creator's principal subaccount.
      const balanceAfterResult = await invoiceCreator.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2,
      });
      expect(balanceAfterResult?.ok).toBeTruthy();
      const balanceAfter = balanceAfterResult.ok.balance.ICRC1_ExampleToken2;
      expect(balanceAfter >= balanceBefore + amountCredited);
    });
    it('should return already verified if invoice already verified | #ICRC1_ExampleToken2 -> ok #VerifiedAlready', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
          amountDue: billedAmountDue,
          verifyPermissions: [installerPrincipal],
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Transfer the amount to pay the invoice balance amount due.
      const transferResult = await invoiceCanisterInstaller.transfer({
        tokenAmount: {
          // is just equal to { ICRC1_ExampleToken2: amount }
          [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: payableTransferAmount,
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm invoice updated as paid.
      const verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.ok?.VerifiedPaid?.invoice).toBeTruthy();
      let { paid, amountPaid, amountDue, verifiedPaidAtTime } =
        verifyResult.ok.VerifiedPaid.invoice;
      const invoiceFirstTime = verifyResult.ok.VerifiedPaid.invoice;
      expect(amountPaid >= amountDue).toBe(true);
      expect(paid).toBe(true);
      expect(verifiedPaidAtTime).toBeTruthy();
      // Confirm invoice already verified paid returns already verified paid if verified again.
      const verifyAgainResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyAgainResult?.ok?.VerifiedAlready?.invoice).toBeTruthy();
      let {
        paid: againPaid,
        amountPaid: againAmountPaid,
        amountDue: againAmount,
        verifiedPaidAtTime: againAtTime,
      } = verifyAgainResult.ok.VerifiedAlready.invoice;
      const invoiceSecondTime = verifyAgainResult.ok.VerifiedAlready.invoice;
      expect(againAmountPaid >= againAmount).toBe(true);
      expect(againPaid).toBe(true);
      expect(againAtTime).toBeTruthy();
      expect(invoiceFirstTime).toStrictEqual(invoiceSecondTime);
    });
    it('should allow for someone on verify permissions list to verify | #ICRC1_ExampleToken2 -> ok (caller on verify permissions)', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
          amountDue: billedAmountDue,
          verifyPermissions: [installerPrincipal],
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Transfer the amount to pay the invoice balance amount due.
      const transferResult = await invoiceCanisterInstaller.transfer({
        tokenAmount: {
          // is just equal to { ICRC1_ExampleToken2: amount }
          [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: payableTransferAmount,
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm invoice updated as paid for principal on verify permissions list.
      const verifyResult = await invoiceCanisterInstaller.verify_invoice({ id });
      expect(verifyResult?.ok?.VerifiedPaid?.invoice).toBeTruthy();
      let { paid, amountPaid, amountDue, verifiedPaidAtTime } =
        verifyResult.ok.VerifiedPaid.invoice;
      expect(amountPaid >= amountDue).toBe(true);
      expect(paid).toBe(true);
      expect(verifiedPaidAtTime).toBeTruthy();
    });
    it('should reject if unauthorized caller | #ICRC1_ExampleToken2 -> err #NotAuthorized', async () => {
      // Create an invoice to test.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
          amountDue: billedAmountDue,
        }),
      );
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id } = createInvoiceResult.ok.invoice;
      // Confirm random unauthorized caller can't verify_invoice.
      let randomVerifyResult = await getRandomActor().verify_invoice({ id });
      expect(randomVerifyResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
      // Confirm the invoice canister deployer isn't a special case if not added to verify permissions list.
      randomVerifyResult = await invoiceCanisterInstaller.verify_invoice({ id });
      expect(randomVerifyResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
      // Confirm the invoice creator isn't a special case either.
      const anotherCreateResult = await invoiceCanisterInstaller.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
          amountDue: billedAmountDue,
        }),
      );
      expect(anotherCreateResult?.ok?.invoice).toBeTruthy();
      randomVerifyResult = await invoiceCreator.verify_invoice({
        id: anotherCreateResult.ok.invoice.id,
      });
      expect(randomVerifyResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
      // Or another random authorized caller.
      randomVerifyResult = await getRandomActor().verify_invoice({
        id: anotherCreateResult.ok.invoice.id,
      });
      expect(randomVerifyResult?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
});
