<script lang="ts">
  import { onMount } from "svelte";
  import { invoke as tauriInvoke } from "@tauri-apps/api/core";
  import Confetti from "$lib/components/Confetti.svelte";
  import ProgressBar from "$lib/components/ProgressBar.svelte";
  import { appState } from "$lib/state.svelte";
  import { goto } from "$app/navigation";
  import { particleButton } from "$lib/actions/particleButton";


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
    }
    return null;
  }

  // Daily Challenge data — initialize from cache immediately to avoid loading flash
  let problems = $state<any[]>(appState.dailyChallenge?.problems ?? []);
  let categoryProgress = $state<Record<string, { completed: number; required: number }>>(
    appState.dailyChallenge?.category_progress ?? {}
  );
  let solvedProblemIds = $state<string[]>(appState.dailyChallenge?.solved_problem_ids ?? []);
  let loading = $state(appState.dailyChallenge === null);

  // Confetti trigger binding
  let triggerScreenConfetti = $state<(x?: number, y?: number) => void>(() => {});

  let challengeLoaderMessage = $state("กำลังโหลดเป้าหมายการฝึกฝนของวันนี้...");
  const challengeMessages = [
    "กำลังวิเคราะห์โครงสร้างข้อมูลท้าทายประจำวัน...",
    "กำลังประมวลผลชุดโจทย์วิเคราะห์ภาษา Python...",
    "กำลังโหลดคะแนนสถิติประจำวันและความคืบหน้า..."
  ];

  function selectRandomLoaderMessage() {
    challengeLoaderMessage = challengeMessages[Math.floor(Math.random() * challengeMessages.length)];
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

  let errorMsg = $state<string | null>(null);

  // Summary counts
  let totalCompleted = $derived(
    Object.values(categoryProgress).reduce((acc, curr) => acc + curr.completed, 0)
  );
  let totalRequired = $derived(
    Object.values(categoryProgress).reduce((acc, curr) => acc + curr.required, 0)
  );

  // Derived sorted checklist: uncompleted categories first, completed categories last
  let sortedChecklist = $derived(
    Object.entries(categoryProgress).sort((a, b) => {
      const isDoneA = a[1].completed >= a[1].required;
      const isDoneB = b[1].completed >= b[1].required;
      if (isDoneA && !isDoneB) return 1;
      if (!isDoneA && isDoneB) return -1;
      return 0; // maintain original order
    })
  );

  // Derived sorted problems:
  // 1. Unsolved problems from uncompleted categories
  // 2. Solved problems from uncompleted categories
  // 3. Unsolved problems from completed categories
  // 4. Solved problems from completed categories
  let sortedProblems = $derived(
    [...problems].sort((a, b) => {
      const isSolvedA = solvedProblemIds.includes(a.id);
      const isSolvedB = solvedProblemIds.includes(b.id);
      
      const progA = categoryProgress[a.category];
      const progB = categoryProgress[b.category];
      const isCatDoneA = progA ? progA.completed >= progA.required : false;
      const isCatDoneB = progB ? progB.completed >= progB.required : false;

      // 1. Sort by category completion status (uncompleted categories go first)
      if (isCatDoneA && !isCatDoneB) return 1;
      if (!isCatDoneA && isCatDoneB) return -1;

      // 2. Sort by problem solved status (unsolved problems go first)
      if (isSolvedA && !isSolvedB) return 1;
      if (!isSolvedA && isSolvedB) return -1;

      return 0;
    })
  );

  let activeUser = $derived(appState.currentUser);

  $effect(() => {
    if (activeUser) {
      const _ = appState.needsRefresh;
      // Use cache if already loaded, otherwise fetch
      if (appState.dailyChallenge) {
        problems = appState.dailyChallenge.problems;
        categoryProgress = appState.dailyChallenge.category_progress;
        solvedProblemIds = appState.dailyChallenge.solved_problem_ids || [];
        loading = false;
      } else {
        loadDailyChallenge();
      }
    }
  });

  async function loadDailyChallenge() {
    loading = true;
    selectRandomLoaderMessage();
    try {
      await appState.prefetchDailyChallenge();
      if (appState.dailyChallenge) {
        problems = appState.dailyChallenge.problems;
        categoryProgress = appState.dailyChallenge.category_progress;
        solvedProblemIds = appState.dailyChallenge.solved_problem_ids || [];
      }
    } catch (e) {
      console.error("Failed to load daily challenge:", e);
      errorMsg = "โปรดซิงค์ข้อมูลกับ Supabase ในหน้าตั้งค่าเพื่อเริ่มต้นเป้าหมายประจำวัน";
    } finally {
      loading = false;
    }
  }

  function startRandomPractice() {
    // Find all unsolved problems
    const unsolved = problems.filter(p => !solvedProblemIds.includes(p.id));
    if (unsolved.length > 0) {
      const randomProb = unsolved[Math.floor(Math.random() * unsolved.length)];
      goto(`/daily?problem=${randomProb.id}`);
      return;
    }
    
    // Fallback: select a random problem overall
    if (problems.length > 0) {
      const randomProb = problems[Math.floor(Math.random() * problems.length)];
      goto(`/daily?problem=${randomProb.id}`);
    }
  }

  function openProblem(probId: string) {
    goto(`/daily?problem=${probId}`);
  }
</script>

{#if loading}
  <div style="padding: 32px; text-align: center; color: var(--text-muted);">
    {challengeLoaderMessage}
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
{:else}
  <div class="screen checklist-fade-in">
    <header class="pg-header" style="display: flex; flex-direction: row; justify-content: space-between; align-items: flex-start; gap: 20px; flex-wrap: wrap;">
      <div>
        <h1 class="pg-title">ความท้าทายประจำวัน</h1>
        <p class="pg-sub">ทำโจทย์ให้ครบทุกหมวดหมู่เพื่อจบเซสชันของวันนี้</p>
      </div>
      <button 
        use:particleButton
        class="btn-submit" 
        style="padding: 8px 16px; border-radius: var(--radius-sm); font-size: 12px; display: flex; align-items: center; gap: 8px;"
        onclick={startRandomPractice}
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" style="width: 14px; height: 14px;">
          <polygon points="5 3 19 12 5 21 5 3"/>
        </svg>
        <span>เริ่มฝึกฝนทันที (สุ่มโจทย์)</span>
      </button>
    </header>

    <div>
      <div style="display: flex; align-items: center; gap: 14px; margin-bottom: 20px;">
        <div style="flex: 1;">
          <ProgressBar value={totalCompleted} max={totalRequired} />
        </div>
        <div class="progress-text">เสร็จสิ้นแล้ว {totalCompleted} / {totalRequired} ข้อ</div>
      </div>

      {#if totalCompleted === totalRequired && totalRequired > 0}
        <div class="card checklist-fade-in" style="background: var(--accent-success-bg); border: 1px solid var(--accent-success-border); display: flex; flex-direction: column; gap: 8px; margin-bottom: 20px; position: relative; overflow: hidden; border-radius: var(--radius-md);">
          <div style="display: flex; align-items: center; gap: 8px;">
            <div class="status-dot online" style="width: 6px; height: 6px;"></div>
            <span style="font-size: 13px; font-weight: 600; color: var(--accent-success);">ภารกิจการฝึกฝนของวันนี้สำเร็จแล้ว!</span>
          </div>
          <p style="font-size: 12px; color: var(--text-secondary); line-height: 1.6; max-width: 65ch; margin: 0;">
            คุณได้พิชิตโจทย์ภาษา Python ครบตามโควตาในวันนี้ ขอปรบมือให้ในวินัยและความสม่ำเสมอในการพัฒนาตนเอง!
          </p>
          <button 
            class="btn-run" 
            style="width: fit-content; padding: 4px 10px; margin-top: 4px; border-color: var(--accent-success-border); background: var(--bg-card); color: var(--accent-success); font-size: 11px;" 
            onclick={() => triggerScreenConfetti()}
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" style="width: 10px; height: 10px; color: inherit;">
              <path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"/>
            </svg>
            <span>ฉลองความสำเร็จ</span>
          </button>
        </div>
      {/if}

      <div class="checklist">
        {#each sortedChecklist as [cat, progress]}
          {@const isDone = progress.completed >= progress.required}
          <div class="check-row" class:done={isDone}>
            <div class="check-icon" class:checked={isDone}>
              {#if isDone}
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                  <polyline points="20 6 9 17 4 12"/>
                </svg>
              {/if}
            </div>
            <div class="check-label">{categoryTranslations[cat] || cat}</div>
            <div 
              class="check-count" 
              class:partial={progress.completed > 0 && !isDone}
              class:none={progress.completed === 0 && !isDone}
            >
              {progress.completed} / {progress.required}
            </div>
          </div>
        {/each}
      </div>
    </div>

    <!-- Problem Browser Grid -->
    <div>
      <div style="font-size: 11px; color: var(--text-muted); margin-bottom: 12px; letter-spacing: 0.04em; text-transform: uppercase;">
        ชุดโจทย์ปัญหาของวันนี้
      </div>
      <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(220px, 1fr)); gap: 10px;">
        {#each sortedProblems as prob}
          {@const isSolved = solvedProblemIds.includes(prob.id)}
          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div 
            class="card" 
            style="cursor: pointer; display: flex; flex-direction: column; gap: 6px; border-color: {isSolved ? 'var(--accent-success-border)' : 'var(--bg-surface-raised)'}; transition: border-color 0.15s;"
            onclick={() => openProblem(prob.id)}
          >
            <div style="display: flex; justify-content: space-between; align-items: start; gap: 8px;">
              <span style="font-size: 13px; font-weight: 500; color: {isSolved ? 'var(--accent-success)' : 'var(--text-primary)'};">
                {prob.title}
              </span>
              {#if isSolved}
                <span style="font-size: 11px; color: var(--accent-success); font-weight: 500;">✓ แก้ไขแล้ว</span>
              {/if}
            </div>
            <span style="font-size: 11px; color: var(--text-muted);">{categoryTranslations[prob.category] || prob.category}</span>
          </div>
        {/each}
      </div>
    </div>
  </div>
{/if}

<Confetti bind:trigger={triggerScreenConfetti} isFullScreen={true} />
