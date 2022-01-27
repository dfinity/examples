<script>
    import { onMount } from 'svelte';
    import { Actor, HttpAgent } from "@dfinity/agent";
    import { AuthClient } from "@dfinity/auth-client";
    import { idlFactory } from '../../../declarations/defi_dapp/defi_dapp.did.js';
    import { Principal } from '@dfinity/principal';
    import { orders, currentOrder } from '../store/order';
    import { canisters } from '../store/store';
    import { FontAwesomeIcon } from 'fontawesome-svelte';
 

    let backendActor;
    let authClient;
    
    let showOrderForm = false;
    let addingOrder = false;
    let cancelingOrder = false;
    let buyingOrder = false;

    const newOrder = {
        fromCanister: "",
        fromAmount: 0,
        toCanister: "",
        toAmount: 0
    };

    onMount(async () => {
        authClient = await AuthClient.create();
        const authenticated = await authClient.isAuthenticated();
        let identity;
        if(authenticated)
            identity = authClient.getIdentity() || undefined;

            const host = process.env.NODE_ENV === 'production' ? 'ic0.app' : 'http://localhost:8000';
            const agent = new HttpAgent({identity, host});
            // development only, comment out in prod
            agent.fetchRootKey();
            // end comment out in prod only
            backendActor = Actor.createActor(idlFactory, {
                agent,
                canisterId: process.env.DEFI_DAPP_CANISTER_ID
            });
        const orderList = await backendActor.list_order();
        orders.set([]);
        orders.set(orderList.reverse());
	});

    async function placeOrder() {
        addingOrder = true;
        const result = await backendActor.place_order(
            Principal.fromText(newOrder.fromCanister),
            newOrder.fromAmount,
            Principal.fromText(newOrder.toCanister),
            newOrder.toAmount);

        if(result.Ok) {
            const orderList = await backendActor.list_order();
            orders.set([]);
            orders.set(orderList.reverse());
        }
        closeOrderForm();
    };

    async function buyOrder(order) {
        buyingOrder = true;
        // Create an order opposite of the one being bought
        currentOrder.set(order);
        const newOrder = {
            fromCanister: order.to,
            fromAmount: order.toAmount,
            toCanister: order.from,
            toAmount: order.fromAmount
        };
        const result = await backendActor.place_order(
            newOrder.fromCanister,
            newOrder.fromAmount,
            newOrder.toCanister,
            newOrder.toAmount
        )

        if(result && result.Ok) {
            const orderList = await backendActor.list_order();
            orders.set([]);
            orders.set(orderList.reverse());
        }
        currentOrder.set({});
        buyingOrder = false;
    };

    function closeOrderForm() {
        showOrderForm = false;
        addingOrder = false;
    };

    async function cancelOrder(id) {
        cancelingOrder = true;
        const order = $orders.find((o) => o.id === id);
        currentOrder.set(order);
        const result = await backendActor.cancel_order(id);
        if(result && result.Ok) {
            const orderList = await backendActor.list_order();
            orders.set([]);
            orders.set(orderList.reverse()); 
        }
        currentOrder.set({});
        cancelingOrder = false;
    };

    async function getTokenSymbol(principal) {
        // Populate token symbols
        try {
            const symbol = await backendActor.symbol(principal);
            return symbol;
        }
        catch {
            return 'ICP';
        }
    };
</script>

