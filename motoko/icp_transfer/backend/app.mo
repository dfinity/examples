import Debug "mo:core/Debug";
import Result "mo:core/Result";
import Error "mo:core/Error";
import Principal "mo:core/Principal";

actor IcpTransfer {
  // ICP Ledger types (matches the ICP ledger Candid interface)
  type Tokens = { e8s : Nat64 };
  type SubAccount = Blob;
  type BlockIndex = Nat64;
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

  // Public API
  type TransferArgs = {
    amount : Tokens;
    toPrincipal : Principal;
    toSubaccount : ?SubAccount;
  };

  public shared func transfer(args : TransferArgs) : async Result.Result<BlockIndex, Text> {
    Debug.print(
      "Transferring "
      # debug_show (args.amount)
      # " tokens to principal "
      # debug_show (args.toPrincipal)
      # " subaccount "
      # debug_show (args.toSubaccount)
    );

    let transferArgs : LedgerTransferArgs = {
      // can be used to distinguish between transactions
      memo = 0;
      // the amount we want to transfer
      amount = args.amount;
      // the ICP ledger charges 10_000 e8s for a transfer
      fee = { e8s = 10_000 };
      // we are transferring from the canister's default subaccount
      from_subaccount = null;
      // convert principal and optional subaccount to an ICP ledger account identifier
      to = args.toPrincipal.toLedgerAccount(args.toSubaccount);
      // null: the ledger stores the current IC time as the transaction timestamp.
      // Note: passing null also disables deduplication — if you need protection
      // against duplicate submissions, pass the current time explicitly instead.
      created_at_time = null;
    };

    try {
      let transferResult = await icpLedger.transfer(transferArgs);

      switch (transferResult) {
        case (#Err(transferError)) {
          return #err("Couldn't transfer funds:\n" # debug_show (transferError));
        };
        case (#Ok(blockIndex)) { return #ok blockIndex };
      };
    } catch (error : Error) {
      return #err("Reject message: " # error.message());
    };
  };
};
