<script>
    import { onMount } from 'svelte';
    import { Actor, HttpAgent } from "@dfinity/agent";
    import { AuthClient } from "@dfinity/auth-client";
    import { idlFactory } from '../../../declarations/backend/backend.did.js';

    let orders = [];
    let backendActor;
    let addingOrder = false;
    let authClient;

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
                canisterId: process.env.BACKEND_CANISTER_ID
            });
            orders = await backendActor.list_order();
        }
	});

    async function addOrder() {
        const result = await backendActor.place_order(
            newOrder.fromAcct,
            newOrder.fromAmount,
            newOrder.toAcct,
            newOrder.toAmount);

        if(result.status === 'Ok') {
            orders.push(result.order);
            orders = [...orders];
            addingOrder = false;
        }
    };

    async function cancelOrder(id) {
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
    };

</script>

<div class="info-container">
    <div>
        <div class="header-container">
            <div class="title">
                <h2>Balance</h2>
            </div>
            <div class="adding-order-btn">
                <div class="balance-text">1234 ICP</div>
            </div>
        </div>  
    </div>
    <div>
        <div class="header-container">
            <div class="title">
                <h2>My Orders</h2>
            </div>
            <div class="adding-order-btn">
                <button on:click={() => addingOrder = true } class="add-btn">+</button>
            </div>
        </div>
        <div>
            <table>
                <thead>
                    <th>ID</th>
                    <th>From Acct</th>
                    <th>Amount</th>
                    <th>To Acct</th>
                    <th>Amount</th>
                    <th></th>
                </thead>
                <tbody>
                    {#if addingOrder}
                    <tr>
                        <td></td>
                        <td><input bind:value={newOrder.fromAcct} /></td>
                        <td><input bind:value={newOrder.fromAmount} type="number" /></td>
                        <td><input bind:value={newOrder.toAcct} /></td>
                        <td><input bind:value={newOrder.toAmount} type="number" /></td>
                        <td><button on:click={addOrder}>Add Order</button></td>
                        <td><button on:click={() => addingOrder = false}>Cancel Add</button></td>
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
                            {#if order.owner.toText() === authClient.getIdentity().getPrincipal().toText()}
                                <button on:click={() => cancelOrder(order.id)}>Cancel Order</button>
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

    .adding-order-btn {
       margin-left: 10px;
    }

    .add-btn {
        margin-top:18px;
        border-radius: 5px;
    }

    .balance-text {
        margin-top:18px;
    }
</style>