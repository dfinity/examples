/*
 Note: Each token type 'description' group consists of the same following testing procedure:

  0) Create a balance recipient and invoice creator adding them as allowed creators.
  (token specific 'describe' section)
    1) Create an invoice to test against. 
    2) Confirm balance recipient's creator subaccount balance is 0.  

Note this (should be) the only test suite that has the results of one it dependent on another (1st and 2nd here). 

  (1st 'it': should correctly recover partial amount paid before invoice has been verified | #<token> -> #ok (case 1: partial refund))
    3) Transfer partial amount to invoice subaccount to recover.
    4) Confirm verify_invoice returns #IncompletePayment with partialAmountPaid equal to the transferred amount (less a transfer fee cost). 
    5) Call to recover that amount using the balance recipient's text encoded address (confirm correctly ok).
    6) Confirm verify_invoice returns #Unpaid.
    7) Double check by confirming get_invoice returns an invoice unpaid with amountPaid equal to 0. 
    8) Confirm balance recipient's creator subaccount balance is now the partial amount recovered (less a transfer fee cost). 
    
  (2nd 'it': should correctly recover amount mistakenly sent after invoice already verified | #<token> -> #ok (case 2: recover lost funds))
    9) Transfer invoice's complete amount due to invoice's subaccount. 
    10) Confirm verify_invoice now returns #VerifiedPaid with amount paid >= amount due.
    11) Transfer much more to invoice subaccount.
    12) Confirm verify_invoice now returns #VerifiedAlready.
    13) Call to recover that amount using the balance recipient's address (confirm correctly ok).
    14) Confirm verify_invoice still returns #VerifiedAlready.
    15) Double check by confirming get_invoice still returns the same invoice/'s data.
    16) Confirm the balance of the balance recipient's creator subaccount is equal to the sum of both amounts (less twice a transfer fee cost). 

  (3rd 'it': should reject if invoice subaccount balance is zero | #<token> -> err #NoBalance)
    1) Create an invoice to test against.
    2) Confirm call to recover returns err #NoBalance.

  (4th 'it': should reject if invoice subaccount balance not enough to cover transfer fee | #ICP -> err #InsufficientTransferAmount)
    1) Create an invoice to test against.
    2) Transfer an amount slightly more than the cost of a transfer fee. 
    3) Confirm verify_invoice returns #IncompletePayment with partialAmountPaid equal to that slightly more difference.
    4) Confirm call to recover returns err #InsufficientTransferAmount.

  (5th 'it': should reject if given invalid destination | #ICP -> err #InvalidDestination)
    1) Create an invoice to test against.
    2) Call to recover with invalid destination confirming err #InvalidDestination is returned. 

And then there are two token-independent #err tests.
*/
import { describe, it, expect } from 'vitest';
import { SupportedTokens } from '../utils/constants.js';
import {
  nnsFundedSecp256k1Actor as invoiceCanisterInstaller, // "original invoice canister deployer"
  nnsFundedSecp256k1Identity as installerPrincipal,
  getRandomActor,
  getActorByIdentity,
  getRandomIdentity,
} from '../utils/identity.js';
import { getTestCreateInvoiceArgs } from '../utils/utils.js';

