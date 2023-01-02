import A          "../../src/invoice/Account";
import Hex        "../../src/invoice/Hex";
import U          "../../src/invoice/Utils";

import NewUtils       "../../src/invoice/ICPUtils";
import Option     "mo:base/Option";

import Blob       "mo:base/Blob";
import Debug      "mo:base/Debug";
import Principal  "mo:base/Principal";
import Result     "mo:base/Result";
import Text       "mo:base/Text";

import ActorSpec "./utils/ActorSpec";
type Group = ActorSpec.Group;

let assertTrue = ActorSpec.assertTrue;
let describe = ActorSpec.describe;
let it = ActorSpec.it;
let skip = ActorSpec.skip;
let pending = ActorSpec.pending;
let run = ActorSpec.run;

let testPrincipal = Principal.fromText("rrkah-fqaaa-aaaaa-aaaaq-cai");
let testCaller = Principal.fromText("ryjl3-tyaaa-aaaaa-aaaba-cai");
let defaultSubaccount = A.defaultSubaccount();
let canisterId = ?testPrincipal; 

func defaultAccountBlob() : Blob {
    let decoded = Hex.decode("082ecf2e3f647ac600f43f38a68342fba5b8e68b085f02592b77f39808a8d2b5");
    switch(decoded){
      case(#err _) {
        return Text.encodeUtf8("");
      };
      case(#ok arr) {
        return Blob.fromArray(arr);
      };
    }
};

func principalAccountBlob() : Blob {
  let decoded = Hex.decode("333ee20adc61d719820ac133d10f010e531ed8496ffcc439145b3df1982552e7");
  switch (decoded) {
    case (#err _) {
      return Text.encodeUtf8("");
    };
    case (#ok arr) {
      return Blob.fromArray(arr);
    };
  }
};

