import Array "mo:base/Array";
import Blob "mo:base/Blob";
import Debug "mo:base/Debug";
import Option "mo:base/Option";
import Principal "mo:base/Principal";
import Text "mo:base/Text";

import Migration "../../src/invoice/modules/Migration";
import SupportedToken "../../src/invoice/modules/supported-token/SupportedToken";
import { icpAdapter } "../../src/invoice/modules/supported-token/token-specific/icp/Adapter";
import { icrc1Adapter } "../../src/invoice/modules/supported-token/token-specific/icrc1/Adapter";
import ActorSpec "./utils/ActorSpec";
import ExpectedValues "./utils/ExpectedValues";

// Note on above imports:
// `SupportedToken` & `Migration` contain modules to unit test.
// `ExpectedValues` contains the manually created "snapshot" values to test against.

// Redeclaring to make it simpler to read.
let sTokens = SupportedToken;
let Adapter_ICP = icpAdapter;
let Adapter_ICRC1 = icrc1Adapter;
let Vals = ExpectedValues.Expected().Vals;

type Group = ActorSpec.Group;
let assertTrue = ActorSpec.assertTrue;
let assertAllTrue = ActorSpec.assertAllTrue;
let describe = ActorSpec.describe;
let it = ActorSpec.it;
let skip = ActorSpec.skip;
let pending = ActorSpec.pending;
let run = ActorSpec.run;

/**
  Tests all the methods of the SupportedToken modules and the migration module.  

  This is made up of:  
    -Addressing specific computations that map ICP and ICRC1 type annotations:  
    (Both have the same set of defined methods though with their own different addressing specific types;  
      perhaps if bounded generics were a thing in Motoko this could be simplified as an "abstract" adapter type)  
      1. The ICP Adapter which uses the AccountIdentifier and Subaccount types  
      2. the ICRC1 Adapter which uses the Account and Subaccount types  
    -Common transformations included that may or may not share type annotions  
      (ie those at the SupportedToken file scope)  
    -Migration for upgrading saved record types from the previous invoice canister's implementation.  

  --Glossary of methods tested--  
    Unit Tests for ICP and ICRC1 Adapter Modules (SupportedToken.mo > TokenSpecific > ICP | ICRC1 > Adapter):  
      isValidSubaccount  
      isValidAddress  
      encodeAddress  
      decodeAddress  
      computeInvoiceSubaccount  
      computeInvoiceSubaccountAddress  
      computeCreatorSubaccount  
      computeCreatorSubaccountAddress  
    Unit Tests for SupportedToken Module (ie those at the outermost scope):
      getTransactionFee  
      unwrapTokenAmount  
      wrapAsTokenAmount  
      getTokenVerbose
      encodeAddress  
      encodeAddressOrUnitErr  
      getAddressOrUnitErr  
      getInvoiceSubaccountAddress  
      getEncodedInvoiceSubaccountAddress  
      getCreatorSubaccountAddress  
      getTransferArgsFromInvoiceSubaccount  
      getTransferArgsFromCreatorSubaccount  
      rewrapTransferResults  
      getDefaultSubaccountAddress  
    Unit Tests for Migration Module  
      convertOne  

    Note that if the input or expected output is well defined (ie a variant) it will have its own case as an `it` 
  defined test (with the exception of rewraping transfer results which just uses one example for each token type). 
  For instance each `SupportedToken` variant is tested as its own it test case for each method in the `SupportedToken.mo`
  Module (in other words typically the tests for ICP | ICP_nns and ICRC1_ExampleToken | ICRC1_ExampleToken2 are identical 
  except for the variant tag used) (future work note: might be useful to procedurally generate each test case for a 
  given method as opposed to manually iterate through all of them).
  
    To make this easier to see at a glance in console output, all the tests' title literals are annotated with this information. 
  Ie: 
    describe("Recognizing Acceptable ICP Subaccounts | isValidSubaccount -> true", ...) followed by the three it tests such
    as it("should return false if it is an incomplete subaccount"), etc. 
  Or:
    describe("Correctly get the transfer fee of each supported token type regardless of variant's argument type | getTransactionFee")
    followed by it("should for ICP token type",...), it("should for ICP_nns token type",...), etc.
*/

// Checks if two icrc1 accounts are equal field to field.
func icrc1AccountsEqual(a : { owner : Principal; subaccount : ?Blob }, b : { owner : Principal; subaccount : ?Blob }) : Bool {
  // As an opt subaccount that is null is equivalent to the default subaccount both are converted as such
  let getAS = Option.get<Blob>(a.subaccount, Blob.fromArrayMut(Array.init(32, 0 : Nat8)));
  let getBS = Option.get<Blob>(b.subaccount, Blob.fromArrayMut(Array.init(32, 0 : Nat8)));
  (Principal.equal(a.owner, b.owner) and Blob.equal(getAS, getBS));
};

