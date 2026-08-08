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
  type TypingCue = {
    label: string;
    tone: "keyword" | "builtin" | "function" | "constant" | "type";
    x: number;
    y: number;
    id: number;
  };
  let typingCue = $state<TypingCue | null>(null);
  let typingCueTimer: number | null = null;
  
  // Animation properties
  let particles: any[] = [];
  let animationId = 0;
  let isRenderingParticles = false;
  let resizeObserver: ResizeObserver | null = null;

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
        color: ["#7aa2f7", "#bb9af7", "#7dcfff", "#9ece6a", "#ff9e64"][Math.floor(Math.random() * 5)]
      });
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
        [/(^#.*$)/, "comment"],
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
    inherit: true,
    rules: [
      // Tokyo Night semantic syntax palette: color carries code meaning.
      { token: "", foreground: "c0caf5" },
      { token: "keyword", foreground: "bb9af7" },
      { token: "keyword.control", foreground: "bb9af7" },
      { token: "keyword.builtin", foreground: "2ac3de" },
      { token: "keyword.constant", foreground: "ff9e64" },
      { token: "keyword.predefined", foreground: "7dcfff" },
      { token: "keyword.magic", foreground: "e0af68" },
      { token: "function.call", foreground: "7aa2f7" },
      { token: "function.definition", foreground: "7aa2f7" },
      { token: "type.class", foreground: "73daca" },
      { token: "identifier", foreground: "c0caf5" },
      { token: "string", foreground: "9ece6a" },
      { token: "string.escape", foreground: "ff9e64" },
      { token: "comment", foreground: "565f89", fontStyle: "italic" },
      { token: "number", foreground: "ff9e64" },
      { token: "operator", foreground: "89ddff" },
      { token: "delimiter", foreground: "a9b1d6" },
      { token: "delimiter.curly", foreground: "bb9af7" },
      { token: "delimiter.bracket", foreground: "e0af68" },
      { token: "delimiter.parenthesis", foreground: "7dcfff" },
      { token: "tag", foreground: "f7768e" },
    ],
    colors: {
      "editor.background": "#1a1b26",
      "editor.foreground": "#c0caf5",
      "editorLineNumber.foreground": "#3b4261",
      "editorLineNumber.activeForeground": "#7aa2f7",
      "editor.lineHighlightBackground": "#202330",
      "editorCursor.foreground": "#c0caf5",
      "editor.selectionBackground": "#33467c",
      "editor.inactiveSelectionBackground": "#29345b",
      "editorIndentGuide.background1": "#292e42",
      "editorIndentGuide.activeBackground1": "#3b4261",
      "editorBracketMatch.background": "#33467c",
      "editorBracketMatch.border": "#7dcfff",
    }
  };

  const commandCues: Record<string, { label: string; tone: TypingCue["tone"] }> = {
    def: { label: "function", tone: "keyword" },
    class: { label: "type", tone: "keyword" },
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

  function showTypingCue(value: string, position: Monaco.Position) {
    if (!appState.isPowerModeActive || !editor) return;
    const line = value.split(/\r?\n/)[position.lineNumber - 1]?.slice(0, position.column - 1) ?? "";
    const word = line.match(/([A-Za-z_]\w*)$/)?.[1];
    if (!word) return;

    const cue = commandCues[word];
    const functionName = line.match(/\bdef\s+([A-Za-z_]\w*)$/)?.[1];
    const resolved = cue ?? (functionName === word ? { label: "function", tone: "function" as const } : null);
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
      monacoModule.languages.setMonarchTokensProvider("python", customPythonTokenizer);
      monacoModule.editor.defineTheme("practice-range-theme", practiceRangeTheme);

      editor = monacoModule.editor.create(containerEl, {
        value: code,
        language: "python",
        theme: "practice-range-theme",
        automaticLayout: true,
        tabSize: 4,
        insertSpaces: true,
        minimap: { enabled: false },
        scrollBeyondLastLine: false,
        lineNumbers: "on",
        lineNumbersMinChars: 3,
        fontSize: 14,
        lineHeight: 22,
        fontFamily: "'Fira Code', 'Cascadia Code', Consolas, monospace",
        padding: { top: 14, bottom: 14 },
        cursorBlinking: "smooth",
        cursorSmoothCaretAnimation: "on",
        smoothScrolling: true,
        renderLineHighlight: "all",
        renderWhitespace: "selection",
        bracketPairColorization: { enabled: true },
        guides: { bracketPairs: "active", indentation: true, highlightActiveIndentation: true },
        autoIndent: "full",
        tabCompletion: "on",
        useTabStops: true,
      });

      editor.addCommand(monacoModule.KeyMod.CtrlCmd | monacoModule.KeyCode.Enter, () => onRun());

      editor.onDidChangeModelContent((event) => {
        if (editor) {
          isUpdatingFromInside = true;
          code = editor.getValue();
          isUpdatingFromInside = false;

          // Trigger typing sparkles if Power Mode is active and it's a typing action
          if (appState.isPowerModeActive && event.changes.length > 0) {
            const position = editor.getPosition();
            if (position) {
              showTypingCue(editor.getValue(), position);
              const coordinates = editor.getScrolledVisiblePosition(position);
              if (coordinates) {
                // coordinates.height represents the height of the cursor/line
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
    background-color: #1a1b26;
  }
</style>