let success = run([
  describe("ICP Tests", [
    describe("Account Identifiers Utilities", [
      it("should generate a valid account identifier", do {
        let account = A.accountIdentifier(testPrincipal, defaultSubaccount);

        let newUtils_account = NewUtils.toAccountIdentifierAddress(testPrincipal, defaultSubaccount);
        let isValid = NewUtils.validateAccountIdentifier(newUtils_account) and A.validateAccountIdentifier(account);
        let equals : Bool = (account == newUtils_account);

        assertTrue(A.validateAccountIdentifier(account) and isValid and equals);
      }),
      it("should convert a principal to a subaccount", do {
        let subaccount = A.principalToSubaccount(testCaller);
        let subaccount_correctLength = subaccount.size() == 32;

        let newUtils_subaccount = NewUtils.subaccountForPrincipal(testCaller);
        let newUtils_subaccount_correctLength = subaccount.size() == 32;
    
        let equals : Bool = (subaccount == newUtils_subaccount);

        // Subaccounts should have a length of 32
        assertTrue(subaccount.size() == 32 and equals and newUtils_subaccount_correctLength);
      }),
      it("should generate a valid default account for a caller", do {
        let subaccount = A.principalToSubaccount(testCaller);
        let accountIdentifier = A.accountIdentifier(testPrincipal, subaccount);

        let newUtils_subaccount = NewUtils.subaccountForPrincipal(testCaller);
        let newUtils_accountIdentifier = NewUtils.toAccountIdentifierAddress(testPrincipal, newUtils_subaccount);
        let isValid = NewUtils.validateAccountIdentifier(newUtils_accountIdentifier) and A.validateAccountIdentifier(accountIdentifier);

        let subaccount_equals = subaccount == newUtils_subaccount;
        let accountId_equals = accountIdentifier == newUtils_accountIdentifier;

        assertTrue(A.validateAccountIdentifier(accountIdentifier) and isValid and subaccount_equals and accountId_equals);
      }),
      it("should convert a #text accountIdentifier to Text", do {
        let account = A.accountIdentifier(testPrincipal, defaultSubaccount);
        let textResult = U.accountIdentifierToText({
          accountIdentifier = #blob(account);
          canisterId = null;
        });

        let newUtils = NewUtils.toHumanReadableForm(NewUtils.toAccountIdentifierAddress(testPrincipal, defaultSubaccount));

        switch (textResult){
          case (#err _) {
            assertTrue(false);
          };
          case (#ok text) {
            let text_equals = newUtils == text;
            assertTrue(text == "082ecf2e3f647ac600f43f38a68342fba5b8e68b085f02592b77f39808a8d2b5" and text_equals);
          };
        }
      }),
      it("should convert a #principal accountIdentifier to Text", do {
        let id = #principal(testCaller);
        let textResult = U.accountIdentifierToText({
          accountIdentifier = id;
          canisterId;
        });

        let newUtils = NewUtils.toHumanReadableForm(NewUtils.toAccountIdentifierAddress(Option.unwrap(canisterId), NewUtils.subaccountForPrincipal(testCaller)));

        switch(textResult){
          case(#err _) {
            assertTrue(false);
          };
          case(#ok text) {
            let principalAccount = "333ee20adc61d719820ac133d10f010e531ed8496ffcc439145b3df1982552e7";
            let equals = newUtils == principalAccount;
            assertTrue(text == principalAccount and equals);
          };
        };
      }),
      it("should convert a #blob accountIdentifier to Text", do {
        let defaultBlob = defaultAccountBlob();
        let textResult = U.accountIdentifierToText({
          accountIdentifier = #blob(defaultBlob);
          canisterId = null;
        });

        let newUtils = NewUtils.toHumanReadableForm(defaultBlob);

        switch (textResult){
          case (#err _) {
            assertTrue(false);
          };
          case (#ok text) {
            let equals = newUtils == text;
            assertTrue(text == "082ecf2e3f647ac600f43f38a68342fba5b8e68b085f02592b77f39808a8d2b5" and equals);
          };
        }
      }),
      it("should convert a #text accountIdentifier to Blob", do {
        let id = #text("082ecf2e3f647ac600f43f38a68342fba5b8e68b085f02592b77f39808a8d2b5");
        let blobResult = U.accountIdentifierToBlob({
          accountIdentifier = id;
          canisterId = null;
        });

        let newUtils = switch (NewUtils.accountIdentifierFromValidText("082ecf2e3f647ac600f43f38a68342fba5b8e68b085f02592b77f39808a8d2b5")) {
          case (#ok blob) { blob };
          case (#err msg) { assertTrue(false); };
        };
        
        switch(blobResult){
          case(#err _) {
            assertTrue(false);
          };
          case(#ok blob) {
            let defaultBlob = defaultAccountBlob();
            assertTrue(blob == defaultBlob and blob == newUtils);
          };
        };
      }),
      it("should convert a #principal accountIdentifier to Blob", do {
        // This should return an accountIdentifier with the canister as a principal and the passed principal as the subaccount
        let id = #principal(testCaller);
        let blobResult = U.accountIdentifierToBlob({
          accountIdentifier = id;
          canisterId;
        });

        let newUtils = NewUtils.toAccountIdentifierAddress(Option.unwrap(canisterId), NewUtils.subaccountForPrincipal(testCaller));

        switch(blobResult){
          case(#err _) {
            assertTrue(false);
          };
          case(#ok blob) {
            let principalAccount = principalAccountBlob();
            let equals = newUtils == blob;
            assertTrue(blob == principalAccount and equals);
          };
        };
      }),
      it("should convert a #blob accountIdentifier to Blob", do {
        let defaultBlob = defaultAccountBlob();
        let id = #blob(defaultBlob);
        let blobResult = U.accountIdentifierToBlob({
          accountIdentifier = id;
          canisterId = null;
        });

        // nothing to map

        switch(blobResult){
          case(#err _) {
            assertTrue(false);
          };
          case(#ok blob) {
            let defaultBlob = defaultAccountBlob();
            assertTrue(blob == defaultBlob);
          };
        }
      }),
      it("should reject an invalid account identifier", do {
        let invalidBlob = Text.encodeUtf8("not valid");
        let id = #blob(invalidBlob);
        let result = U.accountIdentifierToBlob({
          accountIdentifier = id;
          canisterId = null;
        });

        switch(result){
          case(#err _) {
            assertTrue(true and not NewUtils.validateAccountIdentifier(invalidBlob));
          };
          case(#ok _) {
            assertTrue(false);
          };
        };
      }),
    ]),
    describe("Invoice Subaccount Creation", [
      it("should generate a valid invoice ID", do {
        let subaccount = U.generateInvoiceSubaccount({
          caller = testCaller;
          id = 0;
        });

        let newUtils = NewUtils.subaccountForInvoice(0, testCaller);
        let isValid = NewUtils.validateAccountIdentifier(newUtils) and A.validateAccountIdentifier(newUtils);
        let equals = newUtils == subaccount;

        assertTrue(A.validateAccountIdentifier(subaccount) and isValid and equals);
      }),
    ])
  ]),
]);

if(success == false){
  Debug.trap("Tests failed");
}
