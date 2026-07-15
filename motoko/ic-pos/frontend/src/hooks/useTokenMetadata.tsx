import { useQuery } from "@tanstack/react-query";
import { mapTokenMetadata } from "@icp-sdk/canisters/ledger/icrc";
import { useIcrcLedger } from "@/actors";

/** Fallback decimals used while the ledger metadata is loading. */
export const DEFAULT_DECIMALS = 8;

/**
 * Reads the ICRC-1 token symbol, decimals and fee from the ledger metadata
 * (`icrc1:symbol`, `icrc1:decimals`, `icrc1:fee`). Cached indefinitely since
 * token metadata does not change.
 */
export default function useTokenMetadata() {
  const ledger = useIcrcLedger();

  const query = useQuery({
    queryKey: ["token_metadata"],
    queryFn: async () => {
      const response = await ledger.metadata({ certified: false });
      const metadata = mapTokenMetadata(response);
      if (!metadata) {
        throw new Error("Could not read token metadata from the ledger.");
      }
      return {
        symbol: metadata.symbol,
        decimals: metadata.decimals,
        fee: metadata.fee,
      };
    },
    staleTime: Infinity,
  });

  return {
    ...query,
    // Prefer fetched values; fall back to safe defaults while loading.
    symbol: query.data?.symbol ?? "",
    decimals: query.data?.decimals ?? DEFAULT_DECIMALS,
    fee: query.data?.fee,
  };
}
