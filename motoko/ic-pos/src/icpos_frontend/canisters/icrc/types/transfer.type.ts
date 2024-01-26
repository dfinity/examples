import { Account } from "@dfinity/ledger/dist/candid/icrc1_ledger";

export type Transfer = {
  to: Account;
  fee: [] | [bigint];
  from: Account;
  memo: [] | [Uint8Array];
  created_at_time: [] | [bigint];
  amount: bigint;
};
