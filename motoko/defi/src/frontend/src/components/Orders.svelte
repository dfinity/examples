<script>
    import { onMount } from 'svelte';
    import { Actor, HttpAgent } from "@dfinity/agent";
    import { AuthClient } from "@dfinity/auth-client";
    import { idlFactory } from '../../../declarations/defi-dapp/defi-dapp.did.js';

    let orders = [];
    let backendActor;
    let authClient;
    
    let showOrderForm = false;
    let addingOrder = false;
    let cancelingOrder = false;
    let buyingOrder = false;

    const newOrder = {
        fromAcct: "",
        fromAmount: 0,
        toAcct: "",
        toAmount: 0
    };

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

            // TESTING
            const depositAdd = await backendActor.deposit_address();
            orders = await backendActor.list_order();
        }
	});

    async function placeOrder() {
        addingOrder = true;
        const result = await backendActor.place_order(
            newOrder.fromAcct,
            newOrder.fromAmount,
            newOrder.toAcct,
            newOrder.toAmount);

        if(result.status === 'Ok') {
            orders.push(result.order);
            orders = [...orders];
            showOrderForm = false;
        }
        addingOrder = false;
    };

    async function buyOrder(order) {
        buyingOrder = true;
        // Create an order opposite of the one being bought
        const newOrder = {
            from: order.to,
            fromAmount: order.toAmount,
            to: order.from,
            toAmount: order.fromAmount
        };
        const result = await backendActor.place_order(
            newOrder.from,
            newOrder.fromAmount,
            newOrder.to,
            newOrder.toAmount
        )
        console.log(result);
        if(result && result.status === 'Ok') {
            orders.push(result.order);
            orders = [...orders];
        }
        buyingOrder = false;
    };

    async function cancelOrder(id) {
        cancelingOrder = true;
        const result = await backendActor.cancel_order(id);
        if(result && result.status === 'Canceled') {
            const orderIndex = orders.findIndex((order) => {
                return order.id === result.order_id
            });
            if(orderIndex > -1) {
                orders.splice(orderIndex, 1);
                orders = [...orders];
            }
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
                        <th>ID</th>
                        <th>From Acct</th>
                        <th>Amount</th>
                        <th>To Acct</th>
                        <th>Amount</th>
                        <th></th>
                    </thead>
                {/if}
                <tbody>
                    {#if showOrderForm}
                    <tr>
                        <td></td>
                        <td><input class="input-style" placeholder="From Account..." bind:value={newOrder.fromAcct} /></td>
                        <td><input class="input-style" bind:value={newOrder.fromAmount} type="number" /></td>
                        <td><input class="input-style" placeholder="To Account..." bind:value={newOrder.toAcct} /></td>
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
                        <td><button class="action-btn btn-cancel" on:click={() => showOrderForm = false}>Cancel</button></td>
                    </tr>
                    {/if}
                    {#each orders as order}
                    <tr>
                        <td>{order.id}</td>
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
                                        Cancel Order
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
