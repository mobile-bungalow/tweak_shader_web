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
            <IntInput {...inputValueToProps(input)} {change} />
        {:else if input.type === "Bool"}
            <BoolInput {...inputValueToProps(input)} {change} />
        {:else if input.type === "Point"}
            <PointInput {...inputValueToProps(input)} {change} />
        {:else if input.type === "Color"}
            <ColorInput {...inputValueToProps(input)} {change} />
        {:else if input.type === "Image"}
            <!-- these are displayed elsewhere -->
        {/if}
    </div>
</main>

<style>
    .tweak-input {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        padding: 0.75rem;
        background: rgba(255, 255, 255, 0.02);
        border: 1px solid var(--border-color);
        border-radius: 8px;
        transition: border-color 0.2s ease;
    }

    .tweak-input p {
        margin: 0;
        font-size: 0.875rem;
        font-weight: 500;
        color: var(--text-color);
        text-transform: capitalize;
    }
</style>
