import { Outlet, createRootRouteWithContext } from '@tanstack/react-router';
import { Github } from 'lucide-react';
import React from 'react';
import { Toaster } from 'react-hot-toast';
import FullpageLoading from '../components/FullpageLoading';
import { Identity } from '@dfinity/agent';
import { Merchant } from 'src/declarations/icpos/icpos.did';

interface RouterContext {
  identity: Identity
  merchant: Merchant | null
}

export const Route = createRootRouteWithContext<RouterContext>()({
  component: Root,
});


function Root() {
  return (
    <>
      <div className="min-h-screen bg-gradient-to-br from-gray-200 to-gray-500 md:flex md:flex-col md:items-center md:justify-center print-bg">
        <div className="container flex flex-col max-w-lg print:w-full print:max-w-none min-h-screen p-0 m-0 mx-auto prose prose-xl text-black md:rounded-lg bg-slate-50 md:shadow-xl md:h-[750px] md:min-h-0 print-maincontainer">
          <React.Suspense fallback={<FullpageLoading />}>
            <Outlet />
            <Toaster />
          </React.Suspense>
        </div>
        <div className="invisible w-full mt-5 text-center text-gray-600 md:visible print:hidden">
          Developed by:{" "}
          <a
            href="https://github.com/kristoferlund"
            target="_blank"
            className="underline"
            rel="noreferrer"
          >
            Kristofer Lund
          </a>
        </div>
        <div className="invisible w-full mt-3 text-center text-gray-600 md:visible print:hidden">
          <a
            href="https://github.com/kristoferlund/ic-pos"
            target="_blank"
            className="underline"
            rel="noreferrer"

          >
            <Github className="inline-block w-4 h-4" />{" "}
            https://github.com/kristoferlund/ic-pos
          </a>
        </div>
      </div>
    </>
  );

}
