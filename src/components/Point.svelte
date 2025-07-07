<script lang="ts">
    import { RotateCcw } from "lucide-svelte";
    export let value: [number, number];
    export let change: (val: [number, number]) => void;
    export let min: number = 0;
    export let max: number = 1;
    export let _default: [number, number] = [0, 0];

    let x = value[0];
    let y = value[1];

    const handleXChange = () => {
        const newX = Number(x);
        if (!isNaN(newX)) {
            value = [newX, y];
            change(value);
        }
    };

    const handleYChange = () => {
        const newY = Number(y);
        if (!isNaN(newY)) {
            value = [x, newY];
            change(value);
        }
    };

    const handleXSlider = (e: Event) => {
        const target = e.target as HTMLInputElement;
        x = Number(target.value);
        value = [x, y];
        change(value);
    };

    const handleYSlider = (e: Event) => {
        const target = e.target as HTMLInputElement;
        y = Number(target.value);
        value = [x, y];
        change(value);
    };

    const reset = () => {
        x = _default[0];
        y = _default[1];
        value = [x, y];
        change(value);
    };

    $: xPercentage = ((x - min) / (max - min)) * 100;
    $: yPercentage = ((y - min) / (max - min)) * 100;

    // Update local values when prop changes
    $: {
        x = value[0];
        y = value[1];
    }
</script>

<div class="point-input">
    <div class="coordinate-group">
        <label class="coord-label">X</label>
        <div class="slider-container">
            <input
                type="range"
                {min}
                {max}
                step="0.01"
                value={x}
                on:input={handleXSlider}
                class="slider"
            />
            <div class="slider-track"></div>
            <div class="slider-fill" style="width: {xPercentage}%"></div>
            <div class="slider-thumb" style="left: {xPercentage}%"></div>
        </div>
        <input
            type="number"
            step="0.01"
            bind:value={x}
            on:input={handleXChange}
            {min}
            {max}
            class="number-input"
        />
    </div>
    
    <div class="coordinate-group">
        <label class="coord-label">Y</label>
        <div class="slider-container">
            <input
                type="range"
                {min}
                {max}
                step="0.01"
                value={y}
                on:input={handleYSlider}
                class="slider"
            />
            <div class="slider-track"></div>
            <div class="slider-fill" style="width: {yPercentage}%"></div>
            <div class="slider-thumb" style="left: {yPercentage}%"></div>
        </div>
        <input
            type="number"
            step="0.01"
            bind:value={y}
            on:input={handleYChange}
            {min}
            {max}
            class="number-input"
        />
    </div>
    
    <button on:click={reset} class="reset-btn" title="Reset to default">
        <RotateCcw size="12" />
    </button>
</div>

<style>
    .point-input {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        min-width: 200px;
    }

    .coordinate-group {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .coord-label {
        font-size: 0.75rem;
        font-weight: 500;
        color: var(--text-color);
        min-width: 12px;
        text-align: center;
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

    .number-input {
        width: 60px;
        padding: 0.25rem 0.375rem;
        background: rgba(255, 255, 255, 0.05);
        border: 1px solid var(--border-color);
        border-radius: 4px;
        color: var(--text-color);
        font-size: 0.75rem;
        text-align: center;
    }

    .number-input:focus {
        outline: none;
        border-color: var(--accent-color);
        background: rgba(255, 255, 255, 0.08);
    }

    .reset-btn {
        align-self: center;
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