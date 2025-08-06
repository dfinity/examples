import { useInternetIdentity } from "ic-use-internet-identity";
import { LogIn, LogOut, User, Copy, Check } from "lucide-react";
import { useState } from "react";
import { useQueryClient } from "@tanstack/react-query";

export function AuthButton() {
    const { identity, login, clear } = useInternetIdentity();
    const [copied, setCopied] = useState(false);
    const queryClient = useQueryClient();

    const copyPrincipal = async () => {
        if (!identity) return;

        try {
            const principal = identity.getPrincipal().toString();
            await navigator.clipboard.writeText(principal);
            setCopied(true);
            setTimeout(() => setCopied(false), 2000); // Reset after 2 seconds
        } catch (error) {
            console.error("Failed to copy principal:", error);
        }
    };

    const handleLogout = async () => {
        // Clear all queries before logging out
        queryClient.clear();
        // Clear the identity
        await clear();
        // Force a page reload to ensure clean state
        window.location.reload();
    };

    if (identity) {
        return (
            <div className="flex flex-col sm:flex-row items-center gap-3 sm:gap-4">
                <button
                    onClick={copyPrincipal}
                    className="flex items-center gap-2 text-xs sm:text-sm text-gray-400 hover:text-gray-300 transition-colors cursor-pointer group"
                    title="Click to copy full principal"
                >
                    <User className="w-4 h-4" />
                    <span className="font-mono text-xs sm:text-sm">
                        {identity.getPrincipal().toString().slice(0, 6)}...
                    </span>
                    {copied ? (
                        <Check className="w-3 h-3 text-green-400" />
                    ) : (
                        <Copy className="w-3 h-3 opacity-0 group-hover:opacity-100 transition-opacity" />
                    )}
                </button>
                {copied && (
                    <span className="text-xs text-green-400 animate-fade-in">
                        Copied!
                    </span>
                )}
                <button
                    onClick={handleLogout}
                    className="flex items-center gap-2 px-3 sm:px-4 py-2 bg-red-600 hover:bg-red-700 rounded-lg transition-colors text-sm"
                >
                    <LogOut className="w-4 h-4" />
                    <span className="hidden sm:inline">Logout</span>
                    <span className="sm:hidden">Exit</span>
                </button>
            </div>
        );
    }

    return (
        <button
            onClick={login}
            className="flex items-center justify-center gap-2 px-4 sm:px-6 py-2 bg-purple-600 hover:bg-purple-700 rounded-lg transition-colors font-medium text-sm sm:text-base w-full sm:w-auto"
        >
            <LogIn className="w-4 h-4" />
            <span className="hidden sm:inline">Login with Internet Identity</span>
            <span className="sm:hidden">Login with II</span>
        </button>
    );
}
