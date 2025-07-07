<script lang="ts">
    import { onMount } from "svelte";
    import { base } from "$app/paths";

    export let change: (val?: ImageData) => void;
    export let name: string = "input_image";

    let fileInput: HTMLInputElement;
    let imagePreview: string | null = null;
    let isDragging = false;

    // Load placeholder image on mount
    onMount(() => {
        loadPlaceholderImage();
    });

    const loadPlaceholderImage = () => {
        const img = new Image();
        img.onload = () => {
            const canvas = document.createElement("canvas");
            const ctx = canvas.getContext("2d");
            if (ctx) {
                canvas.width = img.width;
                canvas.height = img.height;
                ctx.drawImage(img, 0, 0);
                const imageData = ctx.getImageData(0, 0, img.width, img.height);
                change(imageData);
            }
        };
        img.crossOrigin = "anonymous";
        img.src = `${base}/wolfie.png`;
    };

    const handleFileSelect = (event: Event) => {
        const target = event.target as HTMLInputElement;
        if (target.files && target.files[0]) {
            processFile(target.files[0]);
        }
    };

    const handleDrop = (event: DragEvent) => {
        event.preventDefault();
        isDragging = false;

        if (event.dataTransfer?.files && event.dataTransfer.files[0]) {
            processFile(event.dataTransfer.files[0]);
        }
    };

    const handleDragOver = (event: DragEvent) => {
        event.preventDefault();
        isDragging = true;
    };

    const handleDragLeave = (event: DragEvent) => {
        event.preventDefault();
        isDragging = false;
    };

    const processFile = (file: File) => {
        const reader = new FileReader();
        reader.onload = (e) => {
            imagePreview = e.target?.result as string;
            const img = new Image();
            img.onload = () => {
                const canvas = document.createElement("canvas");
                const ctx = canvas.getContext("2d");
                if (ctx) {
                    canvas.width = img.width;
                    canvas.height = img.height;
                    ctx.drawImage(img, 0, 0);
                    const imageData = ctx.getImageData(
                        0,
                        0,
                        img.width,
                        img.height,
                    );
                    change(imageData);
                }
            };
            img.src = imagePreview;
        };
        reader.readAsDataURL(file);
    };

    const removeImage = () => {
        imagePreview = null;
        if (fileInput) {
            fileInput.value = "";
        }
        loadPlaceholderImage();
    };

    const openFileDialog = () => {
        fileInput.click();
    };
</script>

<div class="image-input">
    <div
        class="image-preview {isDragging ? 'dragging' : ''}"
        on:drop={handleDrop}
        on:dragover={handleDragOver}
        on:dragleave={handleDragLeave}
        on:click={openFileDialog}
    >
        {#if imagePreview}
            <img src={imagePreview} alt="Preview" class="preview-image" />
            <button class="remove-btn" on:click|stopPropagation={removeImage}>
                ×
            </button>
        {:else}
            <img
                src="{base}/wolfie.png"
                alt="Placeholder"
                class="placeholder-image"
            />
            <div class="upload-text">
                <p>Click to upload or drag & drop</p>
                <p class="file-types">PNG, JPG, GIF</p>
            </div>
        {/if}
    </div>

    <input
        type="file"
        accept="image/*"
        bind:this={fileInput}
        on:change={handleFileSelect}
        class="file-input"
    />
</div>

<style>
    .image-input {
        width: 100%;
        max-width: 300px;
    }

    .image-preview {
        position: relative;
        width: 100%;
        height: 200px;
        border: 2px dashed var(--border-color);
        border-radius: 8px;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        background: rgba(255, 255, 255, 0.02);
        transition: all 0.2s ease;
        overflow: hidden;
    }

    .image-preview:hover {
        border-color: var(--accent-color);
        background: rgba(255, 255, 255, 0.05);
    }

    .image-preview.dragging {
        border-color: var(--accent-color);
        background: rgba(255, 255, 255, 0.08);
    }

    .preview-image {
        width: 100%;
        height: 100%;
        object-fit: cover;
        border-radius: 6px;
    }

    .placeholder-image {
        width: 80px;
        height: 80px;
        opacity: 0.6;
        border-radius: 4px;
        margin-bottom: 0.5rem;
    }

    .upload-text {
        text-align: center;
        color: var(--text-color);
        opacity: 0.8;
    }

    .upload-text p {
        margin: 0;
        font-size: 0.875rem;
    }

    .file-types {
        font-size: 0.75rem;
        opacity: 0.6;
        margin-top: 0.25rem;
    }

    .remove-btn {
        position: absolute;
        top: 8px;
        right: 8px;
        width: 24px;
        height: 24px;
        border: none;
        border-radius: 50%;
        background: rgba(0, 0, 0, 0.7);
        color: white;
        cursor: pointer;
        font-size: 16px;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: background-color 0.2s ease;
    }

    .remove-btn:hover {
        background: rgba(255, 0, 0, 0.7);
    }

    .file-input {
        display: none;
    }
</style>
