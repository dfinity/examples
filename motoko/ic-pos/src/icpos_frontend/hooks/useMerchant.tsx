import { useQuery } from "@tanstack/react-query";
import useHandleAgentError from "./useHandleAgentError";
import { useAuth } from "@/lib/auth";
import { useIcPosActor } from "@/actors";

export default function useMerchant() {
  const { actor: pos } = useIcPosActor();
  const { handleAgentError } = useHandleAgentError();
  const { identity } = useAuth();

  return useQuery({
    queryKey: ["merchant"],
    queryFn: async () => {
      try {
        const result = await pos?.getMerchant();

        if (result === undefined) {
          throw new Error("Undefined merchant returned.");
        }

        if (result.status === 200 && result.data) {
          return result.data;
        }
      } catch (e) {
        handleAgentError(e);
        console.error(e);
      }
      return null;
    },
    enabled: !!pos && !!identity,
  });
}
