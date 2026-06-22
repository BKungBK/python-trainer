import { invoke } from "@tauri-apps/api/core";
import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { getVersion } from "@tauri-apps/api/app"; // เพิ่มการนำเข้าตัวดึงเวอร์ชันของ Tauri
import { preloadCode } from "$app/navigation";
import { dev } from "$app/environment";

class AppState {
  currentUser = $state<string | null>(null);
  syncStatus = $state<string>("Idle"); // "Idle", "Syncing", "Success", "Error"
  peerStatus = $state<any>(null);
  needsRefresh = $state<number>(0);
  isLoading = $state<boolean>(true);
  showProfileSelector = $state<boolean>(false);
  loadingMessage = $state<string>("กำลังเริ่มระบบ...");
  isPowerModeActive = $state<boolean>(
    typeof window !== "undefined" ? localStorage.getItem("power_mode_active") !== "false" : true
  );

  togglePowerMode() {
    this.isPowerModeActive = !this.isPowerModeActive;
    if (typeof window !== "undefined") {
      localStorage.setItem("power_mode_active", String(this.isPowerModeActive));
    }
  }

  isErrorActive = $state<boolean>(false);
  private errorTimeoutId: any = null;

  triggerErrorFlash() {
    if (this.errorTimeoutId) {
      clearTimeout(this.errorTimeoutId);
      this.errorTimeoutId = null;
    }

    this.isErrorActive = false;

    // Use a tiny timeout to allow Svelte to remove the class and restart the animation
    setTimeout(() => {
      this.isErrorActive = true;
      this.errorTimeoutId = setTimeout(() => {
        this.isErrorActive = false;
        this.errorTimeoutId = null;
      }, 1800); // 1800ms matching CSS animation duration
    }, 0);
  }

  // Update state variables
  currentVersion = $state<string>("0.0.0"); // เพิ่ม State สำหรับเก็บเวอร์ชันปัจจุบันของแอป
  updateAvailable = $state<boolean>(false);
  updateManifest = $state<any>(null);
  isUpdating = $state<boolean>(false);
  updateProgress = $state<string>("ตรวจพบเวอร์ชันใหม่");

  // Daily challenge cache — prefetched at startup so pages load instantly
  dailyChallenge = $state<{
    problems: any[];
    category_progress: Record<string, { completed: number; required: number }>;
    solved_problem_ids: string[];
  } | null>(null);

  // Public test cases cache keyed by problem ID — avoids repeated SQLite calls
  publicTestCasesCache = new Map<string, any[]>();

  // ฟังก์ชันดึงเวอร์ชันปัจจุบันจากตัวระบบ Tauri
  async loadAppVersion() {
    try {
      this.currentVersion = await getVersion();
    } catch (e) {
      console.error("Failed to load app version:", e);
    }
  }

  async checkForUpdatesSilently(force = false) {
    if (dev && !force) {
      return false;
    }
    try {
      const update = await check();
      if (update && update.available) {
        this.updateManifest = update;
        this.updateAvailable = true;
        return true;
      }
      return false;
    } catch (e) {
      if (force) {
        console.error("Failed to check for updates:", e);
        throw e;
      }
      return false;
    }
  }

  async runAppUpdate() {
    if (!this.updateManifest) return;
    try {
      this.isUpdating = true;
      this.updateProgress = "กำลังดาวน์โหลดไฟล์อัปเดต...";

      await this.updateManifest.downloadAndInstall();

      this.updateProgress = "อัปเดตเสร็จสิ้น! กำลังรีสตาร์ทแอป...";
      setTimeout(async () => {
        await relaunch();
      }, 1500);
    } catch (e) {
      console.error("Update execution failed:", e);
      this.updateProgress = `อัปเดตล้มเหลว: ${e}`;
      setTimeout(() => {
        this.isUpdating = false;
      }, 4000);
    }
  }

  async checkActiveUser() {
    try {
      const active: string | null = await invoke("get_active_user");
      this.currentUser = active;
      return active;
    } catch (e) {
      console.error("Failed to check active user:", e);
      return null;
    }
  }

