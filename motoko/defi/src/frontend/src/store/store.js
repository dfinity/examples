import { writable } from 'svelte/store';

export const canisters = writable(
    [
        {canisterName: 'AkitaDIP20', canisterId: process.env.AKITADIP20_CANISTER_ID},
        {canisterName: 'GoldenDIP20', canisterId: process.env.GOLDENDIP20_CANISTER_ID},
        {canisterName: 'ICP', canisterId: process.env.LEDGER_CANISTER_ID}
    ]
);

