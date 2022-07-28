import A          "./Account";
import CRC32      "./CRC32";
import Hex        "./Hex";
import SHA224     "./SHA224";
import T          "./Types";

import Array      "mo:base/Array";
import Blob       "mo:base/Blob";
import Buffer     "mo:base/Buffer";
import Error      "mo:base/Error";
import Nat8       "mo:base/Nat8";
import Nat32      "mo:base/Nat32";
import Principal  "mo:base/Principal";
import Text       "mo:base/Text";

module {
  type AccountIdentifier = T.AccountIdentifier;

  /**
    * args : { accountIdentifier : AccountIdentifier, canisterId  : ?Principal }
    * Takes an account identifier and returns a Blob
    *
    * Canister ID is required only for Principal, and will return an account identifier using that principal as a subaccount for the provided canisterId
    */
  public func accountIdentifierToBlob (args : T.AccountIdentifierToBlobArgs) : T.AccountIdentifierToBlobResult {
    let accountIdentifier = args.accountIdentifier;
    let canisterId = args.canisterId;
    let err = {
      kind = #InvalidAccountIdentifier;
      message = ?"Invalid account identifier";
    };
    switch (accountIdentifier) {
      case(#text(identifier)){
        switch (Hex.decode(identifier)) {
          case(#ok v){
            let blob = Blob.fromArray(v);
            if(A.validateAccountIdentifier(blob)){
              #ok(blob);
            } else {
              #err(err);
            }
          };
          case(#err _){
            #err(err);
          };
        };
      };
      case(#principal principal){
        switch(canisterId){
          case (null){
            #err({
              kind = #Other;
              message = ?"Canister Id is required for account identifiers of type principal";
            })
          };
          case (? id){
            let identifier = A.accountIdentifier(id, A.principalToSubaccount(principal));
            if(A.validateAccountIdentifier(identifier)){
              #ok(identifier);
            } else {
              #err(err);
            }
          };
        }
      };
      case(#blob(identifier)){
        if(A.validateAccountIdentifier(identifier)){
          #ok(identifier);
        } else {
          #err(err);
        }
      };
    };
  };
  /**
    * args : { accountIdentifier : AccountIdentifier, canisterId  : ?Principal }
    * Takes an account identifier and returns Hex-encoded Text
    *
    * Canister ID is required only for Principal, and will return an account identifier using that principal as a subaccount for the provided canisterId
    */
  public func accountIdentifierToText (args : T.AccountIdentifierToTextArgs) : T.AccountIdentifierToTextResult {
    let accountIdentifier = args.accountIdentifier;
    let canisterId = args.canisterId;
    switch (accountIdentifier) {
      case(#text(identifier)){
        #ok(identifier);
      };
      case(#principal(identifier)){
        let blobResult = accountIdentifierToBlob(args);
        switch(blobResult){
          case(#ok(blob)){
            #ok(Hex.encode(Blob.toArray(blob)));
          };
          case(#err(err)){
            #err(err);
          };
        };
      };
      case(#blob(identifier)){
        let blobResult = accountIdentifierToBlob(args);
        switch(blobResult){
          case(#ok(blob)){
            #ok(Hex.encode(Blob.toArray(blob)));
          };
          case(#err(err)){
            #err(err);
          };
        };
      };
    };
  };

  type GenerateInvoiceSubaccountArgs = {
    caller : Principal;
    id : Nat;
  };
  /**
    * Generates a subaccount for the given principal, to be used as an invoice destination. This is a subaccount, not a full accountIdentifier.
    *
    * args : { caller : Principal, id : Nat }
    * Returns : Principal
    */
  public func generateInvoiceSubaccount (args : GenerateInvoiceSubaccountArgs) : Blob {
    let idHash = SHA224.Digest();
    // Length of domain separator
    idHash.write([0x0A]);
    // Domain separator
    idHash.write(Blob.toArray(Text.encodeUtf8("invoice-id")));
    // Counter as Nonce
    let idBytes = A.beBytes(Nat32.fromNat(args.id));
    idHash.write(idBytes);
    // Principal of caller
    idHash.write(Blob.toArray(Principal.toBlob(args.caller)));

    let hashSum = idHash.sum();
    let crc32Bytes = A.beBytes(CRC32.ofArray(hashSum));
    let buf = Buffer.Buffer<Nat8>(32);
    Blob.fromArray(Array.append(crc32Bytes, hashSum));
  };

  type DefaultAccountArgs = {
    // Hex-encoded AccountIdentifier
    canisterId : Principal;
    principal : Principal;
  };
  public func getDefaultAccount(args : DefaultAccountArgs) : Blob {
    A.accountIdentifier(args.canisterId, A.principalToSubaccount(args.principal));
  };

  public type GetICPAccountIdentifierArgs = {
    principal : Principal;
    subaccount : T.SubAccount;
  };
  public func getICPAccountIdentifier(args : GetICPAccountIdentifierArgs) : Blob {
    A.accountIdentifier(args.principal, args.subaccount);
  };
}
