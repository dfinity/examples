import { Principal } from "@dfinity/principal";
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
const excessiveMeta = {
  amount: 1_000_000n,
  token: {
    symbol: "ICP",
  },
  details: [
    {
      description: "Test invoice",
      meta: new Array(32_001).fill(0),
    },
  ],
  permissions: [],
};
const excessiveCanGet = {
  amount: 1_000_000n,
  token: {
    symbol: "ICP",
  },
  details: [],
  permissions: [
    {
      canGet: new Array(257).fill(Principal.fromText("aaaaa-aa")),
      canVerify: [],
    },
  ],
};
const excessiveCanVerify = {
  amount: 1_000_000n,
  token: {
    symbol: "ICP",
  },
  details: [],
  permissions: [
    {
      canGet: [],
      canVerify: new Array(257).fill(Principal.fromText("aaaaa-aa")),
    },
  ],
};

describe("Testing the creation of invoices", () => {
  it("should handle a correct invoice", async () => {
    const createResult = await defaultActor.create_invoice(testInvoice);
    if ("ok" in createResult) {
      // Test invoice exists
      expect(createResult.ok.invoice).toBeTruthy();

      // Test decoding invoice details
      let metaBlob = Uint8Array.from(createResult.ok.invoice.details[0].meta);
      let decodedMeta = JSON.parse(String.fromCharCode(...metaBlob));
      expect(decodedMeta).toStrictEqual(testMeta);
    } else {
      throw new Error(createResult.err.message);
    }
  });
  it("should return an error if the description is too large", async () => {
    const createResult = await defaultActor.create_invoice(excessiveMeta);
    expect(createResult.err.kind).toStrictEqual({ BadSize: null });
  });
  it("should return an error if the canRead permissions list is too large", async () => {
    const createResult = await defaultActor.create_invoice(excessiveCanGet);
    createResult;
    expect(createResult.err.kind).toStrictEqual({ BadSize: null });
  });
  it("should return an error if the canVerify permissions list is too large", async () => {
    const createResult = await defaultActor.create_invoice(excessiveCanVerify);
    createResult;
    expect(createResult.err.kind).toStrictEqual({ BadSize: null });
  });
});
