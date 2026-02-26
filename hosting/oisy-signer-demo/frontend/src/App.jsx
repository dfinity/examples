import { useEffect, useState } from 'react';
import { Moon, Sun, ExternalLink, Copy, X, Loader2, LogOut } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Switch } from '@/components/ui/switch';
import { OisyIcon } from '@/components/ui/oisyIcon';
import ICPLogo from './assets/icp.svg';
import TICRC1Logo from './assets/ticrc1.svg';
import OISYLogo from './assets/oisy.svg';
import { useOisyWallet } from './hooks/useOisyWallet';
import { TESTICP_LEDGER_ID, TICRC1_LEDGER_ID } from './libs/constants';
import { toMainUnit } from './libs/utils';

// Helper functions for button styling and content
const getButtonVariant = (isConnected, darkMode) => {
  if (!isConnected) return 'connect';
  return darkMode ? 'disconnect-dark' : 'disconnect';
};

const getButtonContent = (isConnected) => {
  if (isConnected) {
    return (
      <>
        <LogOut className="mr-2 h-4 w-4" />
        Disconnect
      </>
    );
  }

  return (
    <>
      <OisyIcon />
      Connect OISY Wallet
    </>
  );
};

export default function App() {
  const {
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
    transferTestIcp,
    transferTIcrc1,
  } = useOisyWallet();

  const [darkMode, setDarkMode] = useState(
    () => window.matchMedia('(prefers-color-scheme: dark)').matches
  );
  const [error, setError] = useState(null);
  const [success, setSuccess] = useState(null);

  useEffect(() => {
    if (success) {
      const timer = setTimeout(() => setSuccess(null), 10000);
      return () => clearTimeout(timer);
    }
  }, [success]);

  useEffect(() => {
    if (error) {
      const timer = setTimeout(() => setError(null), 10000);
      return () => clearTimeout(timer);
    }
  }, [error]);

  const copyToClipboard = (text) => {
    navigator.clipboard
      .writeText(text)
      .then(() => setSuccess('Copied to clipboard'))
      .catch(() => setError('Failed to copy'));
  };

  const handleTransfer = async (token) => {
    const result = token === 'TESTICP' ? await transferTestIcp() : await transferTIcrc1();

    if (result.success && result.blockIndex !== undefined) {
      const ledgerId = token === 'TESTICP' ? TESTICP_LEDGER_ID : TICRC1_LEDGER_ID;
      const url = `https://dashboard.internetcomputer.org/tokens/${ledgerId}/transaction/${result.blockIndex}`;

      setSuccess(
        <span>
          {token} transfer successful.{' '}
          <a href={url} target="_blank" rel="noopener noreferrer" className="underline">
            View on Dashboard
          </a>{' '}
          <span className="opacity-75">(transactions can take up to 10 minutes to appear)</span>
        </span>
      );
    } else {
      setError(result.message);
    }
  };

  return (
    <div
      className={`min-h-screen ${darkMode ? 'dark bg-zinc-900 text-white' : 'bg-white text-zinc-900'} px-4 py-6 sm:px-6 lg:px-8`}
    >
      <div className="mx-auto max-w-4xl space-y-8">
        {/* Header */}
        <header className="flex flex-col items-center justify-between gap-4 sm:flex-row">
          <div className="flex items-center gap-4">
            <img src={OISYLogo} alt="OISY" className="h-10 w-10" />
            <h1 className="text-xl font-semibold">OISY Signer Demo</h1>
          </div>
          <div className="flex items-center gap-4">
            <div className="flex items-center gap-2">
              <Switch checked={darkMode} onCheckedChange={setDarkMode} />
              {darkMode ? <Moon size={18} /> : <Sun size={18} />}
            </div>
            <Button
              onClick={isConnected ? disconnect : connect}
              variant={getButtonVariant(isConnected, darkMode)}
            >
              {getButtonContent(isConnected)}
            </Button>
          </div>
        </header>

        {/* Disconnected Intro */}
        {!isConnected && (
          <div className="max-w-2xl space-y-4 text-sm sm:text-base">
            <p>
              This example demonstrates how to interact with the <strong>OISY Wallet</strong> using
              the <strong>Signer Standard</strong> and <strong>ICRC-1</strong> tokens.
            </p>
            <p>
              After connecting your wallet, you&apos;ll be able to view your balances for{' '}
              <strong>TESTICP</strong> and <strong>TICRC1</strong> and trigger a test transfer of 1
              token to your own principal.
            </p>
            <p>
              You can obtain testnet tokens for free using the{' '}
              <a
                href="https://faucet.internetcomputer.org"
                target="_blank"
                rel="noopener noreferrer"
                className="text-blue-600 underline"
              >
                ICP Faucet
              </a>
              . In OISY, select the <strong>IC (testnet tokens)</strong> network to view them.
            </p>
            <p>This app is purely for demonstration purposes and does not store any user data.</p>
            <p>
              Click <strong>Connect</strong> at the top right to begin, or explore the references
              below to learn more.
            </p>
          </div>
        )}

        {/* Wallet Info & Token Cards */}
        {isConnected && (
          <div className="space-y-6">
            <div className="space-y-1 text-sm">
              <div className="flex flex-wrap items-center gap-2">
                <span className="whitespace-nowrap font-semibold">Principal:</span>
                <span className="break-all">{principal?.toString()}</span>
                <button
                  onClick={() => copyToClipboard(principal.toString())}
                  className="inline-flex h-7 w-7 items-center justify-center rounded text-zinc-500 transition-colors hover:bg-zinc-100 hover:text-zinc-900 focus:outline-none dark:text-zinc-400 dark:hover:bg-zinc-800 dark:hover:text-zinc-100"
                  title="Copy to clipboard"
                >
                  <Copy size={14} />
                </button>
              </div>
              <div className="flex flex-wrap items-center gap-2">
                <span className="whitespace-nowrap font-semibold">AccountIdentifier:</span>
                <span className="break-all">{accountIdentifier?.toHex()}</span>
                <button
                  onClick={() => copyToClipboard(accountIdentifier.toHex())}
                  className="inline-flex h-7 w-7 items-center justify-center rounded text-zinc-500 transition-colors hover:bg-zinc-100 hover:text-zinc-900 focus:outline-none dark:text-zinc-400 dark:hover:bg-zinc-800 dark:hover:text-zinc-100"
                  title="Copy to clipboard"
                >
                  <Copy size={14} />
                </button>
              </div>
            </div>

            {isLoading ? (
              <div className="flex items-center justify-center py-10 text-zinc-500 dark:text-zinc-400">
                <div className="flex items-center gap-3 text-base">
                  <Loader2 className="animate-spin" size={20} />
                  Loading token balances...
                </div>
              </div>
            ) : (
              <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
                {/* TESTICP Card */}
                <div className="space-y-2 rounded-xl border border-zinc-200 p-4 dark:border-zinc-700">
                  <div className="flex items-center gap-2">
                    <img src={ICPLogo} alt="TESTICP" className="h-5 w-5" />
                    <span>TESTICP</span>
                    <a
                      href={`https://dashboard.internetcomputer.org/tokens/${TESTICP_LEDGER_ID}/account/${principal?.toString()}`}
                      target="_blank"
                      rel="noreferrer"
                      className="text-blue-600 underline"
                    >
                      <ExternalLink size={14} />
                    </a>
                  </div>
                  <div className="text-sm">
                    Balance:{' '}
                    {testIcpBalance != null && testIcpMetadata
                      ? toMainUnit(testIcpBalance, testIcpMetadata.decimals)
                      : '...'}
                  </div>
                  <Button onClick={() => handleTransfer('TESTICP')} disabled={isLoading}>
                    Transfer TESTICP
                  </Button>
                  <p className="text-xs text-zinc-500 dark:text-zinc-400">
                    Transfers 1 TESTICP to your own OISY principal for testing.
                  </p>
                </div>

                {/* TICRC1 Card */}
                <div className="space-y-2 rounded-xl border border-zinc-200 p-4 dark:border-zinc-700">
                  <div className="flex items-center gap-2">
                    <img src={TICRC1Logo} alt="TICRC1" className="h-5 w-5" />
                    <span>TICRC1</span>
                    <a
                      href={`https://dashboard.internetcomputer.org/tokens/${TICRC1_LEDGER_ID}/account/${principal?.toString()}`}
                      target="_blank"
                      rel="noreferrer"
                      className="text-blue-600 underline"
                    >
                      <ExternalLink size={14} />
                    </a>
                  </div>
                  <div className="text-sm">
                    Balance:{' '}
                    {tIcrc1Balance != null && tIcrc1Metadata
                      ? toMainUnit(tIcrc1Balance, tIcrc1Metadata.decimals)
                      : '...'}
                  </div>
                  <Button onClick={() => handleTransfer('TICRC1')} disabled={isLoading}>
                    Transfer TICRC1
                  </Button>
                  <p className="text-xs text-zinc-500 dark:text-zinc-400">
                    Transfers 1 TICRC1 to your own OISY principal for testing.
                  </p>
                </div>
              </div>
            )}
          </div>
        )}

        {/* Toasts */}
        {(isLoading || error || success) && (
          <div className="fixed left-1/2 top-4 z-50 w-[90%] max-w-xl -translate-x-1/2 transform sm:w-auto">
            {success && (
              <div className="flex items-start justify-between gap-4 rounded bg-green-500 px-4 py-2 text-white shadow-md">
                <div className="text-sm">{success}</div>
                <button onClick={() => setSuccess(null)}>
                  <X size={16} />
                </button>
              </div>
            )}

            {error && (
              <div className="mt-2 flex items-start justify-between gap-4 rounded bg-red-500 px-4 py-2 text-white shadow-md">
                <div className="text-sm">{error}</div>
                <button onClick={() => setError(null)}>
                  <X size={16} />
                </button>
              </div>
            )}
          </div>
        )}

        {/* Footer */}
        <footer className="mt-10 space-y-2 border-t border-zinc-200 pt-10 text-sm text-zinc-500 dark:border-zinc-700 dark:text-zinc-400">
          <p>References:</p>
          <ul className="list-inside list-disc space-y-1">
            <li>
              <a
                className="inline-flex items-center gap-1 text-blue-600 underline"
                href="https://oisy.com"
                target="_blank"
                rel="noreferrer"
              >
                OISY Wallet <ExternalLink size={14} />
              </a>
            </li>
            <li>
              <a
                className="inline-flex items-center gap-1 text-blue-600 underline"
                href="https://faucet.internetcomputer.org"
                target="_blank"
                rel="noreferrer"
              >
                ICP Faucet (get testnet tokens) <ExternalLink size={14} />
              </a>
            </li>
            <li>
              <a
                className="inline-flex items-center gap-1 text-blue-600 underline"
                href="https://docs.internetcomputer.org/defi/token-standards"
                target="_blank"
                rel="noreferrer"
              >
                Token Standards on ICP <ExternalLink size={14} />
              </a>
            </li>
            <li>
              <a
                className="inline-flex items-center gap-1 text-blue-600 underline"
                href="https://github.com/dfinity/ICRC-1/tree/main/standards/ICRC-1"
                target="_blank"
                rel="noreferrer"
              >
                ICRC-1 Token Standard <ExternalLink size={14} />
              </a>
            </li>
            <li>
              <a
                className="inline-flex items-center gap-1 text-blue-600 underline"
                href="https://docs.internetcomputer.org/defi/token-ledgers/usage/icrc1_ledger_usage#from-a-web-application"
                target="_blank"
                rel="noreferrer"
              >
                Using ICRC-1 Ledger <ExternalLink size={14} />
              </a>
            </li>
            <li>
              <a
                className="inline-flex items-center gap-1 text-blue-600 underline"
                href="https://github.com/dfinity/wg-identity-authentication/blob/main/topics/signer_standards_overview.md"
                target="_blank"
                rel="noreferrer"
              >
                Signer Standards <ExternalLink size={14} />
              </a>
            </li>
            <li>
              <a
                className="inline-flex items-center gap-1 text-blue-600 underline"
                href="https://github.com/slide-computer/signer-js/tree/main"
                target="_blank"
                rel="noreferrer"
              >
                Signer-JS Libraries <ExternalLink size={14} />
              </a>
            </li>
          </ul>
        </footer>
      </div>
    </div>
  );
}
