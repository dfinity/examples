<script>
    import { onMount } from 'svelte';
    import { FontAwesomeIcon } from 'fontawesome-svelte';
    import { Principal } from '@dfinity/principal';
    import { canisters, userBalances } from '../store/store';
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
    let withdrawalAddress = '';
    let withdrawalBlob = '';
    let didCopyDepositAddress = false;
    let didCopyWithdrawalAddress = false;
    let depositMin = 100000;
    let currentToken;
    let depositing = false;
    let withdrawing = false;
    let withdrawingAmount = false;
    let withdrawAmount = 0;
    let fetchingAddress = true;
    let depositBlob;
    let authClient;

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
            authClient = await AuthClient.create();
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
            goldenActor = Actor.createActor(goldenIDL, {
                agent,
                canisterId: process.env.GOLDENDIP20_CANISTER_ID
            })
            ledgerActor = Actor.createActor(ledgerIDL, {
                agent,
                canisterId: process.env.LEDGER_CANISTER_ID
            })
            iiPrincipal = authClient.getIdentity().getPrincipal();
            console.log(iiPrincipal)
            const goldenBalance = await goldenActor.balanceOf(iiPrincipal);
            const akitaBalance = await akitaActor.balanceOf(iiPrincipal);
            depositBlob = await backendActor.getDepositAddress();
            withdrawalBlob = await backendActor.withdrawalAddress();
            withdrawalAddress = toHexString(withdrawalBlob);
            const approved = await ledgerActor.account_balance({account: depositBlob});
            const available = await ledgerActor.account_balance({account: withdrawalBlob});
            let ledgerBalance = 0;
            let approvedLedgerBalance = 0;

            if(approved.e8s) {
                ledgerBalance = approved.e8s;
            }
            if(available.e8s) {
                approvedLedgerBalance = available.e8s         
            }

            const balances = []
            for(let i = 0; i < $canisters.length; i++) {
                const principal = Principal.fromText($canisters[i].canisterId);
                const dexBalance = await backendActor.getBalance(principal);
    
                balances.push({
                    name: $canisters[i].canisterName,
                    symbol: $canisters[i].symbol,
                    canisterBalance: i === 0 ? akitaBalance : i === 1 ? goldenBalance : ledgerBalance,
                    available: $canisters[i].symbol === 'ICP' ? approvedLedgerBalance : undefined,
                    dexBalance: dexBalance,
                    principal: principal
                })
            }
            // svelte hack to update UI
            userBalances.set([...balances])
        }
        else if ($plugWallet.isConnected) {
            console.log("Using Plug for DEX actor");
            backendActor = $plugWallet.plugActor;
        }

        accountAddressBlob = await backendActor.getDepositAddress();
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

    async function depositT(principal) {
        // explicitly set these here to prevent
        // withdraw form from showing
        withdrawing = false;
        withdrawAmount = 0;
        currentToken = undefined;
        // END withdraw
        depositing = true;
        currentToken = principal;
        await akitaActor.approve(Principal.fromText(process.env.DEFI_DAPP_CANISTER_ID), depositMin);
        await goldenActor.approve(Principal.fromText(process.env.DEFI_DAPP_CANISTER_ID), depositMin);
        const result = await backendActor.deposit(principal);
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
                let availableLedgerBalance = 0;
                const availableResponse = await ledgerActor.account_balance({account: withdrawalBlob});
                const response = await ledgerActor.account_balance(param);
                console.log(response)
                if(response.e8s) {
                    ledgerBalance = response.e8s
                }
                if(availableResponse.e8s) {
                    availableLedgerBalance = availableResponse.e8s
                }
                setBalances(canister.canisterName, ledgerBalance, dexBalance, availableLedgerBalance);
            }
            else if(canister && canister.canisterName === 'AkitaDIP20') {
                const dexBalance = await backendActor.getBalance(principal);
                const akitaBalance = await akitaActor.balanceOf(iiPrincipal);
                setBalances(canister.canisterName, akitaBalance, dexBalance);
            }
            else if(canister && canister.canisterName === 'GoldenDIP20') {
                const dexBalance = await backendActor.getBalance(principal);
                const goldenBalance = await goldenActor.balanceOf(iiPrincipal);
                setBalances(canister.canisterName, goldenBalance, dexBalance);
            } 
        }
        depositing = false;
        currentToken = undefined;
        // console.log(`Result: ${JSON.stringify(result)}`);
    }

    async function withdrawT(principal) {
        withdrawingAmount = true;
        currentToken = principal;
        const result = await backendActor.withdraw(currentToken, withdrawAmount)
        currentToken = undefined;
        withdrawingAmount = false;
        console.log(`Result: `, result)
        // If withdraw successful, fetch the new balance
        if(result.Ok) {
            const canister = $canisters.find((canister) => {
                return canister.canisterId === principal.toString();
            })
            if(canister && canister.canisterName === 'ICP') {
                const dexBalance = await backendActor.getBalance(principal);
                let ledgerBalance = 0;
                let availableLedgerBalance = 0;
                const availableResponse = await ledgerActor.account_balance({account: withdrawalBlob});
                const response = await ledgerActor.account_balance({account: depositBlob});
                console.log(response)
                if(response.e8s) {
                    ledgerBalance = response.e8s
                }
                if(availableResponse.e8s) {
                    availableLedgerBalance = availableResponse.e8s
                }
                setBalances(canister.canisterName, ledgerBalance, dexBalance, availableLedgerBalance);
                
            }
            else if(canister && canister.canisterName === 'AkitaDIP20') {
                const dexBalance = await backendActor.getBalance(principal);
                const akitaBalance = await akitaActor.balanceOf(iiPrincipal);

                setBalances(canister.canisterName, akitaBalance, dexBalance);
            }
            else if(canister && canister.canisterName === 'GoldenDIP20') {
                const dexBalance = await backendActor.getBalance(principal);
                const goldenBalance = await goldenActor.balanceOf(iiPrincipal);

                setBalances(canister.canisterName, goldenBalance, dexBalance);
            }            
        }
        withdrawAmount = 0;
    };

    function setBalances(canisterName, canisterBalance, dexBalance, availableLedgerBalance) {
        const balanceObj = $userBalances.find((b) => {
            return b.name === canisterName
        })
        if(balanceObj) {
            balanceObj.canisterBalance = canisterBalance;
            balanceObj.dexBalance = dexBalance;
            balanceObj.available = availableLedgerBalance ?? ''
        }
        userBalances.set([...$userBalances]);    
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

    function copyDepositAddress(text) {
        if(window.isSecureContext) {
            didCopyDepositAddress = true;
            navigator.clipboard.writeText(text);
        }
        setTimeout(() => {
            didCopyDepositAddress = false
        }, 1000)
    };

    function copyWithdrawalAddress(text) {
        if(window.isSecureContext) {
            didCopyWithdrawalAddress = true;
            navigator.clipboard.writeText(text);
        }
        setTimeout(() => {
            didCopyWithdrawalAddress = false
        }, 1000)
    };
</script>

<div class="info-container">
    <div>
        <div class="balance-container">
            <div class="title">
                <div>
                    <h2>My Balances</h2>
                </div>
            </div>
            <table class='balance-table'>
                <thead>
                    <th>Token</th>
                    <th>Canister Balance</th>
                    <th></th>
                    <th>DEX Balance</th>
                </thead>
                <tbody>
                    {#each $userBalances as balance}
                        <tr>
                            <td>
                                {balance.symbol}
                            </td>
                            <td>
                                {balance && balance.canisterBalance ? balance.canisterBalance.toLocaleString() : '0'}
                                {balance && balance.available ? ` / ${balance.available.toLocaleString()}` : ''}
                            </td>
                            <td>
                                <div>
                                    <button title="deposit" disabled={balance.canisterBalance <= 0 || balance.canisterBalance < depositMin} on:click={() => depositT(balance.principal)}>
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
                    <span class="copy-icon" on:click={() => copyDepositAddress(depositAddress)}>
                        <FontAwesomeIcon icon="copy" />
                        {#if didCopyDepositAddress}
                            Copied!
                        {/if}
                    </span>
                {/if}
            </span>
        </div>
        <div class='deposit-address'>
            My Principal:
            <span class="deposit-value-text">
                {#if !withdrawalAddress}
                    <div class="loader"></div>
                {:else}
                    {iiPrincipal.toString()}
                    <span class="copy-icon" on:click={() => copyWithdrawalAddress(iiPrincipal.toString())}>
                        <FontAwesomeIcon icon="copy" />
                        {#if didCopyWithdrawalAddress}
                            Copied!
                        {/if}
                    </span>
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
        display: flex;
    }

    .principal-text {
        margin-top: 28px;
        font-size: 14px;
        color: #a9a9a9;
        margin-left: 10px;
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
