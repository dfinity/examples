<script>
    import NFTGrid from '../components/NFTGrid.svelte';
    import { fetchAllOwnedNfts, isAuthorized } from '../nft.js';
    import {loadSpinner, hideSpinner} from '../components/Loader.svelte';
    // import Button, { Label } from '@smui/button';
    import { login } from '../components/Authenticator.svelte'
    import { Loading, Button } from 'carbon-components-svelte';
</script>
<div class="wallet-view">
    {#await isAuthorized() then isAuthorized}
    {#if !isAuthorized}
    <div id="welcome_message" style="padding-top: 130px; padding-bottom: 130px;">
        <h2>
            Welcome to the nft wallet for the IC
        </h2><br/>
        The wallet can hold countless NFTs, all in a single secure wallet. <br/>Once in your wallet, you can view, send, receive NFTs and more!
        <div class="action">
            Start adding your NFTs here.
            <br><br>
            <Button id="button" variant="raised" style="width: 200px; justify-content: center; background-color: #29ABE2; max-width: none; padding: 0; font-size: 16px;" on:click={login}>
                <!-- <Label>LOGIN</Label> -->
                LOGIN
            </Button>
        </div>
    </div>
    {/if}
    {#if isAuthorized}
        {#await fetchAllOwnedNfts()}
            <div id="loader">
                <Loading withOverlay={false} />
            </div>
            {:then nfts}
            <NFTGrid {nfts}/>
        {/await}
    {/if}
    {/await}
</div>

<style>
    @media (max-width: 375px) {
        .wallet-view {
            width: 100%;
            line-height: 1.2rem;
        }
        h2 {
            font-size: 28px;
        }
    }
    #welcome_message {
        text-align: center;
    }
    .wallet-view {
        width: 100%;
        line-height: 1.5rem;
        display: flex;
        flex-direction: column;
        font-size: 16px;
        justify-content: center;
        align-items: center;
        padding: 0 1em 1em 1em;
        text-align: left;
    }
    .action {
        height: 300px;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }
    #loader {
        margin: auto;
       padding-top: 250px;
    }
    .action button:hover {
        background-color: rgb(46 119 241);
    }
</style>
