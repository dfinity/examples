<script>
    import { onMount } from 'svelte';
    import { Actor, HttpAgent } from "@dfinity/agent";
    import { AuthClient } from "@dfinity/auth-client";
    import { idlFactory } from '../../../declarations/defi_dapp/defi_dapp.did.js';
    import { Principal } from '@dfinity/principal';

    const canisters = [
        {name: 'AkitaDIP20', canisterId: process.env.AKITADIP20_CANISTER_ID},
        {name: 'GoldenDIP20', canisterId: process.env.GOLDENDIP20_CANISTER_ID},
        {name: 'ICP', canisterId: process.env.LEDGER_CANISTER_ID}
    ]
    console.log(canisters)

    let orders = [];
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
        if(authenticated) {
            const identity = authClient.getIdentity() || undefined;
            const agent = new HttpAgent({identity, host: "http://localhost:8000"});
            // development only, comment out in prod
            agent.fetchRootKey();
            // end comment out in prod only
            backendActor = Actor.createActor(idlFactory, {
                agent,
                canisterId: process.env.DEFI_DAPP_CANISTER_ID
            });

            orders = await backendActor.list_order();
            orders = [...orders.reverse()] // show latest
        }
	});

    async function placeOrder() {
        addingOrder = true;
        const result = await backendActor.place_order(
            Principal.fromText(newOrder.fromCanister),
            newOrder.fromAmount,
            Principal.fromText(newOrder.toCanister),
            newOrder.toAmount);

        if(result.Ok) {
            const newOrderList = await backendActor.list_order();
            orders = []; // clear old order list
            orders = [...newOrderList.reverse()];
        }
        closeOrderForm();
    };

    async function buyOrder(order) {
        buyingOrder = true;
        // Create an order opposite of the one being bought
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
            const newOrderList = await backendActor.list_order();
            orders = []; // clear old order list
            orders = [...newOrderList.reverse()];      
        }
        buyingOrder = false;
    };

    function closeOrderForm() {
        showOrderForm = false;
        addingOrder = false;
    };

    async function cancelOrder(id) {
        cancelingOrder = true;
        const result = await backendActor.cancel_order(id);
        if(result && result.Ok) {
            const newOrderList = await backendActor.list_order();
            orders = []; // clear old order list
            orders = [...newOrderList.reverse()];       
        }
        cancelingOrder = false;
    };
</script>

<div class="order-container">
    <div>
        <div class="header-container">
            <div class="title">
                <h2>Orders</h2>
            </div>
            <div class="adding-order-btn">
                <button on:click={() => showOrderForm = true } title="Add new order" class="add-btn">+</button>
            </div>
        </div>
        <div>
            <table>
                {#if orders.length}
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
                                {#each canisters as canister}
                                    <option value={canister.canisterId}>
                                        {canister.name}
                                    </option>
                                {/each}
                            </select>
                        </td>
                        <td><input class="input-style" bind:value={newOrder.fromAmount} type="number" /></td>
                        <td>
                            <select class="input-style" bind:value={newOrder.toCanister}>
                                {#each canisters as canister}
                                    <option value={canister.canisterId}>
                                        {canister.name}
                                    </option>
                                {/each}
                            </select>
                        </td>                       
                        <td><input class="input-style" bind:value={newOrder.toAmount} type="number" /></td>
                        <td><button class="action-btn btn-add" on:click={placeOrder}>
                            <div class="add-btn-text">
                                Add
                            </div>
                            {#if addingOrder}
                                <div class="loader"></div>
                            {/if}
                        </button>
                        </td>
                        <td><button class="action-btn btn-cancel" on:click={closeOrderForm}>Cancel</button></td>
                    </tr>
                    {/if}
                    {#each orders as order}
                    <tr>
                        <td>{order.from}</td>
                        <td>{order.fromAmount}</td>
                        <td>{order.to}</td>
                        <td>{order.toAmount}</td>
                        <td>
                            <button class="action-btn btn-buy">
                                <div class="buy-btn-text" on:click={() => buyOrder(order)}>Buy</div>
                                {#if buyingOrder}
                                    <div class="loader buy-btn-loader"></div>
                                {/if}
                            </button>
                            {#if order.owner.toText() === authClient.getIdentity().getPrincipal().toText()}
                                <button class="action-btn btn-cancel" on:click={() => cancelOrder(order.id)}>
                                    <div class="cancel-btn-text">
                                        Cancel
                                    </div>
                                    {#if cancelingOrder}
                                        <div class="loader cancel-btn-loader"></div>
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
        width: 75vw;
    }
    .order-container {
        text-align: left !important;
        margin-bottom: 36px;
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

    .action-btn {
        padding: 5px 30px;
        border-radius: 5px;
    }

    .btn-add {
        display: flex;
        background-color: green;
    }
    .btn-add:hover {
        background-color: rgb(0, 169, 0);
    }
    .btn-cancel {
        background-color: red;
    }
    .btn-cancel:hover {
        background-color: rgb(255, 48, 48);
    }
    .btn-buy {
        background-color: blue;
    }
    .btn-buy:hover {
        background-color: rgb(90, 90, 255);
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
