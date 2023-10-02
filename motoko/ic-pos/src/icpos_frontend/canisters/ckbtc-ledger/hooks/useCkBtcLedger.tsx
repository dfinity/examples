import { IcrcLedgerCanister, IcrcTokenMetadataResponse } from "@dfinity/ledger";
import { useCallback, useEffect, useState } from "react";

import { Principal } from "@dfinity/principal";
import { useAuth } from "../../../auth/hooks/useAuth";

export default function useCkBtcLedger() {
  const { identity, agent } = useAuth();
  const [ledgerCanister, setLedgerCanister] = useState<
    IcrcLedgerCanister | undefined
  >();

  const [metadata, setMetadata] = useState<IcrcTokenMetadataResponse>();
  const [balance, setBalance] = useState<bigint | null>();

  const getMetadata = useCallback(async () => {
    if (!ledgerCanister) {
      throw new Error("LedgerCanister not initialized");
    }
    setMetadata(await ledgerCanister.metadata({ certified: false }));
  }, [ledgerCanister]);

  const getBalance = useCallback(async () => {
    if (!ledgerCanister || !identity) {
      return null;
    }
    setBalance(
      await ledgerCanister.balance({
        owner: identity.getPrincipal(),
        certified: false,
      })
    );
  }, [ledgerCanister, identity]);

  useEffect(() => {
    if (!identity || !agent || metadata || balance) return;
    const init = async () => {
      const ledgerCanister = IcrcLedgerCanister.create({
        agent,
        canisterId: Principal.fromText(
          // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
          process.env.CANISTER_ID_ICRC1_LEDGER!
        ),
      });
      setLedgerCanister(ledgerCanister);
      await getMetadata();
      await getBalance();
    };
    init();
  }, [identity, agent, getBalance, getMetadata, metadata, balance]);

  return { ledgerCanister, getMetadata, getBalance, metadata, balance };
}
