import { Link, Navigate } from "@tanstack/router";
import { QrCode, X } from "lucide-react";

import { Button } from "../../components/ui/button";
import FullpageLoading from "../../components/FullpageLoading";
import HeaderSection from "../../components/HeaderSection";
import MainSection from "../../components/MainSection";
import Page from "../../components/Page";
import PrincipalPill from "../../components/PrincipalPill";
import QRReader from "../../components/QRReader";
import React from "react";
import { Result } from "react-zxing";
import SendForm from "./components/SendForm";
import { formatCkBtc } from "../../utils/formatCkBtc";
import { useAuth } from "../../auth/hooks/useAuth";
import useCkBtcLedger from "../../canisters/ckbtc-ledger/hooks/useCkBtcLedger";
import { useIcPos } from "../../canisters/ic-pos/hooks/useIcPos";

export default function SendPage() {
  const { merchantState } = useIcPos();
  const { identity, hasLoggedIn } = useAuth();
  const { balance } = useCkBtcLedger();

  const [qrReaderOpen, setQrReaderOpen] = React.useState(false);
  const [principal, setPrincipal] = React.useState("");

  const handleQrResult = (result: Result) => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const p = result as any;
    if (p?.text) {
      setPrincipal(p.toString());
      setQrReaderOpen(false);
    }
  };

  // This page requires authentication
  if (!hasLoggedIn) {
    return <Navigate to="/" />;
  }

  if (!merchantState || !merchantState.merchant || !identity)
    return <FullpageLoading />;

  return (
    <Page>
      <HeaderSection>
        <Link to="/merchant">
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
              <SendForm principal={principal} />
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
