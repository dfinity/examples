<script>
    import { onMount } from 'svelte';
    import { Actor, HttpAgent } from "@dfinity/agent";
    import { AuthClient } from "@dfinity/auth-client";
    import { idlFactory } from '../../../declarations/defi_dapp/defi_dapp.did.js';
    import PlugWallet from '../components/PlugWallet.svelte'


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
        <div class="plug-container">
            <PlugWallet />
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
