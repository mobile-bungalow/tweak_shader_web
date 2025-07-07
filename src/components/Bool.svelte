<script lang="ts">
    export let value: boolean;
    export let change: (val: boolean) => void;
    export let _default: boolean = false;

    const toggle = () => {
        value = !value;
        change(value);
    };

    const reset = () => {
        value = _default;
        change(value);
    };
</script>

<div class="bool-input">
    <button
        on:click={toggle}
        class="toggle-switch"
        class:active={value}
        aria-pressed={value}
    >
        <div class="toggle-track">
            <div class="toggle-thumb" class:active={value}></div>
        </div>
        <span class="toggle-label">{value ? 'ON' : 'OFF'}</span>
    </button>
    <button on:click={reset} class="reset-btn" title="Reset to default">º</button>
</div>

<style>
    .bool-input {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .toggle-switch {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        background: none;
        border: none;
        cursor: pointer;
        padding: 0;
        color: var(--text-color);
        font-size: 0.875rem;
    }

    .toggle-track {
        position: relative;
        width: 40px;
        height: 20px;
        background: var(--border-color);
        border-radius: 10px;
        transition: background-color 0.2s ease;
    }

    .toggle-switch.active .toggle-track {
        background: var(--accent-color);
    }

    .toggle-thumb {
        position: absolute;
        top: 2px;
        left: 2px;
        width: 16px;
        height: 16px;
        background: white;
        border-radius: 50%;
        transition: transform 0.2s ease;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
    }

    .toggle-thumb.active {
        transform: translateX(20px);
    }

    .toggle-label {
        font-weight: 500;
        min-width: 24px;
        text-align: center;
    }

    .toggle-switch.active .toggle-label {
        color: var(--accent-color);
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
        transition: all 0.2s ease;
    }

    .reset-btn:hover {
        background: rgba(255, 255, 255, 0.1);
        border-color: var(--accent-color);
    }
</style>