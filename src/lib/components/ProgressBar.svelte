<script lang="ts">
  import { onMount, onDestroy } from "svelte";

  // Svelte 5 properties
  let { value = 0, max = 100 } = $props();

  let canvasEl: HTMLCanvasElement;
  let wrapperEl: HTMLDivElement;
  let animationId = 0;
  let particles: any[] = [];
  let prevValue = $state<number | null>(null);

  let trackWidth = $state(0);

  // Derive current percentage
  let percentage = $derived(max > 0 ? Math.min(100, Math.max(0, (value / max) * 100)) : 0);
  let isFull = $derived(percentage >= 100);

  // Monitor value change to trigger burst
  $effect(() => {
    if (prevValue === null) {
      prevValue = value;
      return;
    }
    if (value > prevValue) {
      // Trigger a dense burst of sparks when progress increases
      triggerBurst(16);
    }
    prevValue = value;
  });

  function triggerBurst(count: number) {
    if (!canvasEl) return;
    const rect = canvasEl.getBoundingClientRect();
    const leadingEdgeX = rect.width * (percentage / 100);
    const midY = rect.height / 2;

    for (let i = 0; i < count; i++) {
      // Cycle and randomized hue offset to match the sweeping RGB gradient
      const h = (Date.now() / 15 + Math.random() * 40) % 360;
      const baseColor = isFull 
        ? `oklch(78% 0.12 120)` // desaturated success green-gold
        : `oklch(78% 0.12 ${h})`; // desaturated RGB sweep colors

      particles.push({
        x: leadingEdgeX,
        y: midY + (Math.random() - 0.5) * 6, // slightly wider spawn height
        vx: -0.5 - Math.random() * 1.5, // shoot backward (to the left)
        vy: (Math.random() - 0.5) * 2.2, // wider vertical spread
        life: 1.0,
        decay: 0.012 + Math.random() * 0.015, // slower decay, longer life (~0.6s)
        size: 0.6 + Math.random() * 1.0, // tiny sparks
        color: baseColor
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

    const rect = canvasEl.getBoundingClientRect();
    const leadingEdgeX = rect.width * (percentage / 100);
    const midY = rect.height / 2;

    // 1. Emit constant subtle idle sparks
    if (percentage > 0 && Math.random() < 0.08 && particles.length < 30) {
      const h = (Date.now() / 15) % 360;
      const baseColor = isFull 
        ? `oklch(72% 0.08 120)` // steady success green-gold spark
        : `oklch(75% 0.10 ${h})`; // flowing RGB spark

      particles.push({
        x: leadingEdgeX,
        y: midY + (Math.random() - 0.5) * 6, // wider vertical spawn height
        vx: -0.3 - Math.random() * 0.5, // drift backward (to the left)
        vy: (Math.random() - 0.5) * 1.0, // larger vertical velocity spread
        life: 1.0,
        decay: 0.008 + Math.random() * 0.012, // longer life for calm drifting
        size: 0.5 + Math.random() * 0.6, // very tiny
        color: baseColor
      });
    }

    // 2. Update and draw particles
    for (let i = particles.length - 1; i >= 0; i--) {
      const p = particles[i];
      p.x += p.vx;
      p.y += p.vy;
      
      // Calm natural physics: air resistance and subtle upward buoyancy
      p.vx *= 0.98;
      p.vy *= 0.97;
      p.vy -= 0.006; // drifts upward like real hot air circuit embers
      
      p.life -= p.decay;

      if (p.life <= 0) {
        particles.splice(i, 1);
        continue;
      }

      ctx.save();
      ctx.globalAlpha = p.life;
      ctx.fillStyle = p.color;
      
      ctx.beginPath();
      ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
      ctx.fill();
      ctx.restore();
    }

    animationId = requestAnimationFrame(render);
  }

  function handleResize() {
    if (!canvasEl || !wrapperEl) return;
    const rect = wrapperEl.getBoundingClientRect();
    const dpr = window.devicePixelRatio || 1;

    canvasEl.width = rect.width * dpr;
    canvasEl.height = (rect.height + 32) * dpr; // extra height for vertical particle spread

    const ctx = canvasEl.getContext("2d");
    if (ctx) {
      ctx.scale(dpr, dpr);
    }
  }

  onMount(() => {
    handleResize();
    window.addEventListener("resize", handleResize);
    render();
  });

  onDestroy(() => {
    window.removeEventListener("resize", handleResize);
    if (animationId) {
      cancelAnimationFrame(animationId);
    }
  });
</script>

<div bind:this={wrapperEl} class="progress-wrapper-cyber">
  <div class="progress-track-cyber" bind:clientWidth={trackWidth}>
    <div 
      class="progress-fill-cyber" 
      class:full={isFull}
      style="width: {percentage}%;"
    >
      <div 
        class="progress-gradient-flow"
        style="width: {trackWidth * 2}px;"
      ></div>
    </div>
  </div>
  <canvas 
    bind:this={canvasEl} 
    class="progress-particles-canvas"
    style="top: -16px; height: calc(100% + 32px);"
  ></canvas>
</div>

<style>
  /* Clean, non-interfering styles */
</style>
