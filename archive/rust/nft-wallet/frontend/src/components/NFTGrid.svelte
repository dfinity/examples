<svelte:options immutable={true}/>

<script>
    import NFTBox from '../components/NFTBox.svelte';
    export let nfts;
    import { Tile } from "carbon-components-svelte";
    import { pageModule } from '../pages/RegisterView.svelte';
</script>
<div id="action">
    <button class="button1"
        on:click={(e) => pageModule('/register')}>
        + REGISTER
    </button>
    <p>Please add NFTs to your wallet by registering if you haven't already done so.</p>
</div>
<div class="collection">
{#if !nfts.length}
    <div class="section" id="no_collection">
        There is currently no collection.
    </div>
{:else}
    <div class="section">Collection</div>
    <Tile>
        <div class="container">
            {#each nfts as nft}
            <div class="paper-container box">
                <NFTBox {nft}/>
            </div>
            {/each}
        </div>
    </Tile>
{/if}
</div>

<style>
    .container {
        list-style: none;
        display: flex;
        flex-flow: row wrap;
        /* margin: 1em; */
    }
    .box {
        padding: 1em;
    }
    .collection {
        width: inherit;
        padding-top: 10px;
        text-align: center;
    }
    .section {
        font-family: 'Roboto Mono', monospace;
        font-family: 'Noto Sans', sans-serif;
        font-weight: 700;
        font-size: 26px;
        padding-bottom: 1em
    }
    #no_collection {
        font-size: 18px;
    }
    #action {
        padding-bottom : 1em;
        display: flex;
        align-items: center;
    }
    @media (max-width: 375px) {
        #action {
            flex-direction: column;
        }
        p {
            text-align: center;
        }
    }
    @media (max-width: 744px) {
        .button1 {
            margin-right: 1em;
            min-width: 100px;
        }
        p {
            font-size: 12px;
        }
    }
    @media (min-width: 651px) {
        p {
            font-size: 15px;
            display: inline;
        }
        .button1 {
            margin-right: 3em;
            display: inline;
        }
    }
</style>
