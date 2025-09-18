import ReactDOM from "react-dom/client";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { InternetIdentityProvider } from "ic-use-internet-identity";
import { ToastProvider } from "./contexts/ToastContext";
import App from "./App";
import "./index.css";

const queryClient = new QueryClient({
    defaultOptions: {
        queries: {
            retry: (failureCount, error) => {
                // Don't retry on authentication errors
                if (
                    error?.message?.includes("Unauthorized") ||
                    error?.message?.includes("identity")
                ) {
                    return false;
                }
                return failureCount < 2;
            },
            staleTime: 30 * 1000, // 30 seconds
            refetchOnWindowFocus: false,
        },
    },
});

ReactDOM.createRoot(document.getElementById("root")).render(
    <QueryClientProvider client={queryClient}>
        <InternetIdentityProvider>
            <ToastProvider>
                <App />
            </ToastProvider>
        </InternetIdentityProvider>
    </QueryClientProvider>
);
