<script>
    import { onMount } from 'svelte';
    import { Actor, HttpAgent } from "@dfinity/agent";
    import { AuthClient } from "@dfinity/auth-client";
    import { idlFactory } from '../../../declarations/defi_dapp/defi_dapp.did.js';
    import { FontAwesomeIcon } from 'fontawesome-svelte';
    import { Principal } from "@dfinity/principal";

    let backendActor;
    let authClient;

    let balance;

    onMount(async () => {
        authClient = await AuthClient.create();
        const authenticated = await authClient.isAuthenticated();
        if(authenticated) {
            const identity = authClient.getIdentity();
            const agent = new HttpAgent({identity, host: "http://localhost:8000"});
            // development only, comment out in prod
            agent.fetchRootKey();
            // end comment out in prod only
            backendActor = Actor.createActor(idlFactory, {
                agent,
                canisterId: process.env.DEFI_DAPP_CANISTER_ID
            });
            const princiapl = Principal.fromText(process.env.LEDGER_CANISTER_ID)
            const deposit = await backendActor.deposit(princiapl)
            console.log(deposit)
            balance = await backendActor.balance(princiapl);
            console.log(balance)
        }
    });

    async function deposit() {
        console.log(' I am depositing....')
    };

    async function withdraw() {
        console.log('I am withdrawing...')
    };


</script>

<div class="info-container">
    <div>
        <div class="header-container">
            <div class="title">
                <h2>Balance: </h2>
            </div>
            <div class="balance-value-container">
                <div class="balance-text">
                    <h2>
                        {balance}
                        <button on:click={deposit}>
                            <FontAwesomeIcon icon="plus" />
                        </button>
                        <button on:click={withdraw}>
                            <FontAwesomeIcon icon="minus" />
                        </button>
                    </h2>
                    <span>
                    </span>
                </div>
            </div>
        </div> 
    </div>

</div>

<style>
    .info-container {
        text-align: left !important;
    }

    .header-container {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
    }

    .title {
        display: inline;
    }

    .balance-value-container {
       margin-left: 10px;
    }
</style>
