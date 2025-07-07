<script lang="ts">
    import { RotateCcw } from "lucide-svelte";
    export let value: [number, number, number];
    export let change: (val: [number, number, number]) => void;
    export let _default: [number, number, number, number] = [1.0, 1.0, 1.0, 1];

    let r = value[0];
    let g = value[1];
    let b = value[2];

    const handleChannelChange = (
        channel: "r" | "g" | "b",
        newValue: number,
    ) => {
        if (channel === "r") r = newValue;
        if (channel === "g") g = newValue;
        if (channel === "b") b = newValue;

        value = [r, g, b, 255];
        change(value);
    };

    const handleRSlider = (e: Event) => {
        const target = e.target as HTMLInputElement;
        handleChannelChange("r", Number(target.value));
    };

    const handleGSlider = (e: Event) => {
        const target = e.target as HTMLInputElement;
        handleChannelChange("g", Number(target.value));
    };

    const handleBSlider = (e: Event) => {
        const target = e.target as HTMLInputElement;
        handleChannelChange("b", Number(target.value));
    };

    const handleHexChange = (e: Event) => {
        const target = e.target as HTMLInputElement;
        const hex = target.value;
        if (hex.match(/^#[0-9A-Fa-f]{6}$/)) {
            const r = parseInt(hex.slice(1, 3), 16) / 255;
            const g = parseInt(hex.slice(3, 5), 16) / 255;
            const b = parseInt(hex.slice(5, 7), 16) / 255;
            handleChannelChange("r", r);
            handleChannelChange("g", g);
            handleChannelChange("b", b);
        }
    };

    const reset = () => {
        r = _default[0];
        g = _default[1];
        b = _default[2];
        value = [r, g, b];
        change(value);
    };

    $: {
        r = value[0];
        g = value[1];
        b = value[2];
    }

    $: hexValue =
        "#" +
        Math.round(r * 255)
            .toString(16)
            .padStart(2, "0") +
        Math.round(g * 255)
            .toString(16)
            .padStart(2, "0") +
        Math.round(b * 255)
            .toString(16)
            .padStart(2, "0");

    $: rgbString = `rgb(${Math.round(r * 255)}, ${Math.round(g * 255)}, ${Math.round(b * 255)})`;
</script>

<div class="color-input">
    <div class="channels">
        <div class="channel-group">
            <label class="channel-label" style="color: #ff6b6b">R</label>
            <div class="slider-container">
                <input
                    type="range"
                    min="0"
                    max="1"
                    step="0.01"
                    value={r}
                    on:input={handleRSlider}
                    class="slider red-slider"
                />
                <div class="slider-track"></div>
                <div
                    class="slider-fill red-fill"
                    style="width: {r * 100}%"
                ></div>
                <div class="slider-thumb" style="left: {r * 100}%"></div>
            </div>
            <input
                type="number"
                step="0.01"
                bind:value={r}
                on:input={() => handleChannelChange("r", r)}
                min="0"
                max="1"
                class="number-input"
            />
        </div>

        <div class="channel-group">
            <label class="channel-label" style="color: #51cf66">G</label>
            <div class="slider-container">
                <input
                    type="range"
                    min="0"
                    max="1"
                    step="0.01"
                    value={g}
                    on:input={handleGSlider}
                    class="slider green-slider"
                />
                <div class="slider-track"></div>
                <div
                    class="slider-fill green-fill"
                    style="width: {g * 100}%"
                ></div>
                <div class="slider-thumb" style="left: {g * 100}%"></div>
            </div>
            <input
                type="number"
                step="0.01"
                bind:value={g}
                on:input={() => handleChannelChange("g", g)}
                min="0"
                max="1"
                class="number-input"
            />
        </div>

        <div class="channel-group">
            <label class="channel-label" style="color: #4dabf7">B</label>
            <div class="slider-container">
                <input
                    type="range"
                    min="0"
                    max="1"
                    step="0.01"
                    value={b}
                    on:input={handleBSlider}
                    class="slider blue-slider"
                />
                <div class="slider-track"></div>
                <div
                    class="slider-fill blue-fill"
                    style="width: {b * 100}%"
                ></div>
                <div class="slider-thumb" style="left: {b * 100}%"></div>
            </div>
            <input
                type="number"
                step="0.01"
                bind:value={b}
                on:input={() => handleChannelChange("b", b)}
                min="0"
                max="1"
                class="number-input"
            />
        </div>
    </div>

    <div class="color-controls">
        <input
            type="color"
            value={hexValue}
            on:change={handleHexChange}
            class="color-picker"
            title="Color picker"
        />
        <input
            type="text"
            value={hexValue}
            on:change={handleHexChange}
            class="hex-input"
            placeholder="#ffffff"
        />
        <button on:click={reset} class="reset-btn" title="Reset to default">
            <RotateCcw size="14" />
        </button>
    </div>
</div>

<style>
    .color-input {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        min-width: 200px;
    }

    .color-preview {
        width: 100%;
        height: 32px;
        border-radius: 6px;
        border: 2px solid var(--border-color);
        box-shadow: inset 0 1px 3px rgba(0, 0, 0, 0.2);
    }

    .channels {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .channel-group {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .channel-label {
        font-size: 0.75rem;
        font-weight: bold;
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
        border-radius: 3px;
    }

    .red-fill {
        background: #ff6b6b;
    }

    .green-fill {
        background: #51cf66;
    }

    .blue-fill {
        background: #4dabf7;
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
        width: 50px;
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

    .color-controls {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .color-picker {
        width: 32px;
        height: 32px;
        padding: 0;
        border: 2px solid var(--border-color);
        border-radius: 6px;
        cursor: pointer;
        background: none;
    }

    .color-picker::-webkit-color-swatch-wrapper {
        padding: 0;
        border-radius: 4px;
        overflow: hidden;
    }

    .color-picker::-webkit-color-swatch {
        border: none;
        border-radius: 4px;
    }

    .hex-input {
        flex: 1;
        padding: 0.375rem 0.5rem;
        background: rgba(255, 255, 255, 0.05);
        border: 1px solid var(--border-color);
        border-radius: 4px;
        color: var(--text-color);
        font-size: 0.875rem;
        font-family: monospace;
    }

    .hex-input:focus {
        outline: none;
        border-color: var(--accent-color);
        background: rgba(255, 255, 255, 0.08);
    }

    .reset-btn {
        width: 32px;
        height: 32px;
        padding: 0;
        background: rgba(255, 255, 255, 0.05);
        border: 1px solid var(--border-color);
        border-radius: 6px;
        color: var(--text-color);
        cursor: pointer;
        font-size: 0.875rem;
        display: flex;
        align-items: center;
        justify-content: center;
    }
</style>
