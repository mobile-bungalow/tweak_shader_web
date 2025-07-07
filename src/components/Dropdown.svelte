<script lang="ts">
    export let options: { value: string; label: string }[];
    export let value: string;
    export let placeholder = 'Select...';
    export let onChange: (value: string) => void;

    let isOpen = false;
    let dropdownRef: HTMLDivElement;

    const toggleDropdown = () => {
        isOpen = !isOpen;
    };

    const selectOption = (optionValue: string) => {
        value = optionValue;
        onChange(optionValue);
        isOpen = false;
    };

    const handleClickOutside = (event: MouseEvent) => {
        if (dropdownRef && !dropdownRef.contains(event.target as Node)) {
            isOpen = false;
        }
    };

    $: selectedOption = options.find(opt => opt.value === value);
</script>

<svelte:window on:click={handleClickOutside} />

<div class="dropdown" bind:this={dropdownRef}>
    <button class="dropdown-trigger" on:click={toggleDropdown} class:open={isOpen}>
        <span class="dropdown-text">
            {selectedOption?.label || placeholder}
        </span>
        <svg class="dropdown-arrow" class:rotated={isOpen} viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
        </svg>
    </button>
    
    {#if isOpen}
        <div class="dropdown-menu">
            {#each options as option}
                <button 
                    class="dropdown-option"
                    class:selected={option.value === value}
                    on:click={() => selectOption(option.value)}
                >
                    {option.label}
                </button>
            {/each}
        </div>
    {/if}
</div>

<style>
    .dropdown {
        position: relative;
        display: inline-block;
        min-width: 140px;
    }

    .dropdown-trigger {
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 0.5rem 0.75rem;
        background: rgba(255, 255, 255, 0.05);
        border: 1px solid var(--border-color);
        border-radius: 6px;
        color: var(--text-color);
        font-size: 0.875rem;
        cursor: pointer;
    }


    .dropdown-trigger.open {
        border-color: var(--accent-color);
        background: rgba(255, 255, 255, 0.08);
    }

    .dropdown-text {
        flex: 1;
        text-align: left;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .dropdown-arrow {
        width: 16px;
        height: 16px;
        flex-shrink: 0;
    }

    .dropdown-arrow.rotated {
        transform: rotate(180deg);
    }

    .dropdown-menu {
        position: absolute;
        top: 100%;
        left: 0;
        right: 0;
        margin-top: 4px;
        background: rgba(26, 26, 26, 0.95);
        border: 1px solid var(--border-color);
        border-radius: 6px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
        backdrop-filter: blur(8px);
        z-index: 1000;
        max-height: 200px;
        overflow-y: auto;
    }

    .dropdown-option {
        width: 100%;
        display: block;
        padding: 0.5rem 0.75rem;
        background: none;
        border: none;
        color: var(--text-color);
        font-size: 0.875rem;
        cursor: pointer;
        text-align: left;
    }


    .dropdown-option.selected {
        background: var(--accent-color);
        color: white;
    }

    .dropdown-option:first-child {
        border-radius: 6px 6px 0 0;
    }

    .dropdown-option:last-child {
        border-radius: 0 0 6px 6px;
    }

    .dropdown-option:only-child {
        border-radius: 6px;
    }
</style>