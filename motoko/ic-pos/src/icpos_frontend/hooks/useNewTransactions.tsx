import { useQuery } from "@tanstack/react-query";
import useHandleAgentError from "./useHandleAgentError";
import { useInternetIdentity } from "ic-use-internet-identity";
import { icrc1_index } from "../../declarations/icrc1_index/index";
import { Account } from "src/declarations/icrc1_ledger/icrc1_ledger.did";
import { GetAccountTransactionsArgs, TransactionWithId } from "src/declarations/icrc1_index/icrc1_index.did";
import { Principal } from "@dfinity/principal";

export default function useNewTransactions() {
  const { handleAgentError } = useHandleAgentError();
  const { identity } = useInternetIdentity();

  const getAccountTransactions = (principal: Principal) => {
    const account: Account = {
      owner: principal,
      subaccount: []
    };
    const args: GetAccountTransactionsArgs = {
      account,
      start: [],
      max_results: 5n,
    }
    return icrc1_index.get_account_transactions(args);

  }

  const getLatestTransactionId = (principal: Principal) => {
    const item = localStorage.getItem(`${principal.toString()}_latest_transaction_id`);
    return item ? BigInt(item) : 0n;
  }

  const saveLatestTransactionId = (principal: Principal, id: bigint) => {
    localStorage.setItem(`${principal!.toString()}_latest_transaction_id`, id.toString());

  }

  return useQuery({
    queryKey: ['new_transactions'],
    queryFn: async () => {
      try {
        const principal = identity!.getPrincipal();
        const result = await getAccountTransactions(principal);
        if (result === undefined) {
          throw new Error("Undefined balance returned.");
        }
        if ('Err' in result) {
          throw new Error(result.Err.message);
        }
        const newTransactions: TransactionWithId[] = [];
        let latestTransactionId = getLatestTransactionId(principal);
        for (const transaction of result.Ok.transactions.reverse()) {
          if (transaction.id > latestTransactionId) {
            latestTransactionId = transaction.id;
            newTransactions.push(transaction);
          }
        }
        saveLatestTransactionId(principal, latestTransactionId);
        return newTransactions;
      } catch (e) {
        handleAgentError(e);
        console.error(e);
      }
      return null;
    },
    enabled: !!identity,
    refetchInterval: 5000,
  });
}


