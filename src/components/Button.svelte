<script lang="ts">
    export let variant: 'primary' | 'secondary' | 'danger' | 'success' = 'secondary';
    export let size: 'sm' | 'md' | 'lg' = 'md';
    export let disabled = false;
    export let active = false;
    export let title = '';
    export let onclick: (() => void) | undefined = undefined;
</script>

<button
    class="btn"
    class:primary={variant === 'primary'}
    class:secondary={variant === 'secondary'}
    class:danger={variant === 'danger'}
    class:success={variant === 'success'}
    class:sm={size === 'sm'}
    class:md={size === 'md'}
    class:lg={size === 'lg'}
    class:active
    {disabled}
    {title}
    on:click={onclick}
>
    <slot />
</button>

<style>
    .btn {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        border: 1px solid;
        border-radius: 6px;
        font-weight: 500;
        cursor: pointer;
        font-family: inherit;
        white-space: nowrap;
        user-select: none;
    }

    .btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
        pointer-events: none;
    }

    /* Sizes */
    .btn.sm {
        padding: 0.375rem 0.75rem;
        font-size: 0.875rem;
        min-height: 32px;
    }

    .btn.md {
        padding: 0.5rem 1rem;
        font-size: 0.875rem;
        min-height: 36px;
    }

    .btn.lg {
        padding: 0.75rem 1.5rem;
        font-size: 1rem;
        min-height: 44px;
    }

    /* Variants */
    .btn.primary {
        background: var(--accent-color);
        border-color: var(--accent-color);
        color: white;
    }


    .btn.primary.active {
        background: #007ba3;
        border-color: #007ba3;
    }

    .btn.secondary {
        background: rgba(255, 255, 255, 0.05);
        border-color: var(--border-color);
        color: var(--text-color);
    }


    .btn.secondary.active {
        background: rgba(255, 255, 255, 0.15);
        border-color: var(--accent-color);
        color: var(--accent-color);
    }

    .btn.success {
        background: var(--success-color);
        border-color: var(--success-color);
        color: white;
    }


    .btn.danger {
        background: var(--error-color);
        border-color: var(--error-color);
        color: white;
    }

</style>