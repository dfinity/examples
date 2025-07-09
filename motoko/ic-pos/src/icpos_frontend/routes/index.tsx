import { Link, createFileRoute, redirect } from '@tanstack/react-router'
import Page from '../components/Page'
import MainSection from '../components/MainSection'
import { Cog, QrCodeIcon } from "lucide-react";
import HeaderSection from '@/components/HeaderSection';
import { Button } from '@/components/ui/button';
import { useInternetIdentity } from 'ic-use-internet-identity';
import useMerchant from '@/hooks/useMerchant';
import useTokenBalance from '@/hooks/useTokenBalance';
import { formatCkBtc } from '@/utils/formatCkBtc';
import PrincipalPill from '@/components/PrincipalPill';
import LogoutButton from '@/components/LogoutButton';
import ChargeButton from '@/components/merchant/ChargeButton';
import SendButton from '@/components/merchant/SendButton';
import HistoryButton from '@/components/HistoryButton';

export const Route = createFileRoute('/')({
  beforeLoad: ({ context }) => {
    if (!context.identity) {
      throw redirect({
        to: '/login',
      })
    }
    if (!context.merchant) {
      throw redirect({
        to: '/setup'
      })
    }
  },
  component: MerchantPage
})

function MerchantPage() {
  const { identity } = useInternetIdentity();
  const { data: merchant } = useMerchant();
  const { data: balance } = useTokenBalance();

  if (!identity) return;

  return (
    <Page>
      <div className="relative flex flex-col grow">
        <HeaderSection>
          <LogoutButton />
          {merchant?.name}
          <Link to="/config">
            <Button variant="ghost" size="icon">
              <Cog className="w-4 h-4" />
            </Button>
          </Link>
        </HeaderSection>
        <MainSection>
          <div className="flex flex-col items-center justify-between pb-10 space-y-5 grow">
            <div className="grow" />
            <div>{formatCkBtc(balance)} ckBTC</div>
            <PrincipalPill principal={identity?.getPrincipal().toString()} />
            <div className="grow" />
            <ChargeButton />
            <SendButton />
            <Link to="/receive" className="flex items-center gap-2">
              Show store QR code <QrCodeIcon />
            </Link>
            <div className="flex flex-col items-center justify-end grow">
              <HistoryButton />
            </div>
          </div>
        </MainSection>
      </div>
    </Page>
  );
}

