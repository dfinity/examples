import Array "mo:base/Array";
import Blob "mo:base/Blob";
import Buffer "mo:base/Buffer";
import Debug "mo:base/Debug";
import Error "mo:base/Error";
import HashMap "mo:base/HashMap";
import Int "mo:base/Int";
import Nat32 "mo:base/Nat32";
import Nat64 "mo:base/Nat64";
import Nat8 "mo:base/Nat8";
import Option "mo:base/Option";
import Principal "mo:base/Principal";
import Text "mo:base/Text";
import Time "mo:base/Time";
import Binary "mo:encoding/Binary";
import Hex "mo:encoding/Hex";
import CRC32 "mo:hash/CRC32";
import AccountIdentifierBlob "mo:principal/blob/AccountIdentifier";

/****Contains classes ICP.MockLedger and ICRC1.MockLedger.**
    To initialize either class with an initial deposit, 
  pass the not-null initialDeposit record:
    -`ICP.MockLedger(?{ amount : { e8s : Nat64 }; who : Principal })`
    -`ICRC1.MockLedger(?{ amount : Nat; who : Principal })`

  which will deposit the given amount into the account identifier 
  or account of the default subaccount of the whose principal.
  
  Will not log messages out to debug by default, to enable pass:
    `?true` as the second class constructor arg. 
  WIP (to make it throw by passed function). 

  Requires Aviate Labs encoding and principal vessel libraries (for AccountIdentifier validation). 
  https://github.com/aviate-labs */
module MockTokenLedgerCanisters {

  type TokenType = {
    #ICP;
    #ICRC1;
  };

