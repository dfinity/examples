import { useQuery } from "@tanstack/react-query";
import useHandleAgentError from "./useHandleAgentError";
import { useInternetIdentity } from "ic-use-internet-identity";
import { icrc1_ledger } from "../../declarations/icrc1_ledger/index";
import { Account } from "src/declarations/icrc1_ledger/icrc1_ledger.did";

export default function useTokeBalance() {
  const { handleAgentError } = useHandleAgentError();
  const { identity } = useInternetIdentity();

  return useQuery({
    queryKey: ['balance'],
    queryFn: async () => {
      try {
        const account: Account = {
          owner: identity!.getPrincipal(),
          subaccount: []
        };

        const result = await icrc1_ledger.icrc1_balance_of(account);

        if (result === undefined) {
          throw new Error("Undefined balance returned.");
        }

        return result
      } catch (e) {
        handleAgentError(e);
        console.error(e);
      }
      return null;
    },
    enabled: !!identity,
  });
}


