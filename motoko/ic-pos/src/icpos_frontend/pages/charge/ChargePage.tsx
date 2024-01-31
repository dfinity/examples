import { Button } from "../../components/ui/button";
import HeaderSection from "../../components/HeaderSection";
import { Link } from "@tanstack/router";
import MainSection from "../../components/MainSection";
import Page from "../../components/Page";
import { X } from "lucide-react";
import { KeyPad } from "./components/KeyPad";
import { Key } from "./types/key.type";
import { useState } from "react";
import ChargeButton from "./components/ChargeButton";

export default function ChargePage() {
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
          <Link to="/merchant">
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
