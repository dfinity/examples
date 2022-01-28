<script>
    import { onMount } from 'svelte';
    import { Actor, HttpAgent } from "@dfinity/agent";
    import { AuthClient } from "@dfinity/auth-client";
    import { idlFactory } from '../../../declarations/defi_dapp/defi_dapp.did.js';
    import { FontAwesomeIcon } from 'fontawesome-svelte';
    import { Principal } from '@dfinity/principal';
    import { canisters } from '../store/store';
    import { plugWallet } from '../store/auth';

    let backendActor;
    let authClient;
    let depositAddress = '';
    let didCopy = false;
    let balances = [];
    let currentToken;
    let depositing = false;
    let withdrawing = false;

    plugWallet.subscribe(async (value) => {
        if(value.plugActor) {
            const pr = Principal.fromText($canisters[0].canisterId);
            const deposit = await value.plugActor.deposit(pr);
            console.log(deposit)
        }
    })

    onMount(async () => {
        authClient = await AuthClient.create();
        const authenticated = await authClient.isAuthenticated();
        let identity;
        if(authenticated)
            identity = authClient.getIdentity();

        console.log($plugWallet)
        const agent = new HttpAgent({identity, host: "http://localhost:8000"});
        // development only, comment out in prod
        agent.fetchRootKey();
        // end comment out in prod only
        backendActor = Actor.createActor(idlFactory, {
            agent,
            canisterId: process.env.DEFI_DAPP_CANISTER_ID
        });

        for(let i = 0; i < $canisters.length; i++) {
            const principal = Principal.fromText($canisters[i].canisterId);
            const balance = await backendActor.balance(principal);
            balances.push({
                name: $canisters[i].canisterName,
                symbol: $canisters[i].symbol,
                balance: balance,
                principal: principal
            })
        }
        
        // svelte hack to update UI
        balances = [...balances];
        console.log(balances)

        const blob = await backendActor.deposit_address();
        depositAddress = toHexString(blob);

        const principal = Principal.fromText($canisters[2].canisterId);
        const deposit = await backendActor.deposit(principal);
        if(deposit.Ok) {
            balances[2].balance = deposit.Ok;
        }

        const pr = Principal.fromText($canisters[0].canisterId);
        const akiDep = await backendActor.balance(pr);
        if(akiDep.Ok) {
            balances[0].balance = deposit.Ok;
        }
    });

    function toHexString(byteArray) {
        return Array.from(byteArray, function(byte) {
            return ('0' + (byte & 0xFF).toString(16)).slice(-2);
        }).join('').toUpperCase();
    }

    async function deposit() {
        const principal = Principal.fromText(process.env.LEDGER_CANISTER_ID);
        const result = await backendActor.deposit(principal);
        
        if(result.Err) {
            actionError = 'Error - Low funds, please deposit valid amount'
        }
    };

    async function depositT(principal) {
        depositing = true;
        currentToken = principal;
        const result = await backendActor.deposit(principal);
        depositing = false;
        currentToken = undefined;
        console.log(`Result: ${JSON.stringify(result)}`);
    }

    async function withdrawT(principal) {
        withdrawing = true;
        currentToken = principal;
        const result = await backendActor.withdraw(principal, 0)
        currentToken = undefined;
        withdrawing = false;
        console.log(`Result: ${JSON.stringify(result)}`)
    };

    function copyText(text) {
        if(window.isSecureContext) {
            didCopy = true;
            navigator.clipboard.writeText(text);
        }
        setTimeout(() => {
            didCopy = false
        }, 1000)
    };
</script>

<div class="info-container">
    <div>
        <div class="balance-container">
            <div class="title">
                <h2>My Balances</h2>
            </div>
            <table class='balance-table'>
                <thead>
                    <th>Token</th>
                    <th>Balance</th>
                    <th></th>
                </thead>
                <tbody>
                    {#each balances as balance}
                        <tr>
                            <td>{balance.symbol}</td>
                            <td>{balance.balance.toLocaleString()}</td>
                            <td>
                                <div>
                                    <button title="deposit" disabled={balance.balance <= 0} on:click={() => depositT(balance.principal)}>
                                        <div class="add-btn-text">
                                            {#if balance.symbol === 'ICP' && depositing}
                                                <div class="loader"></div>
                                            {:else}
                                                <FontAwesomeIcon icon="plus" />
                                            {/if}
                                        </div>
                                    </button>
                                    <button  title="Withdraw" on:click={() => withdrawT(balance.principal)}>
                                        <div class="add-btn-text">
                                            {#if withdrawing && currentToken === balance.principal}
                                                <div class="loader"></div>
                                            {:else}
                                                <FontAwesomeIcon icon="minus" />
                                            {/if}
                                        </div>
                                    </button>
                                </div>
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
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
    table {
        width: 100%;
        margin-bottom: 10px;
    }
    th {
        width: 50vw;
        font-weight: bold;
        font-size: 20px;
    }
    .info-container {
        text-align: left !important;
        background-color: #333336;
        padding: 10px;
        border-radius: 10px;
        margin-bottom: 16px;
    }

    .balance-container {
        margin-top: 1px;
    }

    .title {
        display: block;
    }

    .action-form {
        margin-top: 8px;
    }

    .error-text {
        font-weight: bold;
        font-style: italic;
        color:#FB8688;
    }

    .balance-table {
        display: block;
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