  /** Prints output to debug console. */
  func debugLog(which : TokenType, method : Text, what : Text) {
    let tokenLiteral : Text = switch which {
      case (#ICP) "ICP";
      case (#ICRC1) "ICRC1";
    };
    Debug.print("\n  MockTokenLedgerCanisters." # tokenLiteral # ".Ledger\n    ." # method # "()" # " says what:\n      " # what # "\n");
  };

  public module ICP {
    let permittedDriftNanos : Nat64 = 60_000_000_000;
    let expectedFee : Nat64 = 10_000;
    let expectedFeeNat : Nat = 10_000;
    let transactionWindowNanos : Nat64 = 86_400_000_000_000;
    type AccountIdentifier = Blob;
    type Subaccount = Blob;
    type Tokens = { e8s : Nat64 };
    type Timestamp = { timestamp_nanos : Nat64 };
    type Memo = Nat64;
    type BlockIndex = Nat64;
    type Hash = Blob;
    type SenderArgs = AccountBalanceArgs;
    type AccountBalanceArgs = { account : AccountIdentifier };
    type TransferArgs = {
      memo : Memo;
      amount : Tokens;
      fee : Tokens;
      from_subaccount : ?Subaccount;
      to : AccountIdentifier;
      created_at_time : ?Timestamp;
    };
    type Result<T, E> = { #Ok : T; #Err : E };
    type TransferResult = Result<BlockIndex, TransferError>;
    type TransferError = {
      #BadFee : { expected_fee : Tokens };
      #InsufficientFunds : { balance : Tokens };
      #TxTooOld : { allowed_window_nanos : Nat64 };
      #TxCreatedInFuture;
      #TxDuplicate : { duplicate_of : BlockIndex };
    };
    func isValidSubaccount(s : Subaccount) : Bool {
      (s.size() == 32);
    };
    func isValidAddress(a : AccountIdentifier) : Bool {
      if (a.size() != 32) { return false };
      let arr = Blob.toArray(a);
      let accIdPart = Array.tabulate(28, func(i : Nat) : Nat8 { arr[i + 4] });
      let checksumPart = Array.tabulate(4, func(i : Nat) : Nat8 { arr[i] });
      let crc32 = CRC32.checksum(accIdPart);
      Array.equal(Binary.BigEndian.fromNat32(crc32), checksumPart, Nat8.equal);
    };

    /****Used to mock calls to an ICP ledger canister.**  
      Can be constructed with initial deposit going to default subaccount account identifier of `who`. */
    public class MockLedger(initialDeposit : { amount : { e8s : Nat64 }; who : Principal }, showDebugLog : ?Bool) {

      let doDebugLog = switch showDebugLog {
        case (?existsValue) existsValue;
        case null false;
      };

      func log(method : Text, what : Text) {
        if (doDebugLog) debugLog(#ICP, method, what);
      };

      let defaultSubaccount = Blob.fromArrayMut(Array.init(32, 0 : Nat8));

      var blockIndex : BlockIndex = 0;
      let balances : HashMap.HashMap<Blob, Nat> = HashMap.HashMap(16, Blob.equal, Blob.hash);
      let transactions : HashMap.HashMap<Text, Nat64> = HashMap.HashMap(16, Text.equal, Text.hash);

      /** Mocks an ICP ledger transfer call. */
      public func transfer(
        caller : Principal,
        // autoReturn : ?(TransferResult and { #TriggerTrap : f -> () };
        {
          memo : Memo;
          amount : Tokens;
          fee : Tokens;
          from_subaccount : ?Subaccount;
          to : AccountIdentifier;
          created_at_time : ?Timestamp;
        } : TransferArgs,
      ) : async TransferResult {
        /* WIP
      switch autoReturn {
        case null { 
          // Proceed  
        };
        case (?exists) {
          switch exists {
            case (#TriggerTrap f) { f(); };
            case (_) {
              return exists;
            };
          };
        };
      };
      */
        if (fee.e8s != expectedFee) {
          log("transfer", debug_show (#Err(#BadFee { expected_fee = { e8s = expectedFee } })));
          return #Err(#BadFee { expected_fee = { e8s = expectedFee } });
        };
        let now = Nat64.fromNat(Int.abs(Time.now()));
        let txTime : Nat64 = switch (created_at_time) {
          case (null) { now };
          case (?ts) { ts.timestamp_nanos };
        };
        if ((txTime > now) and (txTime - now > permittedDriftNanos)) {
          log("transfer", debug_show (#Err(#TxCreatedInFuture)));
          return #Err(#TxCreatedInFuture);
        };
        if ((txTime < now) and (now - txTime > transactionWindowNanos)) {
          log("transfer", debug_show (#Err(#TxTooOld { allowed_window_nanos = transactionWindowNanos })));
          return #Err(#TxTooOld { allowed_window_nanos = transactionWindowNanos });
        };
        if (not isValidAddress(to)) {
          log("transfer", "invalid account identifier " #debug_show (to));
          throw Error.reject(debug_show (to) # " is not a valid account identifier address");
        };
        let sender = do {
          let subAccountBlob = switch from_subaccount {
            case null Blob.toArray(defaultSubaccount);
            case (?sub) {
              if (isValidSubaccount(sub)) {
                Blob.toArray(sub);
              } else [];
            };
          };
          if (subAccountBlob.size() == 0) {
            log("transfer", "invalid subaccount " #debug_show (from_subaccount));
            throw Error.reject(debug_show (from_subaccount) # " is not a valid subaccount for an account identifier address");
          };
          AccountIdentifierBlob.fromPrincipal(caller, ?subAccountBlob);
        };
        let debitBalance = Option.get(balances.get(sender), 0);
        let natAmount = Nat64.toNat(amount.e8s);
        if (debitBalance < (natAmount + expectedFeeNat)) {
          log(
            "transfer",
            debug_show (#Err(#InsufficientFunds { balance = { e8s = Nat64.fromNat(debitBalance) } })),
          );
          return #Err(#InsufficientFunds { balance = { e8s = Nat64.fromNat(debitBalance) } });
        } else {
          let txId = debug_show ({
            sender;
            to;
            amount;
            txTime;
            memo;
          });
          switch (transactions.get(txId)) {
            case (?height) {
              log("transfer", debug_show (#Err(#TxDuplicate { duplicate_of = height })));
              return #Err(#TxDuplicate { duplicate_of = height });
            };
            case null {
              blockIndex += 1;
              transactions.put(txId, blockIndex);
            };
          };
          balances.put(sender, (debitBalance - (natAmount + expectedFeeNat)));
          balances.put(to, (Option.get(balances.get(to), 0) + natAmount));
          log("transfer", debug_show (#Ok(blockIndex)));
          #Ok(blockIndex);
        };
      };

      /** Mocks minting given `amount` into given `recipient` AccountIdentifier. */
      public func deposit_free_money({
        recipient : AccountIdentifier;
        amount : { e8s : Nat64 };
      }) : TransferResult {
        assert isValidAddress(recipient);
        balances.put(recipient, Option.get(balances.get(recipient), 0) + Nat64.toNat(amount.e8s));
        blockIndex += 1;
        log("deposit_free_money", debug_show (#Ok(blockIndex : Nat64)));
        #Ok(blockIndex : Nat64);
      };

      /** Mocks ICP ledger balance query. */
      public func account_balance({ account : AccountIdentifier } : AccountBalanceArgs) : async Tokens {
        if (not isValidAddress(account)) {
          throw Error.reject(debug_show (account) # " is not a valid address");
        };
        let b = { e8s = Nat64.fromNat(Option.get(balances.get(account), 0)) };
        log("account_balance", debug_show ({ inputs = { account }; outputs = b }));
        b;
      };

      let { amount; who } = initialDeposit;
      log("initialDeposit", debug_show ({ amount; who }));
      ignore deposit_free_money({
        recipient = AccountIdentifierBlob.fromPrincipal(who, null);
        amount;
      });
    };
  };

  public module ICRC1 {
    let permittedDriftNanos : Duration = 60_000_000_000;
    let transactionWindowNanos : Duration = 86_400_000_000_000;
    let expectedFee : Nat = 10_000;

    type Account = { owner : Principal; subaccount : ?Subaccount };
    type Subaccount = Blob;
    type Memo = Blob;
    type Timestamp = Nat64;
    type Duration = Nat64;
    type TxIndex = Nat;
    type Tokens = Nat;
    type BalanceArgs = Account;

    type TransferArgs = {
      from_subaccount : ?Subaccount;
      to : Account;
      amount : Tokens;
      fee : ?Tokens;
      memo : ?Memo;
      created_at_time : ?Timestamp;
    };
    type Result<T, E> = { #Ok : T; #Err : E };

    type TransferResult = Result<Tokens, TransferError>;

    type CommonFields = {
      memo : ?Memo;
      fee : ?Tokens;
      created_at_time : ?Timestamp;
    };

    type DeduplicationError = {
      #TooOld;
      #Duplicate : { duplicate_of : TxIndex };
      #CreatedInFuture : { ledger_time : Timestamp };
    };

    type CommonError = {
      #InsufficientFunds : { balance : Tokens };
      #BadFee : { expected_fee : Tokens };
      #TemporarilyUnavailable;
      #GenericError : { error_code : Nat; message : Text };
    };

    type TransferError = DeduplicationError or CommonError or {
      #BadBurn : { min_burn_amount : Tokens };
    };

    func isValidSubaccount(s : Subaccount) : Bool {
      (s.size() == 32);
    };

    func isValidAddress(a : Account) : Bool {
      if (Principal.isAnonymous(a.owner)) {
        return false;
      };
      switch (a.subaccount) {
        case (null) {
          // No subaccount so verify it's not only a reserved principal.
          let pbArr = Blob.toArray(Principal.toBlob(a.owner));
          if (pbArr[pbArr.size() - 1] == 127) {
            // Ends in x7F and thus is a reserved principal, so it is required
            // to have a non-trivial subaccount to be a valid icrc1 account.
            return false;
          };
        };
        case (?blob) { return isValidSubaccount(blob) };
      };
      true;
    };

    /****Used to mock calls to an ICRC1 token-ledger canister.**  
      Can be constructed with initial deposit going to default subaccount account identifier of `who`. */
    public class MockLedger(initialDeposit : { amount : Nat; who : Principal }, showDebugLog : ?Bool) {
      let doDebugLog = switch showDebugLog {
        case (?existsValue) existsValue;
        case null false;
      };

      func log(method : Text, what : Text) {
        if (doDebugLog) debugLog(#ICRC1, method, what);
      };

      let defaultSubaccount = Blob.fromArrayMut(Array.init(32, 0 : Nat8));
      var txIndex : TxIndex = 0;
      let balances : HashMap.HashMap<Text, Nat> = HashMap.HashMap(16, Text.equal, Text.hash);
      let transactions : HashMap.HashMap<Text, Nat> = HashMap.HashMap(16, Text.equal, Text.hash);

      func asKey(a : Account) : Text {
        if (not Option.isSome(a.subaccount)) {
          return debug_show ({ owner = a.owner; subaccount = defaultSubaccount });
        } else {
          debug_show (a);
        };
      };

      /** Mocks an ICRC1 token-ledger transfer call. 
      Should correctly return all but #GenericError and #TemporarilyUnavailable. */
      public func icrc1_transfer(
        caller : Principal,
        {
          from_subaccount : ?Subaccount;
          to : Account;
          amount : Tokens;
          fee : ?Tokens;
          memo : ?Memo;
          created_at_time : ?Timestamp;
        } : TransferArgs,
      ) : async TransferResult {
        let now = Nat64.fromNat(Int.abs(Time.now()));
        let txTime : Timestamp = Option.get(created_at_time, now);
        if ((txTime > now) and (txTime - now > permittedDriftNanos)) {
          log("icrc1_transfer", debug_show (#Err(#CreatedInFuture { ledger_time = now })));
          return #Err(#CreatedInFuture { ledger_time = now });
        };
        if ((txTime < now) and (now - txTime > transactionWindowNanos + permittedDriftNanos)) {
          log("icrc1_transfer", debug_show (#Err(#TooOld)));
          return #Err(#TooOld);
        };
        if (not isValidAddress(to)) {
          log("icrc1_transfer", "invalid account");
          throw Error.reject(debug_show (to) # " is not a valid icrc1 account");
        };
        switch memo {
          case null {};
          case (?m) {
            if (m.size() > 32) {
              log("icrc1_transfer", "invalid memo");
              throw Error.reject(debug_show (memo) # " is not a valid icrc1 transfer arg memo");
              // ref uses assert (memo.size() <= 32)
            };
          };
        };
        switch from_subaccount {
          case null {};
          case (?sub) {
            if (not isValidSubaccount(sub)) {
              log("icrc1_transfer", "invalid subaccount");
              throw Error.reject(debug_show (from_subaccount) # " is not a valid subaccount for an icrc1 address");
            };
          };
        };
        let sender = { owner = caller; subaccount = from_subaccount };
        let debitBalance = Option.get(balances.get(asKey(sender)), 0);
        switch fee {
          case null {};
          case (?f) {
            if (f != expectedFee) {
              log("icrc1_transfer", debug_show (#Err(#BadFee { expected_fee = expectedFee })));
              return #Err(#BadFee { expected_fee = expectedFee });
            };
          };
        };
        if (debitBalance < amount + expectedFee) {
          log("icrc1_transfer", debug_show (#Err(#InsufficientFunds { balance = debitBalance })));
          return #Err(#InsufficientFunds { balance = debitBalance });
        };
        let txId = debug_show ({
          sender = asKey(sender);
          to;
          amount;
          txTime;
          memo;
        });
        switch (transactions.get(txId)) {
          case (?duplicate) {
            log("icrc1_transfer", debug_show (#Err(#Duplicate { duplicate_of = duplicate })));
            return #Err(#Duplicate { duplicate_of = duplicate });
          };
          case null {
            txIndex += 1;
            transactions.put(txId, txIndex);
          };
        };
        balances.put(asKey(sender), (debitBalance - (amount + expectedFee)));
        balances.put(asKey(to), (Option.get(balances.get(asKey(to)), 0) + amount));
        log("icrc1_transfer", debug_show (#Ok(txIndex)));
        #Ok(txIndex);
      };

      /** Mocks an ICRC1 token-ledger canister balance query. */
      public func icrc1_balance_of(a : Account) : async Tokens {
        if (not isValidAddress(a)) {
          throw Error.reject(debug_show (a) # " is not a valid icrc1 account");
        };
        let bal = Option.get(balances.get(asKey(a)), 0);
        log("icrc1_balance_of", debug_show ({ input = a; output = bal }));
        bal;
      };

      /** Mocks minting given `amount` into given `recipient`'s' Account. */
      public func deposit_free_money({
        recipient : Account;
        amount : Tokens;
      }) : TransferResult {
        assert isValidAddress(recipient);
        balances.put(asKey(recipient), Option.get(balances.get(asKey(recipient)), 0) + amount);
        txIndex += 1;
        log("deposit_free_money", debug_show ({ input = { recipient; amount }; output = #Ok(txIndex) }));
        #Ok(txIndex);
      };

      let { amount; who } = initialDeposit;
      log("initialDeposit", debug_show ({ amount; who }));
      ignore deposit_free_money({
        recipient = { owner = who; subaccount = null };
        amount;
      });
    };
  };
};
