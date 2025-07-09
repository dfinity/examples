import { useQuery } from "@tanstack/react-query";
import useHandleAgentError from "./useHandleAgentError";
import { useInternetIdentity } from "ic-use-internet-identity";
import { useIcPosActor } from "@/actors";

export default function useMerchant() {
  const { actor: pos } = useIcPosActor();
  const { handleAgentError } = useHandleAgentError();
  const { identity } = useInternetIdentity();

  return useQuery({
    queryKey: ['merchant'],
    queryFn: async () => {

      try {
        const result = await pos?.getMerchant();

        if (result === undefined) {
          throw new Error("Undefined merchant returned.");
        }

        if (result.status === 200 && result.data.length > 0) {
          return result.data[0];
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

