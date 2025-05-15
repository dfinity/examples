<script>
    import NFTGrid from '../components/NFTGrid.svelte';
    import Copier from '../components/Copier.svelte';
    import { fetchAllOwnedNftsForCollection, fetchCollectionInfo } from '../nft.js';

    export let canister;
    $:_nfts = fetchAllOwnedNftsForCollection(canister);
    $:_collection = fetchCollectionInfo(canister);
</script>

{#await _collection then collection}
<div class="collectionView">
    <h2><img src={collection.icon} alt="{collection.name} icon" class="icon"><Copier text={canister}>{collection.name} ({collection.symbol})</Copier></h2>
    {#await _nfts then nfts}
    <NFTGrid {nfts}/>
    {/await}
</div>
{/await}

