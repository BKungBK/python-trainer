<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { appState } from "$lib/state.svelte";
  import ProgressBar from "$lib/components/ProgressBar.svelte";

  let activeUser = $derived(appState.currentUser);
  let dailyProgress = $state<number>(0);
  let dailyRequired = $state<number>(0);
  let loading = $state(true);
  
  // Peer tracking derived from global state
  let peerStatus = $derived(appState.peerStatus);

  // Format today's date
  const todayStr = new Intl.DateTimeFormat('th-TH', { 
    weekday: 'long', 
    day: 'numeric', 
    month: 'long' 
  }).format(new Date());

  // Get greeting based on time of day
  const greeting = (() => {
    const hour = new Date().getHours();
    if (hour >= 5 && hour < 12) {
      return "สวัสดีตอนเช้า";
    } else if (hour >= 12 && hour < 17) {
      return "สวัสดีตอนบ่าย";
    } else if (hour >= 17 && hour < 22) {
      return "สวัสดีตอนเย็น";
    } else {
      return "สวัสดีตอนดึก";
    }
  })();

  $effect(() => {
    if (activeUser) {
      const _ = appState.needsRefresh;
      if (appState.dailyChallenge) {
        // Read from cache — instant, no network or SQLite call
        const prog = appState.dailyChallenge.category_progress;
        let completed = 0;
        let required = 0;
        for (const cat in prog) {
          completed += prog[cat].completed;
          required += prog[cat].required;
        }
        dailyProgress = completed;
        dailyRequired = required;
        loading = false;
      } else {
        loading = true;
        loadDailyProgress().finally(() => { loading = false; });
      }
    }
  });

  async function loadDailyProgress() {
    try {
      const challenge: {
        category_progress: Record<string, { completed: number; required: number }>;
      } = await invoke("get_daily_challenge");
      
      let completed = 0;
      let required = 0;
      for (const cat in challenge.category_progress) {
        completed += challenge.category_progress[cat].completed;
        required += challenge.category_progress[cat].required;
      }
      dailyProgress = completed;
      dailyRequired = required;
    } catch (e) {
      console.error("Failed to load daily progress:", e);
    }
  }

  function getComputedPeerStatus(peer: typeof peerStatus) {
    if (!peer) return "Offline";
    const lastActive = new Date(peer.last_active);
    const now = new Date();
    const diffMs = now.getTime() - lastActive.getTime();
    if (diffMs > 120000) { // 2 minutes
      return "Offline";
    }
    return peer.status;
  }

  // Get peer dot class
  function getPeerDotClass(status: string) {
    switch (status) {
      case "Online": return "online";
      case "Solving Problem": return "solving";
      case "Idle": return "idle";
      default: return "";
    }
  }
</script>

<div class="screen">
  <header class="pg-header">
    <h1 class="pg-title">{greeting}, {activeUser || "นักเรียน"}</h1>
    <p class="pg-sub">{todayStr}</p>
  </header>

  {#if loading}
    <div style="font-size: 13px; color: var(--text-muted); text-align: center; margin-top: 40px;">
      กำลังโหลดแดชบอร์ด...
    </div>
  {:else}
    <!-- Cards Grid -->
    <div class="cards-grid" style="grid-template-columns: 1fr;">
      <!-- Unified Daily Progress Card -->
      <div class="card">
        <div class="card-label">ความคืบหน้าวันนี้</div>
        <div class="card-value">
          {dailyProgress}
          <span style="font-size: 14px; color: var(--text-muted); font-weight: 400;">/ {dailyRequired} ข้อ</span>
        </div>
        
        <div style="display: flex; align-items: center; gap: 12px; margin-top: 12px; margin-bottom: 8px;">
          <div style="flex: 1;">
            <ProgressBar value={dailyProgress} max={dailyRequired} />
          </div>
          <div class="progress-text" style="font-size: 12px; font-family: var(--font-mono); color: var(--text-secondary);">
            {dailyRequired > 0 ? Math.round((dailyProgress / dailyRequired) * 100) : 0}%
          </div>
        </div>

        <div class="card-sub">
          {#if dailyRequired > 0 && dailyProgress >= dailyRequired}
            ทำเป้าหมายครบทั้งหมดแล้ว! ยอดเยี่ยมมากสำหรับการฝึกฝนวันนี้
          {:else if dailyRequired > 0}
            วันนี้เหลืออีก {dailyRequired - dailyProgress} ข้อ เพื่อบรรลุเป้าหมาย
          {:else}
            ยังไม่ได้ตั้งค่าเป้าหมายประจำวัน
          {/if}
        </div>
      </div>
    </div>

    <!-- Peer Presence Section -->
    <div>
      <div style="font-size: 12px; color: var(--text-muted); margin-bottom: 10px; letter-spacing: 0.04em; text-transform: uppercase;">
        เพื่อนร่วมฝึกฝน
      </div>

      {#if peerStatus}
        {@const computedStatus = getComputedPeerStatus(peerStatus)}
        <div class="friend-card">
          <div class="avatar" style="width: 32px; height: 32px; font-size: 12px;">
            {peerStatus.user_id}
          </div>
          <div class="fc-info">
            <div class="fc-name">{peerStatus.user_id}</div>
            <div class="fc-status">
              <div class="status-dot {getPeerDotClass(computedStatus)}"></div>
              {#if computedStatus === "Solving Problem"}
                กำลังแก้โจทย์ประจำวัน
              {:else if computedStatus === "Online"}
                ออนไลน์
              {:else if computedStatus === "Idle"}
                ไม่ได้ใช้งาน
              {:else}
                ออฟไลน์
              {/if}
            </div>
          </div>
          {#if computedStatus === "Solving Problem"}
            <div class="fc-badge solving">กำลังแก้โจทย์</div>
          {:else}
            <div class="fc-badge">
              {#if computedStatus === "Online"}ออนไลน์{:else if computedStatus === "Offline"}ออฟไลน์{:else if computedStatus === "Idle"}ไม่ได้ใช้งาน{:else}{computedStatus}{/if}
            </div>
          {/if}
        </div>
      {:else}
        <div class="friend-card" style="justify-content: center; padding: 20px; color: var(--text-muted); font-size: 12px;">
          ไม่มีข้อมูลสถานะของเพื่อนในขณะนี้
        </div>
      {/if}
    </div>
  {/if}
</div>