describe('Test recover_invoice_subaccount_balance Functionality', async () => {
  // Assign some locally scoped so can be changed as needed.
  // Rename to be clear who is doing what.
  const balanceHolder = invoiceCanisterInstaller;
  const balanceHolderPrincipal = installerPrincipal.getPrincipal();

  // Adding a new identity to create the invoices as
  // original deployer is acting as the balance holder.
  const invoiceCreatorIdentity = getRandomIdentity();
  const addResult1 = await invoiceCanisterInstaller.add_allowed_creator({
    who: invoiceCreatorIdentity.getPrincipal(),
  });
  // Adding a new identity to be the recipient for the recovered funds
  // so that address's balance will be 0 prior to receiving any funds.
  const recoveredBalanceRecipient = getRandomIdentity();
  const addResult2 = await invoiceCanisterInstaller.add_allowed_creator({
    who: recoveredBalanceRecipient.getPrincipal(),
  });
  // If failed for some reason, abort.
  if (!addResult1.ok || !addResult2.ok) {
    throw new Error(
      'Could not add allowed creator aborting recover_invoice_subaccount_balance tests\ncall result was ' +
        JSON.stringify({ addResult1, addResult2 }),
    );
  }
  const invoiceCreator = getActorByIdentity(invoiceCreatorIdentity);
  const balanceRecipient = getActorByIdentity(recoveredBalanceRecipient);

  // Same used for all tokens in this project.
  const transferFeeAmount = 10000n;
  // Amount to create an invoice with.
  const billableAmountDue = 1000000000n;
  // Amount actually sent in transfer arg.
  const transferBillableAmountDue = billableAmountDue + transferFeeAmount;

  // Test amount 1 sent by a 3rd party to be recovered before verification ("refund of partial payment");
  const testAmount1SentToRecover = billableAmountDue / 10n;
  // Test amount 1 actually sent (plus transfer fee) by the transfer call from the 3rd party.
  const testAmount1ActuallySent = testAmount1SentToRecover + transferFeeAmount;
  // Test amount 1 recovered and actually received by 3rd party.
  // (is also the actual amount sent by the transfer call to the 3rd party for recovering the balance).
  const testAmount1ActuallyRecovered = testAmount1SentToRecover - transferFeeAmount;

  // Test amount 2 mistakenly sent by a 3rd party to be recovered after verification.
  const testAmount2SentToRecover = billableAmountDue * 2n;
  // Test amount 2 actually sent (plus transfer fee) by the transfer call from the 3rd party.
  const testAmount2ActuallySent = testAmount2SentToRecover + transferFeeAmount;
  // Test amount 2 recovered and actually received by transfer call from 3rd party.
  // (is also the actual amount sent by the transfer call to the 3rd party for recovering the balance).
  const testAmount2ActuallyRecovered = testAmount2SentToRecover - transferFeeAmount;

  // Amount insufficient so that it cannot be recovered without losing more than it's worth.
  const irrecoverableAmount = 4095n;
  // Irrecoverable amount as it is a transfer arg.
  const irrecoverableAmountActuallySent = irrecoverableAmount + transferFeeAmount;

  // Taking advantage of the fact icp and icp_ness have same address formats
  // as well as ICRC1_ExampleToken and ICRC1_ExampleToken2, preset the
  // balance recipient's addresses.
  const icpAddressResult = await balanceRecipient.get_caller_address({
    token: SupportedTokens.asVariantUnitObj.ICP,
  });
  const icrc1AddressResult = await balanceRecipient.get_caller_address({
    token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken,
  });
  let recipientAddressii = {
    icpTextAddress: null,
    icpAccountIdentifier: null,
    icrc1TextAddress: null,
    icrc1Account: null,
  };
  // Note both address formats are used for the sake of proof of coverage.
  if (icpAddressResult.ok && icrc1AddressResult.ok) {
    recipientAddressii.icpTextAddress = icpAddressResult.ok.asText;
    recipientAddressii.icpAccountIdentifier = icpAddressResult.ok.asAddress.ICP; // = accountIdentifierBlob
    recipientAddressii.icrc1TextAddress = icrc1AddressResult.ok.asText;
    recipientAddressii.icrc1Account = icrc1AddressResult.ok.asAddress.ICRC1_ExampleToken; // = { owner; subaccount }
  } else {
    throw new Error("Couldn't present known acceptable address values before E2E transfer tests");
  }
  describe('Test Token Non-Specific #err Results Returned From recover_invoice_subaccount_balance', async () => {
    it('should return invoice not found if no invoice exists for given id and caller is authorized | -> err #NotFound', async () => {
      // For consistency use a valid destination.
      const result = await invoiceCreator.recover_invoice_subaccount_balance({
        id: "invalidId",
        destination: { HumanReadable: recipientAddressii.icpTextAddress },
      });
      // Check returned err as expected.
      expect(result?.err?.kind).toStrictEqual({ NotFound: null });
    });
    it('should reject and return err kind #NotAuthorized if caller not authorized | -> err #NotAuthorized', async () => {
      // For consistency use a valid destination.
      const result = await getRandomActor().recover_invoice_subaccount_balance({
        id: "invalidId",
        destination: { HumanReadable: recipientAddressii.icpTextAddress },
      });
      // Check returned err as expected.
      expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
    });
  });
  describe('Test recover_invoice_subaccount_balance Functionality for #ICP Type', async () => {
    // Create an invoice to test against.
    const createInvoiceResult = await invoiceCreator.create_invoice(
      getTestCreateInvoiceArgs({
        whichToken: SupportedTokens.asVariantTagLiteral.ICP,
        amountDue: billableAmountDue,
      }),
    );
    // Check all's as to be expected.
    expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
    const { id, amountDue, paymentAddress, paid, amountPaid } = createInvoiceResult.ok.invoice;
    expect(amountDue).toStrictEqual(billableAmountDue);
    expect(paid).toBe(false);
    expect(amountPaid).toStrictEqual(0n);
    // Check the recipient's address current balance is 0.
    const balanceBeforeResult = await balanceRecipient.get_caller_balance({
      token: SupportedTokens.asVariantUnitObj.ICP,
    });
    expect(
      balanceBeforeResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICP]?.e8s,
    ).toStrictEqual(0n);
    it('should correctly recover partial amount paid before invoice has been verified | #ICP -> #ok (case 1: partial refund)', async () => {
      // Send amount to be recovered as refund of partial payment.
      const transferResult = await balanceHolder.transfer({
        tokenAmount: {
          [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: testAmount1ActuallySent },
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer result ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm incomplete payment with partial amount.
      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({
        IncompletePayment: {
          partialAmountPaid: {
            [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: testAmount1SentToRecover },
          },
        },
      });
      // Call for balance recovery.
      const recoverResult = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: { HumanReadable: recipientAddressii.icpTextAddress },
      });
      // Check returned ok as expected.
      expect(recoverResult?.ok?.transferSuccess);
      expect(recoverResult?.ok?.balanceRecovered);
      const { e8s } = recoverResult.ok.balanceRecovered[SupportedTokens.asVariantTagLiteral.ICP];
      expect(e8s).toStrictEqual(testAmount1ActuallyRecovered);
      // Confirm verifying invoice returns unpaid with zero balance.
      verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({ Unpaid: null });
      // Double check getting invoice returns expected unpaid, zero amount paid balance.
      const getResult = await invoiceCreator.get_invoice({ id });
      expect(getResult?.ok?.invoice).toBeTruthy();
      const { amountDue, paid, amountPaid } = getResult.ok.invoice;
      expect(amountDue).toStrictEqual(billableAmountDue);
      expect(paid).toBe(false);
      expect(amountPaid).toStrictEqual(0n);
      // Check the recipient's address current balance matches what it is expected
      // to be (ie recovery transfer amound indeed went to intended recipient).
      const recoveredBalanceResult = await balanceRecipient.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP,
      });
      expect(
        recoveredBalanceResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICP]?.e8s,
      ).toStrictEqual(testAmount1ActuallyRecovered);
    });
    it(
      'should correctly recover amount mistakenly sent after invoice already verified | #ICP -> #ok (case 2: recover lost funds)',
      async () => {
        // Send amount to be recovered after invoice has already been verified.
        const invoicePaidTransferResult = await balanceHolder.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: transferBillableAmountDue },
          },
          destination: { HumanReadable: paymentAddress },
        });
        // Confirm transfer result ok.
        expect(invoicePaidTransferResult?.ok).toBeTruthy();
        // Confirm the invoice is successfully verified paid as expected.
        const verifiedPaidResult = await invoiceCreator.verify_invoice({ id });
        expect(verifiedPaidResult?.ok?.VerifiedPaid?.invoice).toBeTruthy();
        const verifiedPaidInvoice = verifiedPaidResult.ok.VerifiedPaid.invoice;
        const { paid, amountPaid, amountDue, verifiedPaidAtTime } = verifiedPaidInvoice;
        expect(amountPaid >= amountDue).toBe(true);
        expect(paid).toBe(true);
        expect(verifiedPaidAtTime).toBeTruthy();
        // Mistakenly send more ICP to the invoice already verified paid.
        const mistakenlyTransferredResult = await balanceHolder.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: testAmount2ActuallySent },
          },
          destination: { HumanReadable: paymentAddress },
        });
        // Confirm transfer result ok.
        expect(mistakenlyTransferredResult?.ok).toBeTruthy();
        // Confirm calling verify again returns already verified paid as expected.
        const verifiedAlreadyResult = await invoiceCreator.verify_invoice({ id });
        expect(verifiedPaidInvoice).toStrictEqual(
          verifiedAlreadyResult?.ok?.VerifiedAlready?.invoice,
        );
        // Recover the mistakenly sent ICP.
        const recoverResult = await invoiceCreator.recover_invoice_subaccount_balance({
          id,
          destination: {
            CanisterExpected: {
              [SupportedTokens.asVariantTagLiteral.ICP]: recipientAddressii.icpAccountIdentifier,
            },
          },
        });
        // Check returned ok as expected.
        expect(recoverResult?.ok?.transferSuccess);
        expect(recoverResult?.ok?.balanceRecovered);
        const { e8s } = recoverResult.ok.balanceRecovered[SupportedTokens.asVariantTagLiteral.ICP];
        expect(e8s).toStrictEqual(testAmount2ActuallyRecovered);
        // Confirm invoice remains unchanged / verifying again still returns verified already paid.
        const confirmVerifiedAlreadyResult = await invoiceCreator.verify_invoice({ id });
        expect(verifiedPaidInvoice).toStrictEqual(
          confirmVerifiedAlreadyResult?.ok?.VerifiedAlready?.invoice,
        );
        // Double check getting invoice returns expected paid and amountPaid.
        const getResult = await invoiceCreator.get_invoice({ id });
        expect(verifiedPaidInvoice).toStrictEqual(getResult?.ok?.invoice);
        // Check the recipient's address current balance matches what it is expected
        // to be (ie recovery transfer amound indeed went to intended recipient).
        const recoveredBalanceResult = await balanceRecipient.get_caller_balance({
          token: SupportedTokens.asVariantUnitObj.ICP,
        });
        expect(
          recoveredBalanceResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICP]?.e8s,
        ).toStrictEqual(testAmount1ActuallyRecovered + testAmount2ActuallyRecovered);
      }, // This one may take longer than usual, so extend test timeout.
      { timeout: 25000 },
    );
    it('should reject if invoice subaccount balance is zero | #ICP -> err #NoBalance', async () => {
      // Create an invoice to test against.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP,
          amountDue: billableAmountDue,
        }),
      );
      // Check all's as to be expected.
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, amountDue, paid, amountPaid } = createInvoiceResult.ok.invoice;
      expect(amountDue).toStrictEqual(billableAmountDue);
      expect(paid).toBe(false);
      expect(amountPaid).toStrictEqual(0n);
      const result = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: { HumanReadable: recipientAddressii.icpTextAddress },
      });
      // Check returned err as expected.
      expect(result?.err?.kind).toStrictEqual({ NoBalance: null });
    });
    it('should reject if invoice subaccount balance not enough to cover transfer fee | #ICP -> err #InsufficientTransferAmount', async () => {
      // Create an invoice to test against.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP,
          amountDue: billableAmountDue,
        }),
      );
      // Check invoice created ok.
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Send amount to be "irrecoverably lost".
      const transferResult = await balanceHolder.transfer({
        tokenAmount: {
          [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: irrecoverableAmountActuallySent },
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer result ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm incomplete payment with partial amount.
      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({
        IncompletePayment: {
          partialAmountPaid: {
            [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: irrecoverableAmount },
          },
        },
      });
      // Call for balance recovery.
      const irrecoverableResult = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: { HumanReadable: recipientAddressii.icpTextAddress },
      });
      // Check returned err as expected.
      expect(irrecoverableResult?.err?.kind).toStrictEqual({ InsufficientTransferAmount: null });
    });
    it('should reject if given invalid destination | #ICP -> err #InvalidDestination', async () => {
      // Create an invoice to test against.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP,
          amountDue: billableAmountDue,
        }),
      );
      // Check all's as to be expected.
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id } = createInvoiceResult.ok.invoice;
      // Confirm invalid destination text format is rejected.
      let result = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: { HumanReadable: '🤯🤦🦟' },
      });
      expect(result?.err?.kind).toStrictEqual({ InvalidDestination: null });
      // Check invalid destination account identifier blob format is rejected.
      result = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: {
          CanisterExpected: {
            [SupportedTokens.asVariantTagLiteral.ICP]: Uint8Array.from([]),
          },
        },
      });
      expect(result?.err?.kind).toStrictEqual({ InvalidDestination: null });
    });
  });
  describe('Test recover_invoice_subaccount_balance Functionality for #ICP_nns Type', async () => {
    // Create an invoice to test against.
    const createInvoiceResult = await invoiceCreator.create_invoice(
      getTestCreateInvoiceArgs({
        whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
        amountDue: billableAmountDue,
      }),
    );
    // Check all's as to be expected.
    expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
    const { id, amountDue, paymentAddress, paid, amountPaid } = createInvoiceResult.ok.invoice;
    expect(amountDue).toStrictEqual(billableAmountDue);
    expect(paid).toBe(false);
    expect(amountPaid).toStrictEqual(0n);
    // Check the recipient's address current balance is 0.
    const balanceBeforeResult = await balanceRecipient.get_caller_balance({
      token: SupportedTokens.asVariantUnitObj.ICP_nns,
    });
    expect(
      balanceBeforeResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICP_nns]?.e8s,
    ).toStrictEqual(0n);
    it('should correctly recover partial amount paid before invoice has been verified | #ICP_nns -> #ok (case 1: partial refund)', async () => {
      // Send amount to be recovered as refund of partial payment.
      const transferResult = await balanceHolder.transfer({
        tokenAmount: {
          [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: testAmount1ActuallySent },
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer result ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm incomplete payment with partial amount.
      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({
        IncompletePayment: {
          partialAmountPaid: {
            [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: testAmount1SentToRecover },
          },
        },
      });
      // Call for balance recovery.
      const recoverResult = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: { HumanReadable: recipientAddressii.icpTextAddress },
      });
      // Check returned ok as expected.
      expect(recoverResult?.ok?.transferSuccess);
      expect(recoverResult?.ok?.balanceRecovered);
      const { e8s } =
        recoverResult.ok.balanceRecovered[SupportedTokens.asVariantTagLiteral.ICP_nns];
      expect(e8s).toStrictEqual(testAmount1ActuallyRecovered);
      // Confirm verifying invoice returns unpaid with zero balance.
      verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({ Unpaid: null });
      // Double check getting invoice returns expected unpaid, zero amount paid balance.
      const getResult = await invoiceCreator.get_invoice({ id });
      expect(getResult?.ok?.invoice).toBeTruthy();
      const { amountDue, paid, amountPaid } = getResult.ok.invoice;
      expect(amountDue).toStrictEqual(billableAmountDue);
      expect(paid).toBe(false);
      expect(amountPaid).toStrictEqual(0n);
      // Check the recipient's address current balance matches what it is expected
      // to be (ie recovery transfer amound indeed went to intended recipient).
      const recoveredBalanceResult = await balanceRecipient.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP_nns,
      });
      expect(
        recoveredBalanceResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICP_nns]?.e8s,
      ).toStrictEqual(testAmount1ActuallyRecovered);
    });
    it(
      'should recover amount mistakenly sent after invoice already verified | #ICP_nns -> #ok (case 2: recover lost funds)',
      async () => {
        // Send amount to be recovered after invoice has already been verified.
        const invoicePaidTransferResult = await balanceHolder.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: transferBillableAmountDue },
          },
          destination: { HumanReadable: paymentAddress },
        });
        // Confirm transfer result ok.
        expect(invoicePaidTransferResult?.ok).toBeTruthy();
        // Confirm the invoice is successfully verified paid as expected.
        const verifiedPaidResult = await invoiceCreator.verify_invoice({ id });
        expect(verifiedPaidResult?.ok?.VerifiedPaid?.invoice).toBeTruthy();
        const verifiedPaidInvoice = verifiedPaidResult.ok.VerifiedPaid.invoice;
        const { paid, amountPaid, amountDue, verifiedPaidAtTime } = verifiedPaidInvoice;
        expect(amountPaid >= amountDue).toBe(true);
        expect(paid).toBe(true);
        expect(verifiedPaidAtTime).toBeTruthy();
        // Mistakenly send more ICP to the invoice already verified paid.
        const mistakenlyTransferredResult = await balanceHolder.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: testAmount2ActuallySent },
          },
          destination: { HumanReadable: paymentAddress },
        });
        // Confirm transfer result ok.
        expect(mistakenlyTransferredResult?.ok).toBeTruthy();
        // Confirm calling verify again returns already verified paid as expected.
        const verifiedAlreadyResult = await invoiceCreator.verify_invoice({ id });
        expect(verifiedPaidInvoice).toStrictEqual(
          verifiedAlreadyResult?.ok?.VerifiedAlready?.invoice,
        );
        // Recover the mistakenly sent ICP e8s.
        const recoverResult = await invoiceCreator.recover_invoice_subaccount_balance({
          id,
          destination: {
            CanisterExpected: {
              [SupportedTokens.asVariantTagLiteral.ICP_nns]:
                recipientAddressii.icpAccountIdentifier,
            },
          },
        });
        // Check returned ok as expected.
        expect(recoverResult?.ok?.transferSuccess);
        expect(recoverResult?.ok?.balanceRecovered);
        const { e8s } =
          recoverResult.ok.balanceRecovered[SupportedTokens.asVariantTagLiteral.ICP_nns];
        expect(e8s).toStrictEqual(testAmount2ActuallyRecovered);
        // Confirm invoice remains unchanged / verifying again still returns verified already paid.
        const confirmVerifiedAlreadyResult = await invoiceCreator.verify_invoice({ id });
        expect(verifiedPaidInvoice).toStrictEqual(
          confirmVerifiedAlreadyResult?.ok?.VerifiedAlready?.invoice,
        );
        // Double check getting invoice returns expected paid and amountPaid.
        const getResult = await invoiceCreator.get_invoice({ id });
        expect(verifiedPaidInvoice).toStrictEqual(getResult?.ok?.invoice);
        // Check the recipient's address current balance matches what it is expected
        // to be (ie recovery transfer amound indeed went to intended recipient).
        const recoveredBalanceResult = await balanceRecipient.get_caller_balance({
          token: SupportedTokens.asVariantUnitObj.ICP_nns,
        });
        expect(
          recoveredBalanceResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICP_nns]?.e8s,
        ).toStrictEqual(testAmount1ActuallyRecovered + testAmount2ActuallyRecovered);
      }, // This one may take longer than usual, so extend test timeout.
      { timeout: 25000 },
    );
    it('should reject if invoice subaccount balance is zero | #ICP_nns -> err #NoBalance', async () => {
      // Create an invoice to test against.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
          amountDue: billableAmountDue,
        }),
      );
      // Check all's as to be expected.
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, amountDue, paid, amountPaid } = createInvoiceResult.ok.invoice;
      expect(amountDue).toStrictEqual(billableAmountDue);
      expect(paid).toBe(false);
      expect(amountPaid).toStrictEqual(0n);
      const result = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: { HumanReadable: recipientAddressii.icpTextAddress },
      });
      // Check returned err as expected.
      expect(result?.err?.kind).toStrictEqual({ NoBalance: null });
    });
    it('should reject if invoice subaccount balance not enough to cover transfer fee | #ICP_nns -> err #InsufficientTransferAmount', async () => {
      // Create an invoice to test against.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
          amountDue: billableAmountDue,
        }),
      );
      // Check invoice created ok.
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Send amount to be "irrecoverably lost".
      const transferResult = await balanceHolder.transfer({
        tokenAmount: {
          [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: irrecoverableAmountActuallySent },
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer result ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm incomplete payment with partial amount.
      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({
        IncompletePayment: {
          partialAmountPaid: {
            [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: irrecoverableAmount },
          },
        },
      });
      // Call for balance recovery.
      const irrecoverableResult = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: { HumanReadable: recipientAddressii.icpTextAddress },
      });
      // Check returned err as expected.
      expect(irrecoverableResult?.err?.kind).toStrictEqual({ InsufficientTransferAmount: null });
    });
    it('should reject if given invalid destination | #ICP_nns -> err #InvalidDestination', async () => {
      // Create an invoice to test against.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICP_nns,
          amountDue: billableAmountDue,
        }),
      );
      // Check all's as to be expected.
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id } = createInvoiceResult.ok.invoice;
      // Confirm invalid destination text format is rejected.
      let result = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: { HumanReadable: '🤯🤦🦟' },
      });
      expect(result?.err?.kind).toStrictEqual({ InvalidDestination: null });
      // Check invalid destination account identifier blob format is rejected.
      result = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: {
          CanisterExpected: {
            [SupportedTokens.asVariantTagLiteral.ICP_nns]: Uint8Array.from([]),
          },
        },
      });
      expect(result?.err?.kind).toStrictEqual({ InvalidDestination: null });
    });
  });
  describe('Test recover_invoice_subaccount_balance Functionality for #ICRC1_ExampleToken Type', async () => {
    // Create an invoice to test against.
    const createInvoiceResult = await invoiceCreator.create_invoice(
      getTestCreateInvoiceArgs({
        whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
        amountDue: billableAmountDue,
      }),
    );
    // Check all's as to be expected.
    expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
    const { id, amountDue, paymentAddress, paid, amountPaid } = createInvoiceResult.ok.invoice;
    expect(amountDue).toStrictEqual(billableAmountDue);
    expect(paid).toBe(false);
    expect(amountPaid).toStrictEqual(0n);
    // Check the recipient's address current balance is 0.
    const balanceBeforeResult = await balanceRecipient.get_caller_balance({
      token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken,
    });
    expect(
      balanceBeforeResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken],
    ).toStrictEqual(0n);
    it('should correctly recover partial amount paid before invoice has been verified | #ICRC1_ExampleToken -> #ok (case 1: partial refund)  ', async () => {
      // Send amount to be recovered as refund of partial payment.
      const transferResult = await balanceHolder.transfer({
        tokenAmount: {
          [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: testAmount1ActuallySent,
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer result ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm incomplete payment with partial amount.
      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({
        IncompletePayment: {
          partialAmountPaid: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: testAmount1SentToRecover,
          },
        },
      });
      // Call for balance recovery.
      const recoverResult = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: { HumanReadable: recipientAddressii.icrc1TextAddress },
      });
      // Check returned ok as expected.
      expect(recoverResult?.ok?.transferSuccess);
      expect(recoverResult?.ok?.balanceRecovered);
      const amountRecovered =
        recoverResult.ok.balanceRecovered[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken];
      expect(amountRecovered).toStrictEqual(testAmount1ActuallyRecovered);
      // Confirm verifying invoice returns unpaid with zero balance.
      verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({ Unpaid: null });
      // Double check getting invoice returns expected unpaid, zero amount paid balance.
      const getResult = await invoiceCreator.get_invoice({ id });
      expect(getResult?.ok?.invoice).toBeTruthy();
      const { amountDue, paid, amountPaid } = getResult.ok.invoice;
      expect(amountDue).toStrictEqual(billableAmountDue);
      expect(paid).toBe(false);
      expect(amountPaid).toStrictEqual(0n);
      // Check the recipient's address current balance matches what it is expected
      // to be (ie recovery transfer amound indeed went to intended recipient).
      const recoveredBalanceResult = await balanceRecipient.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken,
      });
      expect(
        recoveredBalanceResult?.ok?.balance?.[
          SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken
        ],
      ).toStrictEqual(testAmount1ActuallyRecovered);
    });
    it(
      'should recover amount mistakenly sent after invoice already verified | #ICRC1_ExampleToken -> #ok (case 2: recover lost funds)',
      async () => {
        // Send amount to be recovered after invoice has already been verified.
        const invoicePaidTransferResult = await balanceHolder.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: transferBillableAmountDue,
          },
          destination: { HumanReadable: paymentAddress },
        });
        // Confirm transfer result ok.
        expect(invoicePaidTransferResult?.ok).toBeTruthy();
        // Confirm the invoice is successfully verified paid as expected.
        const verifiedPaidResult = await invoiceCreator.verify_invoice({ id });
        expect(verifiedPaidResult?.ok?.VerifiedPaid?.invoice).toBeTruthy();
        const verifiedPaidInvoice = verifiedPaidResult.ok.VerifiedPaid.invoice;
        const { paid, amountPaid, amountDue, verifiedPaidAtTime } = verifiedPaidInvoice;
        expect(amountPaid >= amountDue).toBe(true);
        expect(paid).toBe(true);
        expect(verifiedPaidAtTime).toBeTruthy();
        // Mistakenly send more ICRC1 tokens to the invoice already verified paid.
        const mistakenlyTransferredResult = await balanceHolder.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: testAmount2ActuallySent,
          },
          destination: { HumanReadable: paymentAddress },
        });
        // Confirm transfer result ok.
        expect(mistakenlyTransferredResult?.ok).toBeTruthy();
        // Confirm calling verify again returns already verified paid as expected.
        const verifiedAlreadyResult = await invoiceCreator.verify_invoice({ id });
        expect(verifiedPaidInvoice).toStrictEqual(
          verifiedAlreadyResult?.ok?.VerifiedAlready?.invoice,
        );
        // Recover the mistakenly sent ICRC1 tokens.
        const recoverResult = await invoiceCreator.recover_invoice_subaccount_balance({
          id,
          destination: {
            CanisterExpected: {
              [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]:
                recipientAddressii.icrc1Account,
            },
          },
        });
        // Check returned ok as expected.
        expect(recoverResult?.ok?.transferSuccess);
        expect(recoverResult?.ok?.balanceRecovered);
        const amountRecovered =
          recoverResult.ok.balanceRecovered[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken];
        expect(amountRecovered).toStrictEqual(testAmount2ActuallyRecovered);
        // Confirm invoice remains unchanged / verifying again still returns verified already paid.
        const confirmVerifiedAlreadyResult = await invoiceCreator.verify_invoice({ id });
        expect(verifiedPaidInvoice).toStrictEqual(
          confirmVerifiedAlreadyResult?.ok?.VerifiedAlready?.invoice,
        );
        // Double check getting invoice returns expected paid and amountPaid.
        const getResult = await invoiceCreator.get_invoice({ id });
        expect(verifiedPaidInvoice).toStrictEqual(getResult?.ok?.invoice);
        // Check the recipient's address current balance matches what it is expected
        // to be (ie recovery transfer amound indeed went to intended recipient).
        const recoveredBalanceResult = await balanceRecipient.get_caller_balance({
          token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken,
        });
        expect(
          recoveredBalanceResult?.ok?.balance?.[
            SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken
          ],
        ).toStrictEqual(testAmount1ActuallyRecovered + testAmount2ActuallyRecovered);
      }, // This one may take longer than usual, so extend test timeout.
      { timeout: 25000 },
    );
    it('should reject if invoice subaccount balance is zero | #ICRC1_ExampleToken -> err #NoBalance', async () => {
      // Create an invoice to test against.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
          amountDue: billableAmountDue,
        }),
      );
      // Check all's as to be expected.
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, amountDue, paid, amountPaid } = createInvoiceResult.ok.invoice;
      expect(amountDue).toStrictEqual(billableAmountDue);
      expect(paid).toBe(false);
      expect(amountPaid).toStrictEqual(0n);
      const result = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: { HumanReadable: recipientAddressii.icrc1TextAddress },
      });
      // Check returned err as expected.
      expect(result?.err?.kind).toStrictEqual({ NoBalance: null });
    });
    it('should reject if invoice subaccount balance not enough to cover transfer fee | #ICRC1_ExampleToken -> err #InsufficientTransferAmount', async () => {
      // Create an invoice to test against.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
          amountDue: billableAmountDue,
        }),
      );
      // Check invoice created ok.
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Send amount to be "irrecoverably lost".
      const transferResult = await balanceHolder.transfer({
        tokenAmount: {
          [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: irrecoverableAmountActuallySent,
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer result ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm incomplete payment with partial amount.
      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({
        IncompletePayment: {
          partialAmountPaid: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: irrecoverableAmount,
          },
        },
      });
      // Call for balance recovery.
      const irrecoverableResult = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: { HumanReadable: recipientAddressii.icrc1TextAddress },
      });
      // Check returned err as expected.
      expect(irrecoverableResult?.err?.kind).toStrictEqual({ InsufficientTransferAmount: null });
    });
    it('should reject if given invalid destination | #ICRC1_ExampleToken -> err #InvalidDestination', async () => {
      // Create an invoice to test against.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken,
          amountDue: billableAmountDue,
        }),
      );
      // Check all's as to be expected.
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id } = createInvoiceResult.ok.invoice;
      // Confirm invalid destination text format is rejected.
      let result = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: { HumanReadable: '🤯🤦🦟' },
      });
      expect(result?.err?.kind).toStrictEqual({ InvalidDestination: null });
      // Check invalid destination account identifier blob format is rejected.
      result = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: {
          CanisterExpected: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: {
              owner: balanceHolderPrincipal,
              subaccount: [Uint8Array.from([])],
            },
          },
        },
      });
      expect(result?.err?.kind).toStrictEqual({ InvalidDestination: null });
    });
  });
  describe('Test recover_invoice_subaccount_balance Functionality for #ICRC1_ExampleToken2', async () => {
    // Create an invoice to test against.
    const createInvoiceResult = await invoiceCreator.create_invoice(
      getTestCreateInvoiceArgs({
        whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
        amountDue: billableAmountDue,
      }),
    );
    // Check all's as to be expected.
    expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
    const { id, amountDue, paymentAddress, paid, amountPaid } = createInvoiceResult.ok.invoice;
    expect(amountDue).toStrictEqual(billableAmountDue);
    expect(paid).toBe(false);
    expect(amountPaid).toStrictEqual(0n);
    // Check the recipient's address current balance is 0.
    const balanceBeforeResult = await balanceRecipient.get_caller_balance({
      token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2,
    });
    expect(
      balanceBeforeResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2],
    ).toStrictEqual(0n);
    it('should correctly recover partial amount paid before invoice has been verified | #ICRC1_ExampleToken2 -> #ok (case 1: partial refund) ', async () => {
      // Send amount to be recovered as refund of partial payment.
      const transferResult = await balanceHolder.transfer({
        tokenAmount: {
          [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: testAmount1ActuallySent,
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer result ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm incomplete payment with partial amount.
      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({
        IncompletePayment: {
          partialAmountPaid: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: testAmount1SentToRecover,
          },
        },
      });
      // Call for balance recovery.
      const recoverResult = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: { HumanReadable: recipientAddressii.icrc1TextAddress },
      });
      // Check returned ok as expected.
      expect(recoverResult?.ok?.transferSuccess);
      expect(recoverResult?.ok?.balanceRecovered);
      const amountRecovered =
        recoverResult.ok.balanceRecovered[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2];
      expect(amountRecovered).toStrictEqual(testAmount1ActuallyRecovered);
      // Confirm verifying invoice returns unpaid with zero balance.
      verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({ Unpaid: null });
      // Double check getting invoice returns expected unpaid, zero amount paid balance.
      const getResult = await invoiceCreator.get_invoice({ id });
      expect(getResult?.ok?.invoice).toBeTruthy();
      const { amountDue, paid, amountPaid } = getResult.ok.invoice;
      expect(amountDue).toStrictEqual(billableAmountDue);
      expect(paid).toBe(false);
      expect(amountPaid).toStrictEqual(0n);
      // Check the recipient's address current balance matches what it is expected
      // to be (ie recovery transfer amound indeed went to intended recipient).
      const recoveredBalanceResult = await balanceRecipient.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2,
      });
      expect(
        recoveredBalanceResult?.ok?.balance?.[
          SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2
        ],
      ).toStrictEqual(testAmount1ActuallyRecovered);
    });
    it(
      'should recover amount mistakenly sent after invoice already verified | #ICRC1_ExampleToken2 -> #ok (case 2: recover lost funds)',
      async () => {
        // Send amount to be recovered after invoice has already been verified.
        const invoicePaidTransferResult = await balanceHolder.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: transferBillableAmountDue,
          },
          destination: { HumanReadable: paymentAddress },
        });
        // Confirm transfer result ok.
        expect(invoicePaidTransferResult?.ok).toBeTruthy();
        // Confirm the invoice is successfully verified paid as expected.
        const verifiedPaidResult = await invoiceCreator.verify_invoice({ id });
        expect(verifiedPaidResult?.ok?.VerifiedPaid?.invoice).toBeTruthy();
        const verifiedPaidInvoice = verifiedPaidResult.ok.VerifiedPaid.invoice;
        const { paid, amountPaid, amountDue, verifiedPaidAtTime } = verifiedPaidInvoice;
        expect(amountPaid >= amountDue).toBe(true);
        expect(paid).toBe(true);
        expect(verifiedPaidAtTime).toBeTruthy();
        // Mistakenly send more ICRC1 tokens to the invoice already verified paid.
        const mistakenlyTransferredResult = await balanceHolder.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: testAmount2ActuallySent,
          },
          destination: { HumanReadable: paymentAddress },
        });
        // Confirm transfer result ok.
        expect(mistakenlyTransferredResult?.ok).toBeTruthy();
        // Confirm calling verify again returns already verified paid as expected.
        const verifiedAlreadyResult = await invoiceCreator.verify_invoice({ id });
        expect(verifiedPaidInvoice).toStrictEqual(
          verifiedAlreadyResult?.ok?.VerifiedAlready?.invoice,
        );
        // Recover the mistakenly sent ICRC1 tokens.
        const recoverResult = await invoiceCreator.recover_invoice_subaccount_balance({
          id,
          destination: {
            CanisterExpected: {
              [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]:
                recipientAddressii.icrc1Account,
            },
          },
        });
        // Check returned ok as expected.
        expect(recoverResult?.ok?.transferSuccess);
        expect(recoverResult?.ok?.balanceRecovered);
        const amountRecovered =
          recoverResult.ok.balanceRecovered[
            SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2
          ];
        expect(amountRecovered).toStrictEqual(testAmount2ActuallyRecovered);
        // Confirm invoice remains unchanged / verifying again still returns verified already paid.
        const confirmVerifiedAlreadyResult = await invoiceCreator.verify_invoice({ id });
        expect(verifiedPaidInvoice).toStrictEqual(
          confirmVerifiedAlreadyResult?.ok?.VerifiedAlready?.invoice,
        );
        // Double check getting invoice returns expected paid and amountPaid.
        const getResult = await invoiceCreator.get_invoice({ id });
        expect(verifiedPaidInvoice).toStrictEqual(getResult?.ok?.invoice);
        // Check the recipient's address current balance matches what it is expected
        // to be (ie recovery transfer amound indeed went to intended recipient).
        const recoveredBalanceResult = await balanceRecipient.get_caller_balance({
          token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2,
        });
        expect(
          recoveredBalanceResult?.ok?.balance?.[
            SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2
          ],
        ).toStrictEqual(testAmount1ActuallyRecovered + testAmount2ActuallyRecovered);
      }, // This one may take longer than usual, so extend test timeout.
      { timeout: 25000 },
    );
    it('should reject if invoice subaccount balance is zero | #ICRC1_ExampleToken2 -> err #NoBalance', async () => {
      // Create an invoice to test against.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
          amountDue: billableAmountDue,
        }),
      );
      // Check all's as to be expected.
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, amountDue, paid, amountPaid } = createInvoiceResult.ok.invoice;
      expect(amountDue).toStrictEqual(billableAmountDue);
      expect(paid).toBe(false);
      expect(amountPaid).toStrictEqual(0n);
      const result = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: { HumanReadable: recipientAddressii.icrc1TextAddress },
      });
      // Check returned err as expected.
      expect(result?.err?.kind).toStrictEqual({ NoBalance: null });
    });
    it('should reject if invoice subaccount balance not enough to cover transfer fee | #ICRC1_ExampleToken2 -> err #InsufficientTransferAmount', async () => {
      // Create an invoice to test against.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
          amountDue: billableAmountDue,
        }),
      );
      // Check invoice created ok.
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id, paymentAddress } = createInvoiceResult.ok.invoice;
      // Send amount to be "irrecoverably lost".
      const transferResult = await balanceHolder.transfer({
        tokenAmount: {
          [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]:
            irrecoverableAmountActuallySent,
        },
        destination: { HumanReadable: paymentAddress },
      });
      // Confirm transfer result ok.
      expect(transferResult?.ok).toBeTruthy();
      // Confirm incomplete payment with partial amount.
      let verifyResult = await invoiceCreator.verify_invoice({ id });
      expect(verifyResult?.err?.kind).toStrictEqual({
        IncompletePayment: {
          partialAmountPaid: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: irrecoverableAmount,
          },
        },
      });
      // Call for balance recovery.
      const irrecoverableResult = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: { HumanReadable: recipientAddressii.icrc1TextAddress },
      });
      // Check returned err as expected.
      expect(irrecoverableResult?.err?.kind).toStrictEqual({ InsufficientTransferAmount: null });
    });
    it('should reject if given invalid destination | #ICRC1_ExampleToken2 -> err #InvalidDestination', async () => {
      // Create an invoice to test against.
      const createInvoiceResult = await invoiceCreator.create_invoice(
        getTestCreateInvoiceArgs({
          whichToken: SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2,
          amountDue: billableAmountDue,
        }),
      );
      // Check all's as to be expected.
      expect(createInvoiceResult?.ok?.invoice).toBeTruthy();
      const { id } = createInvoiceResult.ok.invoice;
      // Confirm invalid destination text format is rejected.
      let result = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: { HumanReadable: '🤯🤦🦟' },
      });
      expect(result?.err?.kind).toStrictEqual({ InvalidDestination: null });
      // Check invalid destination account identifier blob format is rejected.
      result = await invoiceCreator.recover_invoice_subaccount_balance({
        id,
        destination: {
          CanisterExpected: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: {
              owner: balanceHolderPrincipal,
              subaccount: [Uint8Array.from([])],
            },
          },
        },
      });
      expect(result?.err?.kind).toStrictEqual({ InvalidDestination: null });
    });
  });
});
