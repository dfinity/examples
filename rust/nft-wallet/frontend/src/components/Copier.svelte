<svelte:options immutable={true}/>

<script>
    export let text;
    export let always = false;
    function copy() {
        navigator.clipboard.writeText(text);
    }
</script>

<span on:click={copy} class="copy" class:always tabindex="0" title="Copy to clipboard">
    {#if $$slots.default}
    <div class="slot"><slot/></div>
    {/if}
    <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="152" height="213" viewBox="0 0 152 213" >
        <g color="silver">
            <rect stroke-width="15" id="svg_1" height="161" width="100" y="7" x="7" stroke="var(--stroke-color)" fill="none"/>
            <rect id="svg_10" height="161" width="100" y="45" x="45" stroke-width="15" stroke="var(--stroke-color)" fill="none"/>
        </g>
    </svg>
    <span class="copied">&check;</span>
</span>

<style>
    .copy {
        cursor: pointer;
        --stroke-color: var(--default-color, var(--default-copy-icon-color));
    }
    .copy.always {
        color: var(--default-color, var(--default-copy-icon-color))
    }
    .copy:focus {
        --stroke-color: var(--highlight-color, var(--default-highlight-copy-icon-color));
    }
    .copy.always:focus {
        color: var(--highlight-color, var(--default-highlight-copy-icon-color));
    }
    .copy:not(:focus) .copied {
        display: none;
    }
    .copy:not(.always) svg:not(:first-child) ~ .copied {
        display: none;
    }
    .copy:not(:hover):not(.always) svg:not(:first-child) {
        display: none;
    }
    .slot {
        display: contents;
    }
</style>
