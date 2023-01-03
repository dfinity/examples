const identityUtils = require("./utils/identity");
const { defaultActor, defaultIdentity, balanceHolder } = identityUtils;

const encoder = new TextEncoder();

const FEE = 10000n;

const testMeta = {
  seller: "ExampleSeller",
  token: "1234",
};
const testInvoice = {
  amount: 1000000n,
  token: {
    symbol: "ICP",
  },
  details: [
    {
      description: "Test invoice",
      meta: Array.from(encoder.encode(JSON.stringify(testMeta))),
    },
  ],
  permissions: [],
};

let createResult;

beforeAll(async () => {
  createResult = await defaultActor.create_invoice(testInvoice);
});

const resetBalance = async () => {
  let balance = await defaultActor.get_balance({
    token: {
      symbol: "ICP",
    },
  });
  if ("ok" in balance) {
    let amount = balance.ok.balance;
    if (amount > 0n) {
      // Transfer full balance back to the balance holder
      let result = await defaultActor.transfer({
        amount,
        destinationAddress: "675d3f2043c6bf5d642454cf0890b467673f0bacfd1c85882c0d650d4c6d2abb",
        token: {
          symbol: "ICP",
        }
      });
      return result;
    }
  }
};
afterEach(async () => {
  await resetBalance();
});
afterAll(async () => {
  await resetBalance();
});

jest.setTimeout(60000);

describe("ICP Tests", () => {
  /**
   * Account tests
   */
  describe("Account Tests", () => {
    it("should check a caller's icp balance", async () => {
      const balanceResult = await defaultActor.get_balance({
        token: {
          symbol: "ICP",
        },
      });
      expect(balanceResult).toStrictEqual({ ok: { balance: 0n } });
    });
    it("should fetch the account of the different principals", async () => {
      let identifier = await defaultActor.get_callers_consolidation_address({
        token: {
          symbol: "ICP",
        }
      });
      if ("ok" in identifier) {
        expect(identifier.ok.consolidationAddress).toStrictEqual(
          "289c5ef2c85f8f6109562f1d175f97d80a4d13aa58f2d2cfaa30f5dc7947ce1d"
        );
      } else {
        throw new Error(identifier.err.message);
      }
    });
  });
  /**
   * Invoice Tests
   */
  describe("Invoice Tests", () => {
    it("should allow for querying an invoice by ID", async () => {
      const invoice = await defaultActor.get_invoice({
        id: createResult.ok.invoice.id,
      });
      if ("ok" in invoice) {
        expect(invoice.ok.invoice).toStrictEqual(createResult.ok.invoice);
      } else {
        throw new Error(invoice.err.message);
      }
    });
    it("should reject get_invoice from unauthorized callers", async () => {
      const invoice = await balanceHolder.get_invoice({
        id: createResult.ok.invoice.id,
      });
      expect(invoice.err).toStrictEqual({
        kind: {
          NotAuthorized: null,
        },
        message: ["You do not have permission to view this invoice"],
      });
    });
    it("should allow get_invoice to be called by authorized callers", async () => {
      const invoice = await defaultActor.create_invoice({
        ...testInvoice,
        permissions: [
          {
            canGet: [identityUtils.balanceHolderIdentity.getPrincipal()],
            canVerify: [],
          },
        ],
      });

      const result = await balanceHolder.get_invoice({
        id: invoice.ok.invoice.id,
      });
      expect(result.ok).toBeTruthy();
    });
    it("should not mark a payment verified if the balance has not been paid", async () => {
      let verifyResult = await defaultActor.verify_invoice({
        id: createResult.ok.invoice.id,
      });
      expect(verifyResult).toStrictEqual({
        err: {
          kind: { NotYetPaid: null },
          message: ["Insufficient balance. Current Balance is 0"],
        },
      });
    });
    it("should mark an invoice verified if the balance has been paid", async () => {
      // Transfer balance to the balance holder
      await balanceHolder.transfer({
        amount: createResult.ok.invoice.amount + FEE,
        destinationAddress: createResult.ok?.invoice?.paymentAddress,
        token: {
          symbol: "ICP",
        },
      });

      // Verify the invoice
      let verifyResult = await defaultActor.verify_invoice({
        id: createResult.ok.invoice.id,
      });
      expect(verifyResult.ok?.Paid?.invoice?.paid).toBe(true);
    });
    it("should not allow a caller to verify an invoice if they are not the creator or on the allowlist", async () => {
      const invoice = await defaultActor.create_invoice(testInvoice);
      const result = await balanceHolder.verify_invoice({
        id: invoice.ok.invoice.id,
      });
      expect(result.err).toStrictEqual({
        kind: {
          NotAuthorized: null,
        },
        message: ["You do not have permission to verify this invoice"],
      });
    });
    it("should not allow a caller to verify an invoice if they are not the creator or on the allowlist", async () => {
      const invoice = await defaultActor.create_invoice(testInvoice);
      const result = await balanceHolder.verify_invoice({
        id: invoice.ok.invoice.id,
      });
      expect(result.err).toStrictEqual({
        kind: {
          NotAuthorized: null,
        },
        message: ["You do not have permission to verify this invoice"],
      });
    });
    it("should allow a non-creator caller to verify an invoice if they are on the allowlist", async () => {
      const invoice = await defaultActor.create_invoice({
        ...testInvoice,
        permissions: [
          {
            canGet: [],
            canVerify: [identityUtils.balanceHolderIdentity.getPrincipal()],
          },
        ],
      });
      const result = await balanceHolder.verify_invoice({
        id: invoice.ok.invoice.id,
      });
      expect(result.err.kind).toStrictEqual({ NotYetPaid: null });
    });
  });
  describe("already completed Invoice", () => {
    it("should return AlreadyVerified if an invoice has already been verified", async () => {
      let verifyResult = await defaultActor.verify_invoice({
        id: createResult.ok.invoice.id,
      });
      expect(verifyResult.ok.AlreadyVerified).toBeTruthy();
    });
  });
  /**
   * Transfer Tests
   */
  describe("Transfer Tests", () => {
    it("should increase a caller's icp balance after transferring to that account", async () => {
      resetBalance(); //?
      let destination = await defaultActor.get_callers_consolidation_address({
        token: { symbol: "ICP" },
      });
      let transferResult = await balanceHolder.transfer({
        amount: 1000000n,
        destinationAddress: destination.ok.consolidationAddress,
        token: {
          symbol: "ICP",
        }
      });
      if ("ok" in transferResult) {
        let newBalance = await defaultActor.get_balance({
          token: {
            symbol: "ICP",
          },
        });
        expect(newBalance).toStrictEqual({ ok: { balance: 1000000n - FEE } });
      }
    });
    it("should not allow a caller to transfer to an invalid account", async () => {
      let transferResult = await balanceHolder.transfer({
        amount: 1000000n,
        destinationAddress: "abc123",
        token: {
          symbol: "ICP",
        }
      });
      expect(transferResult).toStrictEqual({
        err: {
          kind: { InvalidDestination: null },
          message: ["Invalid account identifier"],
        },
      });
    });
    it("should not allow a caller to transfer more than their balance", async () => {
      let transferResult = await defaultActor.transfer({
        amount: 1000000n,
        destinationAddress: "675d3f2043c6bf5d642454cf0890b467673f0bacfd1c85882c0d650d4c6d2abb",
        token: {
          symbol: "ICP",
        }
      });
      expect(transferResult.err).toBeTruthy();
    });
  });
});
