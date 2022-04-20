<svelte:options immutable={true}/>

<script>
    import ContentBox from './ContentBox.svelte';
    export let content;
    export let current;
    $:currentElement = content[current];
</script>

<div class="box">
    <ContentBox src={currentElement.value} contentType={currentElement.contentType}>
        <slot name="fallback" contentType={currentElement.contentType} src={currentElement.value} {current}/>
    </ContentBox>
    {#if content.length > 1}
        {#if current > 0}
        <a class="left ui button" href="#{current - 1}">
            &lt;
        </a>
        {/if}
        {#if current < content.length - 1}
        <a class="right ui button" href="#{current + 1}">
            &gt;
        </a>
        {/if}
    {/if}
</div>

<style>
    .left, .right {
        font-size: 5em;
        opacity: 40%;
        position: absolute;
        top: 33%;
        height: 33%;
        user-select: none;
    }
    .left {
        left: 0;
    }
    .right {
        right: 0;
    }
    .box {
        position: relative;
    }
</style>
