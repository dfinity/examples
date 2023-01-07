
import { Principal } from "@dfinity/principal";
const identityUtils = require("./utils/identity");
const { 
  defaultActor, 
  defaultIdentity, 
  balanceHolder, 
  delegatedAdministrator,
  anonymousActor,
  getRandomActor
} = identityUtils;

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
const lessThanMinimumBillableAmount = {
  amount: (FEE*2n)-1n,
  token: {
    symbol: "ICP",
  },
  details: [],
  permissions: [],
}

jest.setTimeout(60000);

beforeAll(async () => {
  // need to make sure these default identity has permission to create invoices
  let result = await delegatedAdministrator.get_allowed_creators_list();
  for (let p of result.ok.allowed) {
    await delegatedAdministrator.remove_allowed_creator({ who: p });
  }
  let defaultIdentityPrincipal = defaultIdentity.getPrincipal();
  result = await delegatedAdministrator.add_allowed_creator({ who: defaultIdentityPrincipal });
  if (result?.err) {  
    throw new Error("Couldn't add default identity to creators allowed list necessary for testing!")
  }
});

describe("Testing the creation of invoices", () => {
  it("should handle a correct invoice", async () => {
    const checkResult = (result) => {
      if ("ok" in result) {
        // Test invoice exists
        expect(result.ok.invoice).toBeTruthy();
  
        // Test decoding invoice details
        let metaBlob = Uint8Array.from(result.ok.invoice.details[0].meta);
        let decodedMeta = JSON.parse(String.fromCharCode(...metaBlob));
        expect(decodedMeta).toStrictEqual(testMeta);
      } else {
        throw new Error(result.err.message);
      }
    }
    checkResult(await defaultActor.create_invoice(testInvoice));
    checkResult(await delegatedAdministrator.create_invoice(testInvoice));
  });
  it("should return an error if the caller is unauthorized to create an invoice", async () => {
    let createResult = await anonymousActor.create_invoice(testInvoice);
    expect(createResult.err.kind).toStrictEqual({ NotAuthorized: null });
    createResult = await getRandomActor().create_invoice(testInvoice);
    expect(createResult.err.kind).toStrictEqual({ NotAuthorized: null });
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
  it("should return an error if the billable amount is less than the minimum required to internally transfer", async () => {
    const createResult = await defaultActor.create_invoice(lessThanMinimumBillableAmount);
    expect(createResult.err.kind).toStrictEqual({ InvalidAmount: null });
  });
});
