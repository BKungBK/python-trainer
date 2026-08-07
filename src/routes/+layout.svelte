<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { invoke } from "@tauri-apps/api/core";
  import { createClient } from "@supabase/supabase-js";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { fade, scale } from "svelte/transition";
  import { smoothScroll } from "$lib/actions/smoothScroll";
  import { appState } from "$lib/state.svelte";
  import "../index.css";

  let { children } = $props();

  // State variables
  let currentUser = $derived(appState.currentUser);
  let syncStatus = $derived(appState.syncStatus);

  let isMaximized = $state(false);
  let isIdle = $state(false);
  let inactivityTimer: any;
  let presenceClient: any = null;
  let presenceChannel: any = null;

  async function setupPresenceRealtime() {
    try {
      const [url, anonKey]: [string, string] = await invoke("get_supabase_config");
      if (!url || !anonKey) return;

      presenceClient = createClient(url, anonKey);
      presenceChannel = presenceClient
        .channel("user-statuses")
        .on(
          "postgres_changes",
          { event: "*", schema: "public", table: "user_status" },
          (payload: any) => {
            const changed = payload.new ?? payload.old;
            if (!changed?.user_id) return;

            if (payload.eventType === "DELETE") {
              appState.onlineUsers = appState.onlineUsers.filter(
                (user: any) => user.user_id !== changed.user_id,
              );
              return;
            }

            appState.onlineUsers = [
              changed,
              ...appState.onlineUsers.filter(
                (user: any) => user.user_id !== changed.user_id,
              ),
            ];
          },
        )
        .subscribe();
    } catch (e) {
      console.error("Failed to subscribe to user presence:", e);
    }
  }

  function resetInactivityTimer() {
    if (inactivityTimer) {
      clearTimeout(inactivityTimer);
    }

    if (!currentUser) return;

    if (isIdle) {
      isIdle = false;
      runHeartbeat();
    }

    inactivityTimer = setTimeout(() => {
      isIdle = true;
      runHeartbeat();
    }, 10 * 1000); // 15 minutes
  }

  // Rotating developer-centric submessages during loading
  let loadingSubMessage = $state("กำลังเตรียมเครื่องมือฝึกฝนสำหรับคุณ");
  const rotatingSubMessages = [
    "กำลังวิเคราะห์โครงสร้างต้นไม้ไวยากรณ์ (AST)...",
    "กำลังตรวจสอบสภาพแวดล้อมและพาร์เซอร์ Python 3...",
    "กำลังซิงค์และประสานข้อมูลความคืบหน้าของวันนี้...",
    "กำลังดึงชุดโจทย์กรณีทดสอบจาก Supabase...",
    "กำลังเริ่มการทำงานของอินสแตนซ์ Monaco Editor...",
    "กำลังเชื่อมต่อคลาวด์โฮสต์คีย์เวิร์ดและโหนดสเตตเตอร์...",
  ];

  $effect(() => {
    let intervalId: any;
    if (appState.isLoading) {
      let index = 0;
      intervalId = setInterval(() => {
        index = (index + 1) % rotatingSubMessages.length;
        loadingSubMessage = rotatingSubMessages[index];
      }, 2000);
    } else {
      loadingSubMessage = "กำลังเตรียมเครื่องมือฝึกฝนสำหรับคุณ";
    }
    return () => {
      if (intervalId) clearInterval(intervalId);
    };
  });

  // Update Discord Rich Presence reactively when page, active user, or idle state changes
  $effect(() => {
    const user = currentUser;
    const pathname = $page.url.pathname;
    const problemId = $page.url.searchParams.get("problem");
    const idle = isIdle;

    if (user) {
      invoke("update_discord_presence", {
        pathname,
        problemId,
        user,
        isIdle: idle,
      }).catch((e) => {
        console.error("Failed to update Discord presence:", e);
      });
    }
  });

  // Window control functions
  function minimizeWindow() {
    getCurrentWindow().minimize();
  }

  function toggleMaximizeWindow() {
    getCurrentWindow().toggleMaximize();
  }

  function closeWindow() {
    getCurrentWindow().close();
  }

  // Load active user and check sync on startup
  onMount(() => {
    let unlistenResize: (() => void) | null = null;
    let interval: any;
    const activityEvents = ["mousemove", "keydown", "click", "scroll"];

    async function init() {
      const active = await appState.checkActiveUser();
      if (active) {
        await appState.prepareApp();
        await setupPresenceRealtime();

        // Initialize inactivity detection
        resetInactivityTimer();
        activityEvents.forEach((event) => {
          window.addEventListener(event, resetInactivityTimer);
        });

        await runHeartbeat();
      } else {
        appState.isLoading = false;
        appState.showProfileSelector = true;
      }

      // Setup window state check and listeners
      const appWindow = getCurrentWindow();
      isMaximized = await appWindow.isMaximized();

      unlistenResize = await appWindow.onResized(async () => {
        isMaximized = await appWindow.isMaximized();
      });

      // Keep the local user's activity status up to date.
      interval = setInterval(async () => {
        await runHeartbeat();
      }, 15000);
    }

    init();

    return () => {
      if (interval) clearInterval(interval);
      if (unlistenResize) unlistenResize();
      if (inactivityTimer) clearTimeout(inactivityTimer);
      if (presenceChannel) presenceChannel.unsubscribe();
      if (presenceClient) presenceClient.removeAllChannels();
      activityEvents.forEach((event) => {
        window.removeEventListener(event, resetInactivityTimer);
      });
    };
  });

  let profileName = $state("");
  let profileError = $state<string | null>(null);

  function openProfileSelector() {
    profileName = currentUser ?? "";
    profileError = null;
    appState.showProfileSelector = true;
  }

  async function selectProfile() {
    const name = profileName.trim();
    if (!name) {
      profileError = "กรุณาใส่ชื่อก่อนดำเนินการต่อ";
      return;
    }

    profileError = null;
    const saved = await appState.setUserName(name);
    if (!saved) {
      profileError = "บันทึกชื่อไม่สำเร็จ กรุณาลองใหม่อีกครั้ง";
      return;
    }

    // Set up inactivity detection on user selection
    resetInactivityTimer();
    const activityEvents = ["mousemove", "keydown", "click", "scroll"];
    activityEvents.forEach((event) => {
      window.removeEventListener(event, resetInactivityTimer);
      window.addEventListener(event, resetInactivityTimer);
    });

    await runHeartbeat();
  }

  async function runHeartbeat() {
    if (!currentUser) return;

    // Determine current status based on page location
    let status = "Online";
    let currentProblemId: string | null = null;

    const pathname = $page.url.pathname;
    if (pathname === "/daily") {
      status = "Solving Problem";
      currentProblemId = $page.url.searchParams.get("problem");
    }

    if (isIdle) {
      status = "Idle";
    }

    try {
      await invoke("send_heartbeat", {
        status,
        currentProblemId,
      });
      await appState.refreshOnlineUsers();
    } catch (e) {
      console.error("Heartbeat error:", e);
    }
  }

