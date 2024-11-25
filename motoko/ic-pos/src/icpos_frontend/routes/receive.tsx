import { createFileRoute, Link, redirect } from '@tanstack/react-router'
import useMerchant from '@/hooks/useMerchant';
import { useInternetIdentity } from 'ic-use-internet-identity';
import Page from '@/components/Page';
import MainSection from '@/components/MainSection';
import HistoryButton from '@/components/HistoryButton';
import PrincipalPill from '@/components/PrincipalPill';
import { QRCodeSVG } from 'qrcode.react';
import HeaderSection from '@/components/HeaderSection';
import { Button } from '@/components/ui/button';
import { Printer, X } from 'lucide-react';

export const Route = createFileRoute('/receive')({
  beforeLoad: ({ context }) => {
    if (!context.identity) {
      throw redirect({
        to: '/',
      })
    }
  },
  component: ReceivePage,
})


function ReceivePage() {
  const { data: merchant } = useMerchant();
  const { identity } = useInternetIdentity();

  const search = window.location.search;
  const params = new URLSearchParams(search);

  if (!identity) return;

  const principal =
    identity?.getPrincipal().toString();

  const amount = params.get("amount");

  const qrCodeValue = amount
    ? `ckbtc:${principal}?amount=${amount}`
    : principal;

  return (
    <Page>
      <div className="relative flex flex-col grow">
        <HeaderSection>
          <Link to={"/"}>
            <Button variant="ghost" size="icon">
              <X className="w-4 h-4" />
            </Button>
          </Link>
          Receive
          <Button
            variant="ghost"
            size="icon"
            className="hover:text-black"
            onClick={() => window.print()}
          >
            <Printer className="w-4 h-4" />
          </Button>
        </HeaderSection>
        <MainSection>
          <div className="flex flex-col items-center justify-between flex-1 pt-10 pb-10 space-y-5 grow">
            <div className="text-4xl font-bold">
              {amount ? `Pay ckBTC: ${amount}` : "Pay with ckBTC"}
            </div>
            <QRCodeSVG value={qrCodeValue} height={300} width={300} />
            <div className="flex flex-col items-center justify-center space-y-3">
              {!params.has("principal") && (
                <div>{merchant?.name}</div>
              )}
              <PrincipalPill
                principal={principal}
                variant="short"
                className="print:hidden"
              />
              <PrincipalPill
                principal={principal}
                variant="full"
                className="hidden print:block"
              />
            </div>
            <HistoryButton />
          </div>
        </MainSection>
      </div>
    </Page>
  );
}

