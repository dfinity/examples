import { createFileRoute, Link, redirect } from '@tanstack/react-router'
import FullpageLoading from '@/components/FullpageLoading';
import Page from '@/components/Page';
import HeaderSection from '@/components/HeaderSection';
import { Button } from '@/components/ui/button';
import { X } from 'lucide-react';
import MainSection from '@/components/MainSection';
import FullpageError from '@/components/FullpageError';
import TransactionRow from '@/components/history/TransactionRow';
import useLatestTransactions from '@/hooks/useLatestTransactions';

export const Route = createFileRoute('/history')({
  beforeLoad: ({ context }) => {
    if (!context.identity) {
      throw redirect({
        to: '/',
      })
    }
  },
  component: HistoryPage,
})

function HistoryPage() {
  const { data: transactions, isPending } = useLatestTransactions();

  if (isPending) return <FullpageLoading />;

  if (!transactions) return <FullpageError />;

  return (
    <Page>
      <HeaderSection>
        <Link
          to={"/"}
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

