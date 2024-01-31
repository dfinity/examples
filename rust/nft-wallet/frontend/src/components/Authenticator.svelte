<script context="module">
    import * as nftAgent from '../nft';
    import Copier from './Copier.svelte';
    import { Dialog, Button } from 'svelte-mui';

    let _isAuthenticated = nftAgent.isAuthenticated();
    let _isAuthorized = nftAgent.isAuthorized();
    async function getCommand() {
        const principal = await nftAgent.getPrincipal();
        return `dfx canister ${nftAgent.isMainnet() ? ' --network ic' : ''} call '${nftAgent.getCanisterId()}' set_authorized '(principal "${principal}", true)'`;
    }
    function retry() {
        _isAuthenticated = nftAgent.isAuthenticated();
    }

    export async function isLoggedIn() {
        const authenticated = await nftAgent.isAuthenticated();
        if (authenticated) {
            return true;
        }
        return false;
    };

    export function login() {
        nftAgent.authenticate(() => {
            _isAuthenticated = nftAgent.isAuthenticated();
            window.location.href = '/'
        });
    }
    export async function _logout() {
        await nftAgent.logout();
        _isAuthenticated = nftAgent.isAuthenticated();
        window.location.href = '/'
    }
    function hideModal() {
        document.getElementsByClassName('authenticator')[0].style.visibility = 'hidden';
    }
    async function showModal() {
        const authenticated = await nftAgent.isAuthenticated();
		const authorized = await nftAgent.isAuthorized();
        if (!authenticated || !authorized) {
            document.getElementsByClassName("authenticator")[0].style.visibility = 'visible';
        }
    }
    showModal();
</script>

<div class="authenticator">
        {#await _isAuthenticated then isAuthenticated}
        {#if isAuthenticated}
            {#await _isAuthorized then isAuthorized}
            {#if !isAuthorized}
            <!-- logged in first time -->
            <Dialog visible=true>
                <div slot="title" class="title"> Welcome!<br> <p>Unregistered User</p> </div>
                <p class="info">
                    {#await getCommand()}
                    <s>click here to copy registration command &cross;</s>
                    {:then command}
                    <Copier always text={command} --default-color="#999">click to copy registration command</Copier>
                    {/await}
                    <br>
                    <br>
                    Type command into your nft-wallet terminal to authorize your identity.
                    Then refresh page.
                </p>
                <Button on:click={hideModal}>Okay</Button>
            </Dialog>
            {/if}
            {:catch}
            <!-- logged in doesn't work  -->
            <Dialog visible=true>
                <div slot="title" class="title">Problem!</div>
                <p>
                    <span class="error">
                        Can't reach canister
                        {#if !nftAgent.isMainnet()}
                        (is the replica running?)
                        {/if}
                    </span>
                </p>
                <Button on:click={hideModal}>Okay</Button>
            </Dialog>
            {/await}
        {/if}
        {:catch}
        <!-- reaching II doesn't work -->
        <Dialog visible=true>
            <div slot="title" class="title">Error</div>
            <p class="error">Could not reach Internet Identity</p>
            <Button on:click={retry}>Retry &circlearrowright;</Button>
        </Dialog>
        {/await}

</div>

<style>
    button {
        background-color: #666;
        color: black;
        font-size: 14px;
    }
    p {
        margin: 0 0 8px;
    }
    .info {
        color: #666;
        font-size: 14px;
    }
    .authenticator {
        visibility: hidden;
        display:flex;
        flex-direction: column;
    }
    .title {
        color: black;
    }
    .content {
        color: black;
    }
</style>
