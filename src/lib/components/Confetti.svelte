<script lang="ts">
  import { onMount, onDestroy } from "svelte";

  interface Props {
    isFullScreen?: boolean;
    trigger?: (x?: number, y?: number) => void;
  }

  let { isFullScreen = false, trigger = $bindable() }: Props = $props();

  let canvasBg: HTMLCanvasElement;
  let canvasFg: HTMLCanvasElement;
  let ctxBg: CanvasRenderingContext2D | null = null;
  let ctxFg: CanvasRenderingContext2D | null = null;

  let animationFrameId: number | null = null;
  let activeTimers: any[] = [];
  let fountainOn = false;

  const GRAVITY = 0.07;
  const DRAG = 0.985;
  const MAX_SPARKS = 2000;
  const MAX_CONFETTI = 800;

  // Hex Palettes from user's script
  const palettes = [
    ['#F09595','#E24B4A','#FAC775','#F0997B'],
    ['#85B7EB','#378ADD','#9FE1CB','#1D9E75'],
    ['#AFA9EC','#7F77DD','#ED93B1','#D4537E'],
    ['#FAC775','#EF9F27','#F0997B','#F09595'],
    ['#5DCAA5','#85B7EB','#AFA9EC','#9FE1CB']
  ];

  // RGB colors from user's script
  const confettiRGB = [
    [255,214,102],[255,107,107],[116,185,255],[120,224,143],[235,235,235],
    [255,180,220],[180,150,255]
  ];

  function pick(arr: any[]) {
    return arr[Math.floor(Math.random() * arr.length)];
  }

  const rand = (a: number, b: number) => a + Math.random() * (b - a);

  // Sprite pre-baking & cache
  const spriteCache = new Map<string, { img: HTMLCanvasElement; ox: number; oy: number }>();
  function spriteKey(r: number, g: number, b: number, bw: number, bh: number, shape: number) {
    return `${r},${g},${b}|${bw}x${bh}|${shape}`;
  }

  function makeSprite(r: number, g: number, b: number, bw: number, bh: number, shape: number) {
    const pad = 2;
    const c = document.createElement('canvas');
    c.width = bw + pad * 2;
    c.height = bh + pad * 2;
    const cctx = c.getContext('2d');
    if (!cctx) return { img: c, ox: pad + bw / 2, oy: pad + bh / 2 };

    cctx.fillStyle = `rgb(${r},${g},${b})`;
    if (shape === 2) {
      cctx.beginPath();
      cctx.arc(pad + bw / 2, pad + bh / 2, bw / 2, 0, Math.PI * 2);
      cctx.fill();
    } else {
      cctx.fillRect(pad, pad, bw, bh);
    }

    const dots = Math.max(10, Math.floor((bw * bh) / 8));
    for (let i = 0; i < dots; i++) {
      const x = pad + Math.random() * bw;
      const y = pad + Math.random() * bh;
      const v = (Math.random() * 60 - 30) | 0;
      const rr = Math.max(0, Math.min(255, r + v));
      const gg = Math.max(0, Math.min(255, g + v));
      const bb = Math.max(0, Math.min(255, b + v));
      cctx.globalAlpha = 0.12 + Math.random() * 0.24;
      cctx.fillStyle = `rgb(${rr},${gg},${bb})`;
      cctx.fillRect(x, y, 1, 1);
    }
    cctx.globalAlpha = 1;
    return { img: c, ox: pad + bw / 2, oy: pad + bh / 2 };
  }

  function getSprite(r: number, g: number, b: number, bw: number, bh: number, shape: number) {
    const key = spriteKey(r, g, b, bw, bh, shape);
    let s = spriteCache.get(key);
    if (!s) {
      s = makeSprite(r, g, b, bw, bh, shape);
      spriteCache.set(key, s);
    }
    return s;
  }

  // Pools
  const sparkPool: any[] = [];
  const sparks: any[] = [];
  const confettiPool: any[] = [];
  const confettiArr: any[] = [];
  let rockets: any[] = [];

  function getSpark() {
    return sparkPool.length ? sparkPool.pop() : {};
  }

  function relSpark(p: any) {
    if (sparkPool.length < MAX_SPARKS) {
      p.active = false;
      sparkPool.push(p);
    }
  }

  function getConf() {
    return confettiPool.length ? confettiPool.pop() : {};
  }

  function relConf(p: any) {
    if (confettiPool.length < MAX_CONFETTI) {
      p.active = false;
      confettiPool.push(p);
    }
  }

  // Spawners
  function spawnSpark(x: number, y: number, vx: number, vy: number, color: string, opts: any = {}) {
    if (sparks.length >= MAX_SPARKS) return;
    const p = getSpark();
    p.active = true;
    p.x = x;
    p.y = y;
    p.vx = vx;
    p.vy = vy;
    p.color = color;
    p.life = opts.life || 1;
    p.maxLife = p.life;
    p.size = opts.size || (1.4 + Math.random() * 1.6);
    p.gravity = opts.gravity !== undefined ? opts.gravity : GRAVITY;
    p.drag = opts.drag !== undefined ? opts.drag : DRAG;
    p.flicker = !!opts.flicker;

    const index = sparks.findIndex(s => !s.active);
    if (index !== -1) {
      sparks[index] = p;
    } else {
      sparks.push(p);
    }
  }

  function spawnRocket(x: number, y: number, vx: number, vy: number) {
    const hue = Math.floor(Math.random() * 360);
    rockets.push({
      active: true,
      x,
      y,
      vx,
      vy,
      size: 2.2,
      gravity: GRAVITY * 0.78,
      drag: 0.994,
      hue,
      color: `hsl(${hue}, 100%, 65%)`
    });
  }

  function spawnConfetti(x: number, y: number, vx: number, vy: number, opts: any = {}) {
    if (confettiArr.length >= MAX_CONFETTI) return;
    const p = getConf();
    p.active = true;
    const layer = opts.layer !== undefined ? opts.layer : (Math.random() < 0.5 ? 0 : Math.random() < 0.7 ? 1 : 2);
    const shape = Math.floor(Math.random() * 3);
    const rgb = pick(confettiRGB);
    const r = Math.round(rgb[0] * 0.78), g = Math.round(rgb[1] * 0.78), b = Math.round(rgb[2] * 0.78);
    const base = layer === 0 ? rand(3, 5) : layer === 1 ? rand(5, 8) : rand(7, 12);
    const aspect = shape === 0 ? rand(1.6, 2.4) : 1;
    const bw = Math.max(2, Math.round(base));
    const bh = shape === 0 ? Math.round(bw * aspect) : bw;
    
    p.x = x;
    p.y = y;
    p.vx = vx;
    p.vy = vy;
    p.layer = layer;
    p.spr = getSprite(r, g, b, bw, bh, shape);
    p.life = opts.life || 2.6;
    p.maxLife = p.life;
    p.rot = rand(0, Math.PI * 2);
    p.vr = rand(-0.08, 0.08) * (layer + 1);
    p.flip = rand(0, Math.PI * 2);
    p.flipSpeed = rand(0.08, 0.22);
    p.sway = rand(0, Math.PI * 2);
    p.swaySpeed = rand(0.03, 0.06);
    p.drift = rand(0.3, 0.8);
    p.baseAlpha = layer === 0 ? 0.55 : layer === 1 ? 0.78 : 0.95;

    const index = confettiArr.findIndex(c => !c.active);
    if (index !== -1) {
      confettiArr[index] = p;
    } else {
      confettiArr.push(p);
    }
  }

  // Get current logical dimensions based on fullScreen flag and canvas scale
  function getDimensions() {
    const dpr = Math.min(window.devicePixelRatio || 1, 1.5);
    if (isFullScreen) {
      return { W: window.innerWidth, H: window.innerHeight, dpr };
    }
    if (canvasBg) {
      return { W: canvasBg.width / dpr, H: canvasBg.height / dpr, dpr };
    }
    return { W: window.innerWidth, H: window.innerHeight, dpr };
  }

  function explode(x: number, y: number, opts: any = {}) {
    const palette = opts.palette || pick(palettes);
    const count = opts.count || (130 + Math.floor(Math.random() * 50));
    const baseSpeed = opts.speed || (4.2 + Math.random() * 2.2);
    const layers = 1 + Math.floor(Math.random() * 2);

    for (let l = 0; l < layers; l++) {
      const layerSpeed = baseSpeed * (1 - l * 0.25);
      const layerCount = Math.floor(count / layers);
      const layerColor = l === 0 ? palette[0] : pick(palette);
      for (let i = 0; i < layerCount; i++) {
        const angle = (Math.PI * 2 * i / layerCount) + (Math.random() * 0.12);
        const speedVar = layerSpeed * (0.85 + Math.random() * 0.3);
        const vx = Math.cos(angle) * speedVar;
        const vy = Math.sin(angle) * speedVar;
        const color = Math.random() < 0.7 ? layerColor : pick(palette);
        spawnSpark(x, y, vx, vy, color, {
          life: 0.9 + Math.random() * 0.45,
          size: 1.6 + Math.random() * 1.5,
          gravity: GRAVITY * (0.8 + Math.random() * 0.3),
          drag: 0.976 + Math.random() * 0.01,
          flicker: Math.random() < 0.35
        });
      }
    }

    if (Math.random() < 0.55) {
      const delay = 180 + Math.random() * 120;
      const timer = setTimeout(() => {
        for (let i = 0; i < 18; i++) {
          const angle = Math.random() * Math.PI * 2;
          const speed = 0.6 + Math.random() * 0.8;
          spawnSpark(x, y, Math.cos(angle) * speed, Math.sin(angle) * speed, pick(palette), {
            life: 0.45 + Math.random() * 0.25,
            size: 0.8 + Math.random(),
            gravity: GRAVITY * 0.55,
            flicker: true
          });
        }
        startLoop();
      }, delay);
      activeTimers.push(timer);
    }
  }

  function launchRocket(targetX: number, targetY: number) {
    const { H } = getDimensions();
    const clampedY = Math.max(targetY, H * 0.18);
    const startX = targetX + (Math.random() * 40 - 20);
    const startY = H + 10;
    const vx = (targetX - startX) / 38;
    const dist = startY - clampedY;
    const vy = -Math.sqrt(2 * (GRAVITY * 0.78) * dist) * 1.02;
    spawnRocket(startX, startY, vx, vy);
    startLoop();
  }

  function spawnFountainBurst() {
    const { W, H } = getDimensions();
    const baseX = W / 2 + (Math.random() * W * 0.6 - W * 0.3);
    const palette = pick(palettes);
    for (let i = 0; i < 4; i++) {
      const angle = -Math.PI / 2 + (Math.random() * 0.6 - 0.3);
      const speed = 1.8 + Math.random() * 1.3;
      spawnSpark(baseX, H - 10, Math.cos(angle) * speed, Math.sin(angle) * speed, pick(palette), {
        life: 0.8 + Math.random() * 0.35,
        size: 1.3 + Math.random() * 1.2,
        gravity: GRAVITY * 0.6,
        flicker: true
      });
    }
  }

  function burstConfettiSide() {
    const { W, H } = getDimensions();

    [0, W].forEach(sx => {
      const dir = sx === 0 ? 1 : -1;
      const count = 45;
      for (let i = 0; i < count; i++) {
        // Softer launch speed and angle
        const angle = -Math.PI / 2.6 + Math.random() * 1.0;
        const speed = 7.5 + Math.random() * 5.5; // Moderated speed for lower intensity
        spawnConfetti(sx, H * 0.7, Math.cos(angle) * speed * dir, Math.sin(angle) * speed, {
          life: 2.8 + Math.random() * 1.1
        });
      }
    });
    startLoop();
  }

  function playAll(clickX?: number, clickY?: number) {
    const { W, H } = getDimensions();
    for (let i = 0; i < 10; i++) {
      const timer = setTimeout(() => {
        let tx, ty;
        if (i === 0 && clickX !== undefined && clickY !== undefined) {
          tx = clickX;
          ty = H * 0.15 + Math.random() * H * 0.3; // directly above the click point
        } else {
          tx = W * 0.15 + Math.random() * W * 0.7;
          ty = H * 0.18 + Math.random() * H * 0.22;
        }
        launchRocket(tx, ty);
      }, i * 150);
      activeTimers.push(timer);
    }
    burstConfettiSide();
    const timerC = setTimeout(burstConfettiSide, 800);
    activeTimers.push(timerC);
  }

  // Exposed trigger binding
  trigger = (customX?: number, customY?: number) => {
    resizeCanvas();
    playAll(customX, customY);
  };

  // Rendering ticks
  function tick() {
    if (!canvasBg || !canvasFg || !ctxBg || !ctxFg) return;

    const { W, H } = getDimensions();

    // 1. Render Background Canvas (Sparks & Rockets - Motion Blur Fade)
    ctxBg.save();
    ctxBg.setTransform(1, 0, 0, 1, 0, 0);
    ctxBg.globalCompositeOperation = "destination-out";
    ctxBg.fillStyle = "rgba(0, 0, 0, 0.26)"; // Motion blur fade rate matching spec
    ctxBg.fillRect(0, 0, canvasBg.width, canvasBg.height);
    ctxBg.restore();

    // Rockets update and draw
    for (let i = rockets.length - 1; i >= 0; i--) {
      const p = rockets[i];
      if (!p.active) continue;

      const prevVy = p.vy;
      p.vx *= p.drag;
      p.vy *= p.drag;
      p.vy += p.gravity;

      // Precise apex detection ( vy crosses 0 )
      if (prevVy < 0 && p.vy >= 0) {
        const fraction = -prevVy / (p.vy - prevVy);
        const apexX = p.x + p.vx * fraction;
        const apexY = p.y + prevVy * fraction;
        
        explode(apexX, apexY, { palette: pick(palettes) });
        p.active = false;
        rockets.splice(i, 1);
        continue;
      }

      const prevX = p.x;
      const prevY = p.y;
      p.x += p.vx;
      p.y += p.vy;

      // Draw rocket trail streak (very fast line rendering)
      ctxBg.strokeStyle = 'hsl(' + p.hue + ', 100%, 85%)';
      ctxBg.lineWidth = p.size;
      ctxBg.globalAlpha = 0.9;
      ctxBg.beginPath();
      ctxBg.moveTo(p.x - p.vx * 1.8, p.y - p.vy * 1.8);
      ctxBg.lineTo(p.x, p.y);
      ctxBg.stroke();
    }

    // Sparks update and draw
    ctxBg.globalCompositeOperation = "lighter";
    for (let i = sparks.length - 1; i >= 0; i--) {
      const p = sparks[i];
      if (!p.active) continue;

      p.vx *= p.drag;
      p.vy *= p.drag;
      p.vy += p.gravity;
      
      const prevX = p.x;
      const prevY = p.y;
      
      p.x += p.vx;
      p.y += p.vy;
      p.life -= 0.017;

      if (p.life <= 0 || p.y > H + 50) {
        p.active = false;
        sparks.splice(i, 1);
        relSpark(p);
        continue;
      }

      const t = Math.max(p.life / p.maxLife, 0);
      let alpha = t;
      if (p.flicker) {
        alpha *= 0.6 + Math.random() * 0.4;
      }

      ctxBg.strokeStyle = p.color;
      ctxBg.lineWidth = p.size * (0.4 + t * 0.6);
      ctxBg.globalAlpha = Math.max(alpha, 0);

      ctxBg.beginPath();
      ctxBg.moveTo(prevX, prevY);
      ctxBg.lineTo(p.x, p.y);
      ctxBg.stroke();
    }
    ctxBg.globalCompositeOperation = "source-over";
    ctxBg.globalAlpha = 1;

    // 2. Render Foreground Canvas (Confetti - Clear Completely)
    ctxFg.save();
    ctxFg.setTransform(1, 0, 0, 1, 0, 0);
    ctxFg.clearRect(0, 0, canvasFg.width, canvasFg.height);
    ctxFg.restore();

    for (let i = confettiArr.length - 1; i >= 0; i--) {
      const p = confettiArr[i];
      if (!p.active) continue;

      const depth = p.layer === 0 ? 0.6 : p.layer === 1 ? 0.85 : 1.0;
      p.sway += p.swaySpeed;
      p.flip += p.flipSpeed;
      p.vy = Math.min(p.vy + GRAVITY * 0.4 * depth, 3.2 * depth + 0.8);
      p.x += Math.sin(p.sway) * p.drift * depth + p.vx * 0.94;
      p.vx *= 0.985;
      p.y += p.vy;
      p.rot += p.vr;
      p.life -= 0.013;

      if (p.life <= 0 || p.y > H + 60) {
        p.active = false;
        confettiArr.splice(i, 1);
        relConf(p);
        continue;
      }

      const fadeStart = 0.4;
      const lifeFade = p.life < fadeStart ? p.life / fadeStart : 1;

      ctxFg.save();
      ctxFg.translate(p.x, p.y);
      ctxFg.rotate(p.rot);
      ctxFg.scale(0.08 + 0.92 * Math.abs(Math.sin(p.flip)), 1);
      ctxFg.globalAlpha = p.baseAlpha * Math.max(lifeFade, 0);
      ctxFg.drawImage(p.spr.img, -p.spr.ox, -p.spr.oy);
      ctxFg.restore();
    }

    if (fountainOn && sparks.length < MAX_SPARKS - 20 && Math.random() < 0.9) {
      spawnFountainBurst();
    }

    // 3. Keep loop alive if any active entities exist
    const hasActiveRockets = rockets.length > 0;
    const hasActiveSparks = sparks.some(s => s.active);
    const hasActiveConfetti = confettiArr.some(c => c.active);

    if (hasActiveRockets || hasActiveSparks || hasActiveConfetti) {
      animationFrameId = requestAnimationFrame(tick);
    } else {
      animationFrameId = null;
      ctxBg.save();
      ctxBg.setTransform(1, 0, 0, 1, 0, 0);
      ctxBg.clearRect(0, 0, canvasBg.width, canvasBg.height);
      ctxBg.restore();
      ctxFg.save();
      ctxFg.setTransform(1, 0, 0, 1, 0, 0);
      ctxFg.clearRect(0, 0, canvasFg.width, canvasFg.height);
      ctxFg.restore();
    }
  }

  function startLoop() {
    if (!animationFrameId) {
      animationFrameId = requestAnimationFrame(tick);
    }
  }

  function resizeCanvas() {
    if (!canvasBg || !canvasFg) return;
    
    let w = window.innerWidth;
    let h = window.innerHeight;
    
    if (!isFullScreen) {
      const rect = canvasBg.getBoundingClientRect();
      w = rect.width || window.innerWidth;
      h = rect.height || window.innerHeight;
      if (w < 10) w = window.innerWidth;
      if (h < 10) h = window.innerHeight;
    }

    const dpr = Math.min(window.devicePixelRatio || 1, 1.5);

    canvasBg.width = w * dpr;
    canvasBg.height = h * dpr;
    canvasFg.width = w * dpr;
    canvasFg.height = h * dpr;

    if (ctxBg) ctxBg.setTransform(dpr, 0, 0, dpr, 0, 0);
    if (ctxFg) ctxFg.setTransform(dpr, 0, 0, dpr, 0, 0);
  }

  onMount(() => {
    if (canvasBg && canvasFg) {
      ctxBg = canvasBg.getContext("2d");
      ctxFg = canvasFg.getContext("2d");
      resizeCanvas();
      window.addEventListener("resize", resizeCanvas);
    }
  });

  onDestroy(() => {
    if (typeof window !== "undefined") {
      window.removeEventListener("resize", resizeCanvas);
    }
    if (animationFrameId) {
      cancelAnimationFrame(animationFrameId);
    }
    clearAllTimers();
  });

  function clearAllTimers() {
    activeTimers.forEach(t => clearTimeout(t));
    activeTimers = [];
  }
</script>

<div class="canvas-container" class:fullscreen={isFullScreen}>
  <canvas
    bind:this={canvasBg}
    class="confetti-canvas bg"
    aria-hidden="true"
  ></canvas>
  <canvas
    bind:this={canvasFg}
    class="confetti-canvas fg"
    aria-hidden="true"
  ></canvas>
</div>

<style>
  .canvas-container {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    z-index: 100;
  }

  .canvas-container.fullscreen {
    position: fixed;
    width: 100vw;
    height: 100vh;
    z-index: 10005;
  }

  .confetti-canvas {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
  }
</style>
