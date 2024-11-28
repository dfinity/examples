import { BadgeCheck, X } from "lucide-react";

import { Button } from "../../../components/ui/button";
import PrincipalPill from "../../../components/PrincipalPill";
import { Transfer } from "../../../canisters/icrc/types/transfer.type";
import { formatCkBtc } from "../../../utils/formatCkBtc";
import { useAuth } from "../../../auth/hooks/useAuth";
import { useCkBtcLedgerAnon } from "../../../canisters/ckbtc-ledger/hooks/useCkBtcLedgerAnon";
import { useEffect } from "react";
import useSound from "use-sound";
import { useState } from "react";

type TransactionOverlayProps = {
  onTransfer?: (transfer: Transfer) => void;
};

export default function TransactionOverlay({
  onTransfer,
}: TransactionOverlayProps) {
  const search = window.location.search;
  const params = new URLSearchParams(search);

  // Hooks
  const { identity } = useAuth();
  const { ckBtcLedger } = useCkBtcLedgerAnon();

  // Local state
  const [latestTransactionIndex, setLatestTransactionIndex] =
    useState<bigint>();
  const [receivedTransfer, setReceivedTransfer] = useState<Transfer>();
  const [close, setClose] = useState<boolean>(false);

  const principal =
    params.get("principal") || identity?.getPrincipal().toString() || "";

  function getLatestTransactionIndex() {
    if (!principal || !ckBtcLedger) return;
    (async () => {
      const blocks = await ckBtcLedger?.get_blocks({
        start: BigInt(0),
        length: BigInt(1),
      });
      if (blocks?.chain_length) {
        setLatestTransactionIndex(blocks.chain_length);
      }
    })();
  }
  useEffect(getLatestTransactionIndex, [principal, ckBtcLedger]);

  useEffect(() => {
    if (!principal || !latestTransactionIndex) return;
    const pollTransactions = setInterval(async () => {
      const transaction = await ckBtcLedger?.get_transactions({
        start: latestTransactionIndex,
        length: BigInt(1),
      });
      if (
        transaction?.transactions &&
        Array.isArray(transaction.transactions) &&
        transaction.transactions.length > 0
      ) {
        setLatestTransactionIndex(latestTransactionIndex + BigInt(1));
        const t = transaction.transactions[0];
        if (
          t &&
          t.kind === "transfer" &&
          t.transfer[0]?.to.owner.toString() === principal
        ) {
          setReceivedTransfer(t.transfer[0]);
          onTransfer && onTransfer(t.transfer[0]);
        }
      }
    }, 15000); // 15 seconds
    return () => clearInterval(pollTransactions);
  }, [principal, latestTransactionIndex, ckBtcLedger, onTransfer]);

  let classNames =
    "absolute top-0 left-0 flex flex-col items-center justify-center w-full h-full space-y-10 text-white bg-cyan-800 md:rounded-lg";
  classNames += close
    ? " animate-out slide-out-to-top duration-150"
    : " animate-in slide-in-from-top";

  const closeAnimation = () => {
    setClose(true);
    const interval = setTimeout(async () => {
      setReceivedTransfer(undefined);
      setClose(false);
    }, 150);
    return () => clearInterval(interval);
  };

  const [playSound] = useSound("/cash-register.mp3");

  // Only show if there is a received transaction
  if (!receivedTransfer) {
    return null;
  }

  !close && playSound();

  return (
    <div className={classNames}>
      <div className="flex justify-end w-full grow">
        <div className="p-5">
          <Button variant="ghost" size="icon" onClick={closeAnimation}>
            <X className="w-4 h-4 hover:text-black" />
          </Button>
        </div>
      </div>
      <BadgeCheck className="w-36 h-36" />
      <div className="text-4xl font-bold">Payment Received!</div>
      <PrincipalPill principal={receivedTransfer?.from.owner} />
      <div>{formatCkBtc(receivedTransfer?.amount)} ckBTC</div>
      <div className="grow" />
    </div>
  );
}
