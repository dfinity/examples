<svelte:options immutable={true}/>

<script>
    export let src;
    export let contentType = "application/octet-stream";
    $:fallback = !contentType.startsWith('image')
        && !contentType.startsWith('video')
        && !contentType.startsWith('audio');
</script>

<div class:fallback>
    {#if contentType.startsWith('image')}
    <img {src} alt="NFT content page">
    {:else if contentType.startsWith('video')}
    <!-- svelte-ignore a11y-media-has-caption -->
    <video {src}></video>
    {:else if contentType.startsWith('audio')}
    <audio {src}></audio>
    {:else}
    <slot {src}>
        <!-- <span class="filler">📝</span> -->
        <img id="default_img" src="/images/background.png" alt="background">
    </slot>
    {/if}
</div>

<style>
    div {
        width: 100%;
        height: fit-content;
        position: relative;
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: white;
    }
    div.fallback {
        background-color: var(--fallback-bg, white);
    }
    div:after {
        padding-bottom: 100%;
        content: "";
        display: block;
    }
    .filler {
        font-size: 5em;
        opacity: 30%;
        text-align: center;
    }
    #default_img {
        opacity: 0.5;
    }
    img {
        display: block;
        margin: auto;
        position: absolute;
        width: 100%;
    }
</style>
