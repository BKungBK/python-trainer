<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import type * as Monaco from "monaco-editor";
  import { appState } from "$lib/state.svelte";

  // Svelte 5 properties
  let { code = $bindable(""), onRun = () => {} } = $props();

  let containerEl: HTMLDivElement;
  let canvasEl: HTMLCanvasElement;
  let editor: Monaco.editor.IStandaloneCodeEditor | null = null;
  let isDestroyed = false;
  let isUpdatingFromInside = false;
  // Animation properties
  let particles: any[] = [];
  let animationId = 0;
  let isRenderingParticles = false;
  let resizeObserver: ResizeObserver | null = null;
  let codeSyncFrame: number | null = null;
  const MAX_PARTICLES = 120;

  type TypingCue = {
    label: string;
    tone: "keyword" | "builtin" | "function" | "constant" | "type";
    x: number;
    y: number;
    id: number;
  };
  let typingCue = $state<TypingCue | null>(null);
  let typingCueTimer: number | null = null;

  function spawnParticles(x: number, y: number) {
    if (!canvasEl) return;
    const count = 5 + Math.floor(Math.random() * 4); // 5-8 particles

    for (let i = 0; i < count; i++) {
      particles.push({
        x: x + (Math.random() - 0.5) * 6,
        y: y + 2,
        vx: (Math.random() - 0.5) * 1.5,
        vy: 0.1 + Math.random() * 0.8, // falling downwards directly
        life: 1.0,
        decay: 0.015 + Math.random() * 0.015, // slower decay for smooth path
        size: 1.5 + Math.random() * 2.0,
        color: Math.random() < 0.4 ? "#ffffff" : (Math.random() < 0.75 ? "#e0e0e0" : "#b8b8b8")
      });
    }
    if (particles.length > MAX_PARTICLES) {
      particles.splice(0, particles.length - MAX_PARTICLES);
    }
    if (!isRenderingParticles) {
      isRenderingParticles = true;
      render();
    }
  }

  function render() {
    if (!canvasEl) return;
    const ctx = canvasEl.getContext("2d");
    if (!ctx) return;

    const dpr = window.devicePixelRatio || 1;
    const width = canvasEl.width / dpr;
    const height = canvasEl.height / dpr;

    ctx.clearRect(0, 0, width, height);

    for (let i = particles.length - 1; i >= 0; i--) {
      const p = particles[i];
      p.x += p.vx;
      p.vy += 0.05; // gravity pulling downwards
      p.y += p.vy;
      p.life -= p.decay;

      if (p.life <= 0) {
        particles.splice(i, 1);
        continue;
      }

      ctx.save();
      ctx.globalAlpha = p.life;
      ctx.fillStyle = p.color;

      ctx.shadowBlur = 4;
      ctx.shadowColor = p.color;
      
      ctx.beginPath();
      ctx.rect(p.x - p.size / 2, p.y - p.size / 2, p.size, p.size);
      ctx.fill();
      ctx.restore();
    }

    if (particles.length > 0) {
      animationId = requestAnimationFrame(render);
    } else {
      animationId = 0;
      isRenderingParticles = false;
    }
  }

  function handleResize() {
    if (!canvasEl || !containerEl) return;
    const rect = containerEl.getBoundingClientRect();
    const dpr = window.devicePixelRatio || 1;

    canvasEl.width = rect.width * dpr;
    canvasEl.height = rect.height * dpr;

    const ctx = canvasEl.getContext("2d");
    if (ctx) {
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    }
  }

  // Custom Python tokenizer for Monaco to support rich highlighting of functions, builtins, etc.
  const customPythonTokenizer: Monaco.languages.IMonarchLanguage = {
    defaultToken: "",
    tokenPostfix: ".python",
    keywords: [
      "and", "as", "assert", "async", "await", "break", "case", "class", 
      "continue", "def", "del", "elif", "else", "except", "exec", "finally", 
      "for", "from", "global", "if", "import", "in", "is", "lambda", "match", 
      "nonlocal", "not", "or", "pass", "raise", "return", "try", "while", 
      "with", "yield"
    ],
    builtins: [
      "abs", "all", "any", "bin", "bool", "chr", "classmethod", "compile", 
      "complex", "delattr", "dict", "dir", "divmod", "enumerate", "eval", 
      "filter", "float", "format", "frozenset", "getattr", "globals", "hasattr", 
      "hash", "help", "hex", "id", "input", "int", "isinstance", "issubclass", 
      "iter", "len", "list", "locals", "map", "max", "min", "next", "object", 
      "oct", "open", "ord", "pow", "print", "property", "range", "repr", 
      "reversed", "round", "set", "setattr", "slice", "sorted", "staticmethod", 
      "str", "sum", "super", "tuple", "type", "vars", "zip"
    ],
    constants: [
      "True", "False", "None"
    ],
    predefined: [
      "self", "cls"
    ],
    magic: [
      "__dict__", "__methods__", "__members__", "__class__", "__bases__", 
      "__name__", "__mro__", "__subclasses__", "__init__", "__import__"
    ],
    brackets: [
      { open: "{", close: "}", token: "delimiter.curly" },
      { open: "[", close: "]", token: "delimiter.bracket" },
      { open: "(", close: ")", token: "delimiter.parenthesis" }
    ],
    tokenizer: {
      root: [
        { include: "@whitespace" },
        { include: "@numbers" },
        { include: "@strings" },
        
        [/[,:;]/, "delimiter"],
        [/[+\-*\/%=<>!&|^~]+/, "operator"],
        [/[{}\[\]()]/, "@brackets"],
        
        [/@[a-zA-Z_]\w*/, "tag"], // decorators
        
        // Class definition matching
        [/(class\s+)([a-zA-Z_]\w*)/, ["keyword", "type.class"]],
        
        // Function definition matching
        [/(def\s+)([a-zA-Z_]\w*)/, ["keyword", "function.definition"]],

        // Assignment targets and dotted properties get their own semantic tone.
        [/[a-zA-Z_]\w*(?=\s*=)/, "variable"],
        [/([.])([a-zA-Z_]\w*)/, ["delimiter", "property"]],
        
        // Function call matching (any identifier followed by '(')
        [
          /[a-zA-Z_]\w*(?=\s*\()/,
          {
            cases: {
              "@builtins": "keyword.builtin",
              "@default": "function.call"
            }
          }
        ],

        // Standard identifiers
        [
          /[a-zA-Z_]\w*/,
          {
            cases: {
              "@keywords": "keyword",
              "@builtins": "keyword.builtin",
              "@constants": "keyword.constant",
              "@predefined": "keyword.predefined",
              "@magic": "keyword.magic",
              "@default": "identifier"
            }
          }
        ]
      ],
      whitespace: [
        [/\s+/, "white"],
        [/#.*$/, "comment"],
        [/'''/, "string", "@endDocString"],
        [/"""/, "string", "@endDblDocString"]
      ],
      endDocString: [
        [/[^']+/, "string"],
        [/\\'/, "string"],
        [/'''/, "string", "@popall"],
        [/'/, "string"]
      ],
      endDblDocString: [
        [/[^"]+/, "string"],
        [/\\"/, "string"],
        [/"""/, "string", "@popall"],
        [/"/, "string"]
      ],
      numbers: [
        [/-?0x([abcdef]|[ABCDEF]|\d)+[lL]?/, "number.hex"],
        [/-?(\d*\.)?\d+([eE][+\-]?\d+)?[jJ]?[lL]?/, "number"]
      ],
      strings: [
        [/'$/, "string.escape", "@popall"],
        [/f'{1,3}/, "string.escape", "@fStringBody"],
        [/'/, "string.escape", "@stringBody"],
        [/"$/, "string.escape", "@popall"],
        [/f"{1,3}/, "string.escape", "@fDblStringBody"],
        [/"/, "string.escape", "@dblStringBody"]
      ],
      fStringBody: [
        [/[^\\'\{\}]+$/, "string", "@popall"],
        [/[^\\'\{\}]+/, "string"],
        [/\{[^\}':!=]+/, "identifier", "@fStringDetail"],
        [/\\./, "string"],
        [/'/, "string.escape", "@popall"],
        [/\\$/, "string"]
      ],
      stringBody: [
        [/[^\\']+$/, "string", "@popall"],
        [/[^\\']+/, "string"],
        [/\\./, "string"],
        [/'/, "string.escape", "@popall"],
        [/\\$/, "string"]
      ],
      fDblStringBody: [
        [/[^\\"\{\}]+$/, "string", "@popall"],
        [/[^\\"\{\}]+/, "string"],
        [/\{[^\}':!=]+/, "identifier", "@fStringDetail"],
        [/\\./, "string"],
        [/"/, "string.escape", "@popall"],
        [/\\$/, "string"]
      ],
      dblStringBody: [
        [/[^\\"]+$/, "string", "@popall"],
        [/[^\\"]+/, "string"],
        [/\\./, "string"],
        [/"/, "string.escape", "@popall"],
        [/\\$/, "string"]
      ],
      fStringDetail: [
        [/[:][^}]+/, "string"],
        [/[!][ars]/, "string"],
        [/=/, "string"],
        [/\}/, "identifier", "@pop"]
      ]
    }
  };

  const practiceRangeTheme: Monaco.editor.IStandaloneThemeData = {
    base: "vs-dark",
    inherit: false,
    rules: [
      { token: "", foreground: "b8b8b8" },
      { token: "keyword", foreground: "c586c0" },
      { token: "keyword.control", foreground: "c586c0" },
      { token: "keyword.builtin", foreground: "4fc1ff" },
      { token: "keyword.constant", foreground: "569cd6" },
      { token: "keyword.predefined", foreground: "9cdcfe" },
      { token: "keyword.magic", foreground: "dcdcaa" },
      { token: "function.call", foreground: "dcdcaa" },
      { token: "function.definition", foreground: "dcdcaa" },
      { token: "type.class", foreground: "4ec9b0" },
      { token: "identifier", foreground: "b8b8b8" },
      { token: "variable", foreground: "9cdcfe" },
      { token: "property", foreground: "9cdcfe" },
      { token: "string", foreground: "ce9178" },
      { token: "string.escape", foreground: "d7ba7d" },
      { token: "comment", foreground: "6a9955", fontStyle: "italic" },
      { token: "number", foreground: "b5cea8" },
      { token: "number.hex", foreground: "b5cea8" },
      { token: "operator", foreground: "d4d4d4" },
      { token: "delimiter", foreground: "808890" },
      { token: "delimiter.curly", foreground: "d7ba7d" },
      { token: "delimiter.bracket", foreground: "dcdcaa" },
      { token: "delimiter.parenthesis", foreground: "9cdcfe" },
      { token: "tag", foreground: "c586c0" },
    ],
    colors: {
      "editor.background": "#141414",
      "editor.foreground": "#b8b8b8",
      "editorLineNumber.foreground": "#555555",
      "editorLineNumber.activeForeground": "#5b9bd5",
      "editor.lineHighlightBackground": "#1f1f1f",
      "editorCursor.foreground": "#ffffff",
      "editor.selectionBackground": "#264f78",
    }
  };

  // Monaco's tokenizer only controls syntax colors. Editing behavior such as
  // quote pairing and Python indentation comes from the language configuration.
  const pythonLanguageConfiguration: Monaco.languages.LanguageConfiguration = {
    comments: {
      lineComment: "#",
      blockComment: ["'''", "'''"]
    },
    brackets: [
      ["{", "}"],
      ["[", "]"],
      ["(", ")"]
    ],
    autoClosingPairs: [
      { open: "{", close: "}" },
      { open: "[", close: "]" },
      { open: "(", close: ")" },
      { open: '"', close: '"', notIn: ["string"] },
      { open: "'", close: "'", notIn: ["string", "comment"] }
    ],
    surroundingPairs: [
      { open: "{", close: "}" },
      { open: "[", close: "]" },
      { open: "(", close: ")" },
      { open: '"', close: '"' },
      { open: "'", close: "'" }
    ],
    autoCloseBefore: ";:.,=}])> \n\t",
    indentationRules: {
      increaseIndentPattern: /^\s*(?:def|class|for|if|elif|else|while|try|with|finally|except|async|match|case)\b.*:\s*(?:#.*)?$/,
      decreaseIndentPattern: /^\s*(?:elif|else|except|finally)\b.*:?\s*(?:#.*)?$/
    },
    folding: {
      offSide: true,
      markers: {
        start: /^\s*#region\b/,
        end: /^\s*#endregion\b/
      }
    }
  };

  function flushCodeSync() {
    if (codeSyncFrame !== null) {
      cancelAnimationFrame(codeSyncFrame);
      codeSyncFrame = null;
    }
    if (!editor || isDestroyed) return;
    isUpdatingFromInside = true;
    code = editor.getValue();
    isUpdatingFromInside = false;
  }

  function scheduleCodeSync() {
    if (codeSyncFrame !== null) return;
    codeSyncFrame = requestAnimationFrame(() => {
      codeSyncFrame = null;
      flushCodeSync();
    });
  }

  const commandCues: Record<string, { label: string; tone: TypingCue["tone"] }> = {
    def: { label: "function", tone: "keyword" },
    class: { label: "type", tone: "type" },
    if: { label: "branch", tone: "keyword" },
    elif: { label: "branch", tone: "keyword" },
    else: { label: "branch", tone: "keyword" },
    for: { label: "loop", tone: "keyword" },
    while: { label: "loop", tone: "keyword" },
    import: { label: "module", tone: "keyword" },
    from: { label: "module", tone: "keyword" },
    return: { label: "exit", tone: "keyword" },
    print: { label: "output", tone: "builtin" },
    input: { label: "input", tone: "builtin" },
    range: { label: "sequence", tone: "builtin" },
    len: { label: "length", tone: "builtin" },
    int: { label: "cast", tone: "builtin" },
    str: { label: "cast", tone: "builtin" },
  };

  function showTypingCue(position: Monaco.Position) {
    if (!appState.isPowerModeActive || !editor) return;

    const line = editor.getModel()?.getLineContent(position.lineNumber)?.slice(0, position.column - 1) ?? "";
    const word = line.match(/([A-Za-z_]\w*)$/)?.[1];
    if (!word) return;

    const cue = commandCues[word];
    const functionName = line.match(/\bdef\s+([A-Za-z_]\w*)$/)?.[1];
    const resolved = cue ?? (functionName === word
      ? { label: "function", tone: "function" as const }
      : null);
    if (!resolved) return;

    const coordinates = editor.getScrolledVisiblePosition(position);
    if (!coordinates) return;

    typingCue = {
      ...resolved,
      x: coordinates.left + 4,
      y: coordinates.top + (coordinates.height || 18),
      id: Date.now(),
    };
    if (typingCueTimer) window.clearTimeout(typingCueTimer);
    typingCueTimer = window.setTimeout(() => {
      typingCue = null;
      typingCueTimer = null;
    }, 900);
  }

  onMount(async () => {
    try {
      const [{ default: editorWorker }, monacoModule] = await Promise.all([
        import("monaco-editor/esm/vs/editor/editor.worker?worker"),
        import("monaco-editor/esm/vs/editor/editor.api")
      ]);

      if (isDestroyed || !containerEl) return;

      // Configure Monaco Environment for Web Workers in Vite
      const win = window as typeof window & {
        MonacoEnvironment?: { getWorker: () => Worker };
      };
      if (!win.MonacoEnvironment) {
        win.MonacoEnvironment = {
          getWorker() {
            return new editorWorker();
          }
        };
      }

      // Register custom python tokenizer & theme
      if (!monacoModule.languages.getLanguages().some(({ id }) => id === "python")) {
        monacoModule.languages.register({ id: "python" });
      }
      monacoModule.languages.setLanguageConfiguration("python", {
        ...pythonLanguageConfiguration,
        onEnterRules: [
          {
            beforeText: /^\s*(?:def|class|for|if|elif|else|while|try|with|finally|except|async|match|case).*?:\s*$/,
            action: { indentAction: monacoModule.languages.IndentAction.Indent }
          }
        ]
      });
      monacoModule.languages.setMonarchTokensProvider("python", customPythonTokenizer);
      monacoModule.editor.defineTheme("practice-range-theme", practiceRangeTheme);

      editor = monacoModule.editor.create(containerEl, {
        value: code,
        language: "python",
        theme: "practice-range-theme",
        autoDetectHighContrast: false,
        "semanticHighlighting.enabled": false,
        automaticLayout: true,
        tabSize: 4,
        insertSpaces: true,
        detectIndentation: false,
        autoIndent: "full",
        autoClosingBrackets: "always",
        autoClosingQuotes: "always",
        minimap: { enabled: false },
        scrollBeyondLastLine: false,
        lineNumbers: "on",
        lineNumbersMinChars: 3,
        lineDecorationsWidth: 4,
        glyphMargin: false,
        fontSize: 14,
        fontFamily: "'Fira Code', 'Cascadia Code', Consolas, monospace",
        padding: { top: 8, bottom: 8 },
        cursorBlinking: "blink",
        cursorSmoothCaretAnimation: "off",
        smoothScrolling: false,
      });
      // Apply the theme after the instance exists. Some Monaco runtimes do not
      // initialize the standalone theme service until editor.create() runs.
      monacoModule.editor.setTheme("practice-range-theme");

      editor.addCommand(monacoModule.KeyMod.CtrlCmd | monacoModule.KeyCode.Enter, () => {
        flushCodeSync();
        onRun();
      });

      editor.onDidChangeModelContent((event) => {
        if (editor) {
          scheduleCodeSync();

          if (appState.isPowerModeActive && event.changes.length > 0) {
            const position = editor.getPosition();
            if (position) {
              showTypingCue(position);
              const coordinates = editor.getScrolledVisiblePosition(position);
              if (coordinates) {
                spawnParticles(coordinates.left, coordinates.top + (coordinates.height || 18));
              }
            }
          }
        }
      });

      // Observe size changes
      resizeObserver = new ResizeObserver(() => {
        handleResize();
      });
      resizeObserver.observe(containerEl);

      // Initialize dimensions and start rendering loop
      handleResize();
    } catch (error) {
      console.error("Monaco editor failed to load:", error);
    }
  });

  // Keep editor content in sync when code prop changes from outside (e.g. changing problems)
  $effect(() => {
    if (editor && !isUpdatingFromInside) {
      const currentValue = editor.getValue();
      if (code !== currentValue) {
        editor.setValue(code || "");
      }
    }
  });

  onDestroy(() => {
    isDestroyed = true;
    if (editor) {
      editor.dispose();
    }
    if (resizeObserver) {
      resizeObserver.disconnect();
    }
    if (animationId) {
      cancelAnimationFrame(animationId);
    }
    if (codeSyncFrame !== null) {
      cancelAnimationFrame(codeSyncFrame);
    }
    if (typingCueTimer) {
      window.clearTimeout(typingCueTimer);
    }
  });
</script>

<div class="editor-wrapper-cyber">
  <div class="editor-container" bind:this={containerEl}></div>
  <canvas bind:this={canvasEl} class="editor-canvas-cyber"></canvas>
  {#if typingCue}
      <div
        class="typing-cue typing-cue-{typingCue.tone}"
        style={`--cue-x: ${typingCue.x}px; --cue-y: ${typingCue.y}px;`}
        aria-live="polite"
      >
        <span class="typing-cue-glyph">✦</span>
        <span>{typingCue.label}</span>
      </div>
  {/if}
</div>

<style>
  .editor-container {
    width: 100%;
    height: 100%;
    min-height: 200px;
    overflow: hidden;
    background-color: #141414;
  }

  .typing-cue {
    position: absolute;
    left: var(--cue-x);
    top: var(--cue-y);
    z-index: 12;
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 3px 7px;
    border: 1px solid currentColor;
    border-radius: 4px;
    color: #7aa2f7;
    background: #202330;
    box-shadow: 0 4px 12px rgba(10, 12, 20, 0.32);
    font-family: var(--font-mono);
    font-size: 10px;
    line-height: 1;
    pointer-events: none;
    transform: translate(0, -115%);
    animation: typing-cue-in 0.22s var(--ease-out-quart), typing-cue-out 0.9s ease-out forwards;
  }

  .typing-cue-glyph {
    color: currentColor;
    font-size: 11px;
  }

  .typing-cue-keyword { color: #bb9af7; }
  .typing-cue-builtin { color: #2ac3de; }
  .typing-cue-function { color: #7aa2f7; }
  .typing-cue-constant { color: #ff9e64; }
  .typing-cue-type { color: #73daca; }

  @keyframes typing-cue-in {
    from { opacity: 0; transform: translate(0, -85%) scale(0.94); }
    to { opacity: 1; transform: translate(0, -115%) scale(1); }
  }

  @keyframes typing-cue-out {
    0%, 65% { opacity: 1; }
    100% { opacity: 0; transform: translate(0, -135%); }
  }

</style>
