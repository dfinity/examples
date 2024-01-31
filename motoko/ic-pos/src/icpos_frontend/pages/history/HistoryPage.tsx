import { useEffect, useState } from "react";

import { Button } from "../../components/ui/button";
import FullpageLoading from "../../components/FullpageLoading";
import HeaderSection from "../../components/HeaderSection";
import { Link } from "@tanstack/router";
import MainSection from "../../components/MainSection";
import Page from "../../components/Page";
import { Principal } from "@dfinity/principal";
import TransactionRow from "./components/TransactionRow";
import { TransactionWithId } from "@dfinity/ledger/dist/candid/icrc1_index";
import { X } from "lucide-react";
import { useAuth } from "../../auth/hooks/useAuth";
import useCkBtcIndex from "../../canisters/ckbtc-index/hooks/useCkBtcIndex";

export default function HistoryPage() {
  const ckBtcIndex = useCkBtcIndex();
  const { identity, hasLoggedIn } = useAuth();

  const search = window.location.search;
  const params = new URLSearchParams(search);
  const principal =
    params.get("principal") || identity?.getPrincipal().toString() || "";

  const [transactions, setTransactions] = useState<TransactionWithId[]>();

  function loadTransactions() {
    if (!principal || !ckBtcIndex) return;
    (async () => {
      const response = await ckBtcIndex.getTransactions({
        max_results: BigInt(6),
        account: {
          owner: Principal.fromText(principal),
        },
      });
      if (response) {
        const transfers = response.transactions.filter(
          (t) => t.transaction.kind === "transfer"
        );
        setTransactions(transfers);
      }
    })();
  }

  useEffect(loadTransactions, [principal, ckBtcIndex]);

  if (!transactions) return <FullpageLoading />;

  return (
    <Page>
      <HeaderSection>
        <Link
          to={hasLoggedIn ? "/merchant" : "/receive"}
          search={{ principal }}
        >
          <Button variant="ghost" size="icon">
            <X className="w-4 h-4" />
          </Button>
        </Link>
        History
        <div className="w-4 h-4" />
      </HeaderSection>
      <MainSection>
        <div className="flex flex-col items-center justify-top w-full grow md:h-[30px]">
          {transactions.map((transaction, index) => (
            <TransactionRow transaction={transaction.transaction} key={index} />
          ))}
        </div>
      </MainSection>
    </Page>
  );
}
