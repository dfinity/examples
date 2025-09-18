import { useInternetIdentity } from "ic-use-internet-identity";
import { createActor, canisterId } from "declarations/backend";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";

const ACTOR_QUERY_KEY = "actor";
export function useActor() {
    const { identity } = useInternetIdentity();
    const queryClient = useQueryClient();

    const actorQuery = useQuery({
        queryKey: [
            ACTOR_QUERY_KEY,
            identity?.getPrincipal().toString() || "anonymous",
        ],
        queryFn: async () => {
            if (!canisterId) {
                throw new Error("Canister ID not available");
            }

            if (!identity) {
                // Create anonymous actor
                return createActor(canisterId);
            }

            // Create authenticated actor
            return createActor(canisterId, {
                agentOptions: {
                    identity,
                },
            });
        },
        staleTime: Infinity,
        enabled: true,
        retry: (failureCount, error) => {
            console.error("Actor creation failed:", error);
            return failureCount < 2; // Retry up to 2 times
        },
    });

    // Clear all dependent queries when identity changes
    useEffect(() => {
        queryClient.invalidateQueries({
            predicate: (query) => {
                return !query.queryKey.includes(ACTOR_QUERY_KEY);
            },
        });
    }, [identity?.getPrincipal().toString(), queryClient]);

    return {
        actor: actorQuery.data || null,
        isFetching: actorQuery.isFetching,
        isError: actorQuery.isError,
        error: actorQuery.error,
    };
}
