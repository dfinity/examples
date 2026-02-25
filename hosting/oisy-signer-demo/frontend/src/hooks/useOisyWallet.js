import { useState, useEffect, useRef, useCallback } from 'react';
import { IcrcLedgerCanister, decodeIcrcAccount, mapTokenMetadata } from '@icp-sdk/canisters/ledger/icrc';
import { HttpAgent } from '@icp-sdk/core/agent';
import { Principal } from '@icp-sdk/core/principal';
import { Signer } from '@slide-computer/signer';
import { SignerAgent } from '@slide-computer/signer-agent';
import { PostMessageTransport } from '@slide-computer/signer-web';
import { AccountIdentifier } from '@icp-sdk/canisters/ledger/icp';
import { toBaseUnits } from '@/libs/utils';
import { TESTICP_LEDGER_ID, TICRC1_LEDGER_ID } from '@/libs/constants';

const SESSION_KEY = 'oisy-principal';

const oisySigner = new Signer({
  transport: new PostMessageTransport({ url: 'https://oisy.com/sign' }),
});

// Initialize read-only state from a principal (no wallet popup needed).
// Balances are read via an anonymous agent — only transfers require the signer.
async function initFromPrincipal(principalText) {
  const principal = Principal.fromText(principalText);
  const accountIdentifier = AccountIdentifier.fromPrincipal({ principal });
  const defaultAgent = await HttpAgent.create({ host: 'https://icp0.io' });
  return { principal, accountIdentifier, defaultAgent };
}

