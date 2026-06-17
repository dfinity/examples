import Array "mo:core/Array";
import Blob "mo:core/Blob";
import Debug "mo:core/Debug";
import Hex "mo:hex";
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
  // Centralized exchanges (CEXs) identify accounts by this blob; wallets and
  // newer integrations prefer the ICRC-1 format (principal + subaccount directly).
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

  // Convert a principal and optional subaccount to its AccountIdentifier as a
  // lowercase hex string — the format shown in block explorers and CEX deposit screens.
  public query func toAccountIdHex(p : Principal, subaccount : ?SubAccount) : async Text {
    let bytes = Array.fromIter(p.toLedgerAccount(subaccount).vals());
    Hex.toText(bytes);
  };

  // Transfer ICP to a recipient identified by principal + optional subaccount.
  // Internally calls Principal.toLedgerAccount to derive the AccountIdentifier.
  // This is the most convenient form when you have a principal.
  public shared func transferToPrincipal(amount : Tokens, toPrincipal : Principal, toSubaccount : ?SubAccount) : async Result.Result<BlockIndex, Text> {
    Debug.print("Transferring " # debug_show amount # " to principal " # debug_show toPrincipal);
    await doTransfer(amount, toPrincipal.toLedgerAccount(toSubaccount));
  };

  // Transfer ICP to a recipient identified by an AccountIdentifier hex string.
  // Use this when an exchange or block explorer gives you the destination as a
  // 64-char hex string rather than as a principal.
  //
  // Note: CRC32 checksum validation is not performed here — the ICP ledger
  // validates it and returns a clear error on mismatch. A fromHex helper with
  // CRC32 validation (equivalent to AccountIdentifier::from_hex in ic-ledger-types)
  // would be a valuable addition to a Motoko ICP library.
  public shared func transferToAccountId(amount : Tokens, toAccountIdHex : Text) : async Result.Result<BlockIndex, Text> {
    switch (Hex.toArray(toAccountIdHex)) {
      case (#err(e)) #err("invalid hex: " # e);
      case (#ok(bytes)) {
        if (bytes.size() != 32) {
          return #err("AccountIdentifier must be 32 bytes (64 hex chars), got " # debug_show(bytes.size()));
        };
        await doTransfer(amount, Blob.fromArray(bytes));
      };
    };
  };
};
