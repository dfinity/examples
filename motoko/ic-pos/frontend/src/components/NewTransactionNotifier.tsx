
import { formatCkBtc } from '@/utils/formatCkBtc';
import useNewTransactions from '@/hooks/useNewTransactions';
import { useEffect } from 'react';
import toast from 'react-hot-toast';
import { queryClient } from '@/main';
import type { IcrcIndexDid } from '@icp-sdk/canisters/ledger/icrc';

type Transaction = IcrcIndexDid.Transaction;
import useSound from 'use-sound';
import { useAuth } from '@/lib/auth';

export default function NewTransactionNotifier() {
  const { identity } = useAuth();
  const { data: newTransactions } = useNewTransactions();
  const [play] = useSound("/cash-register.mp3");

  const principal = identity?.getPrincipal();

  useEffect(() => {
    if (!newTransactions || newTransactions.length === 0) return;

    const handleTransaction = (kind: "mint" | "transfer", transaction: Transaction) => {
      const amount = transaction[kind]?.[0]?.amount;
      const to = transaction[kind]?.[0]?.to.owner;

      if (amount && to && principal?.compareTo(to) === 'eq') {
        toast.success(`Received: ${formatCkBtc(amount)} ckBTC`, { duration: 5000, icon: '💵' });
        play();
      }
    };

    for (const { transaction } of newTransactions) {
      if (transaction.kind === 'mint' || transaction.kind === 'transfer') {
        handleTransaction(transaction.kind, transaction);
      }
    }

    queryClient.invalidateQueries({ queryKey: ['balance'] });
    queryClient.invalidateQueries({ queryKey: ['new_transactions'] });
    queryClient.invalidateQueries({ queryKey: ['latest_transactions'] });
  }, [newTransactions]);

  return null;
}

