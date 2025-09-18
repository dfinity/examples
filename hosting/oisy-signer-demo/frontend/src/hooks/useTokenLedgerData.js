import { useState } from 'react';
import { Principal } from '@dfinity/principal';
import { IcrcLedgerCanister, mapTokenMetadata } from '@dfinity/ledger-icrc';
import { CKUSDC_LEDGER_ID, ICP_LEDGER_ID } from '@/libs/constants';

export function useTokenLedgerData() {
  const [icpBalance, setIcpBalance] = useState(null);
  const [ckUsdcBalance, setCkUsdcBalance] = useState(null);
  const [icpMetadata, setIcpMetadata] = useState();
  const [ckUsdcMetadata, setCkUsdcMetadata] = useState();
  const [isLoading, setIsLoading] = useState(false);

  const load = async (principal, agent) => {
    setIsLoading(true);
    const icpLedgerAgent = IcrcLedgerCanister.create({
      agent,
      canisterId: Principal.fromText(ICP_LEDGER_ID),
    });
    const ckUsdcLedgerAgent = IcrcLedgerCanister.create({
      agent,
      canisterId: Principal.fromText(CKUSDC_LEDGER_ID),
    });

    try {
      const [icpMeta, ckUsdcMeta] = await Promise.all([
        icpLedgerAgent.metadata({ certified: true }),
        ckUsdcLedgerAgent.metadata({ certified: true }),
      ]);
      setIcpMetadata(mapTokenMetadata(icpMeta));
      setCkUsdcMetadata(mapTokenMetadata(ckUsdcMeta));

      const [icpBal, usdcBal] = await Promise.all([
        icpLedgerAgent.balance({ owner: principal }),
        ckUsdcLedgerAgent.balance({ owner: principal }),
      ]);
      setIcpBalance(icpBal);
      setCkUsdcBalance(usdcBal);
    } catch (err) {
      console.error('Error loading token metadata/balance:', err);
    } finally {
      setIsLoading(false);
    }
  };

  return {
    icpBalance,
    usdcBalance: ckUsdcBalance,
    icpMetadata,
    usdcMetadata: ckUsdcMetadata,
    isLoading,
    load,
  };
}
