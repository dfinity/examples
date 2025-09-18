<svelte:options immutable={true}/>

<script>
    import Copier from './Copier.svelte';
    import { getCanisterId } from '../nft';
    let canisterId = getCanisterId();
</script>

<div id="wallet_cid">
    <Copier text={canisterId} --default-color="#666" --highlight-color="#EEE">{canisterId}</Copier>
</div>

<style>
    #wallet_cid{
        display: flex;
        justify-content: center;
        color: rgb(187, 183, 183);
        font-size: 14px;
    }
</style>
