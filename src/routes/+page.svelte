<script lang="ts">
    import { onMount } from "svelte";
    import { DEFAULT_SHADER, inputValueToProps, theme } from "$lib";
    import { base } from "$app/paths";
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
    import {
        autocompletion,
        CompletionContext,
        startCompletion,
        acceptCompletion,
        completionStatus,
        closeCompletion,
        moveCompletionSelection,
        completionKeymap,
    } from "@codemirror/autocomplete";
    import { keymap, EditorView } from "@codemirror/view";
    import { indentMore, indentLess, defaultKeymap } from "@codemirror/commands";
    import { Prec } from "@codemirror/state";
    import Input from "../components/Input.svelte";
    import ImageInput from "../components/Image.svelte";
    import Button from "../components/Button.svelte";
    import Dropdown from "../components/Dropdown.svelte";

    import {
        Play,
        Pause,
        RotateCcw,
        Save,
        Keyboard,
        Link2,
    } from "lucide-svelte";
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
    let editorKey = 0;

    const glslKeywords = [
        "vec2",
        "vec3",
        "vec4",
        "ivec2",
        "ivec3",
        "ivec4",
        "bvec2",
        "bvec3",
        "bvec4",
        "mat2",
        "mat3",
        "mat4",
        "sampler2D",
        "samplerCube",
        "sampler3D",
        "uniform",
        "layout",
        "binding",
        "location",
        "writeonly",
        "readonly",
        "local_size_x",
        "local_size_y",
        "local_size_z",
    ];

    const glslBuiltins = [
        "gl_GlobalInvocationID",
        "gl_LocalInvocationID",
        "gl_WorkGroupID",
        "gl_Position",
        "gl_FragCoord",
        "gl_VertexID",
        "gl_InstanceID",
        "normalize",
        "smoothstep",
        "inversesqrt",
        "imageStore",
        "imageLoad",
        "imageSize",
        "memoryBarrier",
        "texture2D",
        "textureCube",
    ];

    const extractIdentifiers = (source: string) => {
        const identifiers = new Set<string>();

        // Match variable declarations and function names
        const patterns = [
            /(?:float|int|vec[234]|ivec[234]|mat[234]|bool)\s+(\w+)/g,
            /uniform\s+\w+\s+(\w+)/g,
            /(\w+)\s*\(/g, // function calls
        ];

        patterns.forEach((pattern) => {
            let match;
            while ((match = pattern.exec(source)) !== null) {
                const identifier = match[1];
                if (
                    identifier.length > 2 &&
                    !glslKeywords.includes(identifier) &&
                    !glslBuiltins.includes(identifier)
                ) {
                    identifiers.add(identifier);
                }
            }
        });

        return Array.from(identifiers);
    };

    const createCompletionOptions = () => {
        const contextIdentifiers = extractIdentifiers(src);
        return [
            ...glslKeywords.map((keyword) => ({
                label: keyword,
                type: "keyword",
            })),
            ...glslBuiltins.map((builtin) => ({
                label: builtin,
                type: "function",
            })),
            ...contextIdentifiers.map((identifier) => ({
                label: identifier,
                type: "variable",
            })),
        ];
    };

    const glslCompletions = (context: CompletionContext) => {
        const word = context.matchBefore(/\w*/);
        if (!word) return null;
        if (word.from == word.to && !context.explicit) return null;

        const options = createCompletionOptions();
        return {
            from: word.from,
            options: options.filter((opt) =>
                opt.label.toLowerCase().startsWith(word.text.toLowerCase()),
            ),
        };
    };

    const shaderLinter = linter(
        (view) => {
            editorView = view;
            return new Promise((resolve) => {
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

    const updateLinter = () => {
        if (!editorView) return;

        needs_refresh = true;

        try {
            editorView.dispatch(
                setDiagnostics(editorView.state, compilationErrors),
            );
            forceLinting(editorView);
            setTimeout(() => editorView?.dispatch({}), 10);
        } catch (e) {
            console.error("Linter update failed:", e);
        }
    };

    let last = Date.now();

    const loadShaderFromUrl = async () => {
        const urlParams = new URLSearchParams(window.location.search);
        const fileParam = urlParams.get("file");
        const shaderParam = urlParams.get("shader");

        if (fileParam) {
            try {
                const response = await fetch(`${base}${fileParam}`);
                const shaderCode = await response.text();
                src = shaderCode;
                editorKey++;
            } catch (error) {
                console.error("Failed to load shader from file:", error);
            }
        } else if (shaderParam) {
            try {
                src = atob(decodeURIComponent(shaderParam));
                editorKey++;
            } catch (error) {
                console.error("Failed to decode shader from URL:", error);
            }
        }
    };

    const initializeWebGPU = async () => {
        if (!navigator.gpu) {
            webgpuSupported = false;
            return false;
        }

        webgpuSupported = true;
        await init();
        context = await initialize_library();
        return true;
    };

    const initializeShader = async () => {
        try {
            tweakShader = await new TweakShader(src, context);
            inputs = tweakShader.get_input_list();
            compilationErrors = [];
            generalError = null;
            updateLinter();
        } catch (e) {
            handleShaderError(e);
        }
    };

    const setupCanvas = () => {
        canvas.width = 800;
        canvas.height = 450;
        if (tweakShader) draw();
    };

    onMount(async () => {
        await loadShaderFromUrl();

        const webgpuReady = await initializeWebGPU();
        if (!webgpuReady) return;

        await initializeShader();
        setupCanvas();
    });

    const draw = () => {
        if (!tweakShader || !canvas) return;

        frameCount++;
        const time = Date.now();
        const elapsed = time - start;
        const delta = last - time;
        last = Date.now();
        const now = new Date();

        tweakShader.update_resolution(canvasWidth, canvasHeight);
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

        if (!paused) requestAnimationFrame(draw);
    };

    const calculateErrorPosition = (
        reportedLineNum: number,
        col: number,
        srcLines: string[],
    ) => {
        const pragmaCount = srcLines
            .slice(0, reportedLineNum)
            .filter((line) => line.trim().startsWith("#pragma")).length;

        const actualLineNum = reportedLineNum + pragmaCount;
        let from = 0;
        
        // Calculate position more carefully to avoid out-of-bounds
        for (let i = 0; i < Math.min(actualLineNum, srcLines.length); i++) {
            from += srcLines[i].length + 1; // +1 for newline
        }
        
        // Clamp column to line length if we have a valid line
        if (actualLineNum < srcLines.length && col >= 0) {
            const lineLength = srcLines[actualLineNum].length;
            from += Math.min(col, lineLength);
        } else if (col >= 0) {
            from += col;
        }

        const maxPos = src.length;
        const safeFrom = Math.max(0, Math.min(from, maxPos));
        const safeTo = Math.max(0, Math.min(from + 1, maxPos));
        
        return {
            from: safeFrom,
            to: Math.max(safeFrom, safeTo),
        };
    };

    const parseLocationError = (
        line: string,
        srcLines: string[],
    ): Diagnostic | null => {
        const locationMatch = line.match(/at location (\d+):(\d+)/i);
        if (!locationMatch) return null;

        const reportedLineNum = parseInt(locationMatch[1]) - 1;
        const col = parseInt(locationMatch[2]) - 1;
        const message = line.split(" at location")[0].trim();
        const { from, to } = calculateErrorPosition(
            reportedLineNum,
            col,
            srcLines,
        );

        return { from, to, severity: "error", message };
    };

    const parseGeneralError = (line: string): Diagnostic => {
        const maxPos = Math.max(1, src.length);
        const safeFrom = Math.max(0, maxPos - 1);
        const safeTo = maxPos;
        return {
            from: safeFrom,
            to: safeTo,
            severity: "error",
            message: line.trim(),
        };
    };

    const parseCompilationErrors = (errorString: string): Diagnostic[] => {
        const errors: Diagnostic[] = [];
        const lines = errorString.split("\n");
        const srcLines = src.split("\n");

        for (const line of lines) {
            if (!line.trim()) continue;

            const locationError = parseLocationError(line, srcLines);
            if (locationError) {
                errors.push(locationError);
            } else {
                errors.push(parseGeneralError(line));
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

    const recompile = async () => {
        frameCount = 0;
        start = Date.now();

        try {
            compilationErrors = [];
            generalError = null;
            await tweakShader.update_src(src, context);
            inputs = tweakShader.get_input_list();
            updateLinter();
            if (tweakShader && canvas) draw();
        } catch (e) {
            handleShaderError(e);
        }
    };

    let paused = false;
    let autoRecompile = true;
    let canvasWidth = 800;
    let canvasHeight = 450;

    const togglePause = () => {
        paused = !paused;
        if (!paused) requestAnimationFrame(draw);
    };

    const toggleAutoRecompile = () => {
        autoRecompile = !autoRecompile;
    };

    let vimMode = false;
    const toggleVim = () => {
        vimMode = !vimMode;
    };

    let autocompleteEnabled = true;
    const toggleAutocomplete = () => {
        autocompleteEnabled = !autocompleteEnabled;
    };

    const shaderExamples = [
        { label: "Exit the Matrix", value: "/exit_the_matrix.glsl" },
        { label: "Image Input", value: "/image_input.glsl" },
        { label: "Wiggler", value: "/wiggler.glsl" },
        { label: "Compute Multipass", value: "/compute_multipass.glsl" },
    ];

    const loadExample = async (example) => {
        if (example) {
            const url = `${window.location.origin}${base}/?file=${encodeURIComponent(example)}`;
            window.location.href = url;
        } else {
            window.location.href = `${window.location.origin}${base}/`;
        }
    };

    const saveToFile = () => {
        const blob = new Blob([src], { type: "text/plain" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = "shader.glsl";
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
    };

    let linkButtonText = "Save link";
    let linkButtonBlinking = false;

    const saveAsSlug = () => {
        const url = `${window.location.origin}${base}/?shader=${encodeURIComponent(btoa(src))}`;

        navigator.clipboard
            .writeText(url)
            .then(() => {
                linkButtonText = "Copied!";
                linkButtonBlinking = true;
                setTimeout(() => {
                    linkButtonText = "Save link";
                    linkButtonBlinking = false;
                }, 750);
            })
            .catch((err) => {
                console.error("Failed to copy to clipboard:", err);
                prompt("Copy this URL:", url);
            });
    };

    let srcChangeTimeout: number;
    $: if (autoRecompile && src) {
        clearTimeout(srcChangeTimeout);
        srcChangeTimeout = setTimeout(async () => {
            if (tweakShader) {
                await recompile();
            }
        }, 500);
    }
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
                <div class="control-group">
                    <label class="control-label">Time Step</label>
                    <Button
                        variant={paused ? "success" : "secondary"}
                        size="sm"
                        onclick={togglePause}
                        title={paused ? "Resume animation" : "Pause animation"}
                    >
                        {#if paused}<Play size="14" />{:else}<Pause
                                size="14"
                            />{/if}
                        {paused ? "Play" : "Pause"}
                    </Button>
                </div>

                <div class="control-group">
                    <label class="control-label">Auto Recompile</label>
                    <Button
                        variant={autoRecompile ? "primary" : "secondary"}
                        size="sm"
                        onclick={toggleAutoRecompile}
                        title={autoRecompile
                            ? "Disable auto-recompile"
                            : "Enable auto-recompile"}
                    >
                        <RotateCcw size="14" />
                        {autoRecompile ? "Auto" : "Manual"}
                    </Button>
                </div>
                <div class="control-group">
                    <label class="control-label">Example Shaders</label>
                    <Dropdown options={shaderExamples} onChange={loadExample} />
                </div>
            </div>

            <div class="inputs">
                {#each Array.from(inputs) as [k, v]}
                    {#if v.type !== "Image"}
                        <Input
                            label={k.toString()}
                            input={v}
                            change={(val) => {
                                tweakShader.set_input(k.toString(), val);
                            }}
                        ></Input>
                    {/if}
                {/each}
            </div>
        </div>

        <div class="right-column">
            <div class="editor">
                <div class="code-editor-wrapper">
                    {#key editorKey}
                        <CodeMirror
                            class="codemirror"
                            {theme}
                            extensions={[
                                glsl(),
                                shaderLinter,
                                lintGutter(),
                                ...(vimMode ? [vim()] : []),
                                autocompletion({
                                    override: autocompleteEnabled
                                        ? [glslCompletions]
                                        : [],
                                    activateOnTyping: autocompleteEnabled,
                                    maxRenderedOptions: 8,
                                    defaultKeymap: true,
                                }),
                                keymap.of(defaultKeymap),
                                keymap.of(completionKeymap),
                                Prec.highest(keymap.of([
                                    {
                                        key: "Tab",
                                        run: (e) => {
                                            if (completionStatus(e.state)) {
                                                return acceptCompletion(e);
                                            }
                                            return indentMore(e);
                                        },
                                    },
                                    {
                                        key: "Shift-Tab",
                                        run: indentLess,
                                    },
                                ])),
                            ]}
                            bind:value={src}
                        ></CodeMirror>
                    {/key}
                    <div class="error-banner">
                        {#if compilationErrors.length > 0}
                            <span class="error-count"
                                >{compilationErrors.length} error{compilationErrors.length !==
                                1
                                    ? "s"
                                    : ""}</span
                            >
                        {:else if generalError}
                            <span class="error-count">1 error</span>
                        {:else}
                            <span class="success-count">No errors</span>
                        {/if}
                    </div>
                </div>
                <div class="editor-controls">
                    <div class="control-row">
                        <Button
                            variant="primary"
                            size="sm"
                            onclick={() => recompile()}
                            disabled={autoRecompile}
                            title={autoRecompile
                                ? "Auto-recompile is enabled"
                                : "Manually recompile shader"}
                        >
                            <RotateCcw size="14" /> Recompile
                        </Button>

                        <Button
                            variant="secondary"
                            size="sm"
                            title="Save shader to file"
                            onclick={saveToFile}
                        >
                            <Save size="14" /> Save
                        </Button>

                        <Button
                            variant="secondary"
                            size="sm"
                            title="Save permalink"
                            onclick={saveAsSlug}
                            class={linkButtonBlinking ? "blink" : ""}
                        >
                            <Link2 size="14" />
                            {linkButtonText}
                        </Button>

                        <Button
                            variant={vimMode ? "primary" : "secondary"}
                            size="sm"
                            onclick={toggleVim}
                            title={vimMode
                                ? "Disable Vim keybindings"
                                : "Enable Vim keybindings"}
                        >
                            <Keyboard size="14" />
                            {vimMode ? "Vim: ON" : "Vim: OFF"}
                        </Button>

                        <Button
                            variant={autocompleteEnabled
                                ? "primary"
                                : "secondary"}
                            size="sm"
                            onclick={toggleAutocomplete}
                            title={autocompleteEnabled
                                ? "Disable autocomplete"
                                : "Enable autocomplete"}
                        >
                            Autocomplete: {autocompleteEnabled ? "ON" : "OFF"}
                        </Button>
                    </div>
                </div>
                {#if generalError}
                    <div class="general-error">
                        {generalError}
                    </div>
                {/if}
                <div class="image-inputs">
                    {#each Array.from(inputs) as [k, v]}
                        {#if v.type == "Image"}
                            <div class="image-input-section">
                                <h3 class="image-input-title">{k}</h3>
                                <ImageInput
                                    name={k.toString()}
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
                            </div>
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
        max-height: 600px;
        overflow-y: auto;
        border: 1px solid var(--border-color);
        border-radius: 8px;
        position: relative;
    }

    .error-banner {
        position: absolute;
        bottom: 0;
        right: 0;
        padding: 4px 8px;
        background: rgba(0, 0, 0, 0.8);
        border-top-left-radius: 4px;
        font-size: 0.75rem;
        font-weight: 500;
        backdrop-filter: blur(4px);
        z-index: 10;
    }

    .error-count {
        color: #ff6b6b;
    }

    .success-count {
        color: #51cf66;
    }

    :global(.code-editor-wrapper .cm-editor) {
        max-height: 600px;
    }

    :global(.code-editor-wrapper .cm-scroller) {
        max-height: 600px;
        overflow-y: auto;
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

    .controls {
        display: flex;
        flex-wrap: wrap;
        gap: 1rem;
        padding: 1rem;
        background: rgba(255, 255, 255, 0.02);
        border-radius: 8px;
        margin-bottom: 1rem;
    }

    .control-group {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        min-width: 120px;
    }

    .control-label {
        font-size: 0.75rem;
        font-weight: 500;
        color: var(--text-color);
        opacity: 0.8;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .inputs {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
        gap: 1rem;
        margin-top: 1rem;
    }

    .shader-controls {
        padding: 1rem 0;
        border-bottom: 1px solid var(--border-color);
        margin-bottom: 1rem;
    }

    .editor-controls {
        padding: 1rem 0;
        border-top: 1px solid var(--border-color);
        margin-top: 1rem;
    }

    .control-row {
        display: flex;
        gap: 10px;
    }

    :global(.blink) {
        animation: blink 0.5s ease-in-out 3;
    }

    @keyframes blink {
        0%,
        50%,
        100% {
            opacity: 1;
        }
        25%,
        75% {
            opacity: 0.3;
        }
        gap: 0.75rem;
        flex-wrap: wrap;
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

    :global(.cm-selectionBackground),
    :global(.cm-focused .cm-selectionBackground),
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

    .image-inputs {
        margin-top: 1rem;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
    }

    .image-input-section {
        padding: 1rem;
        background: rgba(255, 255, 255, 0.02);
        border: 1px solid var(--border-color);
        border-radius: 8px;
    }

    .image-input-title {
        margin: 0 0 1rem 0;
        font-size: 1rem;
        font-weight: 500;
        color: var(--text-color);
        text-transform: capitalize;
    }

    :global(.cm-tooltip-autocomplete) {
        background: #1a1a1a !important;
        border: 1px solid #333 !important;
        border-radius: 6px !important;
        box-shadow: 0 4px 16px rgba(0, 0, 0, 0.6) !important;
        backdrop-filter: blur(8px) !important;
    }

    :global(.cm-tooltip-autocomplete > ul) {
        background: transparent !important;
        border: none !important;
        border-radius: 6px !important;
        max-height: 200px !important;
        overflow-y: auto !important;
    }

    :global(.cm-tooltip-autocomplete ul li) {
        background: transparent !important;
        color: #e0e0e0 !important;
        padding: 6px 12px !important;
        border-radius: 4px !important;
        margin: 2px !important;
        font-size: 0.8rem !important;
        font-family: "JetBrains Mono", "Fira Code", "Consolas", monospace !important;
    }

    :global(.cm-tooltip-autocomplete ul li[aria-selected]) {
        background: #2d5aa0 !important;
        color: white !important;
    }

    :global(.cm-tooltip-autocomplete ul li:hover) {
        background: #374151 !important;
        color: white !important;
    }

    :global(.cm-completionLabel) {
        color: inherit !important;
        font-weight: 500 !important;
    }

    :global(.cm-completionDetail) {
        color: #9ca3af !important;
        font-style: italic !important;
        margin-left: 8px !important;
    }

    :global(.cm-completionInfo) {
        background: #1f1f1f !important;
        border: 1px solid #333 !important;
        color: #e0e0e0 !important;
        border-radius: 4px !important;
        padding: 8px !important;
        font-size: 0.8rem !important;
    }

    :global(.cm-completionIcon) {
        display: none !important;
    }

    :global(.cm-completionIcon-keyword),
    :global(.cm-completionIcon-function),
    :global(.cm-completionIcon-variable) {
        display: none !important;
    }

    :global(.cm-tooltip-autocomplete ul li[data-type="variable"]) {
        color: #9cdcfe !important;
    }

    :global(.cm-tooltip-autocomplete ul li[data-type="keyword"]) {
        color: #569cd6 !important;
    }

    :global(.cm-tooltip-autocomplete ul li[data-type="function"]) {
        color: #dcdcaa !important;
    }
</style>
