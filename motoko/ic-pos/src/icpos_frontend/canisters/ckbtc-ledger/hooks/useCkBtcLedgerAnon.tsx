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
      host: process.env.DFX_NETWORK === "ic" ? "https://icp0.io" : "http://localhost:4943",
    });

    if (process.env.DFX_NETWORK !== "ic") {
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
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      canisterId: process.env.CANISTER_ID_ICRC1_LEDGER!,
    });
  };

  React.useEffect(() => {
    const actor = createActor();
    setCkBtcLedger(actor);
  }, []);

  return { ckBtcLedger };
}
