<script>
    import { onMount } from 'svelte';
    import { FontAwesomeIcon } from 'fontawesome-svelte';
    import { Principal } from '@dfinity/principal';
    import { canisters, userBalances, createCanisterActor } from '../store/store';
    import { auth, plugWallet } from '../store/auth';
    import { idlFactory as akitaIDL } from "../../declarations/AkitaDIP20/AkitaDIP20.did.js";
    import { idlFactory as goldenIDL } from "../../declarations/GoldenDIP20/GoldenDIP20.did.js";
    import { idlFactory as backendIDL} from "../../declarations/defi_dapp/defi_dapp.did.js";
    import { idlFactory as ledgerIDL} from "../../declarations/ledger/ledger.did.js";
    import { toHexString, hexToBytes, principalToAccountDefaultIdentifier } from '../utils/helpers'
    import { AuthClient } from '@dfinity/auth-client';
    import { HttpAgent } from '@dfinity/agent/lib/cjs/agent';
    import { Null } from '@dfinity/candid/lib/cjs/idl';

    // Global variables
    const host = process.env.DFX_NETWORK === "local"
          ? `http://localhost:8000`
          : "ic0.app";

    let depositAddressBlob;
    let iiPrincipal = '';
    let authType = "anonymous";

    // Actors for corresponding canisters
    // TDOD: Move to a store
    let backendActor;
    let akitaActor;
    let goldenActor;
    let ledgerActor;

    // UI Flags
    let withdrawing = false;
    let depositing = false;
    let didCopyDepositAddress = false;
    let didCopyPrincipal = false;
    let withdrawingAmount = false;
    let fetchingAddress = true;

    // UI Variables
    let currentToken;
    let withdrawAmount = 0;
    // TODO: Use an input box for this value, too
    let depositAmount = 100000;
    let withdrawAddress = '';

    // Subscribe to plug wallet value should a user authenticate with Plug Wallet
    plugWallet.subscribe(async (value) => {
        if(value.plugActor) {
            const pr = Principal.fromText($canisters[0].canisterId);
            const deposit = await value.plugActor.deposit(pr);
        }
    });

    onMount(async () => {
        // Use II as actor
        if($auth.loggedIn) {
            console.log("Using II for DEX actor");
            authType = "II";

            // II must display principle, since it is unique.
            iiPrincipal = $auth.principal;

            // TODO: When using II, display a note on how to deposit.
            // e.g.
            //
            // To transfer tokens, use the DIP canister to transfer tokens to <iiPrincipal>, and the balance will be reflected here.
            // To transfer ICP, use the ledger to transfer ICP to <depositAddress>, and the balance will be reflected here.
            //
            // This can replace the COPY we have at the bottom, as this is not needed when using Plug

            // Create canister actors
            const authClient = await AuthClient.create();
            const identity = authClient.getIdentity();
            const agent = new HttpAgent({identity, host});

            if(process.env.DFX_NETWORK === 'local')
                agent.fetchRootKey();

            backendActor = createCanisterActor(agent, backendIDL, process.env.DEFI_DAPP_CANISTER_ID);
            akitaActor = createCanisterActor(agent, akitaIDL, process.env.AKITADIP20_CANISTER_ID);
            goldenActor = createCanisterActor(agent, goldenIDL, process.env.GOLDENDIP20_CANISTER_ID);
            ledgerActor = createCanisterActor(agent, ledgerIDL, process.env.LEDGER_CANISTER_ID);

            // Fetch initial balances
            const goldenBalance = await goldenActor.balanceOf($auth.principal);
            const akitaBalance = await akitaActor.balanceOf($auth.principal);
            let ledgerBalance = 0;

            depositAddressBlob = await backendActor.getDepositAddress();
            const approved = await ledgerActor.account_balance({account: hexToBytes(principalToAccountDefaultIdentifier(iiPrincipal))});
            if(approved.e8s) {
                ledgerBalance = approved.e8s;
            }

            // Create a balances array and set the userBalance store object
            const balances = []
            console.log('Fetching all user balances');
            const allUserBalances = await backendActor.getBalances();
            console.log('User Balances: ', allUserBalances)
            for(let i = 0; i < $canisters.length; i++) {
                const principal = Principal.fromText($canisters[i].canisterId);
                let token;
                if(allUserBalances.length) {
                    token = allUserBalances.find((bal) => {
                        return bal.token.toString() === principal.toString()
                    });
                }

                const dexBalance = token ? token.amount : 0;

                balances.push({
                    name: $canisters[i].canisterName,
                    symbol: $canisters[i].symbol,
                    canisterBalance: i === 0 ? akitaBalance : i === 1 ? goldenBalance : ledgerBalance,
                    dexBalance: dexBalance,
                    principal: principal
                })
            };

            // Update the store values
            userBalances.set([...balances]);
            console.log('User Balances: ', $userBalances)
        }
        else if ($plugWallet.isConnected) {
            // TODO: Support Plug wallet
            // console.log("Using Plug for DEX actor");
            // authType = "Plug";
            // const principalId = await window.ic.plug.agent.getPrincipal();


            // // Fetch initial balances
            // const goldenBalance = await $plugWallet.plugGoldenActor.balanceOf(principalId);
            // const akitaBalance = await $plugWallet.plugAkitaActor.balanceOf(principalId);
            // let ledgerBalance = 0;
            

            // // When using Plug, the balance displayed should be of the Plug principal
            // const balance = await $plugWallet.plugLedgerActor.account_balance({account: XXX});
            // if(balance.e8s) {
            //     ledgerBalance = balance.e8s;
            // }

            // // Create a balances array and set the userBalance store object
            // const balances = []
            // for(let i = 0; i < $canisters.length; i++) {
            //     const principal = Principal.fromText($canisters[i].canisterId);
            //     const dexBalance = await $plugWallet.plugLedgerActor.getBalance(principal);

            //     balances.push({
            //         name: $canisters[i].canisterName,
            //         symbol: $canisters[i].symbol,
            //         canisterBalance: i === 0 ? akitaBalance : i === 1 ? goldenBalance : ledgerBalance,
            //         dexBalance: dexBalance,
            //         principal: principal
            //     })
            // };

            // // Update the store values
            // userBalances.set([...balances]);

            // // Don't forget to set `depositAddressBlob`, which we will use later
            // depositAddressBlob = await $plugWallet.plugLedgerActor.getDepositAddress();
        }

        fetchingAddress = false;
    });

    async function depositT(principal) {
        // explicitly set these here to prevent
        // withdraw form from showing
        withdrawing = false;
        withdrawAmount = 0;
        currentToken = undefined;
        // END withdraw

        depositing = true;
        currentToken = principal;

        const canister = $canisters.find((canister) => {
            return canister.canisterId === principal.toString();
        })
        if(canister && canister.canisterName === 'ICP') {
            if (authType === "Plug") {
                // TODO: Support Plug wallet
            	// await ledgerActor.transfer(...)
            }
            // transfer ICP correct subaccount on DEX
            await ledgerActor.transfer({
                        memo: BigInt(0x1),
                        amount: { e8s: depositAmount },
                        fee: { e8s: 10000},
                        to: depositAddressBlob,
                        from_subaccount: [],
                        created_at_time: [],
                });

            const result = await backendActor.deposit(principal);
            if(result.Ok) {
                const dexBalance = await backendActor.getBalance(principal);

                let ledgerBalance = 0;
                let response;
                if(authType === "II") {
                    // Update user ICP balance
                    response = await ledgerActor.account_balance({account: hexToBytes(principalToAccountDefaultIdentifier($auth.principal))});
                } else if (authType === "Plug") {
                    // TODO: Support Plug wallett
                    // response = await ledgerActor.account_balance({account: XXX});
                }
                if(response.e8s) {
                    ledgerBalance = response.e8s
                }
                setBalances(canister.canisterName, ledgerBalance, dexBalance);
            }
        }
        else if(canister && canister.canisterName === 'AkitaDIP20') {
            await akitaActor.approve(Principal.fromText(process.env.DEFI_DAPP_CANISTER_ID), depositAmount);

            const result = await backendActor.deposit(principal);
            if(result.Ok) {
                const dexBalance = await backendActor.getBalance(principal);
                const akitaBalance = await akitaActor.balanceOf($auth.principal);

                setBalances(canister.canisterName, akitaBalance, dexBalance);
            }
        }
        else if(canister && canister.canisterName === 'GoldenDIP20') {
            await goldenActor.approve(Principal.fromText(process.env.DEFI_DAPP_CANISTER_ID), depositAmount);

            const result = await backendActor.deposit(principal);
            if(result.Ok) {
                const dexBalance = await backendActor.getBalance(principal);
                const goldenBalance = await goldenActor.balanceOf($auth.principal);

                setBalances(canister.canisterName, goldenBalance, dexBalance);
            }
        }

        depositing = false;
        currentToken = undefined;
    }

    async function withdrawT(principal) {
        withdrawingAmount = true;
        currentToken = principal;
        const withdrawPrincipal = Principal.fromText(withdrawAddress);

        const canister = $canisters.find((canister) => {
            return canister.canisterId === principal.toString();
        })
        if(canister && canister.canisterName === 'ICP') {
            const result = await backendActor.withdraw(currentToken, withdrawAmount, withdrawPrincipal)
            if(result.Ok) {
                const dexBalance = await backendActor.getBalance(principal);
                let ledgerBalance = 0;
                let response;
                if(authType === "II") {
                    // When using II, display the balance in the target account
                    response = await ledgerActor.account_balance({account: hexToBytes(principalToAccountDefaultIdentifier($auth.principal))});
                } else if (authType === "Plug") {
                    // TODO: Support Plug wallet
                    // response = await ledgerActor.account_balance({account: XXX});
                }
                if(response.e8s) {
                    ledgerBalance = response.e8s
                }
                setBalances(canister.canisterName, ledgerBalance, dexBalance);
            }
        }
        else if(canister && canister.canisterName === 'AkitaDIP20') {
            const result = await backendActor.withdraw(currentToken, withdrawAmount, withdrawPrincipal)
            if(result.Ok) {
                const dexBalance = await backendActor.getBalance(principal);
                const akitaBalance = await akitaActor.balanceOf($auth.principal);

                setBalances(canister.canisterName, akitaBalance, dexBalance);
            }
        }
        else if(canister && canister.canisterName === 'GoldenDIP20') {
            const result = await backendActor.withdraw(currentToken, withdrawAmount, withdrawPrincipal)
            if(result.Ok) {
                const dexBalance = await backendActor.getBalance(principal);
                const goldenBalance = await goldenActor.balanceOf($auth.principal);

                setBalances(canister.canisterName, goldenBalance, dexBalance);
            }
        }

        withdrawAmount = 0;
        withdrawAddress = '';
        currentToken = undefined;
        withdrawingAmount = false;
    };

    function setBalances(canisterName, canisterBalance, dexBalance) {
        const balanceObj = $userBalances.find((b) => {
            return b.name === canisterName
        })
        if(balanceObj) {
            balanceObj.canisterBalance = canisterBalance;
            balanceObj.dexBalance = dexBalance;
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
        withdrawAddress = '';
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

    function copyPrincipal(text) {
        if(window.isSecureContext) {
            didCopyPrincipal = true;
            navigator.clipboard.writeText(text);
        }
        setTimeout(() => {
            didCopyPrincipal = false
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
                            </td>
                            <td>
                                <div>
                                    <button title="deposit" disabled={balance.canisterBalance <= 0 || balance.canisterBalance < depositAmount} on:click={() => depositT(balance.principal)}>
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
                                                    <div>
                                                        <input bind:value={withdrawAddress} style="width: 115px" type="text" class="input-style" />
                                                    </div>
                                                    <button disabled={withdrawAmount <= 0 || withdrawAddress === ''} on:click={() => withdrawT(balance.principal)} ><FontAwesomeIcon icon="check" /></button>
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
                    {toHexString(depositAddressBlob)}
                    <span class="copy-icon" on:click={() => copyDepositAddress(toHexString(depositAddressBlob))}>
                        <FontAwesomeIcon icon="copy" />
                        {#if didCopyDepositAddress}
                            Copied!
                        {/if}
                    </span>
                {/if}
            </span>
        </div>
        {#if iiPrincipal}
            <div class='principal'>
                II Principal:
                <span class="principal-value-text">
                    {iiPrincipal}
                    <span class="copy-icon" on:click={() => copyPrincipal(iiPrincipal)}>
                        <FontAwesomeIcon icon="copy" />
                        {#if didCopyPrincipal}
                            Copied!
                        {/if}
                    </span>
                </span>
            </div>
        {/if}
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

    .balance-table {
        display: block;
    }

    .deposit-address {
        display: block;
    }

    .principal {
        display: block;
    }

    .deposit-value-text {
        font-size: 13px;
        color: #a9a9a9
    }

    .principal-value-text {
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
