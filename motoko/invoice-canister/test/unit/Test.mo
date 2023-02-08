import A          "../../src/invoice/Account";
import Hex        "../../src/invoice/Hex";
import U          "../../src/invoice/Utils";

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
        assertTrue(A.validateAccountIdentifier(account));
      }),
      it("should convert a principal to a subaccount", do {
        let subaccount = A.principalToSubaccount(testCaller);
        // Subaccounts should have a length of 32
        assertTrue(subaccount.size() == 32);
      }),
      it("should generate a valid default account for a caller", do {
        let subaccount = A.principalToSubaccount(testCaller);
        let accountIdentifier = A.accountIdentifier(testPrincipal, subaccount);
        assertTrue(A.validateAccountIdentifier(accountIdentifier));
      }),
      it("should convert a #text accountIdentifier to Text", do {
        let account = A.accountIdentifier(testPrincipal, defaultSubaccount);
        let textResult = U.accountIdentifierToText({
          accountIdentifier = #blob(account);
          canisterId = null;
        });
        switch (textResult){
          case (#err _) {
            assertTrue(false);
          };
          case (#ok text) {
            assertTrue(text == "082ecf2e3f647ac600f43f38a68342fba5b8e68b085f02592b77f39808a8d2b5");
          };
        }
      }),
      it("should convert a #principal accountIdentifier to Text", do {
        let id = #principal(testCaller);
        let textResult = U.accountIdentifierToText({
          accountIdentifier = id;
          canisterId;
        });
        switch(textResult){
          case(#err _) {
            assertTrue(false);
          };
          case(#ok text) {
            let principalAccount = "333ee20adc61d719820ac133d10f010e531ed8496ffcc439145b3df1982552e7";
            assertTrue(text == principalAccount);
          };
        };
      }),
      it("should convert a #blob accountIdentifier to Text", do {
        let defaultBlob = defaultAccountBlob();
        let textResult = U.accountIdentifierToText({
          accountIdentifier = #blob(defaultBlob);
          canisterId = null;
        });
        switch (textResult){
          case (#err _) {
            assertTrue(false);
          };
          case (#ok text) {
            assertTrue(text == "082ecf2e3f647ac600f43f38a68342fba5b8e68b085f02592b77f39808a8d2b5");
          };
        }
      }),
      it("should convert a #text accountIdentifier to Blob", do {
        let id = #text("082ecf2e3f647ac600f43f38a68342fba5b8e68b085f02592b77f39808a8d2b5");
        let blobResult = U.accountIdentifierToBlob({
          accountIdentifier = id;
          canisterId = null;
        });
        switch(blobResult){
          case(#err _) {
            assertTrue(false);
          };
          case(#ok blob) {
            let defaultBlob = defaultAccountBlob();
            assertTrue(blob == defaultBlob);
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
        switch(blobResult){
          case(#err _) {
            assertTrue(false);
          };
          case(#ok blob) {
            let principalAccount = principalAccountBlob();
            assertTrue(blob == principalAccount);
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
            assertTrue(true);
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
        
        assertTrue(A.validateAccountIdentifier(subaccount));
      }),
    ])
  ]),
]);

if(success == false){
  Debug.trap("Tests failed");
}
