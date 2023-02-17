import Array "mo:base/Array";
import Blob "mo:base/Blob";
import Debug "mo:base/Debug";
import Option "mo:base/Option";
import Principal "mo:base/Principal";
import Text "mo:base/Text";
import Time "mo:base/Time";
import AccountIdentifierBlob "mo:principal/blob/AccountIdentifier";

// Values known a priori used as unit tests' inputs.
// (Lesson learned: a snapshot generator might be a useful Motoko developer tool.)
module {

  public func decodeTextAsBlob(t : Text) : Blob {
    var res = Text.encodeUtf8("�");
    switch (AccountIdentifierBlob.fromText(t)) {
      case (#ok blob) res := blob;
      case (#err _) assert (false);
    };
    res;
  };

  // Workaround to non-static expression in library or module.
  public class Expected() {
    let emptyBlob = Blob.fromArray([]);
    let excessiveBlob = Blob.fromArrayMut(Array.init(33, 0 : Nat8));

    let defaultSubaccountBlob = Blob.fromArrayMut(Array.init(32, 0 : Nat8));

    // Principal matches the NnsFundedSecpk256k1Identity
    // used as the common test caller as the invoice creator.
    let testCaller = Principal.fromText("hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe");
    // Principal acting as the invoice canister's id.
    let testInvoiceCanisterId = Principal.fromText("q4eej-kyaaa-aaaaa-aaaha-cai");

    let originalVerboseToken = {
      symbol = "ICP";
      decimals = 8;
      meta = ?{
        Issuer = "e8s";
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
            name = "Internet Computer Random Curency One Example Token";
            decimals = 8 : Int;
            fee = 10_000;
            meta = ?{
              Issuer = "This Token is For Demonstration Purposes Only";
              Url = "https://github.com/dfinity/ICRC-1";
            };
          };
          ICRC1_ExampleToken2 = {
            symbol = "_2ICRC1EX";
            name = "Two Internet Computer Random Curency One Example Token";
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
              asAccountIdentifier = AccountIdentifierBlob.fromPrincipal(testCaller, null);
              asText = "2b8fbde99de881f695f279d2a892b1137bfe81a42d7694e064b1be58701e1138";
              subaccount = defaultSubaccountBlob;
              principal = testCaller;
            };
          };
          ICRC1 = {
            Account = {
              asAccount = {
                owner = testCaller;
                subaccount = null;
              };
              asText = "hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe";
              subaccount = defaultSubaccountBlob;
            };
          };
          Invoice = {
            ICP = {
              canisterId = testInvoiceCanisterId;
              creator = testCaller;
              id = 10001;
              InvoiceSubaccount = {
                asAccountIdentifier : Blob = decodeTextAsBlob("235510f80a7d67ce19f332f7880ea5ab2e5cf62984c10e02f96a1dc3f7e0c25c");
                asAccountIdentifierText = "235510f80a7d67ce19f332f7880ea5ab2e5cf62984c10e02f96a1dc3f7e0c25c";
                asFromSubaccount : Blob = decodeTextAsBlob("3cad7458ad4998506dfdc449ddd6e8e92cd00a5bad6765608ab5525049adca79");
              };
              CreatorSubaccount = {
                asAccountIdentifier : Blob = decodeTextAsBlob("5bea0a832af66531a1c2dda9e4f027f6c31dd943af039ff509ae492841b8b980");
                asAccountIdentifierText = "5bea0a832af66531a1c2dda9e4f027f6c31dd943af039ff509ae492841b8b980";
                asFromSubaccount : Blob = decodeTextAsBlob("dcf6284e373b6c51e09a47261efbbf49c295bd538808de32f8bceba3625a6c60");
              };
            };
            ICRC1 = {
              canisterId = testInvoiceCanisterId;
              creator = testCaller;
              id = 10002;
              InvoiceSubaccount = {
                asAccount = {
                  owner = testInvoiceCanisterId;
                  subaccount : ?Blob = ?decodeTextAsBlob("000000004aa1c7563b7c6c79ceac949c1aed4842b6b8d80a53d5aaf9cd6d71a1");
                };
                asText = "3vzsd-5yaaa-aaaaa-aaaha-cakku-hdvmo-34nr4-45leu-tqno2-sccw2-4nqcs-t2wvp-ttlno-gqry7-y";
                // When the from transfer arg.
                asFromSubaccount : Blob = decodeTextAsBlob("000000004aa1c7563b7c6c79ceac949c1aed4842b6b8d80a53d5aaf9cd6d71a1");
              };
              CreatorSubaccount = {
                asAccount = {
                  owner = testInvoiceCanisterId;
                  subaccount : ?Blob = ?decodeTextAsBlob("00000000373b6c51e09a47261efbbf49c295bd538808de32f8bceba3625a6c60");
                };
                asText = "6bcdq-baaaa-aaaaa-aaaha-cajxh-nwfdy-e2i4t-b5657-jhbjl-pktra-en4mx-yxtv2-gys2n-rqby7-y";
                // When the from transfer arg.
                asFromSubaccount : Blob = decodeTextAsBlob("00000000373b6c51e09a47261efbbf49c295bd538808de32f8bceba3625a6c60");
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
                owner = testInvoiceCanisterId;
                subaccount = ?emptyBlob;
              };
              asExcessiveSubaccount = {
                owner = testInvoiceCanisterId;
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
          canisterId = testInvoiceCanisterId;
          creator = testCaller;
          id = 638318;
          amountLessTheFee = 9_999_999_999_991;
          to = {
            ICP = {
              accountIdentifier : Blob = decodeTextAsBlob("2b8fbde99de881f695f279d2a892b1137bfe81a42d7694e064b1be58701e1138");
            };
            ICRC1 = {
              account = {
                owner = testCaller;
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
                amount = { e8s = 9_999_999_999_991 : Nat64 };
                created_at_time = null;
                fee = { e8s = 10_000 : Nat64 };
                memo = 1;
                from_subaccount : ?Blob = ?decodeTextAsBlob("58733b7b022f1b657c6d35c8000dcb279a8340e02b1754c7cf9afc2b307e0d6a");
                to : Blob = decodeTextAsBlob("5bea0a832af66531a1c2dda9e4f027f6c31dd943af039ff509ae492841b8b980");
              };
              toArbitaryValidICPAccountIdentifierResult = {
                amount = { e8s = 9_999_999_999_991 : Nat64 };
                created_at_time = null;
                fee = { e8s = 10_000 : Nat64 };
                memo = 1;
                from_subaccount : ?Blob = ?decodeTextAsBlob("58733b7b022f1b657c6d35c8000dcb279a8340e02b1754c7cf9afc2b307e0d6a");
                to : Blob = decodeTextAsBlob("2b8fbde99de881f695f279d2a892b1137bfe81a42d7694e064b1be58701e1138");
              };
            };
            // #ICRC1_ExampleToken and #ICRC1_ExampleToken2 share same transfer args if inputs are the same.
            ICRC1 = {
              toInvoiceCreatorPrincipalSubaccountResult = {
                amount = 9_999_999_999_991;
                created_at_time = null;
                fee = ?10_000;
                memo = ?Blob.fromArray([1]);
                from_subaccount : ?Blob = ?decodeTextAsBlob("00000000022f1b657c6d35c8000dcb279a8340e02b1754c7cf9afc2b307e0d6a");
                to = {
                  owner = testInvoiceCanisterId;
                  subaccount = ?decodeTextAsBlob("00000000373b6c51e09a47261efbbf49c295bd538808de32f8bceba3625a6c60") : ?Blob;
                };
              };
              toArbitaryValidICRC1AccountResult = {
                amount = 9_999_999_999_991;
                created_at_time = null;
                fee = ?10_000;
                memo = ?Blob.fromArray([1]);
                from_subaccount : ?Blob = ?decodeTextAsBlob("00000000022f1b657c6d35c8000dcb279a8340e02b1754c7cf9afc2b307e0d6a");
                to = {
                  owner = testCaller;
                  subaccount = null;
                };
              };
            };
          };
          SenderAsInvoiceCreatorSubaccount = {
            // #ICP will be refactored to #ICP_nns so there's only one ICP kind.
            icpResult = {
              amount = { e8s = 9_999_999_999_991 : Nat64 };
              created_at_time = null;
              fee = { e8s = 10_000 : Nat64 };
              memo = 1;
              from_subaccount : ?Blob = ?decodeTextAsBlob("dcf6284e373b6c51e09a47261efbbf49c295bd538808de32f8bceba3625a6c60");
              to : Blob = decodeTextAsBlob("2b8fbde99de881f695f279d2a892b1137bfe81a42d7694e064b1be58701e1138");
            };
            // #ICRC1_ExampleToken and #ICRC1_ExampleToken2 share same transfer args if inputs are the same.
            icrc1Result = {
              amount = 9_999_999_999_991;
              created_at_time = null;
              fee = ?10_000;
              memo = ?Blob.fromArray([1]);
              from_subaccount : ?Blob = ?decodeTextAsBlob("00000000373b6c51e09a47261efbbf49c295bd538808de32f8bceba3625a6c60");
              to = {
                owner = testCaller;
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
            creator = testCaller;
            details = null;
            permissions = null;
            amount = 1987654321;
            amountPaid = 0;
            token = originalVerboseToken;
            verifiedAtTime = null;
            paid = false;
            destination : Blob = decodeTextAsBlob("235510f80a7d67ce19f332f7880ea5ab2e5cf62984c10e02f96a1dc3f7e0c25c");
          };
          invoiceB = {
            id : Nat = 10001;
            creator = testCaller;
            details = ?{
              meta = Text.encodeUtf8("Here's some meta details about this invoice");
              description = "Here's a non-specific detail.";
            };
            permissions = ?{
              canGet = [testCaller, testInvoiceCanisterId];
              canVerify = [testCaller, testInvoiceCanisterId];
            };
            amount = 1987654321;
            amountPaid = 1987754321;
            token = originalVerboseToken;
            verifiedAtTime = ?(1_676_022_826_720_541_855 : Int);
            paid = true;
            destination : Blob = decodeTextAsBlob("235510f80a7d67ce19f332f7880ea5ab2e5cf62984c10e02f96a1dc3f7e0c25c");
          };
        };
        Outputs = {
          invoiceA = {
            id : Nat = 10001;
            creator = testCaller;
            details = null;
            permissions = null;
            amountDue = 1987654321;
            amountPaid = 0;
            token = #ICP;
            verifiedPaidAtTime = null;
            paymentAddress = "235510f80a7d67ce19f332f7880ea5ab2e5cf62984c10e02f96a1dc3f7e0c25c";
          };
          invoiceB = {
            id : Nat = 10001;
            creator = testCaller;
            details = ?{
              meta = Text.encodeUtf8("Here's some meta details about this invoice");
              description = "Here's a non-specific detail.";
            };
            permissions = ?{
              canGet = [testCaller, testInvoiceCanisterId];
              canVerify = [testCaller, testInvoiceCanisterId];
            };
            amountDue = 1987654321;
            amountPaid = 1987754321;
            token = #ICP;
            verifiedPaidAtTime = ?(1_676_022_826_720_541_855 : Int);
            paymentAddress = "235510f80a7d67ce19f332f7880ea5ab2e5cf62984c10e02f96a1dc3f7e0c25c";
          };
        };
      };
    };
  };
};
