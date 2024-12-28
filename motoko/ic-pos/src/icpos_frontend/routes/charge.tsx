import { createFileRoute, Link, redirect } from '@tanstack/react-router'
import Page from '@/components/Page';
import HeaderSection from '@/components/HeaderSection';
import { Button } from '@/components/ui/button';
import MainSection from '@/components/MainSection';
import { useState } from 'react';
import ChargeButton from '@/components/charge/ChargeButton';
import { KeyPad } from '@/components/charge/KeyPad';
import { Key } from '@/components/charge/key.type';
import { X } from 'lucide-react';

export const Route = createFileRoute('/charge')({
  beforeLoad: ({ context }) => {
    if (!context.identity) {
      throw redirect({
        to: '/',
      })
    }
  },
  component: ChargePage,
})

function ChargePage() {
  const [amount, setAmount] = useState<string>("0");
  const handleKey = (key: Key) => {
    switch (key) {
      case "decimal":
        if (!amount.includes(".")) {
          setAmount(amount + ".");
        }
        break;
      case "backspace":
        setAmount(amount.slice(0, -1) || "0");
        break;
      case "0":
        if (amount !== "0") {
          setAmount(amount + "0");
        }
        break;
      default:
        if (amount === "0") {
          setAmount(key);
        } else {
          setAmount(amount + key);
        }
        break;
    }
  };

  return (
    <Page>
      <div className="relative flex flex-col grow">
        <HeaderSection>
          <Link to="/">
            <Button variant="ghost" size="icon">
              <X className="w-4 h-4" />
            </Button>
          </Link>
          Charge
          <div className="w-8"></div>
        </HeaderSection>
        <MainSection>
          <div className="flex flex-col items-center justify-between flex-1 pt-10 pb-10 space-y-5 grow">
            <div className="text-4xl font-bold">{amount}</div>
            <div className="flex-grow" />
            <KeyPad onKey={handleKey} />
            <ChargeButton amount={amount} />
          </div>
        </MainSection>
      </div>
    </Page>
  );
}

