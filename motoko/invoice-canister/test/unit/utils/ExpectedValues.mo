import Array "mo:base/Array";
import Blob "mo:base/Blob";
import Debug "mo:base/Debug";
import Nat64 "mo:base/Nat64";
import Option "mo:base/Option";
import Principal "mo:base/Principal";
import Text "mo:base/Text";
import Time "mo:base/Time";
import Hex "mo:encoding/Hex";
import AccountIdentifierBlob "mo:principal/blob/AccountIdentifier";

// Values known a priori used as unit tests' inputs.
// (Lesson learned: a snapshot generator might be a useful Motoko developer tool.)
module {

  public func decodeTextAsBlob(t : Text) : Blob {
    var res = Text.encodeUtf8("�");
    switch (Hex.decode(t)) {
      case (#err(e)) assert (false);
      case (#ok(bs)) res := Blob.fromArray(bs);
    };
    res;
  };

  // Workaround to non-static expression in library or module.
  public class Expected() {
    let emptyBlob = Blob.fromArray([]);
    let excessiveBlob = Blob.fromArrayMut(Array.init(33, 0 : Nat8));
    let defaultSubaccountBlob = Blob.fromArrayMut(Array.init(32, 0 : Nat8));

    let testInputs = {
      caller = Principal.fromText("hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe");
      invoiceCanisterId = Principal.fromText("q4eej-kyaaa-aaaaa-aaaha-cai");
      amountDueNatBaseUnits = 23456789;
      invoiceULID = "6GNGGRXAKGTXG070DV4GW2JKCJ";
      originalTokenVerbose = {
        symbol = "ICP";
        decimals = 8;
        meta = ?{
          Issuer = "e8s";
        };
      };
    };

    // (Note to future self: define and map the values (as they will be used externally)
    // separately in the future, so it is easier to modify and reuse easily, for instance the
    // TransferFees could be redeclared at the end, as a function of dot accessing each VerbsoseToken).
    public let Vals = {
      SupportedTokenTypes = {
        TransferFees = {
          ICP = 10000;
          ICRC1 = 10000;
        };
        UnitTypes = {
          ICP = #ICP;
          ICP_nns = #ICP_nns;
          ICRC1_ExampleToken = #ICRC1_ExampleToken;
          ICRC1_ExampleToken2 = #ICRC1_ExampleToken2;
        };
        VerboseToken = {
          ICP = {
            symbol = "_ICP";
            name = "Internet Computer Protocol Token";
            decimals = 8 : Int;
            fee = 10_000;
            meta = ?{
              Issuer = "e8s - For Demonstration Purposes";
              Url = "https://internetcomputer.org/docs/current/developer-docs/integrations/ledger/interact-with-ledger";
            };
          };
          ICP_nns = {
            symbol = "_ICP_nns";
            name = "Internet Computer Protocol Token NNS";
            decimals = 8 : Int;
            fee = 10_000;
            meta = ?{
              Issuer = "e8s - For Demonstration Purposes";
              Url = "https://github.com/dfinity/sdk/blob/master/docs/cli-reference/dfx-nns.md#_dfx_nns_install";
            };
          };
          ICRC1_ExampleToken = {
            symbol = "_1ICRC1EX";
            name = "Internet Computer Random Currency One Example Token";
            decimals = 8 : Int;
            fee = 10_000;
            meta = ?{
              Issuer = "This Token is For Demonstration Purposes Only";
              Url = "https://github.com/dfinity/ICRC-1";
            };
          };
          ICRC1_ExampleToken2 = {
            symbol = "_2ICRC1EX";
            name = "Two Internet Computer Random Currency One Example Token";
            decimals = 8 : Int;
            fee = 10_000;
            meta = ?{
              Issuer = "This Token is For Demonstration Purposes Only";
              Url = "https://github.com/dfinity/ic/tree/master/rs/rosetta-api/icrc1/ledger";
            };
          };
        };
      };
      Addressing = {
        defaultSubaccount = defaultSubaccountBlob;
        // Values known to be valid / return ok.
        Ok = {
          ICP = {
            AccountIdentifier = {
              asAccountIdentifier = AccountIdentifierBlob.fromPrincipal(testInputs.caller, null);
              asText = "2b8fbde99de881f695f279d2a892b1137bfe81a42d7694e064b1be58701e1138";
              subaccount = defaultSubaccountBlob;
              principal : Principal = testInputs.caller;
            };
          };
          ICRC1 = {
            Account = {
              asAccount = {
                owner : Principal = testInputs.caller;
                subaccount = null;
              };
              asText = "hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe";
              subaccount = defaultSubaccountBlob;
            };
          };
          Invoice = {
            ICP = {
              canisterId : Principal = testInputs.invoiceCanisterId;
              creator : Principal = testInputs.caller;
              id : Text = testInputs.invoiceULID;
              InvoiceSubaccount = {
                asAccountIdentifier : Blob = decodeTextAsBlob("3f766d9137db4ff58575dabbf0ce858251d3fd8104a6023b53f61f91005ddb98");
                asAccountIdentifierText = "3f766d9137db4ff58575dabbf0ce858251d3fd8104a6023b53f61f91005ddb98";
                asFromSubaccount : Blob = decodeTextAsBlob("36d8ffb07f052f9c91d33096018cd17dd5f2dda8eb21078277f2a5b481fb156e");
              };
              CreatorSubaccount = {
                asAccountIdentifier : Blob = decodeTextAsBlob("5bea0a832af66531a1c2dda9e4f027f6c31dd943af039ff509ae492841b8b980");
                asAccountIdentifierText = "5bea0a832af66531a1c2dda9e4f027f6c31dd943af039ff509ae492841b8b980";
                asFromSubaccount : Blob = decodeTextAsBlob("dcf6284e373b6c51e09a47261efbbf49c295bd538808de32f8bceba3625a6c60");
              };
            };
            ICRC1 = {
              canisterId : Principal = testInputs.invoiceCanisterId;
              creator : Principal = testInputs.caller;
              id : Text = testInputs.invoiceULID;
              InvoiceSubaccount = {
                asAccount = {
                  owner : Principal = testInputs.invoiceCanisterId;
                  subaccount : ?Blob = ?decodeTextAsBlob("36d8ffb07f052f9c91d33096018cd17dd5f2dda8eb21078277f2a5b481fb156e");
                };
                asText = "2flje-eiaaa-aaaaa-aaaha-cajw3-d73a7-yff6o-jduzq-syayz-ul52x-zn3kh-leedy-e57su-w2id6-yvnyq-h6";
                // When the from transfer arg.
                asFromSubaccount : Blob = decodeTextAsBlob("36d8ffb07f052f9c91d33096018cd17dd5f2dda8eb21078277f2a5b481fb156e");
              };
              CreatorSubaccount = {
                asAccount = {
                  owner : Principal = testInputs.invoiceCanisterId;
                  subaccount : ?Blob = ?decodeTextAsBlob("dcf6284e373b6c51e09a47261efbbf49c295bd538808de32f8bceba3625a6c60");
                };
                asText = "743p4-2qaaa-aaaaa-aaaha-cao46-yue4n-z3nri-6bgsh-eyppx-p2jyk-k32u4-ibdpd-f6f45-orwew-tmmaq-h6";
                // When the from transfer arg.
                asFromSubaccount : Blob = decodeTextAsBlob("dcf6284e373b6c51e09a47261efbbf49c295bd538808de32f8bceba3625a6c60");
              };
            };
          };
        };
        Err = {
          ICP = {
            AccountIdentifier = {
              asIncorrectCRC32Hash : Blob = decodeTextAsBlob("000000001b4389cc9c4a9a258aa89cc469d20b2cdb3f6b6601e0784a2d067e5f");
              asEmpty = emptyBlob;
              asCorruptedLength = Blob.fromArray([0x01, 0x09, 0x0F, 0x0A]);
              asExcessiveLength = excessiveBlob;
            };
          };
          ICRC1 = {
            emptySubaccount = emptyBlob;
            corruptedLengthSubaccount = Blob.fromArray([0x01, 0x09, 0x0F, 0x0A]);
            excessiveLengthSubaccount = excessiveBlob;
            Account = {
              asEmptySubaccount = {
                owner : Principal = testInputs.invoiceCanisterId;
                subaccount = ?emptyBlob;
              };
              asExcessiveSubaccount = {
                owner : Principal = testInputs.invoiceCanisterId;
                subaccount = ?excessiveBlob;
              };
              asText = {
                // Account with owner as reserved principal must also have a subaccount when encoded as text.
                reservedPrincipal = "ddhvl-oibai-bqibi-ga6xx-6";
              };
            };
          };
        };
      };
      TransferArgs = {
        Inputs = {
          canisterId : Principal = testInputs.invoiceCanisterId;
          creator : Principal = testInputs.caller;
          id : Text = testInputs.invoiceULID;
          amountLessTheFee : Nat = testInputs.amountDueNatBaseUnits;
          to = {
            ICP = {
              accountIdentifier : Blob = decodeTextAsBlob("2b8fbde99de881f695f279d2a892b1137bfe81a42d7694e064b1be58701e1138");
            };
            ICRC1 = {
              account = {
                owner : Principal = testInputs.invoiceCanisterId;
                subaccount = null;
              };
            };
          };
        };
        Outputs = {
          SenderAsInvoiceSubaccount = {
            // #ICP and #ICP_nns share same transfer args if inputs are the same.
            ICP = {
              toInvoiceCreatorPrincipalSubaccountResult = {
                amount = {
                  e8s = Nat64.fromNat(testInputs.amountDueNatBaseUnits);
                };
                created_at_time = null;
                fee = { e8s = 10_000 : Nat64 };
                memo = 1;
                from_subaccount : ?Blob = ?decodeTextAsBlob("36d8ffb07f052f9c91d33096018cd17dd5f2dda8eb21078277f2a5b481fb156e");
                to : Blob = decodeTextAsBlob("5bea0a832af66531a1c2dda9e4f027f6c31dd943af039ff509ae492841b8b980");
              };
              toArbitaryValidICPAccountIdentifierResult = {
                amount = {
                  e8s = Nat64.fromNat(testInputs.amountDueNatBaseUnits);
                };
                created_at_time = null;
                fee = { e8s = 10_000 : Nat64 };
                memo = 1;
                from_subaccount : ?Blob = ?decodeTextAsBlob("36d8ffb07f052f9c91d33096018cd17dd5f2dda8eb21078277f2a5b481fb156e");
                to : Blob = decodeTextAsBlob("2b8fbde99de881f695f279d2a892b1137bfe81a42d7694e064b1be58701e1138");
              };
            };
            // #ICRC1_ExampleToken and #ICRC1_ExampleToken2 share same transfer args if inputs are the same.
            ICRC1 = {
              toInvoiceCreatorPrincipalSubaccountResult = {
                amount : Nat = testInputs.amountDueNatBaseUnits;
                created_at_time = null;
                fee = ?10_000;
                memo = ?Blob.fromArray([1]);
                from_subaccount : ?Blob = ?decodeTextAsBlob("36d8ffb07f052f9c91d33096018cd17dd5f2dda8eb21078277f2a5b481fb156e");
                to = {
                  owner : Principal = testInputs.invoiceCanisterId;
                  subaccount = ?decodeTextAsBlob("dcf6284e373b6c51e09a47261efbbf49c295bd538808de32f8bceba3625a6c60") : ?Blob;
                };
              };
              toArbitaryValidICRC1AccountResult = {
                amount : Nat = testInputs.amountDueNatBaseUnits;
                created_at_time = null;
                fee = ?10_000;
                memo = ?Blob.fromArray([1]);
                from_subaccount : ?Blob = ?decodeTextAsBlob("36d8ffb07f052f9c91d33096018cd17dd5f2dda8eb21078277f2a5b481fb156e");
                to = {
                  owner : Principal = testInputs.invoiceCanisterId;
                  subaccount = null;
                };
              };
            };
          };
          SenderAsInvoiceCreatorSubaccount = {
            // #ICP and #ICP_nns share same transfer args if inputs are the same.
            icpResult = {
              amount = { e8s = Nat64.fromNat(testInputs.amountDueNatBaseUnits) };
              created_at_time = null;
              fee = { e8s = 10_000 : Nat64 };
              memo = 1;
              from_subaccount : ?Blob = ?decodeTextAsBlob("dcf6284e373b6c51e09a47261efbbf49c295bd538808de32f8bceba3625a6c60");
              to : Blob = decodeTextAsBlob("2b8fbde99de881f695f279d2a892b1137bfe81a42d7694e064b1be58701e1138");
            };
            // #ICRC1_ExampleToken and #ICRC1_ExampleToken2 share same transfer args if inputs are the same.
            icrc1Result = {
              amount : Nat = testInputs.amountDueNatBaseUnits;
              created_at_time = null;
              fee = ?10_000;
              memo = ?Blob.fromArray([1]);
              from_subaccount : ?Blob = ?decodeTextAsBlob("dcf6284e373b6c51e09a47261efbbf49c295bd538808de32f8bceba3625a6c60");
              to = {
                owner : Principal = testInputs.invoiceCanisterId;
                subaccount = null;
              };
            };
          };
        };
      };
      TransferResults = {
        Inputs = {
          ICP = {
            ok = #Ok(128 : Nat64);
            Err = {
              insufficientFunds = #Err(#InsufficientFunds { balance = { e8s = 9_000 : Nat64 } });
              tooOld = #Err(#TxTooOld { allowed_window_nanos = 512 : Nat64 });
              createdInFuture = #Err(#TxCreatedInFuture);
              duplicate = #Err(#TxDuplicate { duplicate_of = 10278 : Nat64 });
              badFee = #Err(#BadFee { expected_fee = { e8s = 10_000 : Nat64 } });
            };
          };
          ICRC1 = {
            ok = #Ok(512);
            Err = {
              insufficientFunds = #Err(#InsufficientFunds { balance = 9_000 });
              tooOld = #Err(#TooOld);
              createdInFuture = #Err(#CreatedInFuture { ledger_time = 1225465996 : Nat64 });
              duplicate = #Err(#Duplicate { duplicate_of = 732 });
              badFee = #Err(#BadFee { expected_fee = 10_000 });
              temporarilyUnavailable = #Err(#TemporarilyUnavailable);
              genericError = #Err(#GenericError { error_code = 99; message = "Ledger undergoing upgrade try again later" });
              badBurn = #Err(#BadBurn { min_burn_amount = 10 });
            };
          };
        };
        Outputs = {
          ICP = {
            okLiteral = debug_show (#ok(#ICP(128)));
            ErrLiteral = {
              insufficientFunds = debug_show (#err(#ICP(#InsufficientFunds { balance = { e8s = 9_000 } })));
              tooOld = debug_show (#err(#ICP(#TxTooOld { allowed_window_nanos = 512 })));
              createdInFuture = debug_show (#err(#ICP(#TxCreatedInFuture)));
              duplicate = debug_show (#err(#ICP(#TxDuplicate { duplicate_of = 10278 })));
              badFee = debug_show (#err(#ICP(#BadFee { expected_fee = { e8s = 10_000 } })));
            };
          };
          ICP_nns = {
            okLiteral = debug_show (#ok(#ICP_nns(128)));
            ErrLiteral = {
              insufficientFunds = debug_show (#err(#ICP_nns(#InsufficientFunds { balance = { e8s = 9_000 } })));
              tooOld = debug_show (#err(#ICP_nns(#TxTooOld { allowed_window_nanos = 512 })));
              createdInFuture = debug_show (#err(#ICP_nns(#TxCreatedInFuture)));
              duplicate = debug_show (#err(#ICP_nns(#TxDuplicate { duplicate_of = 10278 })));
              badFee = debug_show (#err(#ICP_nns(#BadFee { expected_fee = { e8s = 10_000 } })));
            };
          };
          ICRC1_ExampleToken = {
            okLiteral = debug_show (#ok(#ICRC1_ExampleToken(512)));
            Err = {
              insufficientFunds = debug_show (#err(#ICRC1_ExampleToken(#InsufficientFunds { balance = 9_000 })));
              tooOld = debug_show (#err(#ICRC1_ExampleToken(#TooOld)));
              createdInFuture = debug_show (#err(#ICRC1_ExampleToken(#CreatedInFuture { ledger_time = 1225465996 })));
              duplicate = debug_show (#err(#ICRC1_ExampleToken(#Duplicate { duplicate_of = 732 })));
              badFee = debug_show (#err(#ICRC1_ExampleToken(#BadFee { expected_fee = 10_000 })));
              temporarilyUnavailable = debug_show (#err(#ICRC1_ExampleToken(#TemporarilyUnavailable)));
              genericError = debug_show (#err(#ICRC1_ExampleToken(#GenericError { error_code = 99; message = "Ledger undergoing upgrade try again later" })));
              badBurn = debug_show (#err(#ICRC1_ExampleToken(#BadBurn { min_burn_amount = 10 })));
            };
          };
          ICRC1_ExampleToken2 = {
            okLiteral = debug_show (#ok(#ICRC1_ExampleToken2(512)));
            Err = {
              insufficientFunds = debug_show (#err(#ICRC1_ExampleToken2(#InsufficientFunds { balance = 9_000 })));
              tooOld = debug_show (#err(#ICRC1_ExampleToken2(#TooOld)));
              createdInFuture = debug_show (#err(#ICRC1_ExampleToken2(#CreatedInFuture { ledger_time = 1225465996 })));
              duplicate = debug_show (#err(#ICRC1_ExampleToken2(#Duplicate { duplicate_of = 732 })));
              badFee = debug_show (#err(#ICRC1_ExampleToken2(#BadFee { expected_fee = 10_000 })));
              temporarilyUnavailable = debug_show (#err(#ICRC1_ExampleToken2(#TemporarilyUnavailable)));
              genericError = debug_show (#err(#ICRC1_ExampleToken2(#GenericError { error_code = 99; message = "Ledger undergoing upgrade try again later" })));
              badBurn = debug_show (#err(#ICRC1_ExampleToken2(#BadBurn { min_burn_amount = 10 })));
            };
          };
        };
      };
      Migration = {
        Inputs = {
          invoiceA = {
            id : Nat = 10001;
            creator : Principal = testInputs.caller;
            details = null;
            permissions = null;
            amount = 1987654321;
            amountPaid = 0;
            token : {
              symbol : Text;
              decimals : Int;
              meta : ?{ Issuer : Text };
            } = testInputs.originalTokenVerbose;
            verifiedAtTime = null;
            paid = false;
            destination : Blob = decodeTextAsBlob("235510f80a7d67ce19f332f7880ea5ab2e5cf62984c10e02f96a1dc3f7e0c25c");
          };
          invoiceB = {
            id : Nat = 10001;
            creator : Principal = testInputs.caller;
            details = ?{
              meta = Text.encodeUtf8("Here's some meta details about this invoice");
              description = "Here's a non-specific detail.";
            };
            permissions = ?{
              canGet = [testInputs.caller : Principal, testInputs.invoiceCanisterId : Principal];
              canVerify = [testInputs.caller : Principal, testInputs.invoiceCanisterId : Principal];
            };
            amount = 1987654321;
            amountPaid = 1987754321;
            token : {
              symbol : Text;
              decimals : Int;
              meta : ?{ Issuer : Text };
            } = testInputs.originalTokenVerbose;
            verifiedAtTime = ?(1_676_022_826_720_541_855 : Int);
            paid = true;
            destination : Blob = decodeTextAsBlob("235510f80a7d67ce19f332f7880ea5ab2e5cf62984c10e02f96a1dc3f7e0c25c");
          };
        };
        Outputs = {
          invoiceA = {
            id : Text = "10001";
            creator : Principal = testInputs.caller;
            details = null;
            permissions = null;
            amountDue = 1987654321;
            amountPaid = 0;
            token = #ICP;
            verifiedPaidAtTime = null;
            paymentAddress = "af3196de20c04e7bbe6c7cd370b4f8838d28749093312ebfe108f0729ab26c40";
          };
          invoiceB = {
            id : Text = "10001";
            creator : Principal = testInputs.caller;
            details = ?{
              meta = Text.encodeUtf8("Here's some meta details about this invoice");
              description = "Here's a non-specific detail.";
            };
            permissions = ?{
              canGet = [testInputs.caller : Principal, testInputs.invoiceCanisterId : Principal];
              canVerify = [testInputs.caller : Principal, testInputs.invoiceCanisterId : Principal];
            };
            amountDue = 1987654321;
            amountPaid = 1987754321;
            token = #ICP;
            verifiedPaidAtTime = ?(1_676_022_826_720_541_855 : Int);
            paymentAddress = "af3196de20c04e7bbe6c7cd370b4f8838d28749093312ebfe108f0729ab26c40";
          };
        };
      };
    };
  };

  /*
  While creating Motoko snapshot should likely be done from scratch so that necessary 
  expected values can be scrutinized when they end up changing unexpectantly, saving 
  this here so it does not need to be written, again, in the event these values are 
  once again needed. 

  public func generate() : async {
    nnsFundedSecp256k1Identity : {
      ICP : {
        defaultSubaccount : {
          accountIdentifier : Blob;
          asText : Text;
        };
        InvoiceSubaccount : {
          subaccount : Blob;
          subaccountAsText : Text;
          address : Blob;
          asText : Text;
        };
        CreatorSubaccount : {
          subaccount : Blob;
          subaccountAsText : Text;
          address : Blob;
          asText : Text;
        };
      };
      ICRC1 : {
        InvoiceSubaccount : {
          subaccount : Blob;
          subaccountAsText : Text;
          address : { owner : Principal; subaccount : ?Blob };
          asText : Text;
        };
        CreatorSubaccount : {
          subaccount : Blob;
          subaccountAsText : Text;
          address : { owner : Principal; subaccount : ?Blob };
          asText : Text;
        };
      };
    };
  } {
    public func getMigratedUlidId() : async Text {
      let { creator; canisterId; id } = testInputs;
      let invoiceIdIn : Nat = 10001;
      SupportedToken.ICP_Adapter.encodeAddress(SupportedToken.ICP_Adapter.computeInvoiceSubaccountAddress(Nat.toText(invoiceIdIn), creator, canisterId));
    };
    // As the ICRC1 Specification for its text encoding strategy
    // may change, this would need to be updated when it does.
    func icrc1_account_to_text(acc : { owner : Principal; subaccount : ?Blob }) : Text {
      switch (acc.subaccount) {
        case (null) { Principal.toText(acc.owner) };
        case (?blob) {
          assert (blob.size() == 32);
          var zeroCount = 0;
          label l for (byte in blob.vals()) {
            if (byte == 0) { zeroCount += 1 } else break l;
          };
          if (zeroCount == 32) {
            Principal.toText(acc.owner);
          } else {
            let principalBytes = Principal.toBlob(acc.owner);
            let buf = Buffer.Buffer<Nat8>(principalBytes.size() + blob.size() - zeroCount + 2);
            for (b in principalBytes.vals()) {
              buf.add(b);
            };
            var j = 0;
            label l for (b in blob.vals()) {
              j += 1;
              if (j <= zeroCount) {
                continue l;
              };
              buf.add(b);
            };
            buf.add(Nat8.fromNat(32 - zeroCount));
            buf.add(Nat8.fromNat(0x7f));
            Principal.toText(Principal.fromBlob(Blob.fromArray(Buffer.toArray(buf))));
          };
        };
      };
    };

    let defaultSubaccountBlob = Blob.fromArrayMut(Array.init(32, 0 : Nat8));

    let testInputs = {
      creator = Principal.fromText("hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe");
      canisterId = Principal.fromText("q4eej-kyaaa-aaaaa-aaaha-cai");
      id = "6GNGGRXAKGTXG070DV4GW2JKCJ";
      amountDue = 23456789;
    };

    let { creator; canisterId; id } = testInputs;
    // ICP invoice subaccount
    let icp_invoiceSubaccount = SupportedToken.ICP_Adapter.computeInvoiceSubaccount(id, creator);
    let icp_invoiceSubaccountAddress = SupportedToken.ICP_Adapter.computeInvoiceSubaccountAddress(id, creator, canisterId);
    // ICP creator subaccount
    let icp_creatorSubaccount = SupportedToken.ICP_Adapter.computeCreatorSubaccount(creator);
    let icp_creatorSubaccountAddress = SupportedToken.ICP_Adapter.computeCreatorSubaccountAddress(creator, canisterId);
    // ICRC1 invoice subaccount
    let icrc1_invoiceSubaccount = SupportedToken.ICRC1_Adapter.computeInvoiceSubaccount(id, creator);
    let icrc1_invoiceSubaccountAddress = SupportedToken.ICRC1_Adapter.computeInvoiceSubaccountAddress(id, creator, canisterId);
    // ICRC1 creator subaccount
    let icrc1_creatorSubaccount = SupportedToken.ICRC1_Adapter.computeCreatorSubaccount(creator);
    let icrc1_creatorSubaccountAddress = SupportedToken.ICRC1_Adapter.computeCreatorSubaccountAddress(creator, canisterId);
    return {
      nnsFundedSecp256k1Identity = {
        ICP = {
          defaultSubaccount = {
            accountIdentifier = AccountIdentifierBlob.fromPrincipal(creator, null);
            asText = Hex.encode(Blob.toArray(AccountIdentifierBlob.fromPrincipal(creator, null)));
          };
          InvoiceSubaccount = {
            subaccount = icp_invoiceSubaccount;
            subaccountAsText = Hex.encode(Blob.toArray(icp_invoiceSubaccount));
            address = icp_invoiceSubaccountAddress;
            asText = Hex.encode(Blob.toArray(icp_invoiceSubaccountAddress));
          };
          CreatorSubaccount = {
            subaccount = icp_creatorSubaccount;
            subaccountAsText = Hex.encode(Blob.toArray(icp_creatorSubaccount));
            address = icp_creatorSubaccountAddress;
            asText = Hex.encode(Blob.toArray(icp_creatorSubaccountAddress));
          };
        };
        ICRC1 = {
          InvoiceSubaccount = {
            subaccount = icrc1_invoiceSubaccount;
            subaccountAsText = Hex.encode(Blob.toArray(icrc1_invoiceSubaccount));
            address = icrc1_invoiceSubaccountAddress;
            asText = icrc1_account_to_text(icrc1_invoiceSubaccountAddress);
          };
          CreatorSubaccount = {
            subaccount = icrc1_creatorSubaccount;
            subaccountAsText = Hex.encode(Blob.toArray(icrc1_creatorSubaccount));
            address = icrc1_creatorSubaccountAddress;
            asText = icrc1_account_to_text(icrc1_creatorSubaccountAddress);
          };
        };
      };
    };
  };
  */ //
};
