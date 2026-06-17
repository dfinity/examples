import Debug "mo:core/Debug";
import Result "mo:core/Result";
import Error "mo:core/Error";
import Principal "mo:core/Principal";

actor IcpTransfer {
  // ICP Ledger types (matches the ICP ledger Candid interface)
  type Tokens = { e8s : Nat64 };
  type SubAccount = Blob;
  type BlockIndex = Nat64;

  // An AccountIdentifier is a 32-byte blob that encodes a principal and an
  // optional subaccount. It is the native account format used by the ICP ledger.
  // Many exchanges and wallets identify accounts by this blob rather than by
  // principal, so being able to work with both formats is important.
  //
  // Use Principal.toLedgerAccount(subaccount) to compute one from a principal.
  type AccountIdentifier = Blob;

  type TimeStamp = { timestamp_nanos : Nat64 };

  type LedgerTransferArgs = {
    memo : Nat64;
    amount : Tokens;
    fee : Tokens;
    from_subaccount : ?SubAccount;
    to : AccountIdentifier;
    created_at_time : ?TimeStamp;
  };

  type TransferError = {
    #BadFee : { expected_fee : Tokens };
    #InsufficientFunds : { balance : Tokens };
    #TxTooOld : { allowed_window_nanos : Nat64 };
    #TxCreatedInFuture;
    #TxDuplicate : { duplicate_of : BlockIndex };
  };

  type TransferResult = { #Ok : BlockIndex; #Err : TransferError };

  // The ICP ledger is a system canister available on both mainnet and
  // the local development network at this well-known principal.
  let icpLedger : actor { transfer : (LedgerTransferArgs) -> async TransferResult } = actor ("ryjl3-tyaaa-aaaaa-aaaba-cai");

  // Shared transfer logic.
  func doTransfer(amount : Tokens, to : AccountIdentifier) : async Result.Result<BlockIndex, Text> {
    let transferArgs : LedgerTransferArgs = {
      memo = 0;
      amount;
      fee = { e8s = 10_000 };
      from_subaccount = null;
      to;
      // null: the ledger stores the current IC time as the transaction timestamp.
      // Note: passing null also disables deduplication — if you need protection
      // against duplicate submissions, pass the current time explicitly instead.
      created_at_time = null;
    };
    try {
      switch (await icpLedger.transfer(transferArgs)) {
        case (#Err(e)) #err("Transfer failed: " # debug_show e);
        case (#Ok(blockIndex)) #ok blockIndex;
      };
    } catch (e) {
      #err("Reject: " # e.message());
    };
  };

  // Convert a principal and optional subaccount to its AccountIdentifier.
  // Expose this as a query so callers can inspect the conversion and use the
  // resulting blob with transferToAccountId.
  public query func toAccountId(p : Principal, subaccount : ?SubAccount) : async AccountIdentifier {
    p.toLedgerAccount(subaccount);
  };

  // Transfer ICP to a recipient identified by principal + optional subaccount.
  // Internally calls Principal.toLedgerAccount to derive the AccountIdentifier.
  // This is the most convenient form when you have a principal.
  public shared func transferToPrincipal(amount : Tokens, toPrincipal : Principal, toSubaccount : ?SubAccount) : async Result.Result<BlockIndex, Text> {
    Debug.print("Transferring " # debug_show amount # " to principal " # debug_show toPrincipal);
    await doTransfer(amount, toPrincipal.toLedgerAccount(toSubaccount));
  };

  // Transfer ICP to a recipient identified by an AccountIdentifier blob.
  // Use this when you already have an account identifier — for example, when
  // an exchange or external service provides the destination as a blob rather
  // than as a principal.
  public shared func transferToAccountId(amount : Tokens, toAccountId : AccountIdentifier) : async Result.Result<BlockIndex, Text> {
    Debug.print("Transferring " # debug_show amount # " to account id " # debug_show toAccountId);
    await doTransfer(amount, toAccountId);
  };
};
