<script lang="ts">
    import { onMount } from "svelte";
    import { DEFAULT_SHADER } from "$lib";
    import init, { TweakShader, initialize_library } from "tweak_shader_wasm";
    import CodeMirror from "svelte-codemirror-editor";
    import { glsl } from "codemirror-lang-glsl";

    let canvas: HTMLCanvasElement;

    onMount(async () => {
        await init();
        let context = await initialize_library();
        let tweakShader = new TweakShader(DEFAULT_SHADER, context);
        tweakShader.update_resolution(canvas.width, canvas.height);
        tweakShader.render(canvas);
    });
</script>

<main>
    <div class="container">
        <div class="leftColumn">
            <canvas width="800" height="450" bind:this={canvas}></canvas>
            <div class="inputs"></div>
        </div>
        <CodeMirror value={DEFAULT_SHADER} lang={glsl()}></CodeMirror>
    </div>
</main>

<style>
    @import "../../static/app.css";

    .container {
        background-color: var(--bg-color);
    }

    canvas {
        image-rendering: pixelated;
        background-color: black;
        border: 1px solid var(--border-color);
        border-radius: 4px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
    }
</style>