</script>

<div
  class="app"
  class:maximized={isMaximized}
  class:app-error={appState.isErrorActive}
>
  <!-- Custom Desktop Titlebar -->
  <div class="titlebar" data-tauri-drag-region>
    <div class="tb-left" data-tauri-drag-region>
      <div class="tb-logo-dot" data-tauri-drag-region></div>
      <div class="tb-name" data-tauri-drag-region>Code Trainer</div>
    </div>

    <div class="tb-controls">
      <button
        class="tb-btn"
        onclick={minimizeWindow}
        aria-label="Minimize"
        tabindex="-1"
      >
        <svg width="10" height="10" viewBox="0 0 10 10">
          <line
            x1="1"
            y1="5"
            x2="9"
            y2="5"
            stroke="currentColor"
            stroke-width="1.2"
          />
        </svg>
      </button>
      <button
        class="tb-btn"
        onclick={toggleMaximizeWindow}
        aria-label={isMaximized ? "Restore" : "Maximize"}
        tabindex="-1"
      >
        {#if isMaximized}
          <svg
            width="10"
            height="10"
            viewBox="0 0 10 10"
            fill="none"
            stroke="currentColor"
            stroke-width="1.2"
          >
            <rect x="3.5" y="1.5" width="5" height="5" />
            <rect x="1.5" y="3.5" width="5" height="5" fill="var(--bg-base)" />
          </svg>
        {:else}
          <svg
            width="10"
            height="10"
            viewBox="0 0 10 10"
            fill="none"
            stroke="currentColor"
            stroke-width="1.2"
          >
            <rect x="1.5" y="1.5" width="7" height="7" />
          </svg>
        {/if}
      </button>
      <button
        class="tb-btn close"
        onclick={closeWindow}
        aria-label="Close"
        tabindex="-1"
      >
        <svg width="10" height="10" viewBox="0 0 10 10">
          <line
            x1="2"
            y1="2"
            x2="8"
            y2="8"
            stroke="currentColor"
            stroke-width="1.2"
          />
          <line
            x1="8"
            y1="2"
            x2="2"
            y2="8"
            stroke="currentColor"
            stroke-width="1.2"
          />
        </svg>
      </button>
    </div>
  </div>

  <div class="app-body">
    <!-- Sidebar Navigation -->
    <aside class="sidebar">
      <div class="sb-section">เมนู</div>

      <a
        href="/"
        class="sb-item"
        class:active={$page.url.pathname === "/"}
        style="--i: 0;"
      >
        <svg
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
        </svg>
        <span>หน้าหลัก</span>
      </a>

      <a
        href="/daily"
        class="sb-item"
        class:active={$page.url.pathname === "/daily"}
        style="--i: 1;"
      >
        <svg
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <circle cx="12" cy="12" r="10" />
          <path d="M12 6v6l4 2" />
        </svg>
        <span>เริ่มฝึกฝน</span>
      </a>

      <a
        href="/daily/list"
        class="sb-item"
        class:active={$page.url.pathname === "/daily/list"}
        style="--i: 2;"
      >
        <svg
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path d="M9 11l3 3L22 4" />
          <path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11" />
        </svg>
        <span>รายการโจทย์วันนี้</span>
      </a>

      <a
        href="/settings"
        class="sb-item"
        class:active={$page.url.pathname === "/settings"}
        style="--i: 3;"
      >
        <svg
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <circle cx="12" cy="12" r="3" />
          <path
            d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"
          />
        </svg>
        <span>ตั้งค่า</span>
      </a>

      <a
        href="/submissions"
        class="sb-item"
        class:active={$page.url.pathname === "/submissions"}
        style="--i: 4;"
      >
        <svg
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <path d="M12 20h9" />
          <path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z" />
        </svg>
        <span>ประวัติการส่ง</span>
      </a>

      <div class="sb-spacer"></div>

      <!-- Sync Status -->
      <div
        style="padding: 0 8px; font-size: 11px; color: var(--text-muted); display: flex; align-items: center; gap: 6px; margin-bottom: 12px;"
      >
        {#if syncStatus === "Syncing"}
          <div
            class="status-dot solving animate-pulse"
            style="width: 5px; height: 5px;"
          ></div>
          <span>กำลังซิงค์ข้อมูล...</span>
        {:else if syncStatus === "Success"}
          <div class="status-dot online" style="width: 5px; height: 5px;"></div>
          <span>ซิงค์ข้อมูลแล้ว</span>
        {:else if syncStatus === "Idle"}
          <div class="status-dot online" style="width: 5px; height: 5px;"></div>
          <span>ออนไลน์</span>
        {:else}
          <div
            class="status-dot"
            style="width: 5px; height: 5px; background: var(--accent-error-bg); border: 1px solid var(--accent-error-border)"
          ></div>
          <span>ออฟไลน์</span>
        {/if}
      </div>

      <!-- Bottom User Presence Card -->
      {#if currentUser}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="sb-user"
          onclick={openProfileSelector}
        >
          <div class="avatar">{currentUser}</div>
          <div class="sb-user-info">
            <div class="sb-user-name">{currentUser}</div>
            <div class="sb-user-status">
              <div class="status-dot {isIdle ? 'idle' : 'online'}"></div>
              <span>{isIdle ? "ไม่ได้ใช้งาน" : "ออนไลน์"}</span>
            </div>
          </div>
          <svg
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            style="width: 12px; height: 12px; color: var(--text-muted);"
          >
            <path d="M6 9l6 6 6-6" />
          </svg>
        </div>
      {/if}
    </aside>

    <!-- Main Workspace Area -->
    <main class="content-area" use:smoothScroll>
      {#if currentUser}
        {@render children()}
      {/if}
    </main>
  </div>
</div>

<!-- Update Overlay Modal -->
{#if appState.updateAvailable}
  <div
    class="modal-overlay"
    style="z-index: 10001;"
    transition:fade={{ duration: 150 }}
  >
    <div
      class="card modal-content"
      transition:scale={{ duration: 200, start: 0.96 }}
      style="background: var(--bg-card); border: 1px solid var(--border); border-radius: var(--radius-md); width: 400px;"
    >
      <h2
        style="text-align: center; margin-bottom: 12px; font-weight: 500; font-size: 16px; color: var(--text-primary);"
      >
        พบเวอร์ชันใหม่ {appState.updateManifest.version}
      </h2>

      <div
        style="font-size: 13px; color: var(--text-secondary); margin-bottom: 20px; text-align: center; max-height: 150px; overflow-y: auto; padding: 10px; background: var(--bg-base); border-radius: 4px; border: 1px solid var(--border);"
      >
        <p style="font-weight: 500; margin-bottom: 6px; text-align: left;">
          มีอะไรใหม่ในเวอร์ชันนี้:
        </p>
        <p style="white-space: pre-wrap; text-align: left; line-height: 1.5;">
          {appState.updateManifest.body ||
            "แก้ไขบั๊กและปรับปรุงความเสถียรทั่วไป"}
        </p>
      </div>

      {#if appState.isUpdating}
        <div
          style="display: flex; flex-direction: column; align-items: center; gap: 12px; margin-top: 16px; width: 100%;"
        >
          <div class="update-progress-bar">
            <div class="update-progress-bar-fill"></div>
          </div>
          <p
            style="font-size: 12px; color: var(--text-muted); font-weight: 500;"
          >
            {appState.updateProgress}
          </p>
        </div>
      {:else}
        <div
          style="display: flex; gap: 10px; justify-content: center; margin-top: 16px;"
        >
          <button
            class="us-opt"
            style="flex: 1; padding: 10px; border-color: transparent; background: transparent; color: var(--text-muted);"
            onclick={() => (appState.updateAvailable = false)}
          >
            ไว้ทีหลัง
          </button>
          <button
            class="us-opt"
            style="flex: 1; padding: 10px; background: var(--accent-blue, #5b9bd5); color: white; border-color: var(--accent-blue, #5b9bd5);"
            onclick={() => appState.runAppUpdate()}
          >
            อัปเดตทันที
          </button>
        </div>
      {/if}
    </div>
  </div>
{/if}

<!-- Onboarding Profile Selector Modal -->
{#if appState.showProfileSelector}
  <div class="modal-overlay" transition:fade={{ duration: 150 }}>
    <div
      class="card modal-content"
      transition:scale={{ duration: 200, start: 0.96 }}
      style="background: var(--bg-card); border: 1px solid var(--border); border-radius: var(--radius-md); position: relative;"
    >
      {#if currentUser}
        <button
          class="modal-close-btn"
          onclick={() => (appState.showProfileSelector = false)}
          aria-label="ปิด"
        >
          <svg
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      {/if}

      <h2
        style="text-align: center; margin-bottom: 24px; font-weight: 500; font-size: 15px; letter-spacing: 0.05em; color: var(--text-primary);"
      >
        ใส่ชื่อของคุณ
      </h2>

      <p style="text-align: center; font-size: 12px; color: var(--text-muted); margin: 0 0 16px; line-height: 1.5;">
        ชื่อนี้จะถูกเก็บไว้ในเครื่องนี้จนกว่าคุณจะเปลี่ยนชื่อ
      </p>

      <input
        type="text"
        bind:value={profileName}
        maxlength="64"
        autocomplete="name"
        placeholder="ชื่อของคุณ"
        aria-label="ชื่อของคุณ"
        onkeydown={(event) => event.key === "Enter" && selectProfile()}
        style="width: 100%; box-sizing: border-box;"
      />

      {#if profileError}
        <p style="font-size: 12px; color: var(--accent-error); margin: 8px 0 0;">
          {profileError}
        </p>
      {/if}

      <button
        class="btn-submit"
        style="width: 100%; margin-top: 16px; padding: 11px;"
        onclick={selectProfile}
      >
        บันทึกชื่อและเริ่มใช้งาน
      </button>

      <p
        style="text-align: center; font-size: 11px; color: var(--text-muted); margin-top: 24px; line-height: 1.5;"
      >
        ชื่อของคุณใช้ระบุประวัติการฝึกและการส่งโค้ดบนเครื่องนี้
      </p>
    </div>
  </div>
{/if}

<!-- Premium Startup/Swithing Loading Screen -->
{#if appState.isLoading}
  <div class="loading-overlay" transition:fade={{ duration: 250 }}>
    <div class="loading-container">
      <div class="loading-spinner">
        <div class="spinner-ring"></div>
        <div class="spinner-core"></div>
      </div>
      <div class="loading-text">{appState.loadingMessage}</div>
      <div class="loading-sub">Code Trainer &middot; {loadingSubMessage}</div>
    </div>
  </div>
{/if}

<style>
  .loading-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: var(--bg-base, #161616);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 10000;
  }

  .loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 20px;
    max-width: 400px;
    text-align: center;
  }

  .loading-spinner {
    position: relative;
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .spinner-ring {
    position: absolute;
    width: 100%;
    height: 100%;
    border: 2px solid var(--border, #333333);
    border-radius: 50%;
  }

  .spinner-core {
    position: absolute;
    width: 100%;
    height: 100%;
    border: 2px solid transparent;
    border-top-color: var(--accent-blue, #5b9bd5);
    border-radius: 50%;
    animation: spin 1s cubic-bezier(0.4, 0, 0.2, 1) infinite;
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }

  .loading-text {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary, #e0e0e0);
    letter-spacing: 0.02em;
    min-height: 20px;
  }

  .loading-sub {
    font-size: 11px;
    color: var(--text-muted, #555555);
  }

  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: var(--bg-overlay);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
  }

  .modal-content {
    width: 340px;
    max-width: 90%;
    padding: 24px;
    box-shadow: var(--modal-shadow);
  }

  .modal-close-btn {
    position: absolute;
    top: 12px;
    right: 12px;
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-xs);
    transition:
      color 0.15s,
      background-color 0.15s;
  }

  .modal-close-btn:hover {
    color: var(--text-primary);
    background-color: var(--bg-surface-raised);
  }

  .modal-close-btn svg {
    width: 14px;
    height: 14px;
  }

  .animate-pulse {
    animation: pulse 2s infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.4;
    }
  }

</style>