<div class="order-container">
    <div>
        <div class="header-container">
            <div class="title">
                <h2>Orders</h2>
            </div>
            <div class="adding-order-btn">
                <button on:click={() => showOrderForm = true } title="Add new order" class="add-btn">
                    <FontAwesomeIcon icon="plus" />
                </button>
            </div>
        </div>
        <div>
            <table>
                {#if $orders.length}
                    <thead>
                        <th>From Account</th>
                        <th>Amount</th>
                        <th>To Account</th>
                        <th>Amount</th>
                        <th></th>
                    </thead>
                {/if}
                <tbody>
                    {#if showOrderForm}
                    <tr>
                        <td>
                            <select class="input-style" bind:value={newOrder.fromCanister}>
                                {#each $canisters as canister}
                                    <option value={canister.canisterId}>
                                        {canister.canisterName}
                                    </option>
                                {/each}
                            </select>
                        </td>
                        <td><input class="input-style" bind:value={newOrder.fromAmount} type="number" /></td>
                        <td>
                            <select class="input-style" bind:value={newOrder.toCanister}>
                                {#each $canisters as canister}
                                    <option value={canister.canisterId}>
                                        {canister.canisterName}
                                    </option>
                                {/each}
                            </select>
                        </td>                       
                        <td><input class="input-style" bind:value={newOrder.toAmount} type="number" /></td>
                        <td>
                            <div>
                                <button class="btn-accept" on:click={placeOrder} title="Place Order" >
                                    <div class="add-btn-text">
                                        {#if addingOrder}
                                            <div class="loader"></div>
                                        {:else}
                                            <FontAwesomeIcon icon="check" />
                                        {/if}
                                    </div>
                                </button>
                                <button class="btn-cancel" on:click={closeOrderForm} title="Cancel" >
                                    <FontAwesomeIcon icon="times" />
                                </button>
                            </div>
                        </td>
                    </tr>
                    {/if}
                    {#each $orders as order}
                    <tr>
                        <td>
                            {#await getTokenSymbol(order.from)}
                            <span>Loading Symbol...</span>
                            {:then symbol}
                            {symbol}
                            {/await}
                        </td>
                        <td>{order.fromAmount}</td>
                        <td>
                            {#await getTokenSymbol(order.to)}
                                <span>Loading Symbol...</span>
                            {:then symbol}
                                {symbol}
                            {/await}
                        </td>
                        <td>{order.toAmount}</td>
                        <td>
                            <button class="btn-accept">
                                {#if buyingOrder && $currentOrder.id === order.id}
                                <div class="loader buy-btn-loader"></div>
                                {:else}
                                    <div class="buy-btn-text" on:click={() => buyOrder(order)}>
                                        <FontAwesomeIcon icon="check" />
                                    </div>
                                {/if}
                            </button>
                            {#if order.owner.toText() === authClient.getIdentity().getPrincipal().toText()}
                                <button class="btn-cancel" on:click={() => cancelOrder(order.id)}>
                                    {#if cancelingOrder && $currentOrder.id === order.id}
                                        <div class="loader cancel-btn-loader"></div>
                                    {:else}
                                        <div class="cancel-btn-text">
                                            <FontAwesomeIcon icon="times" />
                                        </div>
                                    {/if}
                                </button>
                            {/if}
                        </td>
                    </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    </div>
</div>

<style>
    table {
        width: 100%;
    }

    th {
        font-weight: bold;
        font-size: 20px;
    }
    .order-container {
        text-align: left !important;
        margin-bottom: 36px;
        background-color: #333336;
        padding: 10px;
        border-radius: 10px;
    }

    .header-container {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
    }

    .title {
        display: inline;
    }

    .adding-order-btn {
       margin-left: 10px;
    }

    .add-btn {
        margin-top:22px;
        border-radius: 5px;
        padding-top: 3px;
        padding-bottom: 3px;
        padding-left: 8px;
        padding-right: 8px;
    }
    .add-btn:hover {
        background-color: rgb(209, 209, 209);
    }

    .input-style {
        width: 100%;
        padding: 12px 20px;
        margin: 8px 0;
        display: inline-block;
        border: 1px solid #ccc;
        border-radius: 4px;
        box-sizing: border-box;
    }

    .btn-cancel {
        background-color: red;
    }
    .btn-cancel:hover {
        background-color: rgb(255, 48, 48);
    }
    .btn-accept {
        background-color: green;
    }
    .btn-accept:hover {
        background-color: rgb(0, 163, 0);
    }

    .add-btn-text {
        display: inline-flex;
        margin-right: 5px;
    }

    .cancel-btn-text {
        display: inline-flex;
        margin-right: 5px;
    }

    .cancel-btn-loader {
        vertical-align: middle;
    }

    .buy-btn-text {
        display: inline-flex;
    }

    .buy-btn-loader {
        vertical-align: middle;
    }

    .loader {
        display: inline-flex;
        border: 3px solid #f3f3f3;
        border-radius: 50%;
        border-top: 3px solid #3498db;
        width: 12px;
        height: 12px;
        -webkit-animation: spin 2s linear infinite; /* Safari */
        animation: spin 2s linear infinite;
    }

    /* Safari */
    @-webkit-keyframes spin {
      0% { -webkit-transform: rotate(0deg); }
      100% { -webkit-transform: rotate(360deg); }
    }

    @keyframes spin {
      0% { transform: rotate(0deg); }
      100% { transform: rotate(360deg); }
    }
</style>
