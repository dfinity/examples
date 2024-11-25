import { useQuery } from "@tanstack/react-query";
import useHandleAgentError from "./useHandleAgentError";
import { useInternetIdentity } from "ic-use-internet-identity";
import { icrc1_index } from "../../declarations/icrc1_index/index";
import { Account } from "src/declarations/icrc1_ledger/icrc1_ledger.did";
import { GetAccountTransactionsArgs } from "src/declarations/icrc1_index/icrc1_index.did";
import { Principal } from "@dfinity/principal";

export default function useLatestTransactions() {
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

  return useQuery({
    queryKey: ['latest_transactions'],
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
        return result.Ok.transactions;
      } catch (e) {
        handleAgentError(e);
        console.error(e);
      }
      return null;
    },
    enabled: !!identity,
  });
}


