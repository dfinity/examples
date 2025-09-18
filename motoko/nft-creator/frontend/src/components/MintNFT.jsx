import { useState } from "react";
import { useCollectionOwner, useMintNFT } from "../hooks/useQueries";
import { useInternetIdentity } from "ic-use-internet-identity";
import { Principal } from "@dfinity/principal";
import { Sparkles } from "lucide-react";
import { useToast } from "../contexts/ToastContext";

export function MintNFT() {
    const { identity } = useInternetIdentity();
    const { data: collectionOwner, isLoading: isLoadingOwner } =
        useCollectionOwner();
    const { mutate: mintNFT, isPending: isMinting } = useMintNFT();
    const { addError } = useToast();

    const [recipient, setRecipient] = useState("");

    const isOwner =
        identity &&
        collectionOwner &&
        identity.getPrincipal().toString() === collectionOwner.toString();
    console.log("Collection Owner:", collectionOwner?.toString());

    if (isLoadingOwner) {
        return (
            <div className="flex items-center gap-2 text-gray-400 text-sm">
                <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-purple-500"></div>
                Loading collection owner...
            </div>
        );
    }

    if (!collectionOwner) {
        return (
            <div className="text-gray-400 text-sm sm:text-base">
                Collection must be claimed before minting NFTs.
            </div>
        );
    }

    if (!isOwner) {
        return (
            <div className="text-gray-400 text-sm sm:text-base">
                Only the collection owner can mint NFTs.
            </div>
        );
    }

    const handleMint = () => {
        if (!recipient) return;

        try {
            const recipientPrincipal = Principal.fromText(recipient);
            mintNFT({
                to: { owner: recipientPrincipal, subaccount: [] }, // Assuming no subaccount
            });

            // Reset form
            setRecipient("");
        } catch (error) {
            console.error("Invalid principal:", error);
            addError("Invalid principal: " + (error?.message || error));
        }
    };

    return (
        <div className="space-y-4">
            <div className="w-full">
                <label className="block text-xs sm:text-sm font-medium text-gray-300 mb-2">
                    Recipient Principal
                </label>
                <input
                    type="text"
                    value={recipient}
                    onChange={(e) => setRecipient(e.target.value)}
                    placeholder="Enter recipient principal"
                    className="w-full px-3 py-2 text-sm bg-gray-700 border border-gray-600 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent"
                />
            </div>

            <button
                onClick={handleMint}
                disabled={isMinting || !recipient}
                className="flex items-center justify-center gap-2 px-4 py-2 bg-purple-600 hover:bg-purple-700 disabled:bg-gray-600 disabled:cursor-not-allowed rounded-lg transition-colors text-sm w-full sm:w-auto"
            >
                {isMinting ? (
                    <>
                        <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
                        Minting...
                    </>
                ) : (
                    <>
                        <Sparkles className="w-4 h-4" />
                        Mint NFT
                    </>
                )}
            </button>
        </div>
    );
}
