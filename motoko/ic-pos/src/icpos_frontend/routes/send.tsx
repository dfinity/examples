import { createFileRoute, Link, redirect } from '@tanstack/react-router'
import useTokeBalance from '@/hooks/useTokenBalance';
import { useInternetIdentity } from 'ic-use-internet-identity';
import { useState } from 'react';
import Page from '@/components/Page';
import HeaderSection from '@/components/HeaderSection';
import { Button } from '@/components/ui/button';
import { QrCode, X } from 'lucide-react';
import MainSection from '@/components/MainSection';
import { formatCkBtc } from '@/utils/formatCkBtc';
import PrincipalPill from '@/components/PrincipalPill';
import SendForm from '@/components/send/SendForm';
import QRReader from '@/components/QRReader';
import { Result } from 'react-zxing';
import toast from 'react-hot-toast';

export const Route = createFileRoute('/send')({
  beforeLoad: ({ context }) => {
    if (!context.identity) {
      throw redirect({
        to: '/',
      })
    }
  },
  component: SendPage,
})

export default function SendPage() {
  const { identity } = useInternetIdentity();
  const { data: balance } = useTokeBalance();
  const [qrReaderOpen, setQrReaderOpen] = useState(false);
  const [principal, setPrincipal] = useState("");
  const [amount, setAmount] = useState("0");

  function parseQrString(input: string) {
    const regex = /^ckbtc:([^?]+)\?amount="([^"]+)"$/;
    const match = input.match(regex);

    if (!match) {
      throw new Error('Invalid format');
    }

    const principal = match[1];
    const amount = match[2];

    return { principal, amount };
  }

  const handleQrResult = (result: Result) => {

    const text = result.getText();
    try {
      // If text contains ':', we assume it to be an ICRC-22
      // payment request string. If not, we assume it to be
      // just a principal.
      if (text.includes(':')) {
        const { principal, amount } = parseQrString(text);
        setAmount(amount);
        setPrincipal(principal);
      } else {
        setPrincipal(text)
      }
    } catch {
      toast.error("Couldn't parse QR code");
    }
    setQrReaderOpen(false);
  };

  return (
    <Page>
      <HeaderSection>
        <Link to="/">
          <Button variant="ghost" size="icon">
            <X className="w-4 h-4" />
          </Button>
        </Link>
        Send
        <Button
          variant="ghost"
          size="icon"
          className="hover:text-black"
          onClick={() => setQrReaderOpen(true)}
        >
          <QrCode className="w-4 h-4" />
        </Button>
      </HeaderSection>
      <MainSection>
        <div className="flex flex-col items-center justify-between p-5 pb-10 space-y-5 grow">
          <div className="grow" />
          {!qrReaderOpen && (
            <>
              <div>{formatCkBtc(balance)} ckBTC</div>
              <PrincipalPill principal={identity?.getPrincipal().toString()} />
              <div className="grow" />
              <SendForm principal={principal} amount={amount} />
            </>
          )}
          <QRReader
            setVisible={setQrReaderOpen}
            visible={qrReaderOpen}
            onResult={handleQrResult}
          />
        </div>
      </MainSection>
    </Page>
  );
}

