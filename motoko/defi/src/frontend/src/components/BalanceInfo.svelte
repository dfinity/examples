<script>
    import { onMount } from 'svelte';
    import { Actor, HttpAgent } from "@dfinity/agent";
    import { AuthClient } from "@dfinity/auth-client";
    import { idlFactory } from '../../../declarations/defi_dapp/defi_dapp.did.js';
    import { FontAwesomeIcon } from 'fontawesome-svelte';
    import { Principal } from '@dfinity/principal';
import { canisters } from '../store/store.js';

    let backendActor;
    let authClient;
    let depositAddress = '';
    let didCopy = false;

    let balance = 0;

    onMount(async () => {
        authClient = await AuthClient.create();
        const authenticated = await authClient.isAuthenticated();
        let identity;
        if(authenticated)
            identity = authClient.getIdentity();
        const agent = new HttpAgent({identity, host: "http://localhost:8000"});
        // development only, comment out in prod
        agent.fetchRootKey();
        // end comment out in prod only
        backendActor = Actor.createActor(idlFactory, {
            agent,
            canisterId: process.env.DEFI_DAPP_CANISTER_ID
        });
        const icpToken = Principal.fromText(process.env.LEDGER_CANISTER_ID)
        balance = await backendActor.balance(icpToken);
        console.log(balance)
        const blob = await backendActor.deposit_address();
        blob.forEach((i) => {
            depositAddress += i.toString();
        });
        console.log(depositAddress)

    });

    async function deposit() {
        const result = await backendActor.deposit();
    };

    async function withdraw() {
        console.log('I am withdrawing...')
    };

    function copyText(text) {
        if(window.isSecureContext) {
            didCopy = true;
            navigator.clipboard.writeText(text);
        }
        setTimeout(() => {
            didCopy = false
        }, 1000)
    }

</script>

<div class="info-container">
    <div>
        <button on:click={deposit}>
            <FontAwesomeIcon icon="plus" />
        </button>
        <button on:click={withdraw}>
            <FontAwesomeIcon icon="minus" />
        </button>
        <div class="balance-container">
            <div class="title">
                <h2>Balance: </h2>
            </div>
            <div class="balance-value-container">
                <div class="balance-text">
                    <h2>
                        {balance}            
                    </h2>
                </div>
            </div>
        </div> 
        <div class='deposit-address'>
            Deposit Address:
            <span class="deposit-value-text">
                {depositAddress}
            </span>
            <span class="copy-icon" on:click={() => copyText(depositAddress)}>
                <FontAwesomeIcon icon="copy" />
                {#if didCopy}
                    Copied!
                {/if}
            </span>
        </div>
    </div>

</div>

<style>
    .info-container {
        text-align: left !important;
        background-color: #333336;
        padding: 10px;
        border-radius: 10px;
        margin-bottom: 16px;
    }

    .balance-container {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
    }

    .title {
        display: inline;
    }

    .balance-value-container {
       margin-left: 10px;
       display: flex;
       flex-direction: row;
       flex-wrap: wrap;
    }

    .deposit-address {
        display: block;
    }

    .deposit-value-text {
        font-size: 13px;
        color: #a9a9a9
    }

    .copy-icon {
        color: #a9a9a9;
        cursor: pointer;
    }
    .copy-icon:hover {
        color: #d4d4d4;
    }
</style>
