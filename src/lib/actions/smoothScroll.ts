/**
 * Svelte action to enable smooth momentum scrolling on traditional mouse wheels
 * while preserving native continuous scrolling on trackpads.
 */
export function smoothScroll(node: HTMLElement) {
  let targetScrollY = node.scrollTop;
  let currentScrollY = node.scrollTop;
  let animationFrameId: number | null = null;

  function update() {
    const diff = targetScrollY - currentScrollY;
    
    // Stop the loop when close enough to target
    if (Math.abs(diff) < 0.25) {
      node.scrollTop = targetScrollY;
      currentScrollY = targetScrollY;
      animationFrameId = null;
    } else {
      // Lerp coefficient: 0.15 makes scrolling feel fast and responsive, not laggy
      currentScrollY += diff * 0.15;
      node.scrollTop = currentScrollY;
      animationFrameId = requestAnimationFrame(update);
    }
  }

  function handleWheel(e: WheelEvent) {
    // Only intercept vertical scrolling
    if (e.deltaY === 0) return;

    // Traditional mouse wheels typically scroll in large discrete integer steps (e.g. 100, 120).
    // Touchpads/trackpads emit small, continuous, and often fractional delta values.
    const isTraditionalWheel = Number.isInteger(e.deltaY) && 
      (e.deltaY % 120 === 0 || e.deltaY % 100 === 0 || Math.abs(e.deltaY) >= 40);

    if (isTraditionalWheel) {
      e.preventDefault();
      
      const maxScroll = node.scrollHeight - node.clientHeight;
      targetScrollY = Math.max(0, Math.min(maxScroll, targetScrollY + e.deltaY));

      if (animationFrameId === null) {
        currentScrollY = node.scrollTop;
        animationFrameId = requestAnimationFrame(update);
      }
    } else {
      // Trackpad/touchpad scrolling: keep target values in sync with native scroll position
      if (animationFrameId !== null) {
        cancelAnimationFrame(animationFrameId);
        animationFrameId = null;
      }
      
      // Update target positions on next frame to match touchpad navigation
      requestAnimationFrame(() => {
        targetScrollY = node.scrollTop;
        currentScrollY = node.scrollTop;
      });
    }
  }

  // Listen to wheel events. passive: false is required to support preventDefault()
  node.addEventListener("wheel", handleWheel, { passive: false });

  // Handle focus changes (e.g., Tab key focusing out-of-view inputs) to sync targets
  function handleScroll() {
    if (animationFrameId === null) {
      targetScrollY = node.scrollTop;
      currentScrollY = node.scrollTop;
    }
  }
  node.addEventListener("scroll", handleScroll, { passive: true });

  return {
    destroy() {
      node.removeEventListener("wheel", handleWheel);
      node.removeEventListener("scroll", handleScroll);
      if (animationFrameId !== null) {
        cancelAnimationFrame(animationFrameId);
      }
    }
  };
}
