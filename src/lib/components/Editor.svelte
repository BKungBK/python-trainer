<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import * as monaco from "monaco-editor";
  import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
  import { appState } from "$lib/state.svelte";

  // Svelte 5 properties
  let { code = $bindable("") } = $props();

  let containerEl: HTMLDivElement;
  let canvasEl: HTMLCanvasElement;
  let editor: monaco.editor.IStandaloneCodeEditor | null = null;
  let isUpdatingFromInside = false;
  
  // Animation properties
  let particles: any[] = [];
  let animationId = 0;
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
        color: Math.random() < 0.4 ? "#ffffff" : (Math.random() < 0.75 ? "#e0e0e0" : "#b8b8b8")
      });
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

    animationId = requestAnimationFrame(render);
  }

  function handleResize() {
    if (!canvasEl || !containerEl) return;
    const rect = containerEl.getBoundingClientRect();
    const dpr = window.devicePixelRatio || 1;

    canvasEl.width = rect.width * dpr;
    canvasEl.height = rect.height * dpr;

    const ctx = canvasEl.getContext("2d");
    if (ctx) {
      ctx.scale(dpr, dpr);
    }
  }

  // Custom Python tokenizer for Monaco to support rich highlighting of functions, builtins, etc.
  const customPythonTokenizer: monaco.languages.IMonarchLanguage = {
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

  const practiceRangeTheme: monaco.editor.IStandaloneThemeData = {
    base: "vs-dark",
    inherit: true,
    rules: [
      { token: "", foreground: "e0e0e0" }, // primary text
      { token: "keyword", foreground: "c586c0" }, // magenta/purple keywords
      { token: "keyword.control", foreground: "c586c0" },
      { token: "keyword.builtin", foreground: "4fc1ff" }, // light blue builtins (e.g. print, len)
      { token: "keyword.constant", foreground: "569cd6" }, // blue for True, False, None
      { token: "keyword.predefined", foreground: "9cdcfe" }, // light blue for self, cls
      { token: "keyword.magic", foreground: "dcdcaa" }, // yellow for __init__
      { token: "function.call", foreground: "dcdcaa" }, // yellow for function calls
      { token: "function.definition", foreground: "dcdcaa" }, // yellow for function definitions
      { token: "type.class", foreground: "4ec9b0" }, // teal for class names
      { token: "string", foreground: "ce9178" }, // orange-brown for strings
      { token: "comment", foreground: "6a9955", fontStyle: "italic" }, // green for comments
      { token: "number", foreground: "b5cea8" }, // light green for numbers
      { token: "operator", foreground: "d4d4d4" },
      { token: "tag", foreground: "c586c0" }, // decorators
    ],
    colors: {
      "editor.background": "#141414", // matches index.css / design.md bg
      "editor.foreground": "#e0e0e0",
      "editorLineNumber.foreground": "#555555",
      "editorLineNumber.activeForeground": "#5b9bd5", // active accent blue
      "editor.lineHighlightBackground": "#1f1f1f", // card bg
      "editorCursor.foreground": "#ffffff", // white cursor
      "editor.selectionBackground": "#264f78",
    }
  };

  onMount(async () => {
    // Configure Monaco Environment for Web Workers in Vite
    const win = window as any;
    if (!win.MonacoEnvironment) {
      win.MonacoEnvironment = {
        getWorker() {
          return new editorWorker();
        }
      };
    }

    // Register custom python tokenizer & theme
    monaco.languages.setMonarchTokensProvider("python", customPythonTokenizer);
    monaco.editor.defineTheme("practice-range-theme", practiceRangeTheme);

    editor = monaco.editor.create(containerEl, {
      value: code,
      language: "python",
      theme: "practice-range-theme",
      automaticLayout: true,
      tabSize: 4,
      insertSpaces: true,
      minimap: { enabled: false },
      scrollBeyondLastLine: false,
      lineNumbers: "on",
      fontSize: 14,
      fontFamily: "'Fira Code', 'Cascadia Code', Consolas, monospace",
      padding: { top: 8, bottom: 8 },
    });

    editor.onDidChangeModelContent((event) => {
      if (editor) {
        isUpdatingFromInside = true;
        code = editor.getValue();
        isUpdatingFromInside = false;

        // Trigger typing sparkles if Power Mode is active and it's a typing action
        if (appState.isPowerModeActive && event.changes.length > 0) {
          const position = editor.getPosition();
          if (position) {
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
    render();
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
    if (editor) {
      editor.dispose();
    }
    if (resizeObserver) {
      resizeObserver.disconnect();
    }
    if (animationId) {
      cancelAnimationFrame(animationId);
    }
  });
</script>

<div class="editor-wrapper-cyber">
  <div class="editor-container" bind:this={containerEl}></div>
  <canvas bind:this={canvasEl} class="editor-canvas-cyber"></canvas>
</div>

<style>
  .editor-container {
    width: 100%;
    height: 100%;
    min-height: 200px;
    overflow: hidden;
    background-color: #141414;
  }
</style>
