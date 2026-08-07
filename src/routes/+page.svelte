<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { appState } from "$lib/state.svelte";
  import ProgressBar from "$lib/components/ProgressBar.svelte";

  let activeUser = $derived(appState.currentUser);
  let dailyProgress = $state<number>(0);
  let dailyRequired = $state<number>(0);
  let loading = $state(true);
  let onlineUsers = $derived(appState.onlineUsers);
  let visibleOnlineUsers = $derived(onlineUsers.filter((user) => isUserOnline(user)));

  function isUserOnline(user: any) {
    if (user.status === "Offline") return false;
    const lastActive = Date.parse(user.last_active ?? "");
    return Number.isFinite(lastActive) && Date.now() - lastActive <= 45_000;
  }

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

    {#if visibleOnlineUsers.length > 0}
      <section class="user-presence-section" aria-label="ผู้ใช้งาน">
        <div class="section-heading">
          <span class="section-kicker">PRESENCE</span>
          <span class="section-separator" aria-hidden="true">-</span>
          <span class="section-count" aria-label="online users">{visibleOnlineUsers.length}</span>
        </div>
        <div class="user-presence-list">
          {#each visibleOnlineUsers as user (user.user_id)}
            <div class="presence-user-row">
              <div class="presence-avatar-wrap">
                <div class="avatar presence-avatar" aria-hidden="true">
                  {#if user.avatar_url}
                    <img src={user.avatar_url} alt="" loading="lazy" />
                  {:else}
                    {user.user_id.slice(0, 2).toUpperCase()}
                  {/if}
                </div>
                <span class="presence-online-dot" aria-label="Online"></span>
              </div>
              <div class="fc-info">
                <div class="fc-name">
                  {user.user_id}
                  {#if user.user_id === activeUser}<span class="presence-you">คุณ</span>{/if}
                </div>
              </div>
              <span class="presence-status-badge">Online</span>
            </div>
          {/each}
        </div>
      </section>
    {/if}

  {/if}
</div>
