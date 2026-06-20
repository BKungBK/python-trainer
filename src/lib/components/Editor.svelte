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

    editor = monaco.editor.create(containerEl, {
      value: code,
      language: "python",
      theme: "vs-dark",
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
