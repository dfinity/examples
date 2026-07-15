import { useQuery } from "@tanstack/react-query";
import useHandleAgentError from "./useHandleAgentError";
import { useAuth } from "@/lib/auth";
import { useIcrcLedger } from "@/actors";

export default function useTokenBalance() {
  const { handleAgentError } = useHandleAgentError();
  const { identity } = useAuth();
  const ledger = useIcrcLedger();

  return useQuery({
    queryKey: ["balance"],
    queryFn: async () => {
      try {
        const result = await ledger.balance({
          owner: identity!.getPrincipal(),
          certified: false,
        });

        if (result === undefined) {
          throw new Error("Undefined balance returned.");
        }

        return result;
      } catch (e) {
        handleAgentError(e);
        console.error(e);
      }
      return null;
    },
    enabled: !!identity,
  });
}
