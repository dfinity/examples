import { describe, it, expect } from 'vitest';
import { SupportedTokens } from '../utils/constants.js';
import {
  nnsFundedSecp256k1Actor as invoiceCanisterInstaller, // "original invoice canister deployer"
  nnsFundedSecp256k1Identity as installerPrincipal,
  getRandomActor,
  getActorByIdentity,
  getRandomIdentity,
} from '../utils/identity.js';

describe('Test transfer Functionality', async () => {
  // So transferred funds can be checked, adding another allowed creator this caller is
  // also authorized to call get_caller_balance, get_caller_address, and transfer.
  // (will be the 'recipient' of funds).
  const identityToAdd = getRandomIdentity();
  const addAllowedCreatorResult = await invoiceCanisterInstaller.add_allowed_creator({
    who: identityToAdd.getPrincipal(),
  });
  if (!addAllowedCreatorResult.ok) {
    throw new Error(
      'Could not add allowed creator aborting get_caller_balance tests\nresult was ' +
        JSON.stringify(addAllowedCreatorResult),
    );
  }
  // Redeclared to make it more clear whose doing what.
  const transferer = invoiceCanisterInstaller;
  const recipient = getActorByIdentity(identityToAdd);

  // Since it's known all transfer fee costs are same for all tokens involved.
  const presetFee = 10000n;
  // Amount sent to verify text address format as destination works.
  const firstTransferAmount = 2000000n;
  const firstAmountToTransfer = presetFee + firstTransferAmount;
  // Amount sent to verify canister expected address format as destination works.
  const secondTransferAmount = 8000000n;
  const secondAmountToTransfer = presetFee + secondTransferAmount;
  describe('Test #ok Results Returned From transfer', () => {
    /*
    (0) Add allowed creator to be the recipient of transferred funds (above)
    Note: Each token specific #ok result test consists of the following process:
      1) Verify balance of recipient's creator subaccount is 0. 
      2) Get the address and as text of their creator subaccount 
      3) Transfer the first amount using the address in text format
      4) Verify the balance increased by that amount
      5) Transfer the second amount using the address in canister expected format
      6) Verify the balance increased by the sum of both amounts 
    */
    it('should correctly transfer e8s to an address specified as an account identifier & as text | #ICP', async () => {
      // First check the current balance is 0.
      const balanceBeforeResult = await recipient.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP,
      });
      expect(
        balanceBeforeResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICP]?.e8s,
      ).toStrictEqual(0n);
      // Get the corresponding supported token address of this caller ("invoice creator's principal subaccount").
      const addressResult = await recipient.get_caller_address({
        token: SupportedTokens.asVariantUnitObj.ICP,
      });
      expect(addressResult?.ok).toBeTruthy();
      // Use the result to transfer to both formats of address type
      // (account identifier in this case) and as text.
      const { asText, asAddress } = addressResult.ok;
      let transferResult = await transferer.transfer({
        tokenAmount: {
          [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: firstAmountToTransfer },
        },
        destination: { HumanReadable: asText },
      });
      // Confirm transfer result ok.
      expect(transferResult?.ok).toBeTruthy();
      // Check balance has increased by transfer amount less fee.
      let balanceAfterResult = await recipient.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP,
      });
      expect(
        balanceAfterResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICP]?.e8s,
      ).toStrictEqual(firstTransferAmount);
      // Now transfer with destination in supported token address format (account identifier).
      transferResult = await transferer.transfer({
        tokenAmount: {
          [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: secondAmountToTransfer },
        },
        // address looks like { ICP: accountIdentifierBlob }
        destination: { CanisterExpected: asAddress },
      });
      // Check transfer result ok.
      expect(transferResult?.ok).toBeTruthy();
      // Check again balance has increased by transfer amount less fee.
      balanceAfterResult = await recipient.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP,
      });
      expect(
        balanceAfterResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICP]?.e8s,
      ).toStrictEqual(firstTransferAmount + secondTransferAmount);
    });
    it('should correctly transfer e8s to an address specified as an account identifier & as text | #ICP_nns', async () => {
      // First check the current balance is 0.
      const balanceBeforeResult = await recipient.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP_nns,
      });
      expect(
        balanceBeforeResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICP_nns]?.e8s,
      ).toStrictEqual(0n);
      // Get the corresponding supported token address of this caller ("invoice creator's principal subaccount").
      const addressResult = await recipient.get_caller_address({
        token: SupportedTokens.asVariantUnitObj.ICP_nns,
      });
      expect(addressResult?.ok).toBeTruthy();
      // Use the result to transfer to both formats of address type
      // (account identifier in this case) and as text.
      const { asText, asAddress } = addressResult.ok;
      let transferResult = await transferer.transfer({
        tokenAmount: {
          [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: firstAmountToTransfer },
        },
        destination: { HumanReadable: asText },
      });
      // Confirm transfer result ok.
      expect(transferResult?.ok).toBeTruthy();
      // Check balance has increased by transfer amount less fee.
      let balanceAfterResult = await recipient.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP_nns,
      });
      expect(
        balanceAfterResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICP_nns]?.e8s,
      ).toStrictEqual(firstTransferAmount);
      // Now transfer with destination in supported token address format (account identifier).
      transferResult = await transferer.transfer({
        tokenAmount: {
          [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: secondAmountToTransfer },
        },
        // address looks like { ICP_nns: accountIdentifierBlob }
        destination: { CanisterExpected: asAddress },
      });
      // Check transfer result ok.
      expect(transferResult?.ok).toBeTruthy();
      // Check again balance has increased by transfer amount less fee.
      balanceAfterResult = await recipient.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICP_nns,
      });
      expect(
        balanceAfterResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICP_nns]?.e8s,
      ).toStrictEqual(firstTransferAmount + secondTransferAmount);
    });
    it('should correctly transfer icrc1 tokens to an address specified as an icrc1 account & as text | #ICRC1_ExampleToken', async () => {
      // First check the current balance is 0.
      const balanceBeforeResult = await recipient.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken,
      });
      expect(
        balanceBeforeResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken],
      ).toStrictEqual(0n);
      // Get the corresponding supported token address of this caller ("invoice creator's principal subaccount").
      const addressResult = await recipient.get_caller_address({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken,
      });
      expect(addressResult?.ok).toBeTruthy();
      // Use the result to transfer to transfer both to specific address type
      // (account identifier in this case) and as text.
      const { asText, asAddress } = addressResult.ok;
      let transferResult = await transferer.transfer({
        tokenAmount: {
          [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: firstAmountToTransfer,
        },
        destination: { HumanReadable: asText },
      });
      // Check the transfer result ok.
      expect(transferResult?.ok).toBeTruthy();
      // Check balance has increased by transfer amount less fee.
      let balanceAfterResult = await recipient.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken,
      });
      expect(
        balanceAfterResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken],
      ).toStrictEqual(firstTransferAmount);
      // Transfer with destination in supported token address format (account).
      transferResult = await transferer.transfer({
        tokenAmount: {
          [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: secondAmountToTransfer,
        },
        // address looks like { ICRC1_ExampleToken: account }
        destination: { CanisterExpected: asAddress },
      });
      // Confirm transfer result ok.
      expect(transferResult?.ok).toBeTruthy();
      // Check balance has increased by transfer amount less fee again.
      balanceAfterResult = await recipient.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken,
      });
      expect(
        balanceAfterResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken],
      ).toStrictEqual(firstTransferAmount + secondTransferAmount);
    });
    it('should correctly transfer icrc1 tokens to an address specified as an icrc1 account & as text | #ICRC1_ExampleToken2', async () => {
      // First check the current balance is 0.
      const balanceBeforeResult = await recipient.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2,
      });
      expect(
        balanceBeforeResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2],
      ).toStrictEqual(0n);
      // Get the corresponding supported token address of this caller ("invoice creator's principal subaccount").
      const addressResult = await recipient.get_caller_address({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2,
      });
      expect(addressResult?.ok).toBeTruthy();
      // Use the result to transfer to transfer both to specific address type
      // (account identifier in this case) and as text.
      const { asText, asAddress } = addressResult.ok;
      let transferResult = await transferer.transfer({
        tokenAmount: {
          [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: firstAmountToTransfer,
        },
        destination: { HumanReadable: asText },
      });
      // Check the transfer result ok.
      expect(transferResult?.ok).toBeTruthy();
      // Check balance has increased by transfer amount less fee.
      let balanceAfterResult = await recipient.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2,
      });
      expect(
        balanceAfterResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2],
      ).toStrictEqual(firstTransferAmount);
      // Transfer with destination in supported token address format (account).
      transferResult = await transferer.transfer({
        tokenAmount: {
          [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: secondAmountToTransfer,
        },
        // address looks like { ICRC1_ExampleToken2: account }
        destination: { CanisterExpected: asAddress },
      });
      // Confirm transfer result ok.
      expect(transferResult?.ok).toBeTruthy();
      // Check balance has increased by transfer amount less fee again.
      balanceAfterResult = await recipient.get_caller_balance({
        token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken2,
      });
      expect(
        balanceAfterResult?.ok?.balance?.[SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2],
      ).toStrictEqual(firstTransferAmount + secondTransferAmount);
    });
  });
  describe('Test #err Results Return From transfer', async () => {
    // Redeclaring so its clear its different set of test.
    const anAuthorizedCaller = recipient;
    // For consistency using valid inputs other than the one being tested for err.
    let knownAcceptableVals = {
      icpTextAddress: null,
      icpAccountIdentifier: null,
      icrc1TextAddress: null,
      icrc1Account: null,
    };
    const icpAddressResult = await invoiceCanisterInstaller.get_caller_address({
      token: SupportedTokens.asVariantUnitObj.ICP,
    });
    const icrc1AddressResult = await invoiceCanisterInstaller.get_caller_address({
      token: SupportedTokens.asVariantUnitObj.ICRC1_ExampleToken,
    });
    // Note both address formats are used for the sake of proof of coverage.
    if (icpAddressResult.ok && icrc1AddressResult.ok) {
      knownAcceptableVals.icpTextAddress = icpAddressResult.ok.asText;
      knownAcceptableVals.icpAccountIdentifier = icpAddressResult.ok.asAddress.ICP; // = accountIdentifierBlob
      knownAcceptableVals.icrc1TextAddress = icrc1AddressResult.ok.asText;
      knownAcceptableVals.icrc1Account = icrc1AddressResult.ok.asAddress.ICRC1_ExampleToken; // = { owner; subaccount }
    } else {
      throw new Error("Couldn't present known acceptable address values before E2E transfer tests");
    }
    describe('When Caller is Not Authorized | -> err kind #NotAuthorized', () => {
      it('should reject when caller is not authorized | #ICP', async () => {
        const result = await getRandomActor().transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: 1000000n },
          },
          destination: { HumanReadable: knownAcceptableVals.icpTextAddress },
        });
        expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
      });
      it('should reject when caller is not authorized | #ICP_nns', async () => {
        const result = await getRandomActor().transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: 1000000n },
          },
          destination: { HumanReadable: knownAcceptableVals.icpTextAddress },
        });
        expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
      });
      it('should reject when caller is not authorized | #ICRC1_ExampleToken', async () => {
        const result = await getRandomActor().transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: 1000000n,
          },
          destination: { HumanReadable: knownAcceptableVals.icrc1TextAddress },
        });
        expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
      });
      it('should reject when caller is not authorized | #ICRC1_ExampleToken2', async () => {
        const result = await getRandomActor().transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: 1000000n,
          },
          destination: { HumanReadable: knownAcceptableVals.icrc1TextAddress },
        });
        expect(result?.err?.kind).toStrictEqual({ NotAuthorized: null });
      });
    });
    describe('When Caller Tries to Transfer Amount Less Than Transfer Fee | -> err kind #InsufficientTransferAmount', () => {
      // Distinct from not enough balance, which is handled as SupportedTokenTransferErr (see test below)
      it('should reject when the caller is using insufficient e8s | #ICP', async () => {
        const result = await transferer.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: 100n },
          },
          destination: { HumanReadable: knownAcceptableVals.icpTextAddress },
        });
        expect(result?.err?.kind).toStrictEqual({ InsufficientTransferAmount: null });
      });
      it('should reject when the caller is using insufficient e8s | #ICP_nns', async () => {
        const result = await transferer.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: 9999n },
          },
          destination: {
            CanisterExpected: {
              [SupportedTokens.asVariantTagLiteral.ICP_nns]:
                knownAcceptableVals.icpAccountIdentifier,
            },
          },
        });
        expect(result?.err?.kind).toStrictEqual({ InsufficientTransferAmount: null });
      });
      it('should reject when the caller is using insufficient icrc1 tokens | #ICRC1_ExampleToken', async () => {
        const result = await transferer.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: 500n,
          },
          destination: { HumanReadable: knownAcceptableVals.icrc1TextAddress },
        });
        expect(result?.err?.kind).toStrictEqual({ InsufficientTransferAmount: null });
      });
      it('should reject when the caller is using insufficient icrc1 tokens | #ICRC1_ExampleToken2', async () => {
        const result = await anAuthorizedCaller.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: 9999n,
          },
          destination: {
            CanisterExpected: {
              [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]:
                knownAcceptableVals.icrc1Account,
            },
          },
        });
        expect(result?.err?.kind).toStrictEqual({ InsufficientTransferAmount: null });
      });
    });
    describe('When the Destination is Invalid | -> err kind #InvalidDestination', () => {
      const excessive = [];
      for (let i = 0; i < 33; ++i) excessive.push(i);
      it('should reject if given invalid text as an account identifier destination | #ICP', async () => {
        const result = await transferer.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: 100000n },
          },
          destination: { HumanReadable: '⸎⸖' },
        });
        expect(result?.err?.kind).toStrictEqual({ InvalidDestination: null });
      });
      it('should reject if given an invalid account identifier as destination | #ICP', async () => {
        const result = await anAuthorizedCaller.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: 100000n },
          },
          destination: {
            CanisterExpected: {
              [SupportedTokens.asVariantTagLiteral.ICP]: Uint8Array.from(excessive),
            },
          },
        });
        expect(result?.err?.kind).toStrictEqual({ InvalidDestination: null });
      });
      it('should reject if given invalid text as an account identifier destination | #ICP_nns', async () => {
        const result = await transferer.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: 100000n },
          },
          destination: { HumanReadable: '⸖⸎' },
        });
        expect(result?.err?.kind).toStrictEqual({ InvalidDestination: null });
      });
      it('should reject if given an invalid account identifier as destination | #ICP_nns', async () => {
        const result = await anAuthorizedCaller.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: 100000n },
          },
          destination: {
            CanisterExpected: {
              [SupportedTokens.asVariantTagLiteral.ICP_nns]: Uint8Array.from([]),
            },
          },
        });
        expect(result?.err?.kind).toStrictEqual({ InvalidDestination: null });
      });
      it('should reject if given invalid text (reserved principal) as an account destination | #ICRC1_ExampleToken', async () => {
        const result = await transferer.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: 100000n,
          },
          destination: { HumanReadable: 'ddhvl-oibai-bqibi-ga6xx-6' },
        });
        expect(result?.err?.kind).toStrictEqual({ InvalidDestination: null });
      });
      it('should reject if given an invalid account as destination | #ICRC1_ExampleToken', async () => {
        const result = await anAuthorizedCaller.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: 100000n,
          },
          destination: {
            CanisterExpected: {
              [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: {
                owner: installerPrincipal.getPrincipal(),
                subaccount: [Uint8Array.from(excessive)],
              },
            },
          },
        });
        expect(result?.err?.kind).toStrictEqual({ InvalidDestination: null });
      });
      it('should reject if given invalid text as an icrc1 account destination | #ICRC1_ExampleToken2', async () => {
        const result = await transferer.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: 100000n,
          },
          destination: {
            HumanReadable: '!!!!!!!!!!!!!!!!!!!!!!!!!!!!this should be _so_ automated',
          },
        });
        expect(result?.err?.kind).toStrictEqual({ InvalidDestination: null });
      });
      it('should reject if given an invalid account as destination | #ICRC1_ExampleToken2', async () => {
        const result = await anAuthorizedCaller.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: 100000n,
          },
          destination: {
            CanisterExpected: {
              [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: {
                owner: installerPrincipal.getPrincipal(),
                subaccount: [Uint8Array.from([1, 2, 3])],
              },
            },
          },
        });
        expect(result?.err?.kind).toStrictEqual({ InvalidDestination: null });
      });
    });
    describe('Prevent Caller from Transferring More than their Balance | -> err kind #SupportedTokenTransferErr', () => {
      // Note, ICP and ICRC just happen to have same Transfer Err variant tag for InsufficientFunds!
      // Also note there are other Transfer Error types from token canisters, but for now
      // this is going to be enough as enumerating all of them is its own project.
      it('should reject if caller tries transferring more e8s than they have | #ICP', async () => {
        // Verify rejected for ICP if not enough balance.
        const result = await anAuthorizedCaller.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICP]: { e8s: 1000000000000n },
          },
          destination: { HumanReadable: knownAcceptableVals.icpTextAddress },
        });
        expect(
          result?.err?.kind?.SupportedTokenTransferErr?.[SupportedTokens.asVariantTagLiteral.ICP]
            ?.InsufficientFunds,
        ).toBeTruthy();
      });
      it('should reject if caller tries transferring more e8s than they have | #ICP_nns', async () => {
        // Verify rejected for ICP_nns if not enough balance.
        const result = await anAuthorizedCaller.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICP_nns]: { e8s: 1000000000000n },
          },
          destination: {
            CanisterExpected: {
              [SupportedTokens.asVariantTagLiteral.ICP_nns]:
                knownAcceptableVals.icpAccountIdentifier,
            },
          },
        });
        expect(
          result?.err?.kind?.SupportedTokenTransferErr?.[
            SupportedTokens.asVariantTagLiteral.ICP_nns
          ]?.InsufficientFunds,
        ).toBeTruthy();
      });
      it('should reject if caller tries transferring more e8s than they have | #ICRC1_ExampleToken', async () => {
        // Verify rejected for ICRC1_ExampleToken if not enough balance.
        const result = await anAuthorizedCaller.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken]: 1000000000000n,
          },
          destination: { HumanReadable: knownAcceptableVals.icrc1TextAddress },
        });
        expect(
          result?.err?.kind?.SupportedTokenTransferErr?.[
            SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken
          ]?.InsufficientFunds,
        ).toBeTruthy();
      });
      it('should reject if caller tries transferring more e8s than they have | #ICRC1_ExampleToken2', async () => {
        // Verify rejected for ICRC1_ExampleToken2 if not enough balance.
        const result = await anAuthorizedCaller.transfer({
          tokenAmount: {
            [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]: 1000000000000n,
          },
          destination: {
            CanisterExpected: {
              [SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2]:
                knownAcceptableVals.icrc1Account,
            },
          },
        });
        expect(
          result?.err?.kind?.SupportedTokenTransferErr?.[
            SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2
          ]?.InsufficientFunds,
        ).toBeTruthy();
      });
    });
  });
});