export function useOisyWallet() {
  const [isConnected, setIsConnected] = useState(false);
  const [principal, setPrincipal] = useState(null);
  const [accountIdentifier, setAccountIdentifier] = useState(null);
  const [defaultAgent, setDefaultAgent] = useState(null);

  // SignerAgent and signed actors are lazily created on first transfer.
  const signerAgentRef = useRef(null);
  const signedActorsRef = useRef(null);

  const [testIcpMetadata, setTestIcpMetadata] = useState();
  const [tIcrc1Metadata, setTIcrc1Metadata] = useState();
  const [testIcpBalance, setTestIcpBalance] = useState(null);
  const [tIcrc1Balance, setTIcrc1Balance] = useState(null);
  const [isLoading, setIsLoading] = useState(false);

  // Restore session from sessionStorage on mount.
  useEffect(() => {
    const stored = sessionStorage.getItem(SESSION_KEY);
    if (!stored) return;
    let cancelled = false;

    initFromPrincipal(stored).then(({ principal, accountIdentifier, defaultAgent }) => {
      if (cancelled) return;
      setPrincipal(principal);
      setAccountIdentifier(accountIdentifier);
      setDefaultAgent(defaultAgent);
      setIsConnected(true);
    });

    return () => {
      cancelled = true;
    };
  }, []);

  // Fetch balances whenever the agent and principal are available.
  useEffect(() => {
    if (!defaultAgent || !principal) return;

    const fetchBalances = async () => {
      setIsLoading(true);
      try {
        const defaultTestIcpLedger = IcrcLedgerCanister.create({
          agent: defaultAgent,
          canisterId: Principal.fromText(TESTICP_LEDGER_ID),
        });
        const defaultTIcrc1Ledger = IcrcLedgerCanister.create({
          agent: defaultAgent,
          canisterId: Principal.fromText(TICRC1_LEDGER_ID),
        });

        setTestIcpMetadata(mapTokenMetadata(await defaultTestIcpLedger.metadata({ certified: true })));
        setTIcrc1Metadata(
          mapTokenMetadata(await defaultTIcrc1Ledger.metadata({ certified: true }))
        );
        setTestIcpBalance(await defaultTestIcpLedger.balance({ owner: principal }));
        setTIcrc1Balance(await defaultTIcrc1Ledger.balance({ owner: principal }));
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
    //      - e.g. there is no way to resolve a Subaccount when dealing with the AccountIdentifier representation
    //      - most applications hide the Subaccount completely in the UI and only display the Principal to their users
    //    - ICP ledger is different from ICRC-1 ledgers as it has been introduced before ICRC-1 existed. But the ICP ledger supports all ICRC-1 endpoints.
    //      - it is possible to easily convert an IcrcAccount (Principal + optional Subaccount) into an AccountIdentifier used by the ICP ledger
    //      - it is impossible to determine the (optional) Subaccount from an AccountIdentifier
    //    - the Principal + Subaccount representation of the signer lib currently cannot be directly passed to AccountIdentifier.fromPrincipal (subaccount uses a different type)
    //    - AccountIdentifier is typically only needed to transfer ICP to/from exchanges and to look up the transfer history of the ICP ledger
    const icrcAccount = decodeIcrcAccount(accounts[0].owner.toString());
    const principal = icrcAccount.owner;

    sessionStorage.setItem(SESSION_KEY, principal.toText());

    const { accountIdentifier, defaultAgent } = await initFromPrincipal(principal.toText());

    const signerAgent = await SignerAgent.create({
      agent: defaultAgent,
      signer: oisySigner,
      account: principal,
    });
    signerAgentRef.current = signerAgent;
    signedActorsRef.current = {
      testIcp: IcrcLedgerCanister.create({
        agent: signerAgent,
        canisterId: Principal.fromText(TESTICP_LEDGER_ID),
      }),
      tIcrc1: IcrcLedgerCanister.create({
        agent: signerAgent,
        canisterId: Principal.fromText(TICRC1_LEDGER_ID),
      }),
    };

    setDefaultAgent(defaultAgent);
    setPrincipal(principal);
    setAccountIdentifier(accountIdentifier);
    setIsConnected(true);
  };

  const disconnect = () => {
    sessionStorage.removeItem(SESSION_KEY);
    signerAgentRef.current = null;
    signedActorsRef.current = null;
    setIsConnected(false);
    setPrincipal(null);
    setAccountIdentifier(null);
    setDefaultAgent(null);
    setTestIcpBalance(null);
    setTIcrc1Balance(null);
    setTestIcpMetadata(undefined);
    setTIcrc1Metadata(undefined);
    setIsLoading(false);
  };

  // Ensure the signer agent and signed actors are available (reconnects if needed after a page refresh).
  const ensureSignerAgent = useCallback(async () => {
    if (signerAgentRef.current && signedActorsRef.current) return signedActorsRef.current;
    if (!defaultAgent || !principal) throw new Error('Not connected');

    // This opens the OISY popup briefly to re-establish the signer session.
    await oisySigner.accounts();

    const signerAgent = await SignerAgent.create({
      agent: defaultAgent,
      signer: oisySigner,
      account: principal,
    });
    signerAgentRef.current = signerAgent;
    signedActorsRef.current = {
      testIcp: IcrcLedgerCanister.create({
        agent: signerAgent,
        canisterId: Principal.fromText(TESTICP_LEDGER_ID),
      }),
      tIcrc1: IcrcLedgerCanister.create({
        agent: signerAgent,
        canisterId: Principal.fromText(TICRC1_LEDGER_ID),
      }),
    };
    return signedActorsRef.current;
  }, [defaultAgent, principal]);

  const transfer = useCallback(
    async (tokenKey, metadata) => {
      if (!principal || !metadata) {
        return { success: false, message: 'Missing transfer prerequisites.' };
      }

      try {
        const actors = await ensureSignerAgent();
        const ledger = actors[tokenKey];

        const blockIndex = await ledger.transfer({
          to: { owner: principal, subaccount: [] },
          amount: toBaseUnits(1, metadata.decimals),
        });

        return { success: true, message: 'Transfer successful.', blockIndex };
      } catch (err) {
        return { success: false, message: err.message || 'Transfer failed.' };
      }
    },
    [principal, ensureSignerAgent]
  );

  return {
    connect,
    disconnect,
    isConnected,
    principal,
    accountIdentifier,
    isLoading,
    testIcpBalance,
    tIcrc1Balance,
    testIcpMetadata,
    tIcrc1Metadata,
    transferTestIcp: () => transfer('testIcp', testIcpMetadata),
    transferTIcrc1: () => transfer('tIcrc1', tIcrc1Metadata),
  };
}
