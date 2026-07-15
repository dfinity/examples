import { useMemo } from "react";
import { HttpAgent } from "@icp-sdk/core/agent";
import { Principal } from "@icp-sdk/core/principal";
import { IcrcLedgerCanister, IcrcIndexCanister } from "@icp-sdk/canisters/ledger/icrc";
import { createActor } from "./bindings/backend";
import { useAuth } from "./lib/auth";
import {
  host,
  rootKey,
  backendCanisterId,
  icrc1LedgerCanisterId,
  icrc1IndexCanisterId,
} from "./lib/env";

/**
 * The ic-pos backend actor, built with the current identity. Memoized so it is
 * only recreated when the identity changes (e.g. login/logout).
 */
export function useBackendActor() {
  const { identity } = useAuth();

  const actor = useMemo(
    () =>
      createActor(backendCanisterId, {
        agentOptions: { host, rootKey, identity },
      }),
    [identity]
  );

  return { actor };
}

/** The ICRC-1 ledger client, built with the current identity. */
export function useIcrcLedger() {
  const { identity } = useAuth();

  return useMemo(() => {
    const agent = HttpAgent.createSync({ host, rootKey, identity });
    return IcrcLedgerCanister.create({
      agent,
      canisterId: Principal.fromText(icrc1LedgerCanisterId),
    });
  }, [identity]);
}

/** The ICRC-1 index client, built with the current identity. */
export function useIcrcIndex() {
  const { identity } = useAuth();

  return useMemo(() => {
    const agent = HttpAgent.createSync({ host, rootKey, identity });
    return IcrcIndexCanister.create({
      agent,
      canisterId: Principal.fromText(icrc1IndexCanisterId),
    });
  }, [identity]);
}
