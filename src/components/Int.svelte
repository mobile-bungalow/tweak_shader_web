<script lang="ts">
    import { Minus, Plus, RotateCcw } from "lucide-svelte";
    
    export let value: number;
    export let change: (val: number) => void;
    export let min: number = 0;
    export let max: number = 100;
    export let _default: number = 0;

    const handleInput = () => {
        const newValue = Math.round(Number(value));
        if (!isNaN(newValue)) {
            value = newValue;
            change(newValue);
        }
    };

    const handleSliderInput = (e: Event) => {
        const target = e.target as HTMLInputElement;
        const newValue = Math.round(Number(target.value));
        value = newValue;
        change(newValue);
    };

    const reset = () => {
        value = _default;
        change(value);
    };

    const increment = () => {
        if (value < max) {
            value++;
            change(value);
        }
    };

    const decrement = () => {
        if (value > min) {
            value--;
            change(value);
        }
    };

    $: percentage = ((value - min) / (max - min)) * 100;
</script>

<div class="int-input">
    <div class="slider-container">
        <input
            type="range"
            {min}
            {max}
            step="1"
            {value}
            on:input={handleSliderInput}
            class="slider"
        />
        <div class="slider-track"></div>
        <div class="slider-fill" style="width: {percentage}%"></div>
        <div class="slider-thumb" style="left: {percentage}%"></div>
    </div>
    <div class="input-container">
        <button on:click={decrement} class="stepper-btn" disabled={value <= min}>
            <Minus size="12" />
        </button>
        <input
            type="number"
            step="1"
            bind:value
            on:input={handleInput}
            {min}
            {max}
            class="number-input"
        />
        <button on:click={increment} class="stepper-btn" disabled={value >= max}>
            <Plus size="12" />
        </button>
        <button on:click={reset} class="reset-btn" title="Reset to default">
            <RotateCcw size="12" />
        </button>
    </div>
</div>

<style>
    .int-input {
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
        z-index: 2;
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

    .stepper-btn {
        width: 24px;
        height: 24px;
        padding: 0;
        background: rgba(255, 255, 255, 0.05);
        border: 1px solid var(--border-color);
        border-radius: 3px;
        color: var(--text-color);
        cursor: pointer;
        font-size: 0.875rem;
        font-weight: bold;
        display: flex;
        align-items: center;
        justify-content: center;
    }


    .stepper-btn:disabled {
        opacity: 0.4;
        cursor: not-allowed;
    }

    .number-input {
        width: 60px;
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