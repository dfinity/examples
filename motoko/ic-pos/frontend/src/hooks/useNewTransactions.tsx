import { useQuery } from "@tanstack/react-query";
import useHandleAgentError from "./useHandleAgentError";
import { useAuth } from "@/lib/auth";
import { useIcrcIndex } from "@/actors";
import type { IcrcIndexDid } from "@icp-sdk/canisters/ledger/icrc";
import type { Principal } from "@icp-sdk/core/principal";

export default function useNewTransactions() {
  const { handleAgentError } = useHandleAgentError();
  const { identity } = useAuth();
  const index = useIcrcIndex();

  const getLatestTransactionId = (principal: Principal) => {
    const item = localStorage.getItem(
      `${principal.toString()}_latest_transaction_id`
    );
    return item ? BigInt(item) : 0n;
  };

  const saveLatestTransactionId = (principal: Principal, id: bigint) => {
    localStorage.setItem(
      `${principal.toString()}_latest_transaction_id`,
      id.toString()
    );
  };

  return useQuery({
    queryKey: ["new_transactions"],
    queryFn: async () => {
      try {
        const principal = identity!.getPrincipal();
        const result = await index.getTransactions({
          account: { owner: principal },
          max_results: 5n,
        });

        const newTransactions: IcrcIndexDid.TransactionWithId[] = [];
        let latestTransactionId = getLatestTransactionId(principal);
        for (const transaction of result.transactions.reverse()) {
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
