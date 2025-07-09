<script>
    import { onMount } from 'svelte';
    import { Principal } from '@dfinity/principal';
    import { orders, currentOrder } from '../store/order';
    import { auth, plugWallet, anonymous } from '../store/auth';
    import { canisters } from '../store/store';
    import { FontAwesomeIcon } from 'fontawesome-svelte';
    import { AuthClient } from '@dfinity/auth-client';
 

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

    plugWallet.subscribe((value) => {
        if(value.plugActor) {
            console.log('Plug connected, using plug actor')
            backendActor = value.plugActor;
        }
    })
    
    auth.subscribe(async (value) => {
        if(value.loggedIn) {
            backendActor = value.actor
            authClient = await AuthClient.create();
        }
    })

    onMount(async () => {
        // Use II as actor
        if($auth.loggedIn) {
            console.log("In orders, using II");
            backendActor = $auth.actor;
        }
        else if ($plugWallet.isConnected) {
            console.log("Using Plug for DEX actor");
            backendActor = $plugWallet.plugActor;
            console.log(backendActor)
        }
        else {
            console.log('Using anonymous actor');
            backendActor = $anonymous.actor;
        }

        const orderList = await backendActor.getOrders();
        orders.set([]);
        orders.set(orderList.reverse());
	});

    async function placeOrder() {
        addingOrder = true;
        const principal = Principal.fromText($canisters[2].canisterId);
        const balance = await backendActor.getBalance(principal);
        console.log(balance)
        const result = await backendActor.placeOrder(
            Principal.fromText(newOrder.fromCanister),
            newOrder.fromAmount,
            Principal.fromText(newOrder.toCanister),
            newOrder.toAmount);

        console.log(result)
        if(result.Ok) {
            const orderList = await backendActor.getOrders();
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
        const result = await backendActor.placeOrder(
            newOrder.fromCanister,
            newOrder.fromAmount,
            newOrder.toCanister,
            newOrder.toAmount
        )
        if(result && result.Ok) {
            const orderList = await backendActor.getOrders();
            orders.set([]);
            orders.set(orderList.reverse());
        }
        currentOrder.set({});
        buyingOrder = false;
        
        // TODO: Better way to handle balance updates on UI
        // get the balance of the to, get the balance of the from
        window.location.reload();
    };

    function closeOrderForm() {
        showOrderForm = false;
        addingOrder = false;
    };

    async function cancelOrder(id) {
        cancelingOrder = true;
        const order = $orders.find((o) => o.id === id);
        currentOrder.set(order);
        const result = await backendActor.cancelOrder(id);
        if(result && result.Ok) {
            const orderList = await backendActor.getOrders();
            orders.set([]);
            orders.set(orderList.reverse()); 
        }
        currentOrder.set({});
        cancelingOrder = false;
    };

    async function getTokenSymbol(principal) {
       const item =  $canisters.find((canister) => canister.canisterId === principal.toString())
        if(item) {
            return item.symbol;
        }
    };
</script>

<div class="order-container">
    <div>
        <div class="header-container">
            <div class="title">
                <h2>Orders</h2>
            </div>
            {#if $auth.loggedIn || $plugWallet.isConnected}
                <div class="adding-order-btn">
                    <button on:click={() => showOrderForm = true } title="Add new order" class="add-btn">
                        <FontAwesomeIcon icon="plus" />
                    </button>
                </div>
            {/if}
        </div>
        <div>
            <table>
                {#if $orders.length || showOrderForm}
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
                                        {canister.symbol}
                                    </option>
                                {/each}
                            </select>
                        </td>
                        <td><input class="input-style" bind:value={newOrder.fromAmount} type="number" /></td>
                        <td>
                            <select class="input-style" bind:value={newOrder.toCanister}>
                                {#each $canisters as canister}
                                    <option value={canister.canisterId}>
                                        {canister.symbol}
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
                            {#if $auth.loggedIn}
                                <button class="btn-accept" title="Buy order">
                                    {#if buyingOrder && $currentOrder.id === order.id}
                                    <div class="loader buy-btn-loader"></div>
                                    {:else}
                                        <div class="buy-btn-text" on:click={() => buyOrder(order)}>
                                            <FontAwesomeIcon icon="check" />
                                        </div>
                                    {/if}
                                </button>
                                {#if authClient && order.owner.toText() === authClient.getIdentity().getPrincipal().toText()}
                                    <button class="btn-cancel" on:click={() => cancelOrder(order.id)} title="Cancel order">
                                        {#if cancelingOrder && $currentOrder.id === order.id}
                                            <div class="loader cancel-btn-loader"></div>
                                        {:else}
                                            <div class="cancel-btn-text">
                                                <FontAwesomeIcon icon="times" />
                                            </div>
                                        {/if}
                                    </button>
                                {/if}
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

    .btn-cancel {
        background-color: white;
    }
    .btn-cancel:hover {
        background-color: rgb(169, 169, 169);
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
</style>
