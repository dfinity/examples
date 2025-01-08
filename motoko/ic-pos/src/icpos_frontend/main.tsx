import "./index.css";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { RouterProvider, createRouter } from "@tanstack/react-router";
import { InternetIdentityProvider, useInternetIdentity } from "ic-use-internet-identity";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { routeTree } from "./routeTree.gen";
import Actors from "./actors";
import useMerchant from "./hooks/useMerchant";

export const router = createRouter({
  routeTree, context: {
    identity: undefined!,
    merchant: undefined!,
  }
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retryOnMount: false,
      retry: false,
      gcTime: Infinity,
      staleTime: Infinity
    }
  }
});

function InnerRoot() {
  const { identity, isInitializing } = useInternetIdentity();
  const { data: merchant, isPending } = useMerchant();

  if (isInitializing || (identity && isPending)) return null;

  return <RouterProvider router={router} context={{ identity, merchant }} />
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <InternetIdentityProvider>
        <Actors>
          <InnerRoot />
        </Actors>
      </InternetIdentityProvider>
    </QueryClientProvider>
  </StrictMode>
);


