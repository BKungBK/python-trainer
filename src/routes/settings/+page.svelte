<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { appState } from "$lib/state.svelte";

  let activeUser = $state<string | null>(null);
  let pythonPath = $state("");
  let syncStatus = $state("Idle"); // "Idle", "Syncing", "Success", "Error"
  let syncError = $state<string | null>(null);

  // Update states
  let manualUpdateStatus = $state("Idle"); // "Idle", "Checking", "Found", "Latest", "Error"

  // Verification & Save states
  let saveStatus = $state("Idle"); // "Idle", "Saving", "Success", "Error"
  let saveError = $state<string | null>(null);
  let testStatus = $state("Idle"); // "Idle", "Testing", "Success", "Error"
  let testError = $state<string | null>(null);

  onMount(async () => {
    try {
      activeUser = await invoke("get_active_user");
      const path: string | null = await invoke("get_setting", {
        key: "python_path",
      });
      pythonPath = path || "";

      // If no path is saved, auto-detect it
      if (!pythonPath.trim()) {
        const detected: string | null = await invoke("detect_python");
        if (detected) {
          pythonPath = detected;
          testStatus = "Success";
        }
      }
    } catch (e) {
      console.error("Failed to load settings:", e);
    }
  });

  async function switchUser(userId: "NG" | "MR3") {
    try {
      await invoke("select_user", { userId });
      activeUser = userId;
      // Reload layout status
      window.location.reload();
    } catch (e) {
      console.error(e);
    }
  }

  async function handleAutoDetect() {
    testStatus = "Testing";
    testError = null;
    try {
      const detected: string | null = await invoke("detect_python");
      if (detected) {
        pythonPath = detected;
        testStatus = "Success";
      } else {
        testStatus = "Error";
        testError = "ไม่พบ Python ในระบบของคุณ โปรดระบุเส้นทางด้วยตนเอง";
      }
    } catch (e) {
      testStatus = "Error";
      testError = String(e);
    }
  }

  async function saveAllSettings() {
    saveStatus = "Saving";
    saveError = null;
    try {
      // Save python path
      await invoke("save_setting", { key: "python_path", value: pythonPath });

      // Run tests on Python path if configured
      if (pythonPath.trim().length > 0) {
        await testPythonPath();
      }

      saveStatus = "Success";
      setTimeout(() => (saveStatus = "Idle"), 3000);
    } catch (e) {
      saveStatus = "Error";
      saveError = String(e);
    }
  }

  async function testPythonPath() {
    testStatus = "Testing";
    testError = null;
    try {
      const testCode = 'print("OK")';
      // Execute test run against judge
      const results: any[] = await invoke("run_sample", {
        problemId: "dummy",
        code: testCode,
      });
      if (
        results &&
        results.length > 0 &&
        (results[0].verdict === "Passed" || results[0].verdict === "Accepted")
      ) {
        testStatus = "Success";
      } else {
        testStatus = "Error";
        testError =
          results[0]?.error || results[0]?.verdict || "การรัน Python ล้มเหลว";
      }
    } catch (e) {
      testStatus = "Error";
      testError = String(e);
    }
  }

  async function forceSync() {
    syncStatus = "Syncing";
    syncError = null;
    try {
      await invoke("sync_from_supabase");
      syncStatus = "Success";
      setTimeout(() => (syncStatus = "Idle"), 3000);
    } catch (e) {
      syncStatus = "Error";
      syncError = String(e);
    }
  }

  let rerollStatus = $state("Idle"); // "Idle", "Rerolling", "Success", "Error"
  let rerollError = $state<string | null>(null);

  async function handleRerollDaily() {
    if (
      !confirm(
        "คุณแน่ใจหรือไม่ที่จะล้างชุดโจทย์วันนี้และสุ่มโจทย์ประจำวันใหม่?",
      )
    )
      return;
    rerollStatus = "Rerolling";
    rerollError = null;
    try {
      await invoke("reroll_daily_challenge");

      // Clear client-side cache and prefetch the new daily challenge
      appState.dailyChallenge = null;
      await appState.prefetchDailyChallenge();

      rerollStatus = "Success";
      setTimeout(() => (rerollStatus = "Idle"), 3000);
    } catch (e) {
      rerollStatus = "Error";
      rerollError = String(e);
    }
  }
</script>

