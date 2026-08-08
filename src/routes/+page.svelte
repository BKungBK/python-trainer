<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { appState } from "$lib/state.svelte";
  import Avatar from "$lib/components/Avatar.svelte";
  import ProgressBar from "$lib/components/ProgressBar.svelte";

  let activeUser = $derived(appState.currentUser);
  let dailyProgress = $state<number>(0);
  let dailyRequired = $state<number>(0);
  let loading = $state(true);
  let onlineUsers = $derived(appState.onlineUsers);
  let presenceNow = $state(Date.now());
  let visiblePresenceUsers = $derived(onlineUsers.filter((user) => isUserPresent(user)));

  const PRESENCE_STALE_MS = 45_000;

  type PresenceTone = "online" | "idle" | "solving" | "offline";
  type PresenceMeta = {
    label: string;
    tone: PresenceTone;
  };

  const PRESENCE_META: Record<string, PresenceMeta> = {
    Online: { label: "ออนไลน์", tone: "online" },
    Idle: { label: "ไม่ได้ใช้งาน", tone: "idle" },
    "Solving Problem": { label: "กำลังแก้โจทย์", tone: "solving" },
    Offline: { label: "ออฟไลน์", tone: "offline" },
  };

  function getPresenceMeta(status: unknown): PresenceMeta {
    return PRESENCE_META[String(status ?? "")] ?? PRESENCE_META.Online;
  }

  function isUserPresent(user: any) {
    if (user.status === "Offline") return false;
    const lastActive = Date.parse(user.last_active ?? "");
    return Number.isFinite(lastActive) && presenceNow - lastActive <= PRESENCE_STALE_MS;
  }

  onMount(() => {
    const presenceClock = window.setInterval(() => {
      presenceNow = Date.now();
    }, 15_000);

    return () => window.clearInterval(presenceClock);
  });

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

    {#if visiblePresenceUsers.length > 0}
      <section class="user-presence-section" aria-label="ผู้ใช้งาน">
        <div class="section-heading">
          <span class="section-kicker">PRESENCE</span>
          <span class="section-separator" aria-hidden="true">-</span>
          <span class="section-count" aria-label="present users">{visiblePresenceUsers.length}</span>
        </div>
        <div class="user-presence-list">
          {#each visiblePresenceUsers as user (user.user_id)}
            {@const presence = getPresenceMeta(user.status)}
            <div class="presence-user-row">
              <div class="presence-avatar-wrap">
                <div class="avatar presence-avatar" aria-hidden="true">
                  <Avatar src={user.avatar_url} name={user.user_id} />
                </div>
                <span
                  class="presence-online-dot presence-dot-{presence.tone}"
                  aria-label={presence.label}
                ></span>
              </div>
              <div class="fc-info">
                <div class="fc-name">
                  {user.user_id}
                  {#if user.user_id === activeUser}<span class="presence-you">คุณ</span>{/if}
                </div>
              </div>
              <span class="presence-status-badge presence-badge-{presence.tone}">{presence.label}</span>
            </div>
          {/each}
        </div>
      </section>
    {/if}

  {/if}
</div>
