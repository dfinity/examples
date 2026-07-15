import { useMutation } from "@tanstack/react-query";
import { useBackendActor } from "@/actors";
import type { Merchant } from "@/bindings/backend";
import { queryClient } from "@/main";

export default function useUpdateMerchant() {
  const { actor: pos } = useBackendActor();

  return useMutation({
    mutationFn: async (merchant: Merchant) => {
      const result = await pos?.updateMerchant(merchant);

      if (result === undefined) {
        throw new Error("Undefined result returned.");
      }

      if (result.status === 200) {
        queryClient.invalidateQueries({ queryKey: ["merchant"] });
        return;
      }

      console.error(result.status, result.error_text);
      throw new Error(
        result.error_text ?? `Failed to update merchant (status ${result.status}).`
      );
    },
  });
}
