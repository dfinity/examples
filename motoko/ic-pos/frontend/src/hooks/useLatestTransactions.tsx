import { useQuery } from "@tanstack/react-query";
import useHandleAgentError from "./useHandleAgentError";
import { useAuth } from "@/lib/auth";
import { useIcrcIndex } from "@/actors";

export default function useLatestTransactions() {
  const { handleAgentError } = useHandleAgentError();
  const { identity } = useAuth();
  const index = useIcrcIndex();

  return useQuery({
    queryKey: ["latest_transactions"],
    queryFn: async () => {
      try {
        const result = await index.getTransactions({
          account: { owner: identity!.getPrincipal() },
          max_results: 5n,
        });
        return result.transactions;
      } catch (e) {
        handleAgentError(e);
        console.error(e);
      }
      return null;
    },
    enabled: !!identity,
  });
}
