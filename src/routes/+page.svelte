<script lang="ts">
    import { onMount } from "svelte";
    import { DEFAULT_SHADER, theme } from "$lib";
    import init, { TweakShader, initialize_library } from "tweak_shader_wasm";
    import CodeMirror from "svelte-codemirror-editor";
    import { Compartment } from "@codemirror/state";
    import { vim } from "@replit/codemirror-vim";
    import { glsl } from "codemirror-lang-glsl";

    let editor: CodeMirror;
    let canvas: HTMLCanvasElement;
    let tweakShader: TweakShader;

    onMount(async () => {
        await init();
        let context = await initialize_library();
        tweakShader = new TweakShader(DEFAULT_SHADER, context);

        canvas.width = 700;
        canvas.height = 400;
        tweakShader.update_resolution(canvas.width, canvas.height);
        tweakShader.render(canvas);
    });

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
                <button aria-label="pause">Pause</button>
                <div class="outputSelector"></div>
            </div>
            <div class="inputs"></div>
        </div>
        <div class="rightColumn">
            <div class="editor">
                <div class="codemirrorWrapper">
                    <CodeMirror
                        class="codemirror"
                        bind:this={editor}
                        {theme}
                        extensions={vimMode ? [vim()] : []}
                        value={DEFAULT_SHADER}
                        lang={glsl()}
                    ></CodeMirror>
                </div>
                <button aria-label="recompile">Recompile</button>
                <button aria-label="save">Save</button>
                <button onclick={toggleVim} aria-label="vimKeybinds">Vim</button
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

    canvas {
        image-rendering: pixelated;
        width: 100%;
        min-width: 400px;
        max-width: 800px;
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
