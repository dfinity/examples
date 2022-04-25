<svelte:options immutable={true}/>

<script>
    import Copier from './Copier.svelte';
    import ContentBox from './ContentBox.svelte';
    import Paper, { Title, Content } from '@smui/paper';
    export let nft;
</script>

<Paper id="paper" elevation=4 color="grey">
    <a class="ui" href="/{nft.canister}/{nft.index}">
        <div class="preview">
            <ContentBox src={nft.preview?.value} contentType={nft.preview?.contentType}/>
        </div>
    </a>
    <div class="card">
        <a class="ui" href="/{nft.canister}/{nft.index}">
            <Title>{nft.symbol} #{nft.index}</Title>
        </a>
        <Content>
            <a class="ui" href="/{nft.canister}">
                <img class="icon" src={nft.icon} alt="icon">{nft.name}
            </a>
                <Copier text={nft.canister}/>
        </Content>
    </div>
</Paper>

<style>
    .preview {
        border-radius: 5px 5px 0 0;
        background: white;
    }
    .card {
        padding: 7px;
        transition: 0.3s;
        border-radius: 5px;
        background-color: #bdbfca;
        font-weight: 200;
        font-family: 'Noto Sans', sans-serif;
        font-family: 'Roboto Mono', monospace;
    }
</style>
