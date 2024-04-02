// Values known beforehand commonly used throughout E2E tests.
// In the future, will also use snapshots.
export const aPriori = {
  invoiceCanister: {
    canisterId: {
      principal: {
        asText: 'q4eej-kyaaa-aaaaa-aaaha-cai',
      },
    },
    // Used in to_other_address_form for non-trivial subaccount ICRC1 account case.
    icrc1Subaccount: {
      asText:
        '743p4-2qaaa-aaaaa-aaaha-cao46-yue4n-z3nri-6bgsh-eyppx-p2jyk-k32u4-ibdpd-f6f45-orwew-tmmaq-h6',
      subaccount: {
        asText: 'dcf6284e373b6c51e09a47261efbbf49c295bd538808de32f8bceba3625a6c60',
      },
    },
  },
  nnsFundedSecp256k1Identity: {
    asPrincipal: {
      asText: `hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe`,
    },
    // aka "asInvoiceCreatorPrincipalSubaccount"
    asInvoicePrincipalSubaccount: {
      icrc1: {
        asText: `743p4-2qaaa-aaaaa-aaaha-cao46-yue4n-z3nri-6bgsh-eyppx-p2jyk-k32u4-ibdpd-f6f45-orwew-tmmaq-h6`,
      },
      icp: {
        asText: `5bea0a832af66531a1c2dda9e4f027f6c31dd943af039ff509ae492841b8b980`,
      },
    },
    // Account or account identifier with subaccount of 32 zeros.
    asDefaultSubaccount: {
      icp: {
        asText: '2b8fbde99de881f695f279d2a892b1137bfe81a42d7694e064b1be58701e1138',
      },
      icrc1: {
        asText: 'hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe',
      },
    },
  },
};

// In case of variant tags' values need a changin' those can change here.
// Not to be used directly outside of this file (use SupportedTokens below).
const SupportTokenLiterals_ = {
  ICP: 'ICP',
  ICP_nns: 'ICP_nns',
  ICRC1_ExampleToken: 'ICRC1_ExampleToken',
  ICRC1_ExampleToken2: 'ICRC1_ExampleToken2',
};

// Used throughout tests to refer to the supported token type variant members.
// Note! This adds a _lot_ of verbosity to all the tests so I'd like to apologize for that. 
// I didn't know if I'd end up having to change the names or remove or add which token,
// and while there's a simpler way to systematically redeclare variant tags on the Js side,
// that would likely be part of an initiative to procedurally generate each test case used
// for any token to be added instead of manually iterating over all of the included ones,
// which is itself quite a different project all together.
export const SupportedTokens = {
  // Used when referencing variant by tag name.
  asVariantTagLiteral: {
    [`${SupportTokenLiterals_.ICP}`]: SupportTokenLiterals_.ICP,
    [`${SupportTokenLiterals_.ICP_nns}`]: SupportTokenLiterals_.ICP_nns,
    [`${SupportTokenLiterals_.ICRC1_ExampleToken}`]: SupportTokenLiterals_.ICRC1_ExampleToken,
    [`${SupportTokenLiterals_.ICRC1_ExampleToken2}`]: SupportTokenLiterals_.ICRC1_ExampleToken2,
  },
  // Used when passing variant as token type.
  asVariantUnitObj: {
    // Ie is identical to ICP: { ICP: null }
    [`${SupportTokenLiterals_.ICP}`]: { [`${SupportTokenLiterals_.ICP}`]: null },
    // Ie is identical to ICP_nns: { ICP_nns: null }
    [`${SupportTokenLiterals_.ICP_nns}`]: { [`${SupportTokenLiterals_.ICP_nns}`]: null },
    // Ie is identical to ICRC1_ExampleToken: { ICRC1_ExampleToken: null }
    [`${SupportTokenLiterals_.ICRC1_ExampleToken}`]: {
      [`${SupportTokenLiterals_.ICRC1_ExampleToken}`]: null,
    },
    // Ie is identical to ICRC1_ExampleToken2: { ICRC1_ExampleToken2: null }
    [`${SupportTokenLiterals_.ICRC1_ExampleToken2}`]: {
      [`${SupportTokenLiterals_.ICRC1_ExampleToken2}`]: null,
    },
  },
};
