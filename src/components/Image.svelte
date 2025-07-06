<script lang="ts">
    export let change: (val?: ImageData) => void;
    export const reset = () => {
        value = _default;
        change(Number(value));
    };
</script>

<div></div>

<style>
    /* Float input specific styles */
</style>
