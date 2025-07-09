import { useMutation } from "@tanstack/react-query";
import { useIcPosActor } from "@/actors";
import { Merchant } from "src/declarations/icpos/icpos.did";
import { queryClient } from "@/main";

export default function useUpdateMerchant() {
  const { actor: pos } = useIcPosActor();

  return useMutation({
    mutationFn: async (merchant: Merchant) => {

      const result = await pos?.updateMerchant(merchant);

      if (result === undefined) {
        throw new Error("Undefined result returned.");
      }

      if (result.status === 200) {
        queryClient.invalidateQueries({ queryKey: ['merchant'] });
        return;
      }

      console.error(result.status, result.error_text[0]);
      throw new Error(result.error_text[0]);
    }
  });
}

