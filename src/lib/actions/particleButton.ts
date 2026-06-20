interface Particle {
  x: number;
  y: number;
  vx: number;
  vy: number;
  size: number;
  opacity: number;
  life: number;
  maxLife: number;
  color: string;
  isBurst: boolean;
  driftFrequency?: number;
  driftAmplitude?: number;
  driftPhase?: number;
}

export function particleButton(node: HTMLElement) {
  // Ensure the node is relative so we can overlay absolute canvas
  const originalPosition = window.getComputedStyle(node).position;
  if (originalPosition === 'static') {
    node.style.position = 'relative';
  }

  // Create canvas
  const canvas = document.createElement('canvas');
  canvas.className = 'particle-canvas';
  
  // Extend canvas bounds beyond the button to allow particles to float freely outside
  const PADDING = 40;
  canvas.style.position = 'absolute';
  canvas.style.top = `-${PADDING}px`;
  canvas.style.left = `-${PADDING}px`;
  canvas.style.width = `calc(100% + ${PADDING * 2}px)`;
  canvas.style.height = `calc(100% + ${PADDING * 2}px)`;
  canvas.style.pointerEvents = 'none';
  canvas.style.zIndex = '0'; // render behind button contents but visible if button z-index is set up, or z-index = -1 if behind button itself
  
  // Make sure button has higher z-index for text/contents
  const originalZIndex = window.getComputedStyle(node).zIndex;
  if (originalZIndex === 'auto' || originalZIndex === '0') {
    // node.style.zIndex = '1'; // keep it clean
  }

  node.appendChild(canvas);
  const ctx = canvas.getContext('2d');

  let particles: Particle[] = [];
  let isHovering = false;
  let animationFrameId: number | null = null;
  let lastHoverSpawnTime = 0;
  let width = 0;
  let height = 0;

  // Resize canvas to match DPI
  function resize() {
    const rect = node.getBoundingClientRect();
    width = rect.width;
    height = rect.height;
    
    const dpr = window.devicePixelRatio || 1;
    canvas.width = (width + PADDING * 2) * dpr;
    canvas.height = (height + PADDING * 2) * dpr;
    
    if (ctx) {
      ctx.scale(dpr, dpr);
    }
  }

  // Set up resize observer to track button resizing
  const resizeObserver = new ResizeObserver(() => {
    resize();
  });
  resizeObserver.observe(node);
  resize();

  // Helper to get button accent color
  function getAccentColor(): { r: number; g: number; b: number } {
    const style = window.getComputedStyle(node);
    const bg = style.backgroundColor;
    
    // Parse rgb(r, g, b) or rgba(r, g, b, a)
    const match = bg.match(/\d+/g);
    if (match && match.length >= 3) {
      return {
        r: parseInt(match[0], 10),
        g: parseInt(match[1], 10),
        b: parseInt(match[2], 10)
      };
    }
    
    // Fallback to blue accent if parsing fails
    return { r: 91, g: 155, b: 213 };
  }

  function spawnHoverParticle() {
    // Spawn subtle white particles at the bottom of the button
    const pWidth = width;
    const pHeight = height;
    
    const x = PADDING + Math.random() * pWidth;
    // Spawn just slightly above the bottom edge of the button or around it
    const y = PADDING + pHeight - 2 + Math.random() * 4;
    
    particles.push({
      x,
      y,
      vx: (Math.random() - 0.5) * 0.1, // extremely slow horizontal drift
      vy: -0.15 - Math.random() * 0.2,  // float upward very slowly and smoothly
      size: 0.6 + Math.random() * 0.9,   // very small fine dust (0.6px to 1.5px)
      opacity: 0.25 + Math.random() * 0.25, // very faint, subtle visibility
      life: 0,
      maxLife: 80 + Math.random() * 60, // 1.3 to 2.3 seconds
      color: '255, 255, 255',
      isBurst: false,
      driftFrequency: 0.04 + Math.random() * 0.04,
      driftAmplitude: 0.1 + Math.random() * 0.1,
      driftPhase: Math.random() * Math.PI * 2
    });
  }

  function updateAndDraw() {
    if (!ctx) return;
    
    // Clear canvas
    ctx.clearRect(0, 0, width + PADDING * 2, height + PADDING * 2);
    
    const now = performance.now();
    
    // Spawn hover particles periodically (slower spawn rate for sparse, high-end feel)
    if (isHovering && now - lastHoverSpawnTime > 250) {
      spawnHoverParticle();
      lastHoverSpawnTime = now;
    }
    
    // Update and draw particles
    particles = particles.filter(p => {
      p.life++;
      
      // Float upward with subtle sine wave drift
      p.vy += -0.001; // very slight upward acceleration
      p.x += p.vx + Math.sin(p.life * (p.driftFrequency || 0.04) + (p.driftPhase || 0)) * (p.driftAmplitude || 0.1);
      p.y += p.vy;
      
      // Calculate opacity fade out
      const progress = p.life / p.maxLife;
      const currentOpacity = p.opacity * (1 - progress);
      
      if (progress >= 1 || currentOpacity <= 0) {
        return false;
      }
      
      // Draw particle
      ctx.beginPath();
      ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
      ctx.fillStyle = `rgba(${p.color}, ${currentOpacity})`;
      ctx.fill();
      
      return true;
    });
    
    // Stop loop if no particles left and user is not hovering
    if (particles.length === 0 && !isHovering) {
      stopLoop();
    } else {
      animationFrameId = requestAnimationFrame(updateAndDraw);
    }
  }

  function startLoop() {
    if (animationFrameId === null) {
      animationFrameId = requestAnimationFrame(updateAndDraw);
    }
  }

  function stopLoop() {
    if (animationFrameId !== null) {
      cancelAnimationFrame(animationFrameId);
      animationFrameId = null;
    }
    if (ctx) {
      ctx.clearRect(0, 0, width + PADDING * 2, height + PADDING * 2);
    }
  }

  // Event handlers
  function handleMouseEnter() {
    isHovering = true;
    lastHoverSpawnTime = performance.now();
    // Spawn initial particles immediately
    for (let i = 0; i < 2; i++) {
      spawnHoverParticle();
    }
    startLoop();
  }

  function handleMouseLeave() {
    isHovering = false;
  }

  node.addEventListener('mouseenter', handleMouseEnter);
  node.addEventListener('mouseleave', handleMouseLeave);

  return {
    destroy() {
      node.removeEventListener('mouseenter', handleMouseEnter);
      node.removeEventListener('mouseleave', handleMouseLeave);
      resizeObserver.disconnect();
      stopLoop();
      if (canvas.parentNode) {
        canvas.parentNode.removeChild(canvas);
      }
    }
  };
}
