import {
  Actor,
  ActorSubclass,
  AnonymousIdentity,
  HttpAgent,
} from "@dfinity/agent";

import React from "react";
import { _SERVICE } from "@dfinity/ledger/dist/candid/icrc1_ledger";
import { idlFactory } from "@dfinity/ledger/dist/candid/icrc1_ledger.idl";

export function useCkBtcLedgerAnon() {
  const [ckBtcLedger, setCkBtcLedger] = React.useState<
    ActorSubclass<_SERVICE> | undefined
  >();

  const createActor = (): ActorSubclass<_SERVICE> => {
    const agent = new HttpAgent({
      identity: new AnonymousIdentity(),
      host: import.meta.env.VITE_IC_HOST,
    });

    if (import.meta.env.VITE_DFX_NETWORK !== "ic") {
      agent.fetchRootKey().catch((err) => {
        console.warn(
          "Unable to fetch root key. Check to ensure that your local replica is running"
        );
        console.error(err);
      });
    }
    // Creates an actor with using the candid interface and the HttpAgent
    return Actor.createActor(idlFactory, {
      agent,
      canisterId: import.meta.env.VITE_CANISTER_ID_CKBTC_LEDGER,
    });
  };

  React.useEffect(() => {
    const actor = createActor();
    setCkBtcLedger(actor);
  }, []);

  return { ckBtcLedger };
}
