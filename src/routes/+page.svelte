<script lang="ts">
    import { onMount } from "svelte";
    import { DEFAULT_SHADER, theme } from "$lib";
    import init, {
        TweakShader,
        WgpuContext,
        initialize_library,
    } from "tweak_shader_wasm";
    import CodeMirror from "svelte-codemirror-editor";
    import { vim } from "@replit/codemirror-vim";
    import { glsl } from "codemirror-lang-glsl";

    let canvas: HTMLCanvasElement;
    let tweakShader: TweakShader;
    let src = DEFAULT_SHADER;
    let context: WgpuContext;
    let frameCount = 0;
    let start = Date.now();

    onMount(async () => {
        await init();
        context = await initialize_library();
        tweakShader = new TweakShader(src, context);

        canvas.width = 800;
        canvas.height = 450;
        draw();
    });

    let last = Date.now();
    const draw = () => {
        // collect inputs and call tweakShader.set_input(...)
        // update time and date
        frameCount += 1;
        const time = Date.now();
        const elapsed = time - start;
        const delta = last - time;
        last = Date.now();
        // update mouse
        tweakShader.update_resolution(canvas.width, canvas.height);
        tweakShader.update_frame_count(frameCount);
        tweakShader.update_time(elapsed / 1000.0);
        tweakShader.update_delta(delta / 1000.0);
        tweakShader.render(canvas);

        if (!paused) {
            requestAnimationFrame(draw);
        }
    };

    const recompile = () => {
        frameCount = 0;
        start = Date.now();
        console.log(src, context);
        tweakShader.update_src(src);
        draw();
    };

    let paused = false;
    const togglePause = () => {
        paused = !paused;

        if (!paused) {
            requestAnimationFrame(draw);
        }
    };

    let vimMode = false;
    const toggleVim = () => {
        vimMode = !vimMode;
    };
</script>

<main>
    <div class="container">
        <div class="leftColumn">
            <canvas bind:this={canvas}></canvas>
            <div class="controls">
                <div class="stats"></div>
                <button onclick={togglePause} aria-label="pause">Pause</button>
                <button aria-label="ar">Aspect</button>
                <div class="outputSelector"></div>
            </div>
            <div class="inputs"></div>
        </div>
        <div class="rightColumn">
            <div class="editor">
                <div class="code-editor-wrapper">
                    <CodeMirror
                        class="codemirror"
                        {theme}
                        extensions={vimMode ? [vim()] : []}
                        bind:value={src}
                        lang={glsl()}
                    ></CodeMirror>
                </div>
                <button onclick={recompile} aria-label="recompile"
                    >Recompile</button
                >
                <button aria-label="save">Save</button>
                <button onclick={toggleVim} aria-label="vimKeybinds"
                    >Vim Mode</button
                >
            </div>
            <div class="texturePreviews"></div>
        </div>
    </div>
</main>

<style>
    @import "../../static/app.css";

    .container {
        background-color: var(--bg-color);
        display: flex;
        flex-direction: column;
    }

    @media (min-width: 768px) {
        .container {
            flex-direction: row;
        }
    }

    .code-editor-wrapper {
        max-width: 750px;
    }

    canvas {
        image-rendering: pixelated;
        flex: 1;
        width: 100%;
        min-width: 400px;
        max-width: 900px;
        border-radius: 4px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
        height: auto;
    }

    .leftColumn,
    .rightColumn {
        padding: 1rem;
    }

    .rightColumn {
        flex: 1;
    }
</style>
