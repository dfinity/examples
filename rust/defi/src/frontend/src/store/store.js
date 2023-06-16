import { Actor } from '@dfinity/agent';
import { writable } from 'svelte/store';

export const canisters = writable(
    [
        {symbol: 'AKI', canisterName: 'AkitaDIP20', canisterId: process.env.AKITADIP20_CANISTER_ID},
        {symbol: 'GLD', canisterName: 'GoldenDIP20', canisterId: process.env.GOLDENDIP20_CANISTER_ID},
        {symbol: 'ICP', canisterName: 'ICP', canisterId: process.env.LEDGER_CANISTER_ID}
    ]
);

export const canisterActors = writable([]);

export const userBalances = writable([]);

export const createCanisterActor = (agent, idl, canisterId) => {
    return Actor.createActor(idl, {
        agent,
        canisterId
    })
}