  async selectProfile(userId: "NG" | "MR3") {
    try {
      this.showProfileSelector = false;
      this.isLoading = true;
      this.loadingMessage = `กำลังสลับโปรไฟล์ไปยัง ${userId}...`;

      // Mark old user offline before switching
      if (this.currentUser) {
        try {
          await invoke("send_heartbeat", { status: "Offline", currentProblemId: null });
        } catch (e) {
          console.error("Failed to mark old user offline:", e);
        }
      }

      await invoke("select_user", { userId });
      this.currentUser = userId;

      // Clear all caches for the new user
      this.dailyChallenge = null;
      this.publicTestCasesCache = new Map();

      // Perform preparation (will refetch everything)
      await this.prepareApp();

      this.triggerRefresh();
    } catch (e) {
      console.error("Failed to set user:", e);
      this.isLoading = false;
      this.showProfileSelector = true;
    }
  }

  async prepareApp() {
    if (!this.currentUser) return;

    // ดึงเวอร์ชันปัจจุบันเข้ามาเก็บไว้ในระบบตอนเริ่มต้นใช้งานแอปทันที
    await this.loadAppVersion();

    // 0. Check for updates
    this.loadingMessage = "กำลังตรวจเช็กเวอร์ชันใหม่...";
    await this.checkForUpdatesSilently();

    // 1. Sync from Supabase
    this.syncStatus = "Syncing";
    this.loadingMessage = "กำลังตรวจสอบและซิงค์ข้อมูลกับระบบคลาวด์...";
    try {
      await invoke("sync_from_supabase");
      this.syncStatus = "Success";
    } catch (e) {
      console.error("Sync failed during startup:", e);
      this.syncStatus = "Error";
    }

    // 2. Verify Python configuration
    this.loadingMessage = "กำลังตรวจสอบสภาพแวดล้อม Python และคอมไพเลอร์...";
    try {
      let pyPath: string | null = await invoke("get_setting", { key: "python_path" });
      if (!pyPath || !pyPath.trim()) {
        pyPath = await invoke("detect_python");
        if (pyPath) {
          await invoke("save_setting", { key: "python_path", value: pyPath });
        }
      }
      const testCode = 'print("OK")';
      const results: any[] = await invoke("run_sample", { problemId: "dummy", code: testCode });
      if (!results || results.length === 0 || (results[0].verdict !== "Passed" && results[0].verdict !== "Accepted")) {
        console.warn("Python test run failed during startup");
      }
    } catch (e) {
      console.error("Python check failed during startup:", e);
    }

    // 3. Prefetch daily challenge — pages will read from cache instantly
    this.loadingMessage = "กำลังโหลดชุดโจทย์ประจำวัน...";
    await this.prefetchDailyChallenge();

    // 4. Preload SvelteKit route codes and Monaco Editor bundle in parallel during startup screen
    this.loadingMessage = "กำลังเพิ่มประสิทธิภาพการตอบสนองหน้าจอ...";
    try {
      await Promise.all([
        preloadCode("/daily"),
        preloadCode("/daily/list"),
        preloadCode("/settings"),
        preloadCode("/submissions"),
        // Dynamically import Monaco Editor to trigger downloading and parsing by browser in background
        import("monaco-editor").catch((e) => console.error("Monaco preload error:", e))
      ]);
    } catch (e) {
      console.error("Route code preloading failed:", e);
    }

    // Done — everything is ready
    this.isLoading = false;
  }

  async prefetchDailyChallenge() {
    try {
      const challenge: {
        problems: any[];
        category_progress: Record<string, { completed: number; required: number }>;
        solved_problem_ids: string[];
      } = await invoke("get_daily_challenge");
      this.dailyChallenge = challenge;

      // Prefetch public test cases for all problems in parallel
      if (challenge && challenge.problems) {
        await Promise.all(
          challenge.problems.map(async (prob) => {
            try {
              if (!this.publicTestCasesCache.has(prob.id)) {
                const tcs = await invoke("get_public_test_cases", { problemId: prob.id });
                this.publicTestCasesCache.set(prob.id, tcs as any[]);
              }
            } catch (e) {
              console.error(`Failed to prefetch test cases for ${prob.id}:`, e);
            }
          })
        );
      }
    } catch (e) {
      console.error("Failed to prefetch daily challenge:", e);
    }
  }

  // Refresh challenge data after a submission (invalidates Rust cache via backend)
  async refreshDailyChallenge() {
    await this.prefetchDailyChallenge();
  }

  triggerRefresh() {
    this.needsRefresh += 1;
  }
}

export const appState = new AppState();