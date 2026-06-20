<script lang="ts">
  import { onMount } from "svelte";
  import { invoke as tauriInvoke } from "@tauri-apps/api/core";
  import Editor from "$lib/components/Editor.svelte";
  import Confetti from "$lib/components/Confetti.svelte";
  import ProgressBar from "$lib/components/ProgressBar.svelte";
  import { appState } from "$lib/state.svelte";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { particleButton } from "$lib/actions/particleButton";
  import { smoothScroll } from "$lib/actions/smoothScroll";


  // Fallback mock invoke when running outside Tauri context
  async function invoke(cmd: string, args: any = {}): Promise<any> {
    try {
      if (typeof window !== "undefined" && (window as any).__TAURI_INTERNALS__) {
        return await tauriInvoke(cmd, args);
      }
    } catch (e) {
      console.warn("Tauri invoke failed, using mock data fallback", e);
    }

    if (cmd === "get_daily_challenge") {
      return {
        problems: [
          {
            id: "circle_area",
            title: "หาพื้นที่วงกลม",
            description: "จงเขียนโปรแกรมคำนวณพื้นที่วงกลมจากรัศมี r ที่กำหนดให้\nสูตร: Area = pi * r^2 โดยที่ pi = 3.14159",
            category: "Input / Output",
            input_specification: "บรรทัดแรกระบุทศนิยม r แทนรัศมีของวงกลม (0 <= r <= 1000)",
            output_specification: "แสดงพื้นที่วงกลมเป็นทศนิยม 2 ตำแหน่ง",
          },
          {
            id: "even_odd",
            title: "เลขคู่หรือเลขคี่",
            description: "รับจำนวนเต็ม x แล้วตรวจสอบว่าเป็นเลขคู่หรือเลขคี่",
            category: "Conditions",
            input_specification: "จำนวนเต็ม x (-10^9 <= x <= 10^9)",
            output_specification: "แสดงคำว่า 'Even' หากเป็นเลขคู่ หรือ 'Odd' หากเป็นเลขคี่",
          }
        ],
        category_progress: {
          "Input / Output": { completed: 1, required: 1 },
          "Conditions": { completed: 0, required: 1 },
        },
        solved_problem_ids: ["circle_area"]
      };
    } else if (cmd === "get_public_test_cases") {
      return [
        { input: "5.0", expected_output: "78.54" },
        { input: "10.0", expected_output: "314.16" }
      ];
    }
    return null;
  }

  // Daily Challenge data — initialize from cache immediately to avoid loading flash
  let problems = $state<any[]>(appState.dailyChallenge?.problems ?? []);
  let categoryProgress = $state<Record<string, { completed: number; required: number }>>(
    appState.dailyChallenge?.category_progress ?? {}
  );
  let solvedProblemIds = $state<string[]>(appState.dailyChallenge?.solved_problem_ids ?? []);

  // Determine initial selected problem synchronously to prevent initial null render and layout flash
  let initialProblem = (() => {
    if (typeof window === "undefined" || !appState.dailyChallenge) return null;
    const url = new URL(window.location.href);
    const problemId = url.searchParams.get("problem");
    if (problemId) {
      return appState.dailyChallenge.problems.find(p => p.id === problemId) || null;
    }
    // If no problem in URL, auto-select the first unsolved one, or first overall
    const unsolved = appState.dailyChallenge.problems.filter(
      p => !appState.dailyChallenge!.solved_problem_ids.includes(p.id)
    );
    if (unsolved.length > 0) return unsolved[0];
    return appState.dailyChallenge.problems[0] || null;
  })();

  let selectedProblem = $state<any | null>(initialProblem);
  let publicTestCases = $state<any[]>(
    initialProblem && appState.publicTestCasesCache.has(initialProblem.id)
      ? appState.publicTestCasesCache.get(initialProblem.id)!
      : []
  );
  let code = $state<string>(
    initialProblem
      ? `def solve():\n    # เขียนโค้ดของคุณตรงนี้\n    pass\n\nif __name__ == '__main__':\n    solve()\n`
      : ""
  );
  // If cache is already populated, skip the loading state entirely
  let loading = $state(appState.dailyChallenge === null);
  let isSwitchingProblem = $state(initialProblem !== null);

  // Confetti trigger bindings
  let triggerScreenConfetti = $state<(x?: number, y?: number) => void>(() => {});

  // Rotating loading messages
  let challengeLoaderMessage = $state("กำลังโหลดเป้าหมายการฝึกฝนของวันนี้...");
  let runLoaderMessage = $state("กำลังรันโค้ดตัวอย่าง...");
  let submitLoaderMessage = $state("กำลังส่งโค้ด...");

  const challengeMessages = [
    "กำลังวิเคราะห์โครงสร้างข้อมูลท้าทายประจำวัน...",
    "กำลังประมวลผลชุดโจทย์วิเคราะห์ภาษา Python...",
    "กำลังโหลดคะแนนสถิติประจำวันและความคืบหน้า..."
  ];

  const runMessages = [
    "กำลังตรวจสอบและรันกรณีตัวอย่าง...",
    "กำลังพาร์สและคอมไพล์โค้ดในแซนด์บ็อกซ์...",
    "กำลังเปรียบเทียบค่าความถูกต้องไพพาย..."
  ];

  const submitMessages = [
    "กำลังส่งโค้ดประเมินผล...",
    "กำลังวิเคราะห์กรณีทดสอบที่ครอบคลุม...",
    "กำลังวัดค่าทรัพยากรและตรวจความคลาดเคลื่อน..."
  ];

  function selectRandomLoaderMessage(type: "challenge" | "run" | "submit") {
    if (type === "challenge") {
      challengeLoaderMessage = challengeMessages[Math.floor(Math.random() * challengeMessages.length)];
    } else if (type === "run") {
      runLoaderMessage = runMessages[Math.floor(Math.random() * runMessages.length)];
    } else if (type === "submit") {
      submitLoaderMessage = submitMessages[Math.floor(Math.random() * submitMessages.length)];
    }
  }


  // Split Panel Resizing
  let editorWidth = $state(480);
  let isResizing = $state(false);

  function startResize(e: MouseEvent) {
    e.preventDefault();
    isResizing = true;
    const startX = e.clientX;
    const startWidth = editorWidth;

    function doResize(moveEvent: MouseEvent) {
      if (!isResizing) return;
      const deltaX = moveEvent.clientX - startX;
      let newWidth = startWidth - deltaX;

      const minEditorWidth = 380;
      const maxEditorWidth = window.innerWidth - 450;

      if (newWidth < minEditorWidth) newWidth = minEditorWidth;
      if (newWidth > maxEditorWidth) newWidth = maxEditorWidth;

      editorWidth = newWidth;
    }

    function stopResize() {
      isResizing = false;
      window.removeEventListener("mousemove", doResize);
      window.removeEventListener("mouseup", stopResize);
      localStorage.setItem("editor_panel_width", String(editorWidth));
    }

    window.addEventListener("mousemove", doResize);
    window.addEventListener("mouseup", stopResize);
  }

  // Category translation map
  const categoryTranslations: Record<string, string> = {
    "Input / Output": "ข้อมูลเข้า / ข้อมูลออก (Input / Output)",
    "Conditions": "เงื่อนไข (Conditions)",
    "Loops": "การวนซ้ำ (Loops)",
    "Functions": "ฟังก์ชัน (Functions)",
    "Lists": "ลิสต์ / อาร์เรย์ (Lists)",
    "Implementation": "การใช้งานจริง (Implementation)",
    "Math": "คณิตศาสตร์ (Math)",
    "String": "การจัดการข้อความ (String)",
    "Sorting": "การเรียงลำดับ (Sorting)",
    "Searching": "การค้นหาข้อมูล (Searching)",
    "Greedy": "ขั้นตอนวิธีแบบโลภ (Greedy)",
    "Recursion": "การเรียกซ้ำ (Recursion)",
    "Data Structures": "โครงสร้างข้อมูล (Data Structures)",
    "Graph": "กราฟ (Graph)",
    "Dynamic Programming": "การโปรแกรมพลวัต (Dynamic Programming)",
    "Matrix Operations": "การดำเนินการเมทริกซ์ (Matrix Operations)",
    "Numerical Methods": "ระเบียบวิธีเชิงตัวเลข (Numerical Methods)",
    "Simulation": "การจำลองสถานการณ์ (Simulation)",
    "Optimization": "การเพิ่มประสิทธิภาพ (Optimization)",
    "Data Parsing": "การแจกแจงข้อมูล (Data Parsing)",
    "Statistics": "สถิติ (Statistics)",
    "Signal Processing": "การประมวลผลสัญญาณ (Signal Processing)"
  };

  // Judging & Run States
  let isRunningSample = $state(false);
  let isSubmitting = $state(false);
  
  // Results
  let sampleResults = $state<any[] | null>(null);
  let submissionResult = $state<{
    verdict: string;
    score: number;
    passed_count: number;
    total_count: number;
    details: string[];
  } | null>(null);
  let errorMsg = $state<string | null>(null);

  // Summary counts
  let totalCompleted = $derived(
    Object.values(categoryProgress).reduce((acc, curr) => acc + curr.completed, 0)
  );
  let totalRequired = $derived(
    Object.values(categoryProgress).reduce((acc, curr) => acc + curr.required, 0)
  );

  let activeUser = $derived(appState.currentUser);
  let prevUser = $state<string | null>(null);
  let prevRefresh = $state(appState.needsRefresh);

  // Reset workspace state ONLY when profile actually changes or refresh is requested
  $effect(() => {
    const user = activeUser;
    const refresh = appState.needsRefresh;
    
    // On initial mount, just store the initial values to avoid resetting
    if (prevUser === null) {
      prevUser = user;
      prevRefresh = refresh;
      return;
    }
    
    if (user !== prevUser || refresh !== prevRefresh) {
      prevUser = user;
      prevRefresh = refresh;
      
      selectedProblem = null;
      sampleResults = null;
      submissionResult = null;
      errorMsg = null;
      publicTestCases = [];
    }
  });

  // Sync daily challenge data from global cache to local page state
  $effect(() => {
    const challenge = appState.dailyChallenge;
    if (challenge) {
      problems = challenge.problems;
      categoryProgress = challenge.category_progress;
      solvedProblemIds = challenge.solved_problem_ids || [];
      loading = false;
    } else if (activeUser) {
      loadDailyChallenge();
    }
  });

  // Reactive URL query parameter routing
  $effect(() => {
    if (problems.length > 0) {
      const problemId = $page.url.searchParams.get("problem");
      if (problemId) {
        const found = problems.find(p => p.id === problemId);
        if (found) {
          if (!selectedProblem || selectedProblem.id !== found.id) {
            selectProblem(found);
          }
        } else {
          autoSelectRandomOrRedirect();
        }
      } else {
        // If there's no problem parameter in the URL but we already have selectedProblem initialized,
        // just update the URL query parameter silently without rebuilding the component.
        if (selectedProblem) {
          goto(`/daily?problem=${selectedProblem.id}`, { replaceState: true, noScroll: true });
        } else {
          autoSelectRandomOrRedirect();
        }
      }
    }
  });

  function autoSelectRandomOrRedirect() {
    const unsolved = problems.filter(p => !solvedProblemIds.includes(p.id));
    if (unsolved.length > 0) {
      const randomProb = unsolved[Math.floor(Math.random() * unsolved.length)];
      goto(`/daily?problem=${randomProb.id}`, { replaceState: true });
    } else if (problems.length > 0) {
      // All problems completed! Redirect to list view so they see the checklist and celebrate
      goto("/daily/list", { replaceState: true });
    }
  }

  onMount(async () => {
    // If we have an initial problem, show the loading screen for 350ms to allow Monaco to render smoothly
    if (initialProblem) {
      setTimeout(() => {
        isSwitchingProblem = false;
      }, 350);
    }

    const savedWidth = localStorage.getItem("editor_panel_width");
    if (savedWidth) {
      const parsed = parseInt(savedWidth, 10);
      if (!isNaN(parsed) && parsed >= 380) {
        editorWidth = parsed;
      }
    }
  });

  async function loadDailyChallenge() {
    loading = true;
    selectRandomLoaderMessage("challenge");
    try {
      const challenge: {
        problems: any[];
        category_progress: Record<string, { completed: number; required: number }>;
        solved_problem_ids: string[];
      } = await invoke("get_daily_challenge");
      
      problems = challenge.problems;
      categoryProgress = challenge.category_progress;
      solvedProblemIds = challenge.solved_problem_ids || [];
    } catch (e) {
      console.error("Failed to load daily challenge:", e);
      errorMsg = "โปรดซิงค์ข้อมูลกับ Supabase ในหน้าตั้งค่าเพื่อเริ่มต้นเป้าหมายประจำวัน";
    } finally {
      loading = false;
    }
  }

  async function loadDailyChallengeProgressOnly() {
    try {
      await appState.refreshDailyChallenge();
      if (appState.dailyChallenge) {
        categoryProgress = appState.dailyChallenge.category_progress;
        solvedProblemIds = appState.dailyChallenge.solved_problem_ids || [];
      }
    } catch (e) {
      console.error(e);
    }
  }

  async function selectProblem(prob: any) {
    selectedProblem = prob;
    code = `def solve():\n    # เขียนโค้ดของคุณตรงนี้\n    pass\n\nif __name__ == '__main__':\n    solve()\n`;
    sampleResults = null;
    submissionResult = null;
    errorMsg = null;
    publicTestCases = [];

    // Send heartbeat presence
    invoke("send_heartbeat", { status: "Solving Problem", currentProblemId: prob.id }).catch(console.error);

    // Use test case cache to avoid repeated SQLite calls
    if (appState.publicTestCasesCache.has(prob.id)) {
      publicTestCases = appState.publicTestCasesCache.get(prob.id)!;
    } else {
      try {
        const tcs = await invoke("get_public_test_cases", { problemId: prob.id });
        publicTestCases = tcs as any[];
        appState.publicTestCasesCache.set(prob.id, publicTestCases);
      } catch (e) {
        console.error("Failed to fetch public test cases:", e);
      }
    }
  }

  function goBackToChecklist() {
    selectedProblem = null;
    sampleResults = null;
    submissionResult = null;
    errorMsg = null;
    publicTestCases = [];

    // Reset heartbeat presence
    invoke("send_heartbeat", { status: "Online", currentProblemId: null }).catch(console.error);
    
    // Redirect to list
    goto("/daily/list");
  }

  function selectRandomProblem() {
    if (problems.length <= 1) return;
    
    // Try to find unsolved problems first, excluding the current one
    let candidates = problems.filter(p => !solvedProblemIds.includes(p.id) && p.id !== selectedProblem?.id);
    
    // If no unsolved candidates, allow solved ones, excluding the current one
    if (candidates.length === 0) {
      candidates = problems.filter(p => p.id !== selectedProblem?.id);
    }
    
    // Fallback
    if (candidates.length === 0) {
      candidates = problems;
    }
    
    const randomProb = candidates[Math.floor(Math.random() * candidates.length)];
    if (randomProb) {
      goto(`/daily?problem=${randomProb.id}`);
    }
  }

  async function runSample() {
    if (!selectedProblem || isRunningSample) return;
    isRunningSample = true;
    selectRandomLoaderMessage("run");
    sampleResults = null;
    submissionResult = null;
    errorMsg = null;
    try {
      sampleResults = await invoke("run_sample", {
        problemId: selectedProblem.id,
        code: code,
      });
      // Check if any test case in sample failed
      const hasFailure = sampleResults && sampleResults.some(res => res.verdict !== "Passed" && res.verdict !== "Accepted");
      if (hasFailure) {
        appState.triggerErrorFlash();
      }
    } catch (e) {
      errorMsg = String(e);
      appState.triggerErrorFlash();
    } finally {
      isRunningSample = false;
    }
  }

  async function submitSolution(e?: MouseEvent) {
    if (!selectedProblem || isSubmitting) return;
    isSubmitting = true;
    selectRandomLoaderMessage("submit");
    sampleResults = null;
    submissionResult = null;
    errorMsg = null;
    
    const wasCompletedBefore = totalRequired > 0 && totalCompleted === totalRequired;

    try {
      const res: any = await invoke("submit_solution", {
        problemId: selectedProblem.id,
        code: code,
      });
      submissionResult = res;
      
      await loadDailyChallengeProgressOnly();

      if (res && res.score === 100) {
        if (e && e.clientX !== undefined && e.clientY !== undefined) {
          triggerScreenConfetti(e.clientX, e.clientY);
        } else {
          triggerScreenConfetti();
        }
        
        const isCompletedNow = totalRequired > 0 && totalCompleted === totalRequired;
        if (isCompletedNow && !wasCompletedBefore) {
          setTimeout(() => {
            triggerScreenConfetti();
          }, 800);
        }
      } else {
        appState.triggerErrorFlash();
      }
    } catch (e) {
      errorMsg = String(e);
      appState.triggerErrorFlash();
    } finally {
      isSubmitting = false;
    }
  }

  function getEnglishTitle(title: string): string {
    const match = title.match(/\(([^)]+)\)/);
    return match ? match[1] : title;
  }

  function getVerdictClass(verdict: string) {
    if (verdict === "Passed" || verdict === "Accepted") return "t-pass";
    return "t-fail";
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (!selectedProblem) return;
    
    if ((e.ctrlKey || e.metaKey) && e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      runSample();
    }
    
    if ((e.ctrlKey || e.metaKey) && e.key === "Enter" && e.shiftKey) {
      e.preventDefault();
      submitSolution();
    }
  }
