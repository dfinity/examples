<script>
    import { onMount } from 'svelte';
    import { Actor, HttpAgent } from "@dfinity/agent";
    import { AuthClient } from "@dfinity/auth-client";
    import { idlFactory } from '../../../declarations/backend/backend.did.js';

    let orders = [];
    let backendActor;

    onMount(async () => {
        const authClient = await AuthClient.create();
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
        const result = await backendActor.place_order("ICP", 2, "SAMP", 5);
        console.log(result)
        if(result.status === 'Ok') {
            orders.push(result.order);
            orders = [...orders];
        }
    };

    async function cancelOrder(id) {
        const result = await backendActor.cancel_order(id);
    }

</script>

<div class="info-container">
    <div>
        <h2>My Orders</h2>
        <button on:click={addOrder}>Add Order</button>
        <div>
            <table>
                <thead>
                    <th>ID</th>
                    <th>From/Amount</th>
                    <th>To/Amount</th>
                    <th></th>
                </thead>
                <tbody>
                    {#each orders as order}
                    <tr>
                        <td>{order.id}</td>
                        <td>{order.from} - {order.fromAmount} </td>
                        <td>{order.to} - {order.toAmount} </td>
                        <td><button on:click={() => cancelOrder(order.id)}>Cancel Order</button></td>
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
</style>