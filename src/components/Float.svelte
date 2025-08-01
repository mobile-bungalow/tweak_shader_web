<script lang="ts">
    import { RotateCcw } from "lucide-svelte";

    export let value: number;
    export let change: (val: number) => void;
    export let min: number = 0;
    export let max: number = 100;
    export let _default: number = 0;

    const handleInput = () => {
        const newValue = Number(Number(value).toFixed(4));
        if (!isNaN(newValue)) {
            change(newValue);
        }
    };

    const handleSliderInput = (e: Event) => {
        const target = e.target as HTMLInputElement;
        value = Number(target.value);
        change(value);
    };

    const reset = () => {
        value = _default;
        change(value);
    };

    $: percentage = ((value - min) / (max - min)) * 100;
</script>

<div class="float-input">
    <div class="slider-container">
        <input
            type="range"
            {min}
            {max}
            step="0.01"
            {value}
            on:input={handleSliderInput}
            class="slider"
        />
        <div class="slider-track"></div>
        <div class="slider-fill" style="width: {percentage}%"></div>
        <div class="slider-thumb" style="left: {percentage}%"></div>
    </div>
    <div class="input-container">
        <input
            type="number"
            step="0.001"
            bind:value
            on:input={handleInput}
            {min}
            {max}
            class="number-input"
        />
        <button on:click={reset} class="reset-btn" title="Reset to default">
            <RotateCcw size="12" />
        </button>
    </div>
</div>

<style>
    .float-input {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        min-width: 200px;
    }

    .slider-container {
        position: relative;
        flex: 1;
        height: 6px;
        border-radius: 3px;
        background: var(--border-color);
        overflow: hidden;
    }

    .slider {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        opacity: 0;
        cursor: pointer;
        z-index: 3;
    }

    .slider-track {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: var(--border-color);
        border-radius: 3px;
    }

    .slider-fill {
        position: absolute;
        top: 0;
        left: 0;
        height: 100%;
        background: var(--accent-color);
        border-radius: 3px;
    }

    .slider-thumb {
        position: absolute;
        top: 50%;
        width: 12px;
        height: 12px;
        background: white;
        border: 1px solid var(--accent-color);
        border-radius: 50%;
        transform: translate(-50%, -50%);
        z-index: 2;
        pointer-events: none;
    }

    .input-container {
        display: flex;
        align-items: center;
        gap: 0.25rem;
    }

    .number-input {
        width: 70px;
        padding: 0.25rem 0.5rem;
        background: rgba(255, 255, 255, 0.05);
        border: 1px solid var(--border-color);
        border-radius: 4px;
        color: var(--text-color);
        font-size: 0.875rem;
        text-align: center;
    }

    .number-input:focus {
        outline: none;
        border-color: var(--accent-color);
        background: rgba(255, 255, 255, 0.08);
    }

    .reset-btn {
        width: 24px;
        height: 24px;
        padding: 0;
        background: rgba(255, 255, 255, 0.05);
        border: 1px solid var(--border-color);
        border-radius: 3px;
        color: var(--text-color);
        cursor: pointer;
        font-size: 0.75rem;
        display: flex;
        align-items: center;
        justify-content: center;
    }
</style>
