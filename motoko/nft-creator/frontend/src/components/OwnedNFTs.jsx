import { useOwnedNFTs } from "../hooks/useQueries";
import { NFTCard } from "./NFTCard";

export function OwnedNFTs() {
    const { data: ownedNFTs, isLoading, error } = useOwnedNFTs();

    if (isLoading) {
        return (
            <div className="flex items-center gap-2 text-gray-400">
                <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-purple-500"></div>
                Loading your NFTs...
            </div>
        );
    }

    if (error) {
        return (
            <div className="text-red-400">
                Error loading NFTs: {error.message}
            </div>
        );
    }

    if (!ownedNFTs || ownedNFTs.length === 0) {
        return (
            <div className="text-gray-400 text-center py-6 sm:py-8 text-sm sm:text-base">
                You don't own any NFTs yet.
            </div>
        );
    }

    return (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6">
            {ownedNFTs.map((nft) => (
                <NFTCard key={nft.tokenId.toString()} nft={nft} />
            ))}
        </div>
    );
}