</script>

<svelte:window onkeydown={handleKeyDown} />

{#if loading || isSwitchingProblem}
  <div class="challenge-loader">
    <div class="cl-spinner">
      <div class="cl-ring"></div>
      <div class="cl-core"></div>
    </div>
    <div class="cl-msg">{isSwitchingProblem ? "กำลังจัดเตรียมพื้นที่ทำงาน..." : challengeLoaderMessage}</div>
  </div>
{:else if problems.length === 0}
  <div class="screen" style="align-items: center; justify-content: center; gap: 16px; min-height: 100%;">
    <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" style="color: var(--text-muted);">
      <path d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
    </svg>
    <span style="font-weight: 500; font-size: 14px;">ไม่มีโจทย์อยู่ในแคชท้องถิ่น</span>
    <p style="color: var(--text-muted); max-width: 400px; text-align: center; font-size: 13px; line-height: 1.6;">
      โปรดไปที่หน้า "ตั้งค่า" และกดซิงค์ฐานข้อมูลเพื่อดึงโจทย์การเขียนโค้ดลงมาจาก Supabase
    </p>
    <a href="/settings" style="margin-top: 8px;">
      <button class="btn-run" style="padding: 10px 20px;">ไปที่หน้าตั้งค่า</button>
    </a>
  </div>
{:else if selectedProblem}
  <div class="workspace-slide-in" style="display: flex; flex-direction: column; height: 100%;">
    <div class="workspace-header">
      <button class="btn-run" style="padding: 5px 10px;" onclick={goBackToChecklist} aria-label="ย้อนกลับ">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width: 14px; height: 14px;">
          <line x1="19" y1="12" x2="5" y2="12"/>
          <polyline points="12 19 5 12 12 5"/>
        </svg>
      </button>
      <span style="font-size: 12px; color: var(--text-muted); font-weight: 500; text-transform: uppercase; letter-spacing: 0.05em;">
        โจทย์ประจำวัน &middot; {categoryTranslations[selectedProblem.category] || selectedProblem.category}
      </span>
      <div style="flex: 1;"></div>
      <button 
        class="btn-run" 
        style="padding: 6px 12px; gap: 6px; display: flex; align-items: center; border-radius: var(--radius-sm);" 
        onclick={selectRandomProblem} 
        aria-label="สุ่มโจทย์ใหม่"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="width: 13px; height: 13px;">
          <path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.57-8.38l5.67-5.67"/>
        </svg>
        <span style="font-size: 11px; font-weight: 500;">สุ่มโจทย์อื่น</span>
      </button>
    </div>

    <div class="workspace-layout">
      <!-- Left Pane: Problem Details -->
      <div class="prob-left" use:smoothScroll>
        <div class="prob-title-area">
          <h2 class="prob-title">{selectedProblem.title}</h2>
        </div>

        <div>
          <div class="prob-section-label">คำอธิบาย</div>
          <p class="prob-text" style="white-space: pre-wrap;">{selectedProblem.description}</p>
        </div>

        <div>
          <div class="prob-section-label">รูปแบบข้อมูลเข้า</div>
          <p class="prob-text" style="white-space: pre-wrap;">{selectedProblem.input_specification}</p>
        </div>

        <div>
          <div class="prob-section-label">รูปแบบข้อมูลออก</div>
          <p class="prob-text" style="white-space: pre-wrap;">{selectedProblem.output_specification}</p>
        </div>

        <!-- Public Test Cases Examples -->
        {#if publicTestCases.length > 0}
          <div>
            <div class="prob-section-label">ตัวอย่าง</div>
            <div style="display: flex; flex-direction: column; gap: 12px;">
              {#each publicTestCases as tc, index}
                <div class="prob-code" style="background: #141414;">
                  <div style="color: var(--text-muted); font-size: 10px; text-transform: uppercase; margin-bottom: 4px; font-weight: 500;">
                    กรณีตัวอย่าง #{index + 1}
                  </div>
                  <div style="margin-bottom: 8px;">
                    <span style="color: var(--accent-blue);">ข้อมูลเข้า (Input):</span>
                    <pre style="margin-top: 2px; font-family: var(--font-mono); color: var(--text-secondary);">{tc.input || "(ไม่มีข้อมูลเข้า)"}</pre>
                  </div>
                  <div>
                    <span style="color: var(--accent-success);">ข้อมูลออก (Output):</span>
                    <pre style="margin-top: 2px; font-family: var(--font-mono); color: var(--text-secondary);">{tc.expected_output}</pre>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>

      <!-- Resize Handle -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div 
        class="resize-handle" 
        class:active={isResizing}
        onmousedown={startResize}
      ></div>

      <!-- Right Pane: Monaco Editor & Judging Results -->
      <div class="prob-right" style="width: {editorWidth}px;">
        <!-- Editor wrapper -->
        <div class="editor-area">
          <div class="editor-topbar">
            <div class="etab active">solution.py</div>
            <div style="flex: 1;"></div>
            <div style="font-size: 11px; color: var(--text-muted); font-family: var(--font-mono); display: flex; align-items: center; gap: 8px;">
              <button 
                class="btn-spark-toggle" 
                class:active={appState.isPowerModeActive} 
                onclick={() => appState.togglePowerMode()}
                title="เปิด/ปิดเอฟเฟกต์ละอองขณะพิมพ์ (Typing Sparks)"
                aria-label="Toggle typing sparks"
              >
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width: 12px; height: 12px;">
                  <path d="M12 3v3M12 18v3M3 12h3M18 12h3M5.6 5.6l2.1 2.1M16.3 16.3l2.1 2.1M5.6 18.4l2.1-2.1M16.3 7.7l2.1-2.1"/>
                </svg>
              </button>
              <span>Python 3</span>
            </div>
          </div>
          
          <div style="flex: 1; min-height: 0;">
            <Editor bind:code={code} />
          </div>

          <div class="editor-footer">
            <button 
              class="btn-run" 
              onclick={runSample} 
              disabled={isRunningSample || isSubmitting}
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" style="width: 12px; height: 12px;">
                <polygon points="5 3 19 12 5 21 5 3"/>
              </svg>
              {isRunningSample ? runLoaderMessage : "รันโค้ดตัวอย่าง"}
            </button>
            
            <button 
              use:particleButton
              class="btn-submit" 
              onclick={submitSolution} 
              disabled={isRunningSample || isSubmitting}
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" style="width: 12px; height: 12px;">
                <line x1="22" y1="2" x2="11" y2="13"/>
                <polygon points="22 2 15 22 11 13 2 9 22 2"/>
              </svg>
              {isSubmitting ? submitLoaderMessage : "ส่งคำตอบ"}
            </button>
          </div>
        </div>

        <!-- Judging Output Panel -->
        <div class="judge-panel" use:smoothScroll>
          <div class="rp-label">ผลการทดสอบ</div>

          {#if errorMsg}
            <div class="prob-code text-error" style="border-left: 3px solid var(--accent-error); color: var(--accent-error); border-color: var(--accent-error-border); background: var(--accent-error-bg); font-family: var(--font-mono); font-size: 12px; white-space: pre-wrap; padding: 12px 14px; border-radius: var(--radius-sm);">
              <div style="font-weight: 600; font-family: var(--font-sans); margin-bottom: 6px; font-size: 11px; text-transform: uppercase; letter-spacing: 0.04em; color: var(--accent-error);">พบข้อผิดพลาดในการประมวลผล (Execution Error)</div>
              {errorMsg}
            </div>
          {/if}

          <!-- Sample Verdict (Run Sample results) -->
          {#if sampleResults}
            <div style="display: flex; flex-direction: column; gap: 8px;">
              {#each sampleResults as res, index}
                <div class="test-row">
                  <span class="test-badge {getVerdictClass(res.verdict)}">
                    {#if res.verdict === "Passed" || res.verdict === "Accepted"}ผ่าน{:else}ไม่ผ่าน ({res.verdict}){/if}
                  </span>
                  <span class="test-name">ตัวอย่างที่ #{index + 1} ({res.duration_ms} มส.)</span>
                </div>

                {#if res.error}
                  <pre class="prob-code" style="border-left: 3px solid var(--accent-error); color: var(--accent-error); border-color: var(--accent-error-border); background: var(--accent-error-bg); padding: 12px 14px;"><div style="font-weight: 600; font-family: var(--font-sans); margin-bottom: 4px; font-size: 10px; text-transform: uppercase; color: var(--accent-error);">รายละเอียดข้อผิดพลาด (Traceback / Runtime Error)</div>{res.error}</pre>
                {:else if res.verdict !== "Passed" && res.verdict !== "Accepted"}
                  <div class="diff-box">
                    <div class="diff-col">
                      <span class="diff-label">ผลลัพธ์ที่คาดหวัง</span>
                      <span class="exp">{res.expected}</span>
                    </div>
                    <div class="diff-col">
                      <span class="diff-label">ผลลัพธ์ที่ได้</span>
                      <span class="got">{res.actual}</span>
                    </div>
                  </div>
                {/if}
              {/each}
            </div>
          {/if}

          <!-- Submission Verdict (Submit Solution results) -->
          {#if submissionResult}
            <div style="display: flex; flex-direction: column; gap: 8px;">
              <!-- Summary bar -->
              <div class="test-row" style="padding: 4px 0;">
                <span class="test-badge {getVerdictClass(submissionResult.verdict)}" style="font-size: 12px; padding: 4px 10px;">
                  {#if submissionResult.verdict === "Passed" || submissionResult.verdict === "Accepted"}ผ่าน{:else}ไม่ผ่าน ({submissionResult.verdict}){/if}
                </span>
                <span style="font-size: 13px; font-weight: 500;">
                  ผ่าน {submissionResult.passed_count} / {submissionResult.total_count} กรณีทดสอบ
                </span>
              </div>

              <!-- Score bar -->
              <div class="score-row" style="align-items: center;">
                <span class="score-num" style="flex-shrink: 0; min-width: 90px;">{submissionResult.score} / 100 คะแนน</span>
                <div style="flex: 1;">
                  <ProgressBar value={submissionResult.score} max={100} />
                </div>
              </div>

              <!-- Details list -->
              <div style="display: flex; flex-direction: column; gap: 4px; margin-top: 4px;">
                {#each submissionResult.details as tc_verdict, index}
                  <div 
                    style="
                      display: flex; 
                      justify-content: space-between; 
                      align-items: center; 
                      padding: 7px 10px; 
                      background: var(--bg-card); 
                      border: 1px solid var(--border-muted); 
                      border-radius: var(--radius-xs);
                      border-left: 3px solid {tc_verdict === 'Passed' ? 'var(--accent-success)' : 'var(--accent-error)'};
                    "
                  >
                    <span style="font-size: 11px; color: var(--text-secondary);">กรณีทดสอบที่ #{index + 1}</span>
                    <span style="font-size: 11px; font-weight: 500; color: {tc_verdict === 'Passed' ? 'var(--accent-success)' : 'var(--accent-error)'};">
                      {#if tc_verdict === 'Passed'}ผ่าน{:else}ไม่ผ่าน ({tc_verdict}){/if}
                    </span>
                  </div>
                {/each}
              </div>
            </div>
          {/if}
        </div>
      </div>
    </div>
  </div>
{:else}
  <div style="padding: 32px; text-align: center; color: var(--text-muted);">
    กำลังจัดเตรียมพื้นที่ทำงาน...
  </div>
{/if}

<Confetti bind:trigger={triggerScreenConfetti} isFullScreen={true} />

<style>
  .challenge-loader {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 16px;
    height: 100%;
    min-height: 300px;
    animation: cl-fade-in 0.3s ease;
  }

  @keyframes cl-fade-in {
    from { opacity: 0; transform: translateY(8px); }
    to   { opacity: 1; transform: translateY(0); }
  }

  .cl-spinner {
    position: relative;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .cl-ring {
    position: absolute;
    width: 100%;
    height: 100%;
    border: 1.5px solid var(--border, #333);
    border-radius: 50%;
  }

  .cl-core {
    position: absolute;
    width: 100%;
    height: 100%;
    border: 1.5px solid transparent;
    border-top-color: var(--accent-blue, #5b9bd5);
    border-radius: 50%;
    animation: cl-spin 0.9s cubic-bezier(0.4, 0, 0.2, 1) infinite;
  }

  @keyframes cl-spin {
    from { transform: rotate(0deg); }
    to   { transform: rotate(360deg); }
  }

  .cl-msg {
    font-size: 12px;
    color: var(--text-muted, #555);
    letter-spacing: 0.01em;
    animation: cl-blink 2.5s ease-in-out infinite;
  }

  @keyframes cl-blink {
    0%, 100% { opacity: 0.6; }
    50%       { opacity: 1; }
  }
</style>
