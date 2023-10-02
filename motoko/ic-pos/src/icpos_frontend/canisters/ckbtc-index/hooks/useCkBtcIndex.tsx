import { IcrcIndexCanister } from "@dfinity/ledger";
import { Principal } from "@dfinity/principal";
import React from "react";
import { useAuth } from "../../../auth/hooks/useAuth";

export default function useCkBtcIndex() {
  const { identity, agent } = useAuth();
  const [indexCanister, setIndexCanister] = React.useState<
    IcrcIndexCanister | undefined
  >();

  React.useEffect(() => {
    if (!identity) return;
    const init = async () => {
      const indexCanister = IcrcIndexCanister.create({
        agent,
        canisterId: Principal.fromText(
          // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
          process.env.CANISTER_ID_ICRC1_INDEX!
        ),
      });
      setIndexCanister(indexCanister);
    };
    init();
  }, [identity, agent]);

  return indexCanister;
}
