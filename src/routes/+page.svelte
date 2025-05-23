<script lang="ts">
    import { onMount } from "svelte";
    import { DEFAULT_SHADER, inputValueToProps, theme } from "$lib";
    import init, {
        TweakShader,
        WgpuContext,
        initialize_library,
    } from "tweak_shader_wasm";
    import CodeMirror from "svelte-codemirror-editor";
    import { vim } from "@replit/codemirror-vim";
    import { glsl } from "codemirror-lang-glsl";
    import Input from "../components/Input.svelte";
    import ImageInput from "../components/Image.svelte";

    let inputs: Map<String, any> = new Map();
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
        inputs = tweakShader.get_input_list();

        canvas.width = 800;
        canvas.height = 450;
        draw();
    });

    let last = Date.now();

    const draw = () => {
        frameCount += 1;
        const time = Date.now();
        const elapsed = time - start;
        const delta = last - time;
        last = Date.now();
        let now = new Date();

        tweakShader.update_resolution(canvas.width, canvas.height);
        tweakShader.update_frame_count(frameCount);
        tweakShader.update_time(elapsed / 1000.0);
        tweakShader.update_datetime(
            now.getFullYear(),
            now.getMonth(),
            now.getDay(),
            now.getSeconds(),
        );
        tweakShader.update_delta(delta / 1000.0);
        tweakShader.render(canvas);

        if (!paused) {
            requestAnimationFrame(draw);
        }
    };

    const recompile = () => {
        frameCount = 0;
        start = Date.now();
        tweakShader.update_src(src);
        inputs = tweakShader.get_input_list();
        inputs.forEach((v, k) => {
            console.log(v, k);
            tweakShader.set_input(k.toString(), v.current);
        });
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
        <div class="left-column">
            <canvas
                bind:this={canvas}
                onmouseup={() => {
                    tweakShader.set_mouse_up();
                }}
                onmousedown={() => {
                    tweakShader.set_mouse_down();
                }}
                onmousemove={(ev) => {
                    tweakShader.set_mouse_position(ev.clientX, ev.clientY);
                }}
            ></canvas>
            <div class="controls">
                <div class="stats"></div>
                <button onclick={togglePause} aria-label="pause">Pause</button>
                <button aria-label="aspect ratio">Aspect</button>
                <button aria-label="select target">Target</button>
                <div class="outputSelector"></div>
            </div>
            <div class="inputs">
                {#each Array.from(inputs) as [k, v]}
                    <Input
                        label={k.toString()}
                        input={v}
                        change={(val) => {
                            tweakShader.set_input(k.toString(), val);
                        }}
                    ></Input>
                {/each}
            </div>
        </div>
        <div class="right-column">
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
                <div class="button-row"></div>
                <button onclick={recompile} aria-label="recompile"
                    >Recompile</button
                >
                <button aria-label="save">Save</button>
                <button onclick={toggleVim} aria-label="vimKeybinds"
                    >Vim Mode</button
                >
                <div class="image-inputs">
                    {#each Array.from(inputs) as [k, v]}
                        {#if v.type == "Image"}
                            <p>{k}</p>
                            <ImageInput
                                {...inputValueToProps(v)}
                                change={(val) => {
                                    if (val != undefined) {
                                        tweakShader.load_texture(
                                            k.toString(),
                                            val,
                                        );
                                    } else {
                                        tweakShader.remove_texture(
                                            k.toString(),
                                        );
                                    }
                                }}
                            ></ImageInput>
                        {/if}
                    {/each}
                </div>
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

    .left-column,
    .right-column {
        padding: 1rem;
    }

    .right-column {
        flex: 1;
    }
</style>
