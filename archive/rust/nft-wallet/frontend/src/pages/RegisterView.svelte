<script>
    import { addTransaction, transactionHistory } from '../storage/transactionHistory.js';
    import * as nftAgent from '../nft';
    import { Principal } from '@dfinity/principal';
    import Loader, {loadSpinner, hideSpinner} from '../components/Loader.svelte';
    import {Form, FormGroup, TextInput, Loading, Button, InlineLoading } from "carbon-components-svelte";

    let loading = false;
    let canister;
    let index;
    let showButton = false;
    let message;
    let nextPage = true;
    $:canSubmit = validPrincipal(canister) && typeof index === 'number';

    let validCanister;
    function validPrincipal(principal) {
        try {
            Principal.fromText(principal);
            return true;
        } catch {
            return false;
        }
    }
    function validateCanister() {
        validCanister = validPrincipal(canister);
    }

    function removeError() {
        validCanister = undefined;
    }
    async function register() {
        if (!canSubmit) {return}
        loading = true;
        const collection = await nftAgent.fetchAllOwnedNfts();
        index = Number(index);
        collection.forEach(nft => {
            if (Number(nft.index) === index) {
                loading = false;
                showButton = false;
                message = "NFT by that index is already registered, \nbut will continue anyway...";
                showSnackbar();
            }
        });
        const result = await nftAgent.register(canister, index);
        loading = false;
        if (result) {
            result.status === "fail" ? nextPage = false : nextPage = true;
            if (result.status === "success") {
                addTransaction(index, `Registered NFT from canister ${canister}`);
            }
            message = result.message;
            showButton = true;
            showSnackbar();
        };
    }
    function showSnackbar() {
        document.getElementById("snackbar").className = "show";
    }
    function hideSnackbar() {
        const element = document.getElementById("snackbar");
        element.className = "";
        if (nextPage) {
            page("/");
        }
    }

</script>
<script context="module">
    import page from "page";
    export const pageModule = page;
</script>

<div class="register-view">
    <Loading active={loading}/>
    <div id="snackbar">{message}
        {#if showButton}
        <button id="snack_button" on:click={hideSnackbar}>Okay</button>
        {/if}
    </div>
    {#await nftAgent.isAuthorized() then isAuthorized}
    {#if isAuthorized}
    <Form on:submit={(e)=> {
        e.preventDefault();
        register()}}
        style="padding: 50px 40px 10px; border: solid 1px grey; broder-radius: 10px; border-radius: 15px;">
        <h2>Register a new NFT</h2>
        <FormGroup>
            <TextInput size="large" labelText="NFT Canister ID" placeholder="Principal" bind:value={canister} on:blur={validateCanister} on:focus={removeError}/>
            <div id="nft_cid_help" class="form_text">
                Please enter the principal of your NFT management canister.
            </div>
            {#if validCanister === false}
            <span class="error">Invalid principal</span>
            {/if}
        </FormGroup>
        <FormGroup>
            <TextInput min={0} step={1} size="large" type="number" labelText="Index #" placeholder="Token ID" bind:value={index}/>
            <div id="nft_index_help" class="form_text">
                Provide the specific token id associated with the NFT you are registering.
            </div>
            {#if index === null}
            <span class="error">Missing index</span>
            {/if}
        </FormGroup>
        <FormGroup>
            <Button style="width: 100%; max-width: none; justify-content: center; font-size: 16px; padding: 0;" type="submit" disabled={canSubmit? false: true}>REGISTER</Button>
        </FormGroup>
    </Form>
    {:else}
    <div class="inlineLoad"><InlineLoading/>Redirecting...</div>
    <p>You must be an authorized user to register new NFTs to this wallet.</p>
    {#await setTimeout(()=> page('/'), 3000)}
       ...
    {/await}
    {/if}
    {/await}
</div>

<style>
    .register-view {
        margin: auto;
        max-width: 650px;
        padding-top: 10%;
        padding-bottom: 10%;
    }
    h2 {
        text-align: center;
        padding-bottom: 1em;
    }
    p {
        text-align: center;
    }
    .inlineLoad {
        padding-bottom: 20px;
        margin: auto;
        width: fit-content;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    #snack_button {
        border-radius: 4px;
        background-color: transparent;
        margin-top: 10px;
        background-color: #fcc56f;
        width: auto;
        border: solid 2px #fcc56f;
        color: black;
        width: 40%;
    }
</style>
