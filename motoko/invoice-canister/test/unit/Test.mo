import NewUtils   "../../src/invoice/ICPUtils";

import Array      "mo:base/Array";
import Blob       "mo:base/Blob";
import Debug      "mo:base/Debug";
import Principal  "mo:base/Principal";
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
let defaultSubaccount = Blob.fromArrayMut(Array.init(32, 0 : Nat8));

let success = run([
  describe("ICP Tests", [
    describe("Account Identifiers Transformations", [
      it("should generate a valid account identifier with the default subaccount", do {
        let account = NewUtils.toAccountIdentifierAddress(testPrincipal, defaultSubaccount);
        assertTrue(NewUtils.accountIdentifierIsValid(account));
      }),
      it("should create a valid subaccount from a principal", do {
        let subaccount = NewUtils.subaccountForPrincipal(testCaller);
        // Subaccounts should have a length of 32
        assertTrue(subaccount.size() == 32);
      }),
      it("should create a valid subaccount for an invoice", do {
        let subaccount = NewUtils.subaccountForInvoice(0, testCaller);
        // Subaccounts should have a length of 32
        assertTrue(subaccount.size() == 32);
      }),
      it("should create a valid account identifier from a principal and subaccount based on a principal", do {
        let subaccount = NewUtils.subaccountForPrincipal(testCaller);
        let accountIdentifier = NewUtils.toAccountIdentifierAddress(testPrincipal, subaccount);
        assertTrue(NewUtils.accountIdentifierIsValid(accountIdentifier));
      }),
      it("should create a valid account identifier from a principal and subaccount based on an invoice", do {
        let subaccount = NewUtils.subaccountForInvoice(0, testCaller);
        let accountIdentifier = NewUtils.toAccountIdentifierAddress(testPrincipal, subaccount);
        assertTrue(NewUtils.accountIdentifierIsValid(accountIdentifier));
      }),
      it("should convert a valid account identifier blob into human readable form", do {
        let account = NewUtils.toHumanReadableForm(NewUtils.toAccountIdentifierAddress(testPrincipal, defaultSubaccount));
        assertTrue("082ecf2e3f647ac600f43f38a68342fba5b8e68b085f02592b77f39808a8d2b5" == account);
      }),
      it("should convert an account identifier with a subaccount created from a principal into a human readable form ", do {
        let accountIdText = NewUtils.toHumanReadableForm(NewUtils.toAccountIdentifierAddress(testPrincipal, NewUtils.subaccountForPrincipal(testCaller)));
        let principalAccount = "333ee20adc61d719820ac133d10f010e531ed8496ffcc439145b3df1982552e7";
        assertTrue(principalAccount == accountIdText);
      }),
      it("should convert a valid account identifier blob to text", do {
        let blobAid = NewUtils.toAccountIdentifierAddress(testPrincipal, defaultSubaccount);
        let acccountIdText = NewUtils.toHumanReadableForm(blobAid);
        assertTrue("082ecf2e3f647ac600f43f38a68342fba5b8e68b085f02592b77f39808a8d2b5" == acccountIdText);
      }),
      it("should convert a valid textual account identifier to a valid account identifier blob", do {
        switch (NewUtils.accountIdentifierFromValidText("082ecf2e3f647ac600f43f38a68342fba5b8e68b085f02592b77f39808a8d2b5")) {
          case (#ok blob) { assertTrue(NewUtils.accountIdentifierIsValid(blob)) };
          case (#err msg) { assertTrue(false) };
        };
      }),
      it("should reject an invalid blob form account identifier", do {
        assertTrue(not NewUtils.accountIdentifierIsValid(Text.encodeUtf8("not valid")));
      }),
      it("should reject an invalid textual form account identifier", do {
        switch (NewUtils.accountIdentifierFromValidText("not valid")) {
          case (#err msg) { assertTrue(true) }; 
          case (#ok err) { assertTrue(false) };
        };
      }),
    ]),
  ]),
]);

if(success == false){
  Debug.trap("Tests failed");
}
