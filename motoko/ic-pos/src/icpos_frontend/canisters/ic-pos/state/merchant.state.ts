import { Merchant } from "../../../../declarations/icpos/icpos.did";
import { atom } from "recoil";

type MerchantStateType = {
  initialized: boolean;
  merchant: Merchant | undefined;
};

export const MerchantState = atom<MerchantStateType>({
  key: "MerchantState",
  default: { initialized: false, merchant: undefined },
});
