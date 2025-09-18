import { useState, useEffect } from 'react';
import { IcrcLedgerCanister } from '@dfinity/ledger-icrc';
import { HttpAgent } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { Signer } from '@slide-computer/signer';
import { SignerAgent } from '@slide-computer/signer-agent';
import { PostMessageTransport } from '@slide-computer/signer-web';
import { AccountIdentifier } from '@dfinity/ledger-icp';
import { decodeIcrcAccount, mapTokenMetadata } from '@dfinity/ledger-icrc';
import { toBaseUnits } from '@/libs/utils';
import { CKUSDC_LEDGER_ID, ICP_LEDGER_ID } from '@/libs/constants';

export function useOisyWallet() {
  const [isConnected, setIsConnected] = useState(false);
  const [principal, setPrincipal] = useState(null);
  const [accountIdentifier, setAccountIdentifier] = useState(null);
  const [defaultAgent, setDefaultAgent] = useState(null);
  const [oisySignerAgent, setOisySignerAgent] = useState(null);
  const [oisyIcpActor, setOisyIcpActor] = useState(null);
  const [oisyCkUsdcActor, setOisyCkUsdcActor] = useState(null);

  const [icpMetadata, setIcpMetadata] = useState();
  const [ckUsdcMetadata, setCkUsdcMetadata] = useState();
  const [icpBalance, setIcpBalance] = useState(null);
  const [ckUsdcBalance, setCkUsdcBalance] = useState(null);
  const [isLoading, setIsLoading] = useState(false);

  const oisyTransport = new PostMessageTransport({ url: 'https://oisy.com/sign' });
  const oisySigner = new Signer({ transport: oisyTransport });

  useEffect(() => {
    if (oisySignerAgent && !oisyIcpActor && !oisyCkUsdcActor) {
      const oisyIcpActor = IcrcLedgerCanister.create({
        agent: oisySignerAgent,
        canisterId: Principal.fromText(ICP_LEDGER_ID),
      });
      const oisyCkUsdcActor = IcrcLedgerCanister.create({
        agent: oisySignerAgent,
        canisterId: Principal.fromText(CKUSDC_LEDGER_ID),
      });
      setOisyIcpActor(oisyIcpActor);
      setOisyCkUsdcActor(oisyCkUsdcActor);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [oisySignerAgent]);

  useEffect(() => {
    const fetchBalances = async () => {
      if (!defaultAgent || !principal) return;
      setIsLoading(true);
      try {
        const defaultIcpLedgerAgent = IcrcLedgerCanister.create({
          agent: defaultAgent,
          canisterId: Principal.fromText(ICP_LEDGER_ID),
        });
        const defaultCkUsdcLedgerAgent = IcrcLedgerCanister.create({
          agent: defaultAgent,
          canisterId: Principal.fromText(CKUSDC_LEDGER_ID),
        });

        setIcpMetadata(mapTokenMetadata(await defaultIcpLedgerAgent.metadata({ certified: true })));
        setCkUsdcMetadata(
          mapTokenMetadata(await defaultCkUsdcLedgerAgent.metadata({ certified: true }))
        );
        setIcpBalance(await defaultIcpLedgerAgent.balance({ owner: principal }));
        setCkUsdcBalance(await defaultCkUsdcLedgerAgent.balance({ owner: principal }));
      } catch (e) {
        console.error('Failed to fetch balances', e);
      } finally {
        setIsLoading(false);
      }
    };

    fetchBalances();
  }, [defaultAgent, principal]);

  const connect = async () => {
    const accounts = await oisySigner.accounts();

    // notes:
    //    - IcrcAccount is the recommended way of dealing with accounts as it is standardized and more transparent
    //      - e.g. there is now way to resolve a Subaccount when dealing with the AccountIdentifier representation
    //      - most applications hide the Subaccount completely in the UI and only display the Principal to their users
    //    - ICP ledger is different from ICRC-1 ledgers as it has been introduced before ICRC-1 existed. But the ICP ledger supports all ICRC-1 endpoints.
    //      - it is possible to easily convert an IcrcAccount (Principal + optional Subaccount) into an AccountIdentifier used by the ICP ledger
    //      - it is impossible to determine the (optional) Subaccount from an AccountIdentifier
    //    - the Principal + Subaccount representation of the signer lib currently cannot be directly passed to AccountIdentifier.fromPrincipal (subaccount uses a different type)
    //    - AccountIdentifier is typically only needed to transfer ICP to/from exchanges and to look up the transfer history of the ICP ledger
    const icrcAccount = decodeIcrcAccount(accounts[0].owner.toString());
    const principal = icrcAccount.owner;
    const accountIdentifier = AccountIdentifier.fromPrincipal({ principal });

    const defaultAgent = await HttpAgent.create({ host: 'https://icp0.io' });
    const signerAgent = await SignerAgent.create({
      agent: defaultAgent,
      signer: oisySigner,
      account: principal,
    });

    setDefaultAgent(defaultAgent);
    setOisySignerAgent(signerAgent);
    setPrincipal(principal);
    setAccountIdentifier(accountIdentifier);
    setIsConnected(true);
  };

  const disconnect = () => {
    setIsConnected(false);
    setPrincipal(null);
    setAccountIdentifier(null);
    setDefaultAgent(null);
    setOisySignerAgent(null);
    setOisyIcpActor(null);
    setOisyCkUsdcActor(null);
    setIcpBalance(null);
    setCkUsdcBalance(null);
    setIcpMetadata(undefined);
    setCkUsdcMetadata(undefined);
    setIsLoading(false);
  };

  const transfer = async (ledger, metadata) => {
    if (!ledger || !principal || !metadata) {
      return { success: false, message: 'Missing transfer prerequisites.' };
    }

    try {
      const blockIndex = await ledger.transfer({
        to: { owner: principal, subaccount: [] },
        amount: toBaseUnits(1, metadata.decimals),
      });

      return { success: true, message: 'Transfer successful.', blockIndex };
    } catch (err) {
      return { success: false, message: err.message || 'Transfer failed.' };
    }
  };

  return {
    connect,
    disconnect,
    isConnected,
    principal,
    accountIdentifier,
    isLoading,
    icpBalance,
    ckUsdcBalance,
    icpMetadata,
    ckUsdcMetadata,
    transferIcp: () => transfer(oisyIcpActor, icpMetadata),
    transferCkUsdc: () => transfer(oisyCkUsdcActor, ckUsdcMetadata),
  };
}
