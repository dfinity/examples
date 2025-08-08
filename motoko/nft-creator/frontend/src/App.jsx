import { useInternetIdentity } from "ic-use-internet-identity";
import { CollectionClaim } from "./components/CollectionClaim";
import { MintNFT } from "./components/MintNFT";
import { OwnedNFTs } from "./components/OwnedNFTs";
import { AuthButton } from "./components/AuthButton";
import { Heart } from "lucide-react";

function App() {
    const { identity, isInitializing } = useInternetIdentity();

    if (isInitializing) {
        return (
            <div className="min-h-screen bg-gray-900 flex items-center justify-center">
                <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-purple-500"></div>
            </div>
        );
    }

    return (
        <div className="min-h-screen bg-gray-900 text-white">
            <div className="container mx-auto px-4 sm:px-6 lg:px-8 py-4 sm:py-8">
                {/* Header */}
                <div className="flex flex-col sm:flex-row sm:justify-between sm:items-center gap-4 mb-6 sm:mb-8">
                    <h1 className="text-2xl sm:text-3xl font-bold bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent text-center sm:text-left">
                        NFT Collection Manager
                    </h1>
                    <div className="flex justify-center sm:justify-end">
                        <AuthButton />
                    </div>
                </div>

                {identity ? (
                    <div className="space-y-6 sm:space-y-8">
                        {/* Collection Management */}
                        <div className="bg-gray-800 rounded-lg p-4 sm:p-6 border border-gray-700">
                            <h2 className="text-lg sm:text-xl font-semibold mb-3 sm:mb-4 text-purple-300">
                                Collection Management
                            </h2>
                            <CollectionClaim />
                        </div>

                        {/* Minting Section */}
                        <div className="bg-gray-800 rounded-lg p-4 sm:p-6 border border-gray-700">
                            <h2 className="text-lg sm:text-xl font-semibold mb-3 sm:mb-4 text-purple-300">
                                Mint NFT
                            </h2>
                            <MintNFT />
                        </div>

                        {/* Owned NFTs */}
                        <div className="bg-gray-800 rounded-lg p-4 sm:p-6 border border-gray-700">
                            <h2 className="text-lg sm:text-xl font-semibold mb-3 sm:mb-4 text-purple-300">
                                Your NFTs
                            </h2>
                            <OwnedNFTs />
                        </div>
                    </div>
                ) : (
                    <div className="text-center py-8 sm:py-16">
                        <div className="bg-gray-800 rounded-lg p-6 sm:p-8 max-w-sm sm:max-w-md mx-auto border border-gray-700">
                            <h2 className="text-xl sm:text-2xl font-semibold mb-3 sm:mb-4">
                                Welcome to NFT Collection Manager
                            </h2>
                            <p className="text-gray-400 mb-4 sm:mb-6 text-sm sm:text-base">
                                Please authenticate with Internet Identity to
                                manage your NFT collection.
                            </p>
                            <div className="flex justify-center">
                                <AuthButton />
                            </div>
                        </div>
                    </div>
                )}

                {/* Footer */}
                <footer className="mt-12 sm:mt-16 text-center text-gray-500 text-xs sm:text-sm px-4">
                    <div className="flex flex-col sm:flex-row items-center justify-center gap-1">
                        <span>
                            © 2025. Built with{" "}
                            <Heart className="inline w-4 h-4 text-red-500" />{" "}
                            using
                        </span>
                        <a
                            href="https://caffeine.ai"
                            target="_blank"
                            rel="noopener noreferrer"
                            className="text-purple-400 hover:text-purple-300 transition-colors"
                        >
                            caffeine.ai
                        </a>
                    </div>
                </footer>
            </div>
        </div>
    );
}

export default App;