// Makes the console output more readable for the excessively longer test title literals.
func nlts(t : Text) : Text { t # "\n\n\t\t\t\t\t\t\t\t" };

let success = run([
  describe(
    "Supported Token Tests",
    [
      describe(
        "Token Standard Specific Addressing Computations",
        [
          describe(
            "ICP Adapter AccountIdentifier and Subaccount Computations",
            [
              describe(
                "Recognizing Faulty ICP Subaccounts | isValidSubaccount -> false",
                [
                  it(
                    "should return false if it is an empty subaccount",
                    do {
                      assertTrue(not Adapter_ICP.isValidSubaccount(Vals.Addressing.Err.ICP.AccountIdentifier.asEmpty));
                    },
                  ),
                  it(
                    "should return false if it is an incomplete subaccount",
                    do {
                      assertTrue(not Adapter_ICP.isValidSubaccount(Vals.Addressing.Err.ICP.AccountIdentifier.asCorruptedLength));
                    },
                  ),
                  it(
                    "should return false if it is an excessive subaccount",
                    do {
                      assertTrue(not Adapter_ICP.isValidSubaccount(Vals.Addressing.Err.ICP.AccountIdentifier.asExcessiveLength));
                    },
                  ),
                ],
              ),
              describe(
                "Recognizing Acceptable ICP Subaccounts | isValidSubaccount -> true",
                [
                  it(
                    "should return true if it is default subaccount blob of 32 0s",
                    do {
                      assertTrue(Adapter_ICP.isValidSubaccount(Vals.Addressing.defaultSubaccount));
                    },
                  ),
                  it(
                    "should return true if it is a known acceptable non trivial subaccount blob",
                    do {
                      assertTrue(Adapter_ICP.isValidSubaccount(Vals.Addressing.Ok.Invoice.ICP.InvoiceSubaccount.asFromSubaccount));
                    },
                  ),
                ],
              ),
              describe(
                "Recognizing Faulty Account Identifiers | isValidAddress -> false",
                [
                  it(
                    "should return false if it is an empty account identifier blob",
                    do {
                      assertTrue(not Adapter_ICP.isValidAddress(Vals.Addressing.Err.ICP.AccountIdentifier.asEmpty));
                    },
                  ),
                  it(
                    "should return false if it is an incomplete account identifier blob",
                    do {
                      assertTrue(not Adapter_ICP.isValidAddress(Vals.Addressing.Err.ICP.AccountIdentifier.asCorruptedLength));
                    },
                  ),
                  it(
                    "should return false if it is an account identifier blob of excessive length",
                    do {
                      assertTrue(not Adapter_ICP.isValidAddress(Vals.Addressing.Err.ICP.AccountIdentifier.asExcessiveLength));
                    },
                  ),
                  it(
                    "should return false if it as an account identifier blob with incorrect crc32 hash",
                    do {
                      assertTrue(not Adapter_ICP.isValidAddress(Vals.Addressing.Err.ICP.AccountIdentifier.asIncorrectCRC32Hash));
                    },
                  ),
                ],
              ),
              describe(
                "Recognizing Acceptable Account Identifiers | isValidAddress -> true",
                [
                  it(
                    "should return true if it is the known default subaccount of an account identifier",
                    do {
                      assertTrue(Adapter_ICP.isValidAddress(Vals.Addressing.Ok.ICP.AccountIdentifier.asAccountIdentifier));
                    },
                  ),
                  it(
                    "should return true if it is a known valid account identifier blob",
                    do {
                      assertTrue(Adapter_ICP.isValidSubaccount(Vals.Addressing.Ok.Invoice.ICP.InvoiceSubaccount.asAccountIdentifier));
                    },
                  ),
                ],
              ),
              it(
                "should encode acceptable account identifier | encodeAddress AcountIdentifier -> Text",
                do {
                  let result = Adapter_ICP.encodeAddress(Vals.Addressing.Ok.ICP.AccountIdentifier.asAccountIdentifier);
                  assertTrue(result == Vals.Addressing.Ok.ICP.AccountIdentifier.asText);
                },
              ),
              it(
                "should reject faulty text to be decoded into an account identifier | decodeAddress #err",
                do {
                  var wasOk : Bool = true;
                  switch (Adapter_ICP.decodeAddress("⸎⸖")) {
                    case (#ok a) {};
                    case (#err) wasOk := false;
                  };
                  assertTrue(not wasOk);
                },
              ),
              it(
                "should correctly decode an account identifier after checking if source text valid | decodeAddress #ok",
                do {
                  var accountIdResult = Blob.fromArray([]);
                  switch (Adapter_ICP.decodeAddress(Vals.Addressing.Ok.ICP.AccountIdentifier.asText)) {
                    case (#ok a) accountIdResult := a;
                    case (#err) {};
                  };
                  assertTrue(Blob.equal(accountIdResult, Vals.Addressing.Ok.ICP.AccountIdentifier.asAccountIdentifier));
                },
              ),
              it(
                "should compute an invoice subaccount from an id and creator's principal | computeInvoiceSubaccount",
                do {
                  let { id; creator; InvoiceSubaccount } = Vals.Addressing.Ok.Invoice.ICP;
                  let computed = Adapter_ICP.computeInvoiceSubaccount(id, creator);
                  assertTrue(Blob.equal(computed, InvoiceSubaccount.asFromSubaccount));
                },
              ),
              it(
                "should compute an invoice subaccount's account identifier from an id, principal and canister id | computeInvoiceSubaccountAddress",
                do {
                  let {
                    canisterId;
                    creator;
                    id;
                    InvoiceSubaccount;
                  } = Vals.Addressing.Ok.Invoice.ICP;
                  let computed = Adapter_ICP.computeInvoiceSubaccountAddress(id, creator, canisterId);
                  assertTrue(Blob.equal(computed, InvoiceSubaccount.asAccountIdentifier));
                },
              ),
              it(
                "should compute an invoice creator's subaccount from a principal | computeCreatorSubaccount",
                do {
                  let { creator; CreatorSubaccount } = Vals.Addressing.Ok.Invoice.ICP;
                  let computed = Adapter_ICP.computeCreatorSubaccount(creator);
                  assertTrue(Blob.equal(computed, CreatorSubaccount.asFromSubaccount));
                },
              ),
              it(
                "should compute an invoice creator's subaccount account identifier from their principal and a canister id | computeCreatorSubaccountAddress",
                do {
                  let {
                    canisterId;
                    creator;
                    CreatorSubaccount;
                  } = Vals.Addressing.Ok.Invoice.ICP;
                  let computed = Adapter_ICP.computeCreatorSubaccountAddress(creator, canisterId);
                  assertTrue(Blob.equal(computed, CreatorSubaccount.asAccountIdentifier));
                },
              ),
            ],
          ),
          describe(
            "ICRC1 Adapter Account and Subaccount Computations",
            [
              describe(
                "Recognizing Faulty ICRC1 Subaccounts | isValidSubaccount -> false",
                [
                  it(
                    "should return false if it is an empty subaccount blob",
                    do {
                      assertTrue(not Adapter_ICRC1.isValidSubaccount(Vals.Addressing.Err.ICRC1.corruptedLengthSubaccount));
                    },
                  ),
                  it(
                    "should return false if it is an incomplete subaccount blob",
                    do {
                      assertTrue(not Adapter_ICRC1.isValidSubaccount(Vals.Addressing.Err.ICRC1.emptySubaccount));
                    },
                  ),
                  it(
                    "should return false if it is a subaccount blob of excessive length",
                    do {
                      assertTrue(not Adapter_ICRC1.isValidSubaccount(Vals.Addressing.Err.ICRC1.excessiveLengthSubaccount));
                    },
                  ),
                ],
              ),
              describe(
                "Recognizing Acceptable ICRC1 Subaccounts | isValidSubaccount -> true",
                [
                  it(
                    "should return true if it is default subaccount blob of 32 0s",
                    do {
                      assertTrue(Adapter_ICRC1.isValidSubaccount(Vals.Addressing.defaultSubaccount));
                    },
                  ),
                  it(
                    "should return true if it is a known non trivial acceptable subaccount blob",
                    do {
                      assertTrue(Adapter_ICRC1.isValidSubaccount(Vals.Addressing.Ok.Invoice.ICRC1.InvoiceSubaccount.asFromSubaccount));
                    },
                  ),
                ],
              ),
              describe(
                "Recognizing Faulty ICRC1 Accounts | isValidAddress -> false",
                [
                  it(
                    "should return false if its subaccount blob is empty",
                    do {
                      assertTrue(not Adapter_ICRC1.isValidAddress(Vals.Addressing.Err.ICRC1.Account.asEmptySubaccount));
                    },
                  ),
                  it(
                    "should return false if its subaccount blob is of excessive length",
                    do {
                      assertTrue(not Adapter_ICRC1.isValidAddress(Vals.Addressing.Err.ICRC1.Account.asExcessiveSubaccount));
                    },
                  ),
                ],
              ),
              describe(
                "Recognizing Acceptable ICRC1 Accounts | isValidAddress -> true",
                [
                  it(
                    "should return true if it is the known default subaccount of an icrc1 account",
                    do {
                      assertTrue(Adapter_ICRC1.isValidAddress(Vals.Addressing.Ok.ICRC1.Account.asAccount));
                    },
                  ),
                  it(
                    "should return true if it is a known valid icrc1 account",
                    do {
                      assertTrue(Adapter_ICRC1.isValidAddress(Vals.Addressing.Ok.Invoice.ICRC1.InvoiceSubaccount.asAccount));
                    },
                  ),
                ],
              ),
              it(
                "should encode acceptable icrc1 account | encodeAddress Acount -> Text",
                do {
                  let result = Adapter_ICRC1.encodeAddress(Vals.Addressing.Ok.ICRC1.Account.asAccount);
                  assertTrue(result == Vals.Addressing.Ok.ICRC1.Account.asText);
                },
              ),
              it(
                "should reject faulty text to be decoded into an icrc1 account | decodeAddress #err",
                do {
                  var wasOk : Bool = true;
                  switch (Adapter_ICRC1.decodeAddress("⸎⸖")) {
                    case (#ok a) {};
                    case (#err) wasOk := false;
                  };
                  switch (Adapter_ICRC1.decodeAddress(Vals.Addressing.Err.ICRC1.Account.asText.reservedPrincipal)) {
                    case (#ok a) {};
                    case (#err) wasOk := false;
                  };
                  assertTrue(not wasOk);
                },
              ),
              it(
                "should correctly decode an account after checking if source text valid | decodeAddress #ok",
                do {
                  var accountResult = Vals.Addressing.Err.ICRC1.Account.asExcessiveSubaccount;
                  switch (Adapter_ICRC1.decodeAddress(Vals.Addressing.Ok.ICRC1.Account.asText)) {
                    case (#ok a) accountResult := a;
                    case (#err) {};
                  };
                  assertTrue(icrc1AccountsEqual(accountResult, Vals.Addressing.Ok.ICRC1.Account.asAccount));
                },
              ),
              it(
                "should compute an invoice subaccount from an id and creator's principal | computeInvoiceSubaccount",
                do {
                  let { id; creator; InvoiceSubaccount } = Vals.Addressing.Ok.Invoice.ICRC1;
                  let computed = Adapter_ICRC1.computeInvoiceSubaccount(id, creator);
                  assertTrue(Blob.equal(computed, InvoiceSubaccount.asFromSubaccount));
                },
              ),
              it(
                "should compute an invoice subaccount's icrc1 account from an id, principal and canister id | computeInvoiceSubaccountAddress",
                do {
                  let {
                    canisterId;
                    creator;
                    id;
                    InvoiceSubaccount;
                  } = Vals.Addressing.Ok.Invoice.ICRC1;
                  let computed = Adapter_ICRC1.computeInvoiceSubaccountAddress(id, creator, canisterId);
                  assertTrue(icrc1AccountsEqual(computed, InvoiceSubaccount.asAccount));
                },
              ),
              it(
                "should compute an invoice creator's subaccount from a principal | computeCreatorSubaccount",
                do {
                  let { creator; CreatorSubaccount } = Vals.Addressing.Ok.Invoice.ICRC1;
                  let computed = Adapter_ICRC1.computeCreatorSubaccount(creator);
                  assertTrue(Blob.equal(computed, CreatorSubaccount.asFromSubaccount));
                },
              ),
              it(
                "should compute an invoice creator's subaccount icrc1 account from a principal and canister id | computeCreatorSubaccountAddress",
                do {
                  let {
                    canisterId;
                    creator;
                    CreatorSubaccount;
                  } = Vals.Addressing.Ok.Invoice.ICRC1;
                  let computed = Adapter_ICRC1.computeCreatorSubaccountAddress(creator, canisterId);
                  assertTrue(icrc1AccountsEqual(computed, CreatorSubaccount.asAccount));
                },
              ),
            ],
          ),
        ],
      ),
      describe(
        "Supported Token Types' and Methods",
        [
          describe(
            nlts("Correctly get the transfer fee of each supported token type regardless of variant's argument type | getTransactionFee"),
            [
              it(
                "should for ICP token type",
                do {
                  let { ICP } = Vals.SupportedTokenTypes.TransferFees;
                  let icpFeeCorrect = ICP == sTokens.getTransactionFee((#ICP));
                  let icpFeeCorrect2 = ICP == sTokens.getTransactionFee((#ICP({ random = 10 })));
                  assertTrue(icpFeeCorrect and icpFeeCorrect2);
                },
              ),
              it(
                "should for ICP_nns token type",
                do {
                  let { ICP } = Vals.SupportedTokenTypes.TransferFees;
                  let icpnnsFeeCorrect = ICP == sTokens.getTransactionFee((#ICP_nns));
                  let icpnnsFeeCorrect2 = ICP == sTokens.getTransactionFee((#ICP_nns((2, 5, 10, 100))));
                  assertTrue(icpnnsFeeCorrect and icpnnsFeeCorrect2);
                },
              ),
              it(
                "should for ICRC1_ExampleToken type",
                do {
                  let { ICRC1 } = Vals.SupportedTokenTypes.TransferFees;
                  let icrc1ET1feeCorrect = ICRC1 == sTokens.getTransactionFee((#ICRC1_ExampleToken));
                  let icrc1ET1feeCorrect2 = ICRC1 == sTokens.getTransactionFee((#ICRC1_ExampleToken(#ICP)));
                  assertTrue(icrc1ET1feeCorrect and icrc1ET1feeCorrect2);
                },
              ),
              it(
                "should for ICRC1_ExampleToken2 type",
                do {
                  let { ICRC1 } = Vals.SupportedTokenTypes.TransferFees;
                  let icrc1ET2feeCorrect = ICRC1 == sTokens.getTransactionFee((#ICRC1_ExampleToken2));
                  let icrc1ET2feeCorrect2 = ICRC1 == sTokens.getTransactionFee((#ICRC1_ExampleToken({ random = "RecordFieldValue" })));
                  assertTrue(icrc1ET2feeCorrect and icrc1ET2feeCorrect2);
                },
              ),
            ],
          ),
          describe(
            nlts("Correctly unwrap each supported token amount type into unit type and base units | unwrapTokenAmount"),
            [
              it(
                "should for ICP token type",
                do {
                  let { ICP } = Vals.SupportedTokenTypes.UnitTypes;
                  let (icpType, amount) = sTokens.unwrapTokenAmount(#ICP({ e8s = 20_000_000 }));
                  let equals = (icpType == ICP) and (amount == 20_000_000);
                  assertTrue(equals);
                },
              ),
              it(
                "should for ICP_nns token type",
                do {
                  let { ICP_nns } = Vals.SupportedTokenTypes.UnitTypes;
                  let (icpnnsType, amountnns) = sTokens.unwrapTokenAmount(#ICP_nns({ e8s = 100_000_000 }));
                  let equals = (icpnnsType == ICP_nns) and (amountnns == 100_000_000);
                  assertTrue(equals);
                },
              ),
              it(
                "should for ICRC1_ExampleToken type",
                do {
                  let { ICRC1_ExampleToken } = Vals.SupportedTokenTypes.UnitTypes;
                  let (icrc1ET1Type, amountirc1ET1) = sTokens.unwrapTokenAmount(#ICRC1_ExampleToken(999_999_999_999_999));
                  let equals = (icrc1ET1Type == ICRC1_ExampleToken) and (amountirc1ET1 == 999_999_999_999_999);
                  assertTrue(equals);
                },
              ),
              it(
                "should for ICRC1_ExampleToken2 type",
                do {
                  let { ICRC1_ExampleToken2 } = Vals.SupportedTokenTypes.UnitTypes;
                  let (icrc1ET2Type, amountirc1ET2) = sTokens.unwrapTokenAmount(#ICRC1_ExampleToken2(999_999_999_999_999));
                  let equals = (icrc1ET2Type == ICRC1_ExampleToken2) and (amountirc1ET2 == 999_999_999_999_999);
                  assertTrue(equals);
                },
              ),
            ],
          ),
          describe(
            nlts("Correctly wrap base unit amounts into each supported token amount type | wrapAsTokenAmount"),
            [
              it(
                "should for ICP token type",
                do {
                  let icpTokenAmount = sTokens.wrapAsTokenAmount(#ICP, 20_000_000);
                  let equals = (debug_show (#ICP { e8s = 20_000_000 }) == debug_show (icpTokenAmount));
                  assertTrue(equals);
                },
              ),
              it(
                "should for ICP_nns token type",
                do {
                  let icpnnsTokenAmount = sTokens.wrapAsTokenAmount(#ICP_nns, 20_000_000);
                  let equals = (debug_show (#ICP_nns { e8s = 20_000_000 }) == debug_show (icpnnsTokenAmount));
                  assertTrue(equals);
                },
              ),
              it(
                "should for ICRC1_ExampleToken type",
                do {
                  let icrc1ET1TokenAmount = sTokens.wrapAsTokenAmount(#ICRC1_ExampleToken, 999_999_999_999_999);
                  let equals = (debug_show (#ICRC1_ExampleToken(999_999_999_999_999)) == debug_show (icrc1ET1TokenAmount));
                  assertTrue(equals);
                },
              ),
              it(
                "should for ICRC1_ExampleToken2 type",
                do {
                  let icrc1ET2TokenAmount = sTokens.wrapAsTokenAmount(#ICRC1_ExampleToken2, 999_999_999_999_999);
                  let equals = (debug_show (#ICRC1_ExampleToken2(999_999_999_999_999)) == debug_show (icrc1ET2TokenAmount));
                  assertTrue(equals);
                },
              ),
            ],
          ),
          describe(
            nlts("Correctly get each supported verbose token type | getTokenVerbose"),
            [
              it(
                "should for ICP token type",
                do {
                  let { ICP } = Vals.SupportedTokenTypes.VerboseToken;
                  let equals = debug_show (ICP) == debug_show (sTokens.getTokenVerbose((#ICP)));
                  assertTrue(equals);
                },
              ),
              it(
                "should for ICP_nns token type",
                do {
                  let { ICP_nns } = Vals.SupportedTokenTypes.VerboseToken;
                  let equals = debug_show (ICP_nns) == debug_show (sTokens.getTokenVerbose((#ICP_nns)));
                  assertTrue(equals);
                },
              ),
              it(
                "should for ICRC1_ExampleToken type",
                do {
                  let { ICRC1_ExampleToken } = Vals.SupportedTokenTypes.VerboseToken;
                  let equals = debug_show (ICRC1_ExampleToken) == debug_show (sTokens.getTokenVerbose((#ICRC1_ExampleToken)));
                  assertTrue(equals);
                },
              ),
              it(
                "should for ICRC1_ExampleToken2 type",
                do {
                  let { ICRC1_ExampleToken2 } = Vals.SupportedTokenTypes.VerboseToken;
                  let equals = debug_show (ICRC1_ExampleToken2) == debug_show (sTokens.getTokenVerbose((#ICRC1_ExampleToken2)));
                  assertTrue(equals);
                },
              ),
            ],
          ),
          describe(
            nlts("Correctly encode valid addresses | encodeAddress Address -> Text"),
            [
              it(
                "should for ICP token type",
                do {
                  let icpAddressText = Vals.Addressing.Ok.ICP.AccountIdentifier.asText;
                  let icpAddressToTextResult = sTokens.encodeAddress(#ICP(Vals.Addressing.Ok.ICP.AccountIdentifier.asAccountIdentifier));
                  let icpEquals = (icpAddressText == icpAddressToTextResult);
                  assertTrue(icpEquals);
                },
              ),
              it(
                "should for ICP_nns token type",
                do {
                  let icpnnsAddressText = Vals.Addressing.Ok.ICP.AccountIdentifier.asText;
                  let icpnnsAddressToTextResult = sTokens.encodeAddress(#ICP_nns(Vals.Addressing.Ok.ICP.AccountIdentifier.asAccountIdentifier));
                  let icpnnsEquals = (icpnnsAddressText == icpnnsAddressToTextResult);
                  assertTrue(icpnnsEquals);
                },
              ),
              it(
                "should for ICRC1_ExampleToken type",
                do {
                  let icrc1ET1AddressText = Vals.Addressing.Ok.ICRC1.Account.asText;
                  let icrc1ET1AddressToTextResult = sTokens.encodeAddress(#ICRC1_ExampleToken(Vals.Addressing.Ok.ICRC1.Account.asAccount));
                  let icrc1ET1Equals = (icrc1ET1AddressText == icrc1ET1AddressToTextResult);
                  assertTrue(icrc1ET1Equals);
                },
              ),
              it(
                "should for ICRC1_ExampleToken2 type",
                do {
                  let icrc1ET2AddressText = Vals.Addressing.Ok.ICRC1.Account.asText;
                  let icrc1ET2AddressToTextResult = sTokens.encodeAddress(#ICRC1_ExampleToken2(Vals.Addressing.Ok.ICRC1.Account.asAccount));
                  let icrc1ET2Equals = (icrc1ET2AddressText == icrc1ET2AddressToTextResult);
                  assertTrue(icrc1ET2Equals);
                },
              ),
            ],
          ),
          describe(
            nlts("Correctly check if valid and encode an address if so | encodeAddress #ok Address -> Text"),
            [
              it(
                "should as a result #ok argument for ICP token type",
                do {
                  let icpAddressText = Vals.Addressing.Ok.ICP.AccountIdentifier.asText;
                  var icpAddressToTextResult = "Ø";
                  switch (sTokens.encodeAddressOrUnitErr(#ICP(Vals.Addressing.Ok.ICP.AccountIdentifier.asAccountIdentifier))) {
                    case (#ok res) icpAddressToTextResult := res;
                    case (#err) {};
                  };
                  let icpEquals = (icpAddressText == icpAddressToTextResult);
                  assertTrue(icpEquals);
                },
              ),
              it(
                "should as a result #ok argument for ICP_nns token type",
                do {
                  let icpnnsAddressText = Vals.Addressing.Ok.ICP.AccountIdentifier.asText;
                  var icpnnsAddressToTextResult = "Ø";
                  switch (sTokens.encodeAddressOrUnitErr(#ICP_nns(Vals.Addressing.Ok.ICP.AccountIdentifier.asAccountIdentifier))) {
                    case (#ok res) icpnnsAddressToTextResult := res;
                    case (#err) {};
                  };
                  let icpnnsEquals = (icpnnsAddressText == icpnnsAddressToTextResult);
                  assertTrue(icpnnsEquals);
                },
              ),
              it(
                "should as a result #ok argument for ICRC1_ExampleToken type",
                do {
                  let icrc1ET1AddressText = Vals.Addressing.Ok.ICRC1.Account.asText;
                  var icrc1ET1ddressToTextResult = "Ø";
                  switch (sTokens.encodeAddressOrUnitErr(#ICRC1_ExampleToken(Vals.Addressing.Ok.ICRC1.Account.asAccount))) {
                    case (#ok res) icrc1ET1ddressToTextResult := res;
                    case (#err) {};
                  };
                  let icrc1ET1Equals = (icrc1ET1AddressText == icrc1ET1ddressToTextResult);
                  assertTrue(icrc1ET1Equals);
                },
              ),
              it(
                "should as a result #ok argument for ICRC1_ExampleToken2 type",
                do {
                  let icrc1ET2AddressText = Vals.Addressing.Ok.ICRC1.Account.asText;
                  var icrc1ET2ddressToTextResult = "Ø";
                  switch (sTokens.encodeAddressOrUnitErr(#ICRC1_ExampleToken2(Vals.Addressing.Ok.ICRC1.Account.asAccount))) {
                    case (#ok res) icrc1ET2ddressToTextResult := res;
                    case (#err) {};
                  };
                  let icrc1ET2Equals = (icrc1ET2AddressText == icrc1ET2ddressToTextResult);
                  assertTrue(icrc1ET2Equals);
                },
              ),
            ],
          ),
          describe(
            nlts("Reject invalid addresses that cannot be encoded | encodeAddress #err Address -> Text"),
            [
              it(
                "should as a result #err for ICP token type",
                do {
                  var ok = true;
                  switch (sTokens.encodeAddressOrUnitErr(#ICP(Vals.Addressing.Err.ICP.AccountIdentifier.asIncorrectCRC32Hash))) {
                    case (#ok res) {};
                    case (#err) ok := false;
                  };
                  assertTrue(not ok);
                },
              ),
              it(
                "should as a result #err for ICP_nns token type",
                do {
                  var ok = true;
                  switch (sTokens.encodeAddressOrUnitErr(#ICP_nns(Vals.Addressing.Err.ICP.AccountIdentifier.asExcessiveLength))) {
                    case (#ok res) {};
                    case (#err) ok := false;
                  };
                  assertTrue(not ok);
                },
              ),
              it(
                "should as a result #err for ICRC1_ExampleToken type",
                do {
                  var ok = true;
                  switch (sTokens.encodeAddressOrUnitErr(#ICRC1_ExampleToken(Vals.Addressing.Err.ICRC1.Account.asExcessiveSubaccount))) {
                    case (#ok res) {};
                    case (#err) ok := false;
                  };
                  assertTrue(not ok);
                },
              ),
              it(
                "should as a result #err for ICRC1_ExampleToken2 type",
                do {
                  var ok = true;
                  switch (sTokens.encodeAddressOrUnitErr(#ICRC1_ExampleToken2(Vals.Addressing.Err.ICRC1.Account.asEmptySubaccount))) {
                    case (#ok res) {};
                    case (#err) ok := false;
                  };
                  assertTrue(not ok);
                },
              ),
            ],
          ),
          describe(
            nlts("Correctly get an address decoded from checking valid text | getAddressOrUnitErr #ok Text -> Address"),
            [
              it(
                "should as a result #ok argument for ICP token type",
                do {
                  var equal = false;
                  switch (sTokens.getAddressOrUnitErr(#ICP, #HumanReadable(Vals.Addressing.Ok.ICP.AccountIdentifier.asText))) {
                    case (#err) {};
                    case (#ok address) {
                      switch address {
                        case (#ICP address) equal := Blob.equal(address, Vals.Addressing.Ok.ICP.AccountIdentifier.asAccountIdentifier);
                        case (#ICP_nns _) {};
                        case (#ICRC1_ExampleToken _) {};
                        case (#ICRC1_ExampleToken2 _) {};
                      };
                    };
                  };
                  assertTrue(equal);
                },
              ),
              it(
                "should as a result #ok argument for ICP_nns token type",
                do {
                  var equal = false;
                  switch (sTokens.getAddressOrUnitErr(#ICP_nns, #HumanReadable(Vals.Addressing.Ok.ICP.AccountIdentifier.asText))) {
                    case (#err) {};
                    case (#ok address) {
                      switch address {
                        case (#ICP _) {};
                        case (#ICP_nns address) equal := Blob.equal(address, Vals.Addressing.Ok.ICP.AccountIdentifier.asAccountIdentifier);
                        case (#ICRC1_ExampleToken _) {};
                        case (#ICRC1_ExampleToken2 _) {};
                      };
                    };
                  };
                  assertTrue(equal);
                },
              ),
              it(
                "should as a result #ok argument for ICRC1_ExampleToken type",
                do {
                  var equal = false;
                  switch (sTokens.getAddressOrUnitErr(#ICRC1_ExampleToken, #HumanReadable(Vals.Addressing.Ok.ICRC1.Account.asText))) {
                    case (#err) {};
                    case (#ok address) {
                      switch address {
                        case (#ICP _) {};
                        case (#ICP_nns _) {};
                        case (#ICRC1_ExampleToken address) equal := icrc1AccountsEqual(address, Vals.Addressing.Ok.ICRC1.Account.asAccount);
                        case (#ICRC1_ExampleToken2 _) {};
                      };
                    };
                  };
                  assertTrue(equal);
                },
              ),
              it(
                "should as a result #ok argument for ICRC1_ExampleToken2 type",
                do {
                  var equal = false;
                  switch (sTokens.getAddressOrUnitErr(#ICRC1_ExampleToken2, #HumanReadable(Vals.Addressing.Ok.ICRC1.Account.asText))) {
                    case (#err) {};
                    case (#ok address) {
                      switch address {
                        case (#ICP _) {};
                        case (#ICP_nns _) {};
                        case (#ICRC1_ExampleToken _) {};
                        case (#ICRC1_ExampleToken2 address) equal := icrc1AccountsEqual(address, Vals.Addressing.Ok.ICRC1.Account.asAccount);
                      };
                    };
                  };
                  assertTrue(equal);
                },
              ),
            ],
          ),
          describe(
            nlts("Reject invalid text to be decoded as an address | getAddressOrUnitErr #err Text -> Address"),
            [
              it(
                "should as a result #err for ICP token type",
                do {
                  var isErr = false;
                  switch (sTokens.getAddressOrUnitErr(#ICP, #HumanReadable("🤯🤦🦟"))) {
                    case (#err) isErr := true;
                    case (#ok _) {};
                  };
                  assertTrue(isErr);
                },
              ),
              it(
                "should as a result #err for ICP_nns token type",
                do {
                  var isErr = false;
                  switch (sTokens.getAddressOrUnitErr(#ICP_nns, #HumanReadable("🤯🤦🦟"))) {
                    case (#err) isErr := true;
                    case (#ok _) {};
                  };
                  assertTrue(isErr);
                },
              ),
              it(
                "should as a result #err for ICRC1_ExampleToken type",
                do {
                  var isErr = false;
                  switch (sTokens.getAddressOrUnitErr(#ICP_nns, #HumanReadable("🤯🤦🦟"))) {
                    case (#err) isErr := true;
                    case (#ok _) {};
                  };
                  assertTrue(isErr);
                },
              ),
              it(
                "should as a result #err for ICRC1_ExampleToken2 type",
                do {
                  var isErr = false;
                  switch (sTokens.getAddressOrUnitErr(#ICP_nns, #HumanReadable("🤯🤦🦟"))) {
                    case (#err) isErr := true;
                    case (#ok _) {};
                  };
                  assertTrue(isErr);
                },
              ),
            ],
          ),
          describe(
            nlts("Correctly get an invoice subaccount address | getInvoiceSubaccountAddress"),
            [
              it(
                "should for ICP token type",
                do {
                  let { canisterId; creator; id; InvoiceSubaccount } = Vals.Addressing.Ok.Invoice.ICP;
                  var icpEquals = false;
                  switch (sTokens.getInvoiceSubaccountAddress({ token = #ICP; id; creator; canisterId })) {
                    case (#ICP address) icpEquals := Blob.equal(address, InvoiceSubaccount.asAccountIdentifier);
                    case (#ICP_nns _) {};
                    case (#ICRC1_ExampleToken _) {};
                    case (#ICRC1_ExampleToken2 _) {};
                  };
                  assertTrue(icpEquals);
                },
              ),
              it(
                "should for ICP_nns token type",
                do {
                  let { canisterId; creator; id; InvoiceSubaccount } = Vals.Addressing.Ok.Invoice.ICP;
                  var icpnnsEquals = false;
                  switch (sTokens.getInvoiceSubaccountAddress({ token = #ICP_nns; id; creator; canisterId })) {
                    case (#ICP _) {};
                    case (#ICP_nns address) icpnnsEquals := Blob.equal(address, InvoiceSubaccount.asAccountIdentifier);
                    case (#ICRC1_ExampleToken _) {};
                    case (#ICRC1_ExampleToken2 _) {};
                  };
                  assertTrue(icpnnsEquals);
                },
              ),
              it(
                "should for ICRC1_ExampleToken type",
                do {
                  let {
                    canisterId;
                    creator;
                    id;
                    InvoiceSubaccount;
                  } = Vals.Addressing.Ok.Invoice.ICRC1;
                  var ircrc1ET1Equals = false;
                  switch (sTokens.getInvoiceSubaccountAddress({ token = #ICRC1_ExampleToken; id; creator; canisterId })) {
                    case (#ICP _) {};
                    case (#ICP_nns _) {};
                    case (#ICRC1_ExampleToken address) {
                      ircrc1ET1Equals := icrc1AccountsEqual(address, InvoiceSubaccount.asAccount);
                    };
                    case (#ICRC1_ExampleToken2 _) {};
                  };
                  assertTrue(ircrc1ET1Equals);
                },
              ),
              it(
                "should for ICRC1_ExampleToken2 type",
                do {
                  let {
                    canisterId;
                    creator;
                    id;
                    InvoiceSubaccount;
                  } = Vals.Addressing.Ok.Invoice.ICRC1;
                  var ircrc1ET2Equals = false;
                  switch (sTokens.getInvoiceSubaccountAddress({ token = #ICRC1_ExampleToken2; id; creator; canisterId })) {
                    case (#ICP _) {};
                    case (#ICP_nns _) {};
                    case (#ICRC1_ExampleToken _) {};
                    case (#ICRC1_ExampleToken2 address) ircrc1ET2Equals := icrc1AccountsEqual(address, InvoiceSubaccount.asAccount);
                  };
                  assertTrue(ircrc1ET2Equals);
                },
              ),
            ],
          ),
          describe(
            nlts("Correctly get an encoded invoice subaccount address | getEncodedInvoiceSubaccountAddress"),
            [
              it(
                "should for ICP token type",
                do {
                  let { canisterId; creator; id; InvoiceSubaccount } = Vals.Addressing.Ok.Invoice.ICP;
                  let result = sTokens.getEncodedInvoiceSubaccountAddress({
                    token = #ICP;
                    id;
                    creator;
                    canisterId;
                  });
                  assertTrue((result == InvoiceSubaccount.asAccountIdentifierText));
                },
              ),
              it(
                "should for ICP_nns token type",
                do {
                  let { canisterId; creator; id; InvoiceSubaccount } = Vals.Addressing.Ok.Invoice.ICP;
                  let result = sTokens.getEncodedInvoiceSubaccountAddress({
                    token = #ICP_nns;
                    id;
                    creator;
                    canisterId;
                  });
                  assertTrue((result == InvoiceSubaccount.asAccountIdentifierText));
                },
              ),
              it(
                "should for ICRC1_ExampleToken type",
                do {
                  let {
                    canisterId;
                    creator;
                    id;
                    InvoiceSubaccount;
                  } = Vals.Addressing.Ok.Invoice.ICRC1;
                  let result = sTokens.getEncodedInvoiceSubaccountAddress({
                    token = #ICRC1_ExampleToken;
                    id;
                    creator;
                    canisterId;
                  });
                  assertTrue((result == InvoiceSubaccount.asText));
                },
              ),
              it(
                "should for ICRC1_ExampleToken2 type",
                do {
                  let {
                    canisterId;
                    creator;
                    id;
                    InvoiceSubaccount;
                  } = Vals.Addressing.Ok.Invoice.ICRC1;
                  let result = sTokens.getEncodedInvoiceSubaccountAddress({
                    token = #ICRC1_ExampleToken2;
                    id;
                    creator;
                    canisterId;
                  });
                  assertTrue((result == InvoiceSubaccount.asText));
                },
              ),
            ],
          ),
          describe(
            nlts("Correctly get a creator's subaccount address | getInvoiceSubaccountAddress"),
            [
              it(
                "should for ICP token type",
                do {
                  let {
                    canisterId;
                    creator;
                    CreatorSubaccount;
                  } = Vals.Addressing.Ok.Invoice.ICP;
                  var icpEquals = false;
                  switch (sTokens.getCreatorSubaccountAddress({ token = #ICP; creator; canisterId })) {
                    case (#ICP address) icpEquals := Blob.equal(address, CreatorSubaccount.asAccountIdentifier);
                    case (#ICP_nns _) {};
                    case (#ICRC1_ExampleToken _) {};
                    case (#ICRC1_ExampleToken2 _) {};
                  };
                  assertTrue(icpEquals);
                },
              ),
              it(
                "should for ICP_nns token type",
                do {
                  let {
                    canisterId;
                    creator;
                    CreatorSubaccount;
                  } = Vals.Addressing.Ok.Invoice.ICP;
                  var icpnnsEquals = false;
                  switch (sTokens.getCreatorSubaccountAddress({ token = #ICP_nns; creator; canisterId })) {
                    case (#ICP _) {};
                    case (#ICP_nns address) icpnnsEquals := Blob.equal(address, CreatorSubaccount.asAccountIdentifier);
                    case (#ICRC1_ExampleToken _) {};
                    case (#ICRC1_ExampleToken2 _) {};
                  };
                  assertTrue(icpnnsEquals);
                },
              ),
              it(
                "should for ICRC1_ExampleToken type",
                do {
                  let {
                    canisterId;
                    creator;
                    CreatorSubaccount;
                  } = Vals.Addressing.Ok.Invoice.ICRC1;
                  var ircrc1ET1Equals = false;
                  switch (sTokens.getCreatorSubaccountAddress({ token = #ICRC1_ExampleToken; creator; canisterId })) {
                    case (#ICP _) {};
                    case (#ICP_nns _) {};
                    case (#ICRC1_ExampleToken address) ircrc1ET1Equals := icrc1AccountsEqual(address, CreatorSubaccount.asAccount);
                    case (#ICRC1_ExampleToken2 _) {};
                  };
                  assertTrue(ircrc1ET1Equals);
                },
              ),
              it(
                "should for ICRC1_ExampleToken2 type",
                do {
                  let {
                    canisterId;
                    creator;
                    CreatorSubaccount;
                  } = Vals.Addressing.Ok.Invoice.ICRC1;
                  var ircrc1ET2Equals = false;
                  switch (sTokens.getCreatorSubaccountAddress({ token = #ICRC1_ExampleToken2; creator; canisterId })) {
                    case (#ICP _) {};
                    case (#ICP_nns _) {};
                    case (#ICRC1_ExampleToken _) {};
                    case (#ICRC1_ExampleToken2 address) ircrc1ET2Equals := icrc1AccountsEqual(address, CreatorSubaccount.asAccount);
                  };
                  assertTrue(ircrc1ET2Equals);
                },
              ),
            ],
          ),
          describe(
            nlts("Correctly get the transfer args from as invoice subaccount, to as invoice creator subaccount | getTransferArgsFromInvoiceSubaccount (to creator's subaccount)"),
            [
              it(
                "should for ICP token type",
                do {
                  let {
                    creator;
                    id;
                    amountLessTheFee;
                  } = Vals.TransferArgs.Inputs;
                  let argsIn = {
                    to = #ICP(Vals.Addressing.Ok.Invoice.ICP.CreatorSubaccount.asAccountIdentifier);
                    amountLessTheFee;
                    fee = Vals.SupportedTokenTypes.TransferFees.ICP;
                    id;
                    creator;
                  };
                  var literalResult = "≟";
                  switch (sTokens.getTransferArgsFromInvoiceSubaccount(argsIn)) {
                    case (#ICP tr) {
                      // literalResult the result as a debug_show, efficient means of checking equality.
                      literalResult := debug_show (tr);
                    };
                    case (#ICP_nns _) {};
                    case (#ICRC1_ExampleToken _) {};
                    case (#ICRC1_ExampleToken2 _) {};
                  };
                  let expectedResult = debug_show (Vals.TransferArgs.Outputs.SenderAsInvoiceSubaccount.ICP.toInvoiceCreatorPrincipalSubaccountResult);
                  assertTrue((literalResult == expectedResult));
                },
              ),
              it(
                "should for ICP_nns token type",
                do {
                  let {
                    creator;
                    id;
                    amountLessTheFee;
                  } = Vals.TransferArgs.Inputs;
                  let argsIn = {
                    to = #ICP_nns(Vals.Addressing.Ok.Invoice.ICP.CreatorSubaccount.asAccountIdentifier);
                    amountLessTheFee;
                    fee = Vals.SupportedTokenTypes.TransferFees.ICP;
                    id;
                    creator;
                  };
                  var literalResult = "≟";
                  switch (sTokens.getTransferArgsFromInvoiceSubaccount(argsIn)) {
                    case (#ICP _) {};
                    case (#ICP_nns tr) {
                      // Get the result as a debug_show, efficient means of checking equality.
                      literalResult := debug_show (tr);
                    };
                    case (#ICRC1_ExampleToken _) {};
                    case (#ICRC1_ExampleToken2 _) {};
                  };
                  let expectedResult = debug_show (Vals.TransferArgs.Outputs.SenderAsInvoiceSubaccount.ICP.toInvoiceCreatorPrincipalSubaccountResult);
                  assertTrue((literalResult == expectedResult));
                },
              ),
              it(
                "should for ICRC1_ExampleToken type",
                do {
                  let {
                    creator;
                    id;
                    amountLessTheFee;
                  } = Vals.TransferArgs.Inputs;
                  let argsIn = {
                    to = #ICRC1_ExampleToken(Vals.Addressing.Ok.Invoice.ICRC1.CreatorSubaccount.asAccount);
                    amountLessTheFee;
                    fee = Vals.SupportedTokenTypes.TransferFees.ICRC1;
                    id;
                    creator;
                  };
                  var literalResult = "≟";
                  switch (sTokens.getTransferArgsFromInvoiceSubaccount(argsIn)) {
                    case (#ICP _) {};
                    case (#ICP_nns _) {};
                    case (#ICRC1_ExampleToken tr) {
                      literalResult := debug_show (tr);
                    };
                    case (#ICRC1_ExampleToken2 _) {};
                  };
                  let expectedResult = debug_show (Vals.TransferArgs.Outputs.SenderAsInvoiceSubaccount.ICRC1.toInvoiceCreatorPrincipalSubaccountResult);
                  assertTrue((literalResult == expectedResult));
                },
              ),
              it(
                "should for ICRC1_ExampleToken2 type",
                do {
                  let {
                    creator;
                    id;
                    amountLessTheFee;
                  } = Vals.TransferArgs.Inputs;
                  let argsIn = {
                    to = #ICRC1_ExampleToken2(Vals.Addressing.Ok.Invoice.ICRC1.CreatorSubaccount.asAccount);
                    amountLessTheFee;
                    fee = Vals.SupportedTokenTypes.TransferFees.ICRC1;
                    id;
                    creator;
                  };
                  var literalResult = "≟";
                  switch (sTokens.getTransferArgsFromInvoiceSubaccount(argsIn)) {
                    case (#ICP _) {};
                    case (#ICP_nns _) {};
                    case (#ICRC1_ExampleToken _) {};
                    case (#ICRC1_ExampleToken2 tr) {
                      literalResult := debug_show (tr);
                    };
                  };
                  let expectedResult = debug_show (Vals.TransferArgs.Outputs.SenderAsInvoiceSubaccount.ICRC1.toInvoiceCreatorPrincipalSubaccountResult);
                  assertTrue((literalResult == expectedResult));
                },
              ),
            ],
          ),
          describe(
            nlts("Correctly get transfer args from invoice subaccount, to as an arbitrary address | getTransferArgsFromInvoiceSubaccount (to any address)"),
            [
              it(
                "should for ICP token type",
                do {
                  let {
                    creator;
                    id;
                    amountLessTheFee;
                    to;
                  } = Vals.TransferArgs.Inputs;
                  let argsIn = {
                    to = #ICP(to.ICP.accountIdentifier);
                    amountLessTheFee;
                    fee = Vals.SupportedTokenTypes.TransferFees.ICP;
                    id;
                    creator;
                  };
                  var literalResult = "≟";
                  switch (sTokens.getTransferArgsFromInvoiceSubaccount(argsIn)) {
                    case (#ICP tr) {
                      // literalResult the result as a debug_show, efficient means of checking equality.
                      literalResult := debug_show (tr);
                    };
                    case (#ICP_nns _) {};
                    case (#ICRC1_ExampleToken _) {};
                    case (#ICRC1_ExampleToken2 _) {};
                  };
                  let expectedResult = debug_show (Vals.TransferArgs.Outputs.SenderAsInvoiceSubaccount.ICP.toArbitaryValidICPAccountIdentifierResult);
                  assertTrue((literalResult == expectedResult));
                },
              ),
              it(
                "should for ICP_nns token type",
                do {
                  let {
                    creator;
                    id;
                    amountLessTheFee;
                    to;
                  } = Vals.TransferArgs.Inputs;
                  let argsIn = {
                    to = #ICP_nns(to.ICP.accountIdentifier);
                    amountLessTheFee;
                    fee = Vals.SupportedTokenTypes.TransferFees.ICP;
                    id;
                    creator;
                  };
                  var literalResult = "≟";
                  switch (sTokens.getTransferArgsFromInvoiceSubaccount(argsIn)) {
                    case (#ICP _) {};
                    case (#ICP_nns tr) {
                      literalResult := debug_show (tr);
                    };
                    case (#ICRC1_ExampleToken _) {};
                    case (#ICRC1_ExampleToken2 _) {};
                  };
                  let expectedResult = debug_show (Vals.TransferArgs.Outputs.SenderAsInvoiceSubaccount.ICP.toArbitaryValidICPAccountIdentifierResult);
                  assertTrue((literalResult == expectedResult));
                },
              ),
              it(
                "should for ICRC1_ExampleToken type",
                do {
                  let {
                    creator;
                    id;
                    amountLessTheFee;
                    to;
                  } = Vals.TransferArgs.Inputs;
                  let argsIn = {
                    to = #ICRC1_ExampleToken(to.ICRC1.account);
                    amountLessTheFee;
                    fee = Vals.SupportedTokenTypes.TransferFees.ICRC1;
                    id;
                    creator;
                  };
                  var literalResult = "≟";
                  switch (sTokens.getTransferArgsFromInvoiceSubaccount(argsIn)) {
                    case (#ICP _) {};
                    case (#ICP_nns _) {};
                    case (#ICRC1_ExampleToken tr) {
                      literalResult := debug_show (tr);
                    };
                    case (#ICRC1_ExampleToken2 _) {};
                  };
                  let expectedResult = debug_show (Vals.TransferArgs.Outputs.SenderAsInvoiceSubaccount.ICRC1.toArbitaryValidICRC1AccountResult);
                  assertTrue((literalResult == expectedResult));
                },
              ),
              it(
                "should for ICRC1_ExampleToken2 type",
                do {
                  let {
                    creator;
                    id;
                    amountLessTheFee;
                    to;
                  } = Vals.TransferArgs.Inputs;
                  let argsIn = {
                    to = #ICRC1_ExampleToken2(to.ICRC1.account);
                    amountLessTheFee;
                    fee = Vals.SupportedTokenTypes.TransferFees.ICRC1;
                    id;
                    creator;
                  };
                  var literalResult = "≟";
                  switch (sTokens.getTransferArgsFromInvoiceSubaccount(argsIn)) {
                    case (#ICP _) {};
                    case (#ICP_nns _) {};
                    case (#ICRC1_ExampleToken _) {};
                    case (#ICRC1_ExampleToken2 tr) {
                      literalResult := debug_show (tr);
                    };
                  };
                  let expectedResult = debug_show (Vals.TransferArgs.Outputs.SenderAsInvoiceSubaccount.ICRC1.toArbitaryValidICRC1AccountResult);
                  assertTrue((literalResult == expectedResult));
                },
              ),
            ],
          ),
          describe(
            nlts("Correctly get the transfer args from creator subaccount, to an arbitrary address | getTransferArgsFromCreatorSubaccount (to any address)"),
            [
              it(
                "should for ICP token type",
                do {
                  let {
                    canisterId;
                    creator;
                    id;
                    amountLessTheFee;
                    to;
                  } = Vals.TransferArgs.Inputs;
                  let argsIn = {
                    to = #ICP(to.ICP.accountIdentifier);
                    amountLessTheFee;
                    fee = Vals.SupportedTokenTypes.TransferFees.ICP;
                    id;
                    creator;
                    canisterId;
                  };
                  var literalResult = "≟";
                  switch (sTokens.getTransferArgsFromCreatorSubaccount(argsIn)) {
                    case (#ICP tr) {
                      // literalResult the result as a debug_show, efficient means of checking equality.
                      literalResult := debug_show (tr);
                    };
                    case (#ICP_nns _) {};
                    case (#ICRC1_ExampleToken _) {};
                    case (#ICRC1_ExampleToken2 _) {};
                  };
                  let expectedResult = debug_show (Vals.TransferArgs.Outputs.SenderAsInvoiceCreatorSubaccount.icpResult);
                  assertTrue((literalResult == expectedResult));
                },
              ),
              it(
                "should for ICP_nns token type",
                do {
                  let {
                    canisterId;
                    creator;
                    id;
                    amountLessTheFee;
                    to;
                  } = Vals.TransferArgs.Inputs;
                  let argsIn = {
                    to = #ICP_nns(to.ICP.accountIdentifier);
                    amountLessTheFee;
                    fee = Vals.SupportedTokenTypes.TransferFees.ICP;
                    id;
                    creator;
                    canisterId;
                  };
                  var literalResult = "≟";
                  switch (sTokens.getTransferArgsFromCreatorSubaccount(argsIn)) {
                    case (#ICP _) {};
                    case (#ICP_nns tr) {
                      literalResult := debug_show (tr);
                    };
                    case (#ICRC1_ExampleToken _) {};
                    case (#ICRC1_ExampleToken2 _) {};
                  };
                  let expectedResult = debug_show (Vals.TransferArgs.Outputs.SenderAsInvoiceCreatorSubaccount.icpResult);
                  assertTrue((literalResult == expectedResult));
                },
              ),
              it(
                "should for ICRC1_ExampleToken type",
                do {
                  let {
                    canisterId;
                    creator;
                    id;
                    amountLessTheFee;
                    to;
                  } = Vals.TransferArgs.Inputs;
                  let argsIn = {
                    to = #ICRC1_ExampleToken(to.ICRC1.account);
                    amountLessTheFee;
                    fee = Vals.SupportedTokenTypes.TransferFees.ICP;
                    id;
                    creator;
                    canisterId;
                  };
                  var literalResult = "≟";
                  switch (sTokens.getTransferArgsFromCreatorSubaccount(argsIn)) {
                    case (#ICP _) {};
                    case (#ICP_nns _) {

                    };
                    case (#ICRC1_ExampleToken tr) {
                      literalResult := debug_show (tr);
                    };
                    case (#ICRC1_ExampleToken2 _) {};
                  };
                  let expectedResult = debug_show (Vals.TransferArgs.Outputs.SenderAsInvoiceCreatorSubaccount.icrc1Result);
                  assertTrue((literalResult == expectedResult));
                },
              ),
              it(
                "should for ICRC1_ExampleToken2 type",
                do {
                  let {
                    canisterId;
                    creator;
                    id;
                    amountLessTheFee;
                    to;
                  } = Vals.TransferArgs.Inputs;
                  let argsIn = {
                    to = #ICRC1_ExampleToken2(to.ICRC1.account);
                    amountLessTheFee;
                    fee = Vals.SupportedTokenTypes.TransferFees.ICP;
                    id;
                    creator;
                    canisterId;
                  };
                  var literalResult = "≟";
                  switch (sTokens.getTransferArgsFromCreatorSubaccount(argsIn)) {
                    case (#ICP _) {};
                    case (#ICP_nns _) {

                    };
                    case (#ICRC1_ExampleToken _) {};
                    case (#ICRC1_ExampleToken2 tr) {
                      literalResult := debug_show (tr);
                    };
                  };
                  let expectedResult = debug_show (Vals.TransferArgs.Outputs.SenderAsInvoiceCreatorSubaccount.icrc1Result);
                  assertTrue((literalResult == expectedResult));
                },
              ),
            ],
          ),
          describe(
            nlts("Correctly wrap each specific supported token types transfer result into invoice transfer result | rewrapTransferResults"),
            [
              it(
                "should for ICP token type",
                do {
                  let { ok; Err } = Vals.TransferResults.Inputs.ICP;
                  let {
                    insufficientFunds;
                    tooOld;
                    createdInFuture;
                    duplicate;
                    badFee;
                  } = Err;
                  var icpEqual = Vals.TransferResults.Outputs.ICP.okLiteral == debug_show (sTokens.rewrapTransferResults(#ICP(ok)));
                  icpEqual := icpEqual and (
                    Vals.TransferResults.Outputs.ICP.ErrLiteral.badFee == debug_show (sTokens.rewrapTransferResults(#ICP(badFee))),
                  );
                  icpEqual := icpEqual and (
                    Vals.TransferResults.Outputs.ICP.ErrLiteral.insufficientFunds == debug_show (sTokens.rewrapTransferResults(#ICP(insufficientFunds))),
                  );
                  icpEqual := icpEqual and (
                    Vals.TransferResults.Outputs.ICP.ErrLiteral.tooOld == debug_show (sTokens.rewrapTransferResults(#ICP(tooOld))),
                  );
                  icpEqual := icpEqual and (
                    Vals.TransferResults.Outputs.ICP.ErrLiteral.createdInFuture == debug_show (sTokens.rewrapTransferResults(#ICP(createdInFuture))),
                  );
                  icpEqual := icpEqual and (
                    Vals.TransferResults.Outputs.ICP.ErrLiteral.duplicate == debug_show (sTokens.rewrapTransferResults(#ICP(duplicate))),
                  );
                  assertTrue(icpEqual);
                },
              ),
              it(
                "should for ICP_nns token type",
                do {
                  let { ok; Err } = Vals.TransferResults.Inputs.ICP;
                  let {
                    insufficientFunds;
                    tooOld;
                    createdInFuture;
                    duplicate;
                    badFee;
                  } = Err;

                  var icpnnsEqual = Vals.TransferResults.Outputs.ICP_nns.okLiteral == debug_show (sTokens.rewrapTransferResults(#ICP_nns(ok)));
                  icpnnsEqual := icpnnsEqual and (
                    Vals.TransferResults.Outputs.ICP_nns.ErrLiteral.badFee == debug_show (sTokens.rewrapTransferResults(#ICP_nns(badFee))),
                  );
                  icpnnsEqual := icpnnsEqual and (
                    Vals.TransferResults.Outputs.ICP_nns.ErrLiteral.insufficientFunds == debug_show (sTokens.rewrapTransferResults(#ICP_nns(insufficientFunds))),
                  );
                  icpnnsEqual := icpnnsEqual and (
                    Vals.TransferResults.Outputs.ICP_nns.ErrLiteral.tooOld == debug_show (sTokens.rewrapTransferResults(#ICP_nns(tooOld))),
                  );
                  icpnnsEqual := icpnnsEqual and (
                    Vals.TransferResults.Outputs.ICP_nns.ErrLiteral.createdInFuture == debug_show (sTokens.rewrapTransferResults(#ICP_nns(createdInFuture))),
                  );
                  icpnnsEqual := icpnnsEqual and (
                    Vals.TransferResults.Outputs.ICP_nns.ErrLiteral.duplicate == debug_show (sTokens.rewrapTransferResults(#ICP_nns(duplicate))),
                  );
                  assertTrue(icpnnsEqual);
                },
              ),
              it(
                "should for ICRC1_ExampleToken type",
                do {
                  let { ok; Err } = Vals.TransferResults.Inputs.ICRC1;
                  let {
                    insufficientFunds;
                    tooOld;
                    createdInFuture;
                    duplicate;
                    badFee;
                    temporarilyUnavailable;
                    genericError;
                    badBurn;
                  } = Err;

                  var icrc1Equal = Vals.TransferResults.Outputs.ICRC1_ExampleToken.okLiteral == debug_show (sTokens.rewrapTransferResults(#ICRC1_ExampleToken(ok)));
                  icrc1Equal := icrc1Equal and (
                    Vals.TransferResults.Outputs.ICRC1_ExampleToken.Err.tooOld == debug_show (sTokens.rewrapTransferResults(#ICRC1_ExampleToken(tooOld))),
                  );
                  icrc1Equal := icrc1Equal and (
                    Vals.TransferResults.Outputs.ICRC1_ExampleToken.Err.duplicate == debug_show (sTokens.rewrapTransferResults(#ICRC1_ExampleToken(duplicate))),
                  );
                  icrc1Equal := icrc1Equal and (
                    Vals.TransferResults.Outputs.ICRC1_ExampleToken.Err.createdInFuture == debug_show (sTokens.rewrapTransferResults(#ICRC1_ExampleToken(createdInFuture))),
                  );
                  icrc1Equal := icrc1Equal and (
                    Vals.TransferResults.Outputs.ICRC1_ExampleToken.Err.insufficientFunds == debug_show (sTokens.rewrapTransferResults(#ICRC1_ExampleToken(insufficientFunds))),
                  );
                  icrc1Equal := icrc1Equal and (
                    Vals.TransferResults.Outputs.ICRC1_ExampleToken.Err.badFee == debug_show (sTokens.rewrapTransferResults(#ICRC1_ExampleToken(badFee))),
                  );
                  icrc1Equal := icrc1Equal and (
                    Vals.TransferResults.Outputs.ICRC1_ExampleToken.Err.temporarilyUnavailable == debug_show (sTokens.rewrapTransferResults(#ICRC1_ExampleToken(temporarilyUnavailable))),
                  );
                  icrc1Equal := icrc1Equal and (
                    Vals.TransferResults.Outputs.ICRC1_ExampleToken.Err.genericError == debug_show (sTokens.rewrapTransferResults(#ICRC1_ExampleToken(genericError))),
                  );
                  icrc1Equal := icrc1Equal and (
                    Vals.TransferResults.Outputs.ICRC1_ExampleToken.Err.badBurn == debug_show (sTokens.rewrapTransferResults(#ICRC1_ExampleToken(badBurn))),
                  );
                  assertTrue(icrc1Equal);
                },
              ),
              it(
                "should for ICRC1_ExampleToken2 type",
                do {
                  let { ok; Err } = Vals.TransferResults.Inputs.ICRC1;
                  let {
                    insufficientFunds;
                    tooOld;
                    createdInFuture;
                    duplicate;
                    badFee;
                    temporarilyUnavailable;
                    genericError;
                    badBurn;
                  } = Err;
                  // And for the grand finale checking rewrapping of the icrc_example_token2 transfer result types.
                  var icrc1ET2Equal = Vals.TransferResults.Outputs.ICRC1_ExampleToken2.okLiteral == debug_show (sTokens.rewrapTransferResults(#ICRC1_ExampleToken2(ok)));
                  icrc1ET2Equal := icrc1ET2Equal and (
                    Vals.TransferResults.Outputs.ICRC1_ExampleToken2.Err.tooOld == debug_show (sTokens.rewrapTransferResults(#ICRC1_ExampleToken2(tooOld))),
                  );
                  icrc1ET2Equal := icrc1ET2Equal and (
                    Vals.TransferResults.Outputs.ICRC1_ExampleToken2.Err.duplicate == debug_show (sTokens.rewrapTransferResults(#ICRC1_ExampleToken2(duplicate))),
                  );
                  icrc1ET2Equal := icrc1ET2Equal and (
                    Vals.TransferResults.Outputs.ICRC1_ExampleToken2.Err.createdInFuture == debug_show (sTokens.rewrapTransferResults(#ICRC1_ExampleToken2(createdInFuture))),
                  );
                  icrc1ET2Equal := icrc1ET2Equal and (
                    Vals.TransferResults.Outputs.ICRC1_ExampleToken2.Err.insufficientFunds == debug_show (sTokens.rewrapTransferResults(#ICRC1_ExampleToken2(insufficientFunds))),
                  );
                  icrc1ET2Equal := icrc1ET2Equal and (
                    Vals.TransferResults.Outputs.ICRC1_ExampleToken2.Err.badFee == debug_show (sTokens.rewrapTransferResults(#ICRC1_ExampleToken2(badFee))),
                  );
                  icrc1ET2Equal := icrc1ET2Equal and (
                    Vals.TransferResults.Outputs.ICRC1_ExampleToken2.Err.temporarilyUnavailable == debug_show (sTokens.rewrapTransferResults(#ICRC1_ExampleToken2(temporarilyUnavailable))),
                  );
                  icrc1ET2Equal := icrc1ET2Equal and (
                    Vals.TransferResults.Outputs.ICRC1_ExampleToken2.Err.genericError == debug_show (sTokens.rewrapTransferResults(#ICRC1_ExampleToken2(genericError))),
                  );
                  icrc1ET2Equal := icrc1ET2Equal and (
                    Vals.TransferResults.Outputs.ICRC1_ExampleToken2.Err.badBurn == debug_show (sTokens.rewrapTransferResults(#ICRC1_ExampleToken2(badBurn))),
                  );
                  assertTrue(icrc1ET2Equal);
                },
              ),
            ],
          ),
          describe(
            nlts("Correctly get default subaccount address | getDefaultSubaccountAddress"),
            [
              it(
                "should for ICP token type",
                do {
                  let {
                    asAccountIdentifier;
                    asText = expectedAsText;
                    principal;
                  } = Vals.Addressing.Ok.ICP.AccountIdentifier;
                  let { asText; asAddress } = sTokens.getDefaultSubaccountAddress(#ICP, principal);
                  let asTextEquals = (expectedAsText == asText);
                  let asAddressEquals = debug_show (asAddress) == debug_show (#ICP(asAccountIdentifier));
                  assertTrue(asTextEquals and asAddressEquals);
                },
              ),
              it(
                "should for ICP_nns token type",
                do {
                  let {
                    asAccountIdentifier;
                    asText = expectedAsText;
                    principal;
                  } = Vals.Addressing.Ok.ICP.AccountIdentifier;
                  let { asText; asAddress } = sTokens.getDefaultSubaccountAddress(#ICP_nns, principal);
                  let asTextEquals = (expectedAsText == asText);
                  let asAddressEquals = debug_show (asAddress) == debug_show (#ICP_nns(asAccountIdentifier));
                  assertTrue(asTextEquals and asAddressEquals);
                },
              ),
              it(
                "should for ICRC1_ExampleToken type",
                do {
                  let { asAccount; asText = expectedAsText } = Vals.Addressing.Ok.ICRC1.Account;
                  let { asText; asAddress } = sTokens.getDefaultSubaccountAddress(#ICRC1_ExampleToken, asAccount.owner);
                  let asTextEquals = (expectedAsText == asText);
                  let asAddressEquals = debug_show (asAddress) == debug_show (#ICRC1_ExampleToken(asAccount));
                  assertTrue(asTextEquals and asAddressEquals);
                },
              ),
              it(
                "should for ICRC1_ExampleToken2 type",
                do {
                  let { asAccount; asText = expectedAsText } = Vals.Addressing.Ok.ICRC1.Account;
                  let { asText; asAddress } = sTokens.getDefaultSubaccountAddress(#ICRC1_ExampleToken2, asAccount.owner);
                  let asTextEquals = (expectedAsText == asText);
                  let asAddressEquals = debug_show (asAddress) == debug_show (#ICRC1_ExampleToken2(asAccount));
                  assertTrue(asTextEquals and asAddressEquals);
                },
              ),
            ],
          ),
        ],
      ),
      describe(
        "Unit Test for Migration Module",
        [
          it(
            "should correctly convert an unpaid original invoice canister invoice record type with no permissions or details to Invoice_ record type",
            do {
              // convertOne -> unpaid / no permissions / no details
              let migrated = Migration.convertOne(Vals.Addressing.Ok.Invoice.ICP.canisterId, Vals.Migration.Inputs.invoiceA);
              assertTrue(debug_show (migrated) == debug_show (Vals.Migration.Outputs.invoiceA));
            },
          ),
          it(
            "should correctly convert a paid original invoice canister invoice record type with permissions and details to Invoice_ record type",
            do {
              // convertOne -> unpaid / no permissions / no details
              let migrated = Migration.convertOne(Vals.Addressing.Ok.Invoice.ICP.canisterId, Vals.Migration.Inputs.invoiceB);
              assertTrue(debug_show (migrated) == debug_show (Vals.Migration.Outputs.invoiceB));
            },
          ),
        ],
      ),
    ],
  ),
]);

if (success == false) {
  Debug.trap("Tests failed");
};