<div class="screen" style="max-width: 800px; margin: 0 auto; width: 100%;">
  <header class="pg-header">
    <h1 class="pg-title">ตั้งค่า</h1>
    <p class="pg-sub">บันทึกอยู่ในเครื่องในฐานข้อมูลการฝึกฝนของคุณ</p>
  </header>

  <div class="settings-form">
    <!-- Active Identity Switcher -->
    <div
      style="padding: 8px; margin: -8px; border-radius: var(--radius-sm); transition: background-color var(--duration-state) var(--ease-out-quart);"
      class:flash-highlight={saveStatus === "Success"}
    >
      <div class="settings-section-title">ผู้ใช้ปัจจุบัน</div>
      <div class="user-switcher">
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="us-opt"
          class:active={activeUser === "NG"}
          onclick={() => switchUser("NG")}
        >
          NG
        </div>
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="us-opt"
          class:active={activeUser === "MR3"}
          onclick={() => switchUser("MR3")}
        >
          MR3
        </div>
      </div>
    </div>

    <!-- Python Path Configuration -->
    <div
      style="padding: 8px; margin: -8px; border-radius: var(--radius-sm); transition: background-color var(--duration-state) var(--ease-out-quart);"
      class:flash-highlight={saveStatus === "Success"}
    >
      <div class="settings-section-title">การตั้งค่า Python</div>
      <div style="display: flex; flex-direction: column; gap: 8px;">
        <p
          style="font-size: 13px; color: var(--text-secondary); line-height: 1.6;"
        >
          กำหนดเส้นทาง (Path) ไปยังไฟล์โปรแกรม Python
          โดยค่าเริ่มต้นระบบจะพยายามรันคำสั่ง `python` หรือ `python3`
          จากตัวแปรสภาพแวดล้อม (Environment Variables) ของระบบโดยอัตโนมัติ
        </p>
        <div style="display: flex; gap: 10px; margin-top: 8px;">
          <input
            type="text"
            placeholder="e.g. C:\Users\Username\AppData\Local\Programs\Python\Python311\python.exe"
            bind:value={pythonPath}
            style="flex: 1;"
          />
          <button
            type="button"
            class="btn-run"
            onclick={handleAutoDetect}
            style="flex-shrink: 0;"
          >
            ค้นหาอัตโนมัติ
          </button>
        </div>

        {#if testStatus === "Testing"}
          <span style="font-size: 11px; color: var(--text-muted);"
            >กำลังตรวจสอบสภาพแวดล้อม Python...</span
          >
        {:else if testStatus === "Success"}
          <span
            style="font-size: 11px; color: var(--accent-success); font-weight: 500;"
            >✓ ตรวจสอบการรัน Python สำเร็จแล้ว</span
          >
        {:else if testStatus === "Error"}
          <span
            style="font-size: 11px; color: var(--accent-error); font-weight: 500;"
            >✗ ตรวจสอบล้มเหลว: {testError}</span
          >
        {/if}
      </div>
    </div>

    <!-- Database Sync -->
    <div
      style="padding: 8px; margin: -8px; border-radius: var(--radius-sm); transition: background-color var(--duration-state) var(--ease-out-quart);"
      class:flash-highlight={saveStatus === "Success"}
    >
      <div class="settings-section-title">ซิงค์ฐานข้อมูล</div>
      <div style="display: flex; flex-direction: column; gap: 12px;">
        <p
          style="font-size: 13px; color: var(--text-secondary); line-height: 1.6;"
        >
          ดึงข้อมูลโจทย์ หมวดหมู่ และการตั้งค่าเคสทดสอบล่าสุดลงมาจากฐานข้อมูล
          Supabase
        </p>
        <div
          style="display: flex; align-items: center; gap: 12px; margin-top: 4px;"
        >
          <button
            class="btn-run"
            onclick={forceSync}
            disabled={syncStatus === "Syncing"}
          >
            {syncStatus === "Syncing" ? "กำลังซิงค์..." : "ซิงค์ข้อมูลทันที"}
          </button>

          {#if syncStatus === "Syncing"}
            <span style="font-size: 11px; color: var(--text-muted);"
              >กำลังดาวน์โหลด...</span
            >
          {:else if syncStatus === "Success"}
            <span
              style="font-size: 11px; color: var(--accent-success); font-weight: 500;"
              >✓ ซิงค์ข้อมูลสำเร็จแล้ว</span
            >
          {:else if syncStatus === "Error"}
            <span
              style="font-size: 11px; color: var(--accent-error); font-weight: 500;"
              >✗ ซิงค์ข้อมูลล้มเหลว: {syncError}</span
            >
          {/if}
        </div>
      </div>
    </div>

    <!-- Daily Challenge Reroll -->
    <div
      style="padding: 8px; margin: -8px; border-radius: var(--radius-sm); transition: background-color var(--duration-state) var(--ease-out-quart);"
      class:flash-highlight={saveStatus === "Success"}
    >
      <div class="settings-section-title">การจัดการโจทย์ประจำวัน</div>
      <div style="display: flex; flex-direction: column; gap: 12px;">
        <p
          style="font-size: 13px; color: var(--text-secondary); line-height: 1.6;"
        >
          ล้างชุดโจทย์ประจำวันของวันนี้ออกจากเครื่องและจาก Supabase
          เพื่อให้ระบบทำการสุ่มเลือกโจทย์ใหม่ที่มีในคลังมาให้ฝึกฝนแทน
        </p>
        <div
          style="display: flex; align-items: center; gap: 12px; margin-top: 4px;"
        >
          <button
            class="btn-run"
            onclick={handleRerollDaily}
            disabled={rerollStatus === "Rerolling"}
            style="border-color: var(--accent-error-border); color: var(--accent-error);"
          >
            {rerollStatus === "Rerolling"
              ? "กำลังรีเซ็ต..."
              : "รีเซ็ตและสุ่มโจทย์วันนี้ใหม่"}
          </button>

          {#if rerollStatus === "Rerolling"}
            <span style="font-size: 11px; color: var(--text-muted);"
              >กำลังส่งคำขอล้างข้อมูล...</span
            >
          {:else if rerollStatus === "Success"}
            <span
              style="font-size: 11px; color: var(--accent-success); font-weight: 500;"
              >✓ รีเซ็ตและสุ่มโจทย์ใหม่สำเร็จแล้ว!
              (กรุณากลับไปที่หน้าความท้าทายประจำวัน)</span
            >
          {:else if rerollStatus === "Error"}
            <span
              style="font-size: 11px; color: var(--accent-error); font-weight: 500;"
              >✗ รีเซ็ตล้มเหลว: {rerollError}</span
            >
          {/if}
        </div>
      </div>
    </div>

    <!-- Application Updates -->
    <div
      style="padding: 8px; margin: -8px; border-radius: var(--radius-sm); transition: background-color var(--duration-state) var(--ease-out-quart);"
      class:flash-highlight={saveStatus === "Success"}
    >
      <div class="settings-section-title">การอัปเดตแอปพลิเคชัน</div>
      <div style="display: flex; flex-direction: column; gap: 12px;">
        <p
          style="font-size: 13px; color: var(--text-secondary); line-height: 1.6;"
        >
          ตรวจหาเวอร์ชันใหม่ของแอปพลิเคชันเพื่อเข้าถึงฟีเจอร์ล่าสุดและการปรับปรุงประสิทธิภาพ
        </p>
        <div
          style="display: flex; align-items: center; gap: 12px; margin-top: 4px;"
        >
          <button
            class="btn-run"
            onclick={async () => {
              manualUpdateStatus = "Checking";
              try {
                const found = await appState.checkForUpdatesSilently(true);
                if (found) {
                  manualUpdateStatus = "Found";
                } else {
                  manualUpdateStatus = "Latest";
                  setTimeout(() => (manualUpdateStatus = "Idle"), 3000);
                }
              } catch (err) {
                manualUpdateStatus = "Error";
                setTimeout(() => (manualUpdateStatus = "Idle"), 4000);
              }
            }}
            disabled={manualUpdateStatus === "Checking"}
          >
            {manualUpdateStatus === "Checking"
              ? "กำลังตรวจเช็ก..."
              : "ตรวจหาเวอร์ชันใหม่"}
          </button>

          {#if manualUpdateStatus === "Checking"}
            <span style="font-size: 11px; color: var(--text-muted);"
              >กำลังติดต่อ GitHub...</span
            >
          {:else if manualUpdateStatus === "Found"}
            <span
              style="font-size: 11px; color: var(--accent-blue, #5b9bd5); font-weight: 500;"
              >พบเวอร์ชันใหม่ {appState.updateManifest?.version}!
              หน้าต่างแจ้งเตือนได้เปิดขึ้นแล้ว</span
            >
          {:else if manualUpdateStatus === "Latest"}
            <span
              style="font-size: 11px; color: var(--accent-success); font-weight: 500;"
              >✓ คุณใช้เวอร์ชันล่าสุดอยู่แล้ว (v{appState.updateManifest
                ?.version})</span
            >
          {:else if manualUpdateStatus === "Error"}
            <span
              style="font-size: 11px; color: var(--accent-error); font-weight: 500;"
              >✗ เกิดข้อผิดพลาดในการตรวจสอบอัปเดต</span
            >
          {/if}
        </div>
      </div>
    </div>

    <!-- Form Actions -->
    <div class="divider" style="margin-top: 12px;"></div>
    <div
      style="display: flex; align-items: center; gap: 14px; padding-top: 4px;"
    >
      <button
        class="btn-submit"
        style="padding: 10px 24px; font-size: 13px;"
        onclick={saveAllSettings}
        disabled={saveStatus === "Saving"}
      >
        {saveStatus === "Saving" ? "กำลังบันทึก..." : "บันทึกการเปลี่ยนแปลง"}
      </button>

      {#if saveStatus === "Success"}
        <span
          style="font-size: 12px; color: var(--accent-success); font-weight: 500;"
          >✓ บันทึกตั้งค่าสำเร็จแล้ว</span
        >
      {:else if saveStatus === "Error"}
        <span
          style="font-size: 12px; color: var(--accent-error); font-weight: 500;"
          >✗ บันทึกตั้งค่าล้มเหลว: {saveError}</span
        >
      {:else}
        <span style="font-size: 12px; color: var(--text-muted);"
          >การเปลี่ยนแปลงมีผลทันที</span
        >
      {/if}
    </div>
  </div>
</div>
