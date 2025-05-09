<script>
    import WalletView from './pages/WalletView.svelte';
    import NFTView from './pages/NFTView.svelte';
    import CollectionView from './pages/CollectionView.svelte';
    import CanisterId from './components/CanisterId.svelte';
    import TransferView from './pages/TransferView.svelte';
    import Authenticator, {isLoggedIn, _logout, login} from './components/Authenticator.svelte';
    import RegisterView, {pageModule} from './pages/RegisterView.svelte';
    import Transactions from './components/Transactions.svelte';

    export let page = {};

    import "carbon-components-svelte/css/all.css";
    import { Theme } from "carbon-components-svelte";
    let theme = "g80";

    function goHome() {
       pageModule('/');
    };

</script>

<Theme bind:theme persist persistKey="__carbon-theme" />
<div class="navBar">
        <div id="title">
            <img id="wallet" src="/images/wallet.png" alt="wallet"/>
            <a href="/" id="nft_wallet_title"class="ui">NFT Wallet</a>
        </div>

        <div class="buttons">
            <button type="button" id="home_button" class="nav_button button"
                on:click|preventDefault={goHome}>Home
            </button>
            <button type="button" id="register_button" class="nav_button button">
                <a class="nav_b" href="/register">Register</a>
            </button>
            <div class="subnav">
                <button type="button" class="nav_button button" id="account_button">Account</button>

                <div class="subnav-content">
                    {#await isLoggedIn() then loggedIn}
                        {#if !loggedIn}
                        <div class="log" type="button" on:click={login}>Login</div>
                        {:else}
                        <div class="log" type="button" on:click={_logout}>Logout</div>
                        {/if}
                    {/await}
                    <a href="/transactions">Transactions</a>
                    <div id="walletID_container">Wallet Canister ID
                        <div id="walletID">
                            <CanisterId/>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>

<main class="main">
    <Authenticator/>
    <div class="content">
        {#if page.nft}
        <NFTView nft={page.nft} current={page.nftCurrent}/>
        {:else if page.register}
        <RegisterView/>
        {:else if page.transfer}
        <TransferView nft={page.transfer} pageState={page}/>
        {:else if page.collection}
        <CollectionView canister={page.collection}/>
        {:else if page.transactions}
        <Transactions />
        {:else}
        <WalletView/>
        {/if}
    </div>
    <footer id="footer">
        <img id="ic" src="/images/ic-badge-powered-by_bg-dark.svg" alt="powered by ic">
    </footer>
</main>

<style lang="scss" global>
    .navBar {
        border-bottom: solid 3px #7f7f7f;
        padding: 1em;
        background-color: grey;
    }
    main.main {
            margin: 0 15px auto;
            height: fit-content;
        }
    @media (min-width: 640px) {
        .navBar {
            display: grid;
            grid-template-columns: 1fr 45%;
            grid-template-areas: "title buttons";
        }
        .buttons {
            justify-content: flex-end;
        }
    }
    .content {
        min-height: calc(100vh - 100px - 3em - var(--footer-height));
        margin-top: 1em;
    }
    #title {
        grid-area: title;
        display: flex;
        align-items: center;
        margin-bottom: 0;
        font-size:xx-large;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
        font-weight: 500;
    }

    @media (max-width: 639px) {
        #title {
            justify-content: center;
        }
        .navBar {
            display: flex;
            flex-direction: column;
            justify-content: center;
        }
        .buttons {
            justify-content: center;
            display: flex;
            margin-top: 1.5em;
        }
    }
    img#wallet {
        grid-area: icon;
        object-fit: contain;
        max-height: 60px;
        height: 1em;
        margin-right: 10px;
    }
    a#nft_wallet_title {
        font-size-adjust:inherit;
    }
    .buttons {
        grid-area: buttons;
        display: flex;
        align-items: center;
    }
    #home_button:hover {
        border: solid 1px #30ace3;
        display: flex;
        justify-content: center;
    }

    #register_button:hover {
        border: solid 1px #893385;
    }
    #account_button:hover {
        border: solid 1px #fcc56f;
        color: white;
        display: flex;
        justify-content: center;
    }
    img#ic {
        margin: auto;
    }
    button {
        margin: 0 5px;
    }
    .nav_button {
        border-radius: 4px;
        background-color: transparent;
        margin: 5px;
    }
    a.nav_b {
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        text-decoration: none;
        color: white;
    }
    .buttons button:hover {
        cursor: pointer;
    }
    .button {
        color: white;
        width: 100px;
        border: none;
    }
    .log:hover{
        cursor: pointer;
    }
    #walletID {
        height: 1.5em;
        width: 220px;
        display: flex;
        align-items: flex-end;
    }
    #walletID_container {
        border-top: 1px solid white;
        margin-top: 4px;
    }
    #cid {
        margin: 0 15px;
    }
    #cid_menu {
        display: flex;
        flex-direction: column;
    }
    #footer {
        position: relative;
        width: fit-content;
        margin: 0 auto;
        height: var(--footer-height);
    }
</style>
