import { Merchant, _SERVICE } from "../../../../declarations/icpos/icpos.did";

import { ActorSubclass } from "@dfinity/agent";
import { MerchantState } from "../state/merchant.state";
import React from "react";
import { createActor } from "../../../../declarations/icpos";
import { useAuth } from "../../../auth/hooks/useAuth";
import { useRecoilState } from "recoil";

export function useIcPos() {
  const { isAuthenticated, authClient, hasLoggedIn, identity, agent } =
    useAuth();
  const [icPos, setIcPos] = React.useState<ActorSubclass<_SERVICE> | null>(
    null
  );
  const [merchantState, setMerchantState] = useRecoilState(MerchantState);

  React.useEffect(() => {
    if (!isAuthenticated || !authClient || !hasLoggedIn || !agent) return;
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const actor = createActor(process.env.CANISTER_ID_ICPOS!, {
      agent,
    });
    setIcPos(actor);
  }, [isAuthenticated, authClient, hasLoggedIn, identity, agent]);

  React.useEffect(() => {
    if (!icPos) return;
    icPos.getMerchant().then((response) => {
      if (response.status === 200) {
        if (!response.data) return;
        setMerchantState({
          initialized: true,
          merchant: response.data[0],
        });
        return;
      }
      setMerchantState({
        initialized: true,
        merchant: undefined,
      });
    });
  }, [icPos, setMerchantState]);

  const updateMerchant = async (merchant: Merchant) => {
    if (!icPos) return;
    const response = await icPos.updateMerchant(merchant);
    if (response.status === 200) {
      if (!response.data) return;
      setMerchantState({
        initialized: true,
        merchant: response.data[0],
      });
    }
    return response;
  };

  return { merchantState, updateMerchant };
}
