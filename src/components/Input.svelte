<script lang="ts">
    import FloatInput from "./Float.svelte";
    import IntInput from "./Int.svelte";
    import BoolInput from "./Bool.svelte";
    import PointInput from "./Point.svelte";
    import ColorInput from "./Color.svelte";
    import { inputValueToProps, type WasmInputValue } from "$lib";

    type InputVariantType =
        | "Float"
        | "Int"
        | "Bool"
        | "Point"
        | "Color"
        | "Image";

    export let label: String;
    export let input: WasmInputValue;
    export let change: (val: any) => void;
</script>

<main>
    <div class="tweak-input">
        {#if label}
            <p>{label}</p>
        {/if}
        {#if input.type === "Float"}
            <FloatInput {...inputValueToProps(input)} {change} />
        {:else if input.type === "Int"}
            <IntInput {value} {change} />
        {:else if input.type === "Bool"}
            <BoolInput {value} {change} />
        {:else if input.type === "Point"}
            <PointInput {value} {change} />
        {:else if input.type === "Color"}
            <ColorInput {value} {change} />
        {:else if input.type === "Image"}
            <!-- these are displayed elsewhere -->
        {/if}
    </div>
</main>

<style>
    .tweak-input {
        gap: 10px;
        display: flex;
    }
</style>
