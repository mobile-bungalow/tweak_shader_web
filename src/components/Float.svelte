<script lang="ts">
    export let value: number;
    export let change: (val: number) => void;

    export let min: number = 0;
    export let max: number = 100;
    export let _default: number = 0;

    export const reset = () => {
        value = _default;
        change(Number(value));
    };
</script>

<input
    type="number"
    step="0.01"
    bind:value
    on:change={() => change(Number(value))}
    {min}
    {max}
/>

<style>
    /* Float input specific styles */
</style>
