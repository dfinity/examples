<script>
    import { onMount } from 'svelte';
    import { FontAwesomeIcon } from 'fontawesome-svelte';
    import { Principal } from '@dfinity/principal';
    import { canisters } from '../store/store';
    import { auth, plugWallet } from '../store/auth';
    import { idlFactory as akitaIDL } from "../../../declarations/AkitaDIP20/AkitaDIP20.did.js";
    import { idlFactory as goldenIDL } from "../../../declarations/GoldenDIP20/GoldenDIP20.did.js";
    import { idlFactory as backendIDL} from '../../../declarations/defi_dapp/defi_dapp.did.js';
    import { idlFactory as ledgerIDL} from '../../../declarations/ledger/ledger.did.js';
    import { Actor } from '@dfinity/agent';
    import { AuthClient } from '@dfinity/auth-client';
    import { HttpAgent } from '@dfinity/agent/lib/cjs/agent';


    let backendActor;
    let accountAddressBlob;
    let depositAddress = '';
    let didCopy = false;
    let balances = [];
    let currentToken;
    let depositing = false;
    let withdrawing = false;
    let withdrawingAmount = false;
    let withdrawAmount = 0;
    let fetchingAddress = true;

    let iiPrincipal;
    let akitaActor;
    let goldenActor;
    let ledgerActor;

    plugWallet.subscribe(async (value) => {
        if(value.plugActor) {
            const pr = Principal.fromText($canisters[0].canisterId);
            const deposit = await value.plugActor.deposit(pr);
            console.log(deposit)
        }
    })

    onMount(async () => {
        // Use II as actor
        if($auth.loggedIn) {
            console.log("Using II for DEX actor");
            // backendActor = $auth.actor;

            const authClient = await AuthClient.create();
            const identity = authClient.getIdentity();
            const agent = new HttpAgent({identity, host: "http:localhost:8000"});
            agent.fetchRootKey();
            backendActor = Actor.createActor(backendIDL, {
                agent,
                canisterId: process.env.DEFI_DAPP_CANISTER_ID
            });
            akitaActor = Actor.createActor(akitaIDL, {
                agent,
                canisterId: process.env.AKITADIP20_CANISTER_ID
            });
            // goldenActor = Actor.createActor(goldenIDL, {
            //     agent,
            //     canister: process.env.GOLDENDIP20_CANISTER_ID
            // })
            ledgerActor = Actor.createActor(ledgerIDL, {
                agent,
                canisterId: process.env.LEDGER_CANISTER_ID
            })
            iiPrincipal = authClient.getIdentity().getPrincipal();
            console.log(iiPrincipal)
            const goldenBalance = 0; // await goldenActor.balanceOf(iiPrincipal);
            const akitaBalance = 0; //await akitaActor.balanceOf(iiPrincipal);
            const depositAddress = await backendActor.getDepositAddress();
            const param = {
                account: depositAddress
            }
            const response = await ledgerActor.account_balance(param);
            let ledgerBalance = 0;
            if(response.e8s) {
                ledgerBalance = response.e8s;
            }
            console.log(`Ledger Balance: `, ledgerBalance);
            console.log(akitaBalance);
            // console.log(goldenBalance)

            for(let i = 0; i < $canisters.length; i++) {
                const principal = Principal.fromText($canisters[i].canisterId);
                const dexBalance = await backendActor.getBalance(principal);
    
                balances.push({
                    name: $canisters[i].canisterName,
                    symbol: $canisters[i].symbol,
                    canisterBalance: i === 0 ? akitaBalance : i === 1 ? goldenBalance : ledgerBalance,
                    dexBalance: dexBalance,
                    principal: principal
                })
            }
            // svelte hack to update UI
            balances = [...balances];
            console.log(balances)
        }
        else if ($plugWallet.isConnected) {
            console.log("Using Plug for DEX actor");
            backendActor = $plugWallet.plugActor;
        }

        accountAddressBlob = await backendActor.depositAddress();
        depositAddress = toHexString(accountAddressBlob);
        fetchingAddress = false;
    });

    function toHexString(byteArray) {
        return Array.from(byteArray, function(byte) {
            return ('0' + (byte & 0xFF).toString(16)).slice(-2);
        }).join('').toUpperCase();
    };

    function hexToBytes(hex) {
        for (var bytes = [], c = 0; c < hex.length; c += 2)
            bytes.push(parseInt(hex.substr(c, 2), 16));
        return bytes;
    };

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
        const akitaBalance = await akitaActor.balanceOf(iiPrincipal);
        console.log(akitaBalance)
        const result = await backendActor.deposit(Principal.fromText(process.env.AKITADIP20_CANISTER_ID));
        depositing = false;
        currentToken = undefined;
        console.log(`Result: ${JSON.stringify(result)}`);
    }

    async function withdrawT(principal) {
        withdrawingAmount = true;
        currentToken = principal;
        const result = await backendActor.withdraw(principal, withdrawAmount)
        currentToken = undefined;
        withdrawingAmount = false;
        console.log(`Result: `, result)
        // If withdraw successful, fetch the new balance
        if(result.Ok) {
            const canister = $canisters.find((canister) => {
                return canister.canisterId === principal.toString();
            })
            if(canister && canister.canisterName === 'ICP') {
                const param = {
                    account: hexToBytes(depositAddress)
                }
                const dexBalance = await backendActor.getBalance(principal);
                let ledgerBalance = 0;
                const response = await ledgerActor.account_balance(param);
                console.log(response)
                if(response.e8s) {
                    ledgerBalance = response.e8s
                }
                setBalances(canister.canisterName, ledgerBalance, dexBalance);
                
            }
            else if(canister && canister.canisterName === 'AkitaDIP20') {
                const dexBalance = await backendActor.getBalance(principal);
                const akitaBalance = await akitaActor.getBalance(principal);

                setBalances(canister.canisterName, akitaBalance, dexBalance);
            }
            else if(canister && canister.canisterName === 'GoldenDIP20') {
                const dexBalance = await backendActor.getBalance(principal);
                const goldenBalance = await goldenActor.getBalance(principal);

                setBalances(canister.canisterName, goldenBalance, dexBalance);
            }
            
        }
        withdrawAmount = 0;
    };

    function setBalances(canisterName, canisterBalance, dexBalance) {
        const balanceObj = balances.find((b) => {
            return b.name === canisterName
        })
        if(balanceObj) {
            balanceObj.canisterBalance = canisterBalance;
            balanceObj.dexBalance = dexBalance;
            console.log(balanceObj)
        }
        balances = [...balances];    
    }

    function beginWithdrawProcess(token) {
        currentToken = token;
        withdrawing = true;
    }

    function cancelWithdrawProcess(e) {
        e.stopPropagation();
        withdrawing = false;
        withdrawAmount = 0;
        currentToken = undefined;
    }

    async function approveDIP20() {
        const result = await akitaActor.approve(Principal.fromText(process.env.DEFI_DAPP_CANISTER_ID), 10000000);
        console.log(result)
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
                    <th>Canister Balance</th>
                    <th></th>
                    <th>DEX Balance</th>
                </thead>
                <tbody>
                    {#each balances as balance}
                        <tr>
                            <td>
                                {balance.symbol}
                            </td>
                            <td>
                                {balance && balance.canisterBalance ? balance.canisterBalance.toLocaleString() : '0'}
                            </td>
                            <td>
                                <div>
                                    <button title="deposit" disabled={balance.canisterBalance <= 0} on:click={() => depositT(balance.principal)}>
                                        <div class="add-btn-text">
                                            {#if depositing && currentToken === balance.principal}
                                                <div class="loader"></div>
                                            {:else}
                                                <FontAwesomeIcon icon="arrow-right" />
                                            {/if}
                                        </div>
                                    </button>
                                    <button  title="Withdraw" disabled={balance.dexBalance <= 0} on:click={() => beginWithdrawProcess(balance.principal)} >
                                        <div class="add-btn-text">
                                            {#if withdrawingAmount && currentToken === balance.principal}
                                                <div class="loader"></div>
                                            {:else}
                                            {#if withdrawing && currentToken === balance.principal}
                                                    <div>
                                                        <input bind:value={withdrawAmount} style="width: 115px" type="number" class="input-style" />
                                                    </div>
                                                    <button disabled={withdrawAmount <= 0} on:click={() => withdrawT(balance.principal)} ><FontAwesomeIcon icon="check" /></button>
                                                    <button on:click={(e) => (cancelWithdrawProcess(e))}><FontAwesomeIcon icon="times" /></button>
                                                {:else}
                                                    <FontAwesomeIcon icon="arrow-left" />
                                                {/if}
                                            {/if}
                                        </div>
                                    </button>
                                </div>
                            </td>
                            <td>
                                {balance && balance.dexBalance ? balance.dexBalance.toLocaleString() : '0'}
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div> 
        <div class='deposit-address'>
            Deposit Address:
            <span class="deposit-value-text">
                {#if fetchingAddress}
                    <div class="loader"></div>
                {:else}
                    {depositAddress}
                {/if}
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
