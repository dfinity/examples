import { Outlet, createRootRouteWithContext } from '@tanstack/react-router';
import React from 'react';
import { Toaster } from 'react-hot-toast';
import { Identity } from '@dfinity/agent';
import { Merchant } from 'src/declarations/icpos/icpos.did';
import NewTransactionNotifer from '@/components/NewTransactionNotifier';

interface RouterContext {
  identity: Identity
  merchant: Merchant | null
}

export const Route = createRootRouteWithContext<RouterContext>()({
  component: Root,
});

function Root() {
  return (
    <div className="min-h-screen md:flex md:flex-col md:items-center md:justify-center">
      <div className="container flex flex-col max-w-lg print:w-full print:max-w-none h-dvh p-0 m-0 mx-auto prose prose-xl text-black md:rounded-lg bg-slate-50 md:shadow-xl md:h-[750px] md:min-h-0 print-maincontainer">
        <React.Suspense fallback={null}>
          <Outlet />
        </React.Suspense>
      </div>
      <Toaster />
      <NewTransactionNotifer />
    </div>
  );

}
