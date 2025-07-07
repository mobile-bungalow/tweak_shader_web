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
    import {
        linter,
        type Diagnostic,
        forceLinting,
        lintGutter,
        setDiagnostics,
    } from "@codemirror/lint";
    import Input from "../components/Input.svelte";
    import ImageInput from "../components/Image.svelte";

    let inputs: Map<String, any> = new Map();
    let canvas: HTMLCanvasElement;
    let tweakShader: TweakShader;
    let src = DEFAULT_SHADER;
    let context: WgpuContext;
    let frameCount = 0;
    let start = Date.now();
    let webgpuSupported = false;
    let compilationErrors: Diagnostic[] = [];
    let generalError: string | null = null;
    let editorView: any = null;
    let needs_refresh = false;

    const shaderLinter = linter(
        (view) => {
            editorView = view; // Store the current view
            console.log(
                "Linter called with",
                compilationErrors.length,
                "errors",
            );
            return new Promise((resolve) => {
                // Resolve immediately with current diagnostics
                resolve(compilationErrors);
            });
        },
        {
            delay: 50,
            needsRefresh: () => {
                const ref = needs_refresh;
                needs_refresh = false;
                return ref;
            },
        },
    );

    // Function to force immediate linter update
    const updateLinter = () => {
        if (editorView) {
            needs_refresh = true;
            
            // Method 1: Directly set diagnostics
            try {
                editorView.dispatch(setDiagnostics(editorView.state, compilationErrors));
            } catch (e) {
                console.log("setDiagnostics failed:", e);
            }
            
            // Method 2: Force linting
            try {
                forceLinting(editorView);
            } catch (e) {
                console.log("forceLinting failed:", e);
            }
            
            // Method 3: Dispatch empty transaction to trigger updates
            setTimeout(() => {
                if (editorView) {
                    editorView.dispatch({});
                }
            }, 10);
        }
    };

    onMount(async () => {
        if (!navigator.gpu) {
            webgpuSupported = false;
            return;
        }

        webgpuSupported = true;
        await init();
        context = await initialize_library();
        try {
            tweakShader = new TweakShader(src, context);
            inputs = tweakShader.get_input_list();
            compilationErrors = [];
            generalError = null;
            updateLinter();
        } catch (e) {
            handleShaderError(e);
        }

        canvas.width = 800;
        canvas.height = 450;
        if (tweakShader) {
            draw();
        }
    });

    let last = Date.now();

    const draw = () => {
        if (!tweakShader || !canvas) {
            return;
        }

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

    const parseCompilationErrors = (errorString: string): Diagnostic[] => {
        const errors: Diagnostic[] = [];
        const lines = errorString.split("\n");
        const srcLines = src.split("\n");

        for (const line of lines) {
            if (line.trim()) {
                // Try to parse "at location line:column" format (e.g., "Unknown variable: normalized_coords at location 15:23")
                const locationMatch = line.match(/at location (\d+):(\d+)/i);
                if (locationMatch) {
                    let reportedLineNum = parseInt(locationMatch[1]) - 1; // Convert to 0-based
                    const col = parseInt(locationMatch[2]) - 1; // Convert to 0-based

                    // Count #pragma lines before the reported line to adjust line number
                    let pragmaCount = 0;
                    for (
                        let i = 0;
                        i < reportedLineNum && i < srcLines.length;
                        i++
                    ) {
                        if (srcLines[i].trim().startsWith("#pragma")) {
                            pragmaCount++;
                        }
                    }

                    // Adjust line number by adding back pragma lines
                    const actualLineNum = reportedLineNum + pragmaCount;

                    // Extract the message part before "at location"
                    const message = line.split(" at location")[0].trim();

                    // Calculate actual character position
                    let from = 0;
                    for (
                        let i = 0;
                        i < actualLineNum && i < srcLines.length;
                        i++
                    ) {
                        from += srcLines[i].length + 1; // +1 for newline
                    }
                    from += col;

                    errors.push({
                        from: Math.max(0, from),
                        to: Math.max(0, from + 1),
                        severity: "error",
                        message: message,
                    });
                } else {
                    // If no specific line info, add as general error at end
                    errors.push({
                        from: Math.max(0, src.length - 1),
                        to: src.length,
                        severity: "error",
                        message: line.trim(),
                    });
                }
            }
        }

        return errors;
    };

    const handleShaderError = (error: any) => {
        const errorString = error.toString();

        if (errorString.includes("location ") || errorString.includes("\n")) {
            compilationErrors = parseCompilationErrors(errorString);
            generalError = null;
        } else {
            generalError = errorString;
            compilationErrors = [];
        }

        updateLinter();
    };

    const recompile = () => {
        frameCount = 0;
        start = Date.now();
        try {
            compilationErrors = [];
            generalError = null;
            tweakShader.update_src(src, context);
            inputs = tweakShader.get_input_list();
            updateLinter(); // Force linter refresh on successful recompile
            if (tweakShader && canvas) {
                draw();
            }
        } catch (e) {
            handleShaderError(e);
        }
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
            {#if webgpuSupported}
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
            {:else}
                <div class="webgpu-placeholder">
                    <p>WebGPU is not supported in this browser.</p>
                    <p>
                        Please use a WebGPU-enabled browser to view this shader.
                    </p>
                </div>
            {/if}
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
                        extensions={vimMode
                            ? [vim(), glsl(), shaderLinter, lintGutter()]
                            : [glsl(), shaderLinter, lintGutter()]}
                        bind:value={src}
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
                {#if generalError}
                    <div class="general-error">
                        <strong>Error:</strong>
                        {generalError}
                    </div>
                {/if}
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
        max-width: 1600px;
        margin: 0 auto;
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

    .webgpu-placeholder {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        box-sizing: border-box;
        margin: 10px;
        flex: 1;
        width: 100%;
        min-width: 400px;
        max-width: 900px;
        min-height: 225px;
        border-radius: 4px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
        background-color: var(--bg-color);
        border: 2px dashed var(--text-color);
        color: var(--text-color);
        text-align: center;
        padding: 2rem;
    }

    .left-column,
    .right-column {
        padding: 1rem;
    }

    .right-column {
        flex: 1;
    }

    :global(.cm-selectionBackground) {
        background-color: #3a3a3a !important;
    }

    :global(.cm-focused .cm-selectionBackground) {
        background-color: #3a3a3a !important;
    }

    :global(.cm-editor .cm-selectionLayer .cm-selectionBackground) {
        background-color: #3a3a3a !important;
    }

    :global(.cm-gutters) {
        background-color: #1f1f1f !important;
        border-right: 1px solid #333 !important;
    }

    :global(.cm-lineNumbers .cm-gutterElement) {
        color: #666 !important;
    }

    .general-error {
        background-color: var(--error-color);
        color: white;
        padding: 0.75rem;
        margin: 0.5rem 0;
        border-radius: 4px;
        border-left: 4px solid #d32f2f;
        font-family: monospace;
        font-size: 0.9rem;
        white-space: pre-wrap;
    }

    /* CodeMirror lint styling */
    :global(.cm-diagnostic) {
        padding: 2px 4px 2px 6px;
        margin-left: -1px;
        display: block;
        white-space: pre-wrap;
        font-size: 0.8rem;
    }

    :global(.cm-diagnostic-error) {
        border-left: 4px solid #d11;
        background-color: rgba(221, 17, 17, 0.6);
    }

    :global(.cm-lintRange-error) {
        background-image: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" width="6" height="3"><path d="m0 3 l2 -2 l1 0 l2 2 l1 0" stroke="%23d11" fill="none" stroke-width=".7"/></svg>');
        background-repeat: repeat-x;
        background-position: left bottom;
    }

    /* Lint gutter styling */
    :global(.cm-gutter-lint) {
        width: 1.2em;
        background-color: #1a1a1a !important;
    }

    :global(.cm-lint-marker) {
        width: 1em;
        height: 1em;
        border-radius: 50%;
        margin: 0.1em;
    }

    :global(.cm-lint-marker-error) {
        background-color: #d11;
        position: relative;
    }

    /* Always show error tooltips */
    :global(.cm-lint-marker-error::after) {
        content: attr(title);
        position: absolute;
        left: 1.5em;
        top: 0;
        background: rgba(51, 51, 51, 0.95);
        color: #fff;
        padding: 4px 8px;
        border-radius: 3px;
        font-size: 0.75rem;
        white-space: nowrap;
        z-index: 1000;
        border: 1px solid #666;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
        pointer-events: none;
    }

    /* Show diagnostic ranges without hover */
    :global(.cm-lintPoint) {
        position: relative;
    }

    :global(.cm-lintPoint::after) {
        content: attr(data-message);
        position: absolute;
        left: 0;
        top: 1.2em;
        background: rgba(51, 51, 51, 0.95);
        color: #fff;
        padding: 2px 6px;
        border-radius: 3px;
        font-size: 0.7rem;
        white-space: nowrap;
        z-index: 999;
        border: 1px solid #666;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
        pointer-events: none;
    }
</style>
