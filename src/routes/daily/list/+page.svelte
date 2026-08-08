<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke as tauriInvoke } from "@tauri-apps/api/core";
  import { appState } from "$lib/state.svelte";
  import ProgressBar from "$lib/components/ProgressBar.svelte";

  type Progress = { completed: number; required: number };
  type Problem = { id: string; title: string; category: string };
  type Challenge = {
    problems: Problem[];
    category_progress: Record<string, Progress>;
    solved_problem_ids: string[];
  };

  const categoryTranslations: Record<string, string> = {
    "Input / Output": "ข้อมูลเข้า / ข้อมูลออก (Input / Output)",
    Conditions: "เงื่อนไข (Conditions)",
    Loops: "การวนซ้ำ (Loops)",
    Functions: "ฟังก์ชัน (Functions)",
    Lists: "ลิสต์ / อาร์เรย์ (Lists)",
    Implementation: "การใช้งานจริง (Implementation)",
    Math: "คณิตศาสตร์ (Math)",
    String: "การจัดการข้อความ (String)",
    Sorting: "การเรียงลำดับ (Sorting)",
    Searching: "การค้นหาข้อมูล (Searching)",
    Greedy: "ขั้นตอนวิธีแบบโลภ (Greedy)",
    Recursion: "การเรียกซ้ำ (Recursion)",
    "Data Structures": "โครงสร้างข้อมูล (Data Structures)",
    Graph: "กราฟ (Graph)",
    "Dynamic Programming": "การโปรแกรมพลวัต (Dynamic Programming)",
    "Matrix Operations": "การดำเนินการเมทริกซ์ (Matrix Operations)",
    "Numerical Methods": "ระเบียบวิธีเชิงตัวเลข (Numerical Methods)",
    Simulation: "การจำลองสถานการณ์ (Simulation)",
    Optimization: "การเพิ่มประสิทธิภาพ (Optimization)",
    "Data Parsing": "การแจกแจงข้อมูล (Data Parsing)",
    Statistics: "สถิติ (Statistics)",
    "Signal Processing": "การประมวลผลสัญญาณ (Signal Processing)",
    "Teacher Problems": "โจทย์อาจารย์ (Teacher Problems)",
  };

  const teacherTags = [
    "เงื่อนไข",
    "การวนซ้ำ",
    "ฟังก์ชัน",
    "ลิสต์",
    "คณิตศาสตร์",
    "String",
    "Sorting",
    "Searching",
    "Graph",
    "DP",
    "Recursion",
    "Greedy",
    "Data Structures",
    "Simulation",
    "Statistics",
  ];
  const PREVIEW_COUNT = 8;

  async function invoke(cmd: string, args: Record<string, unknown> = {}) {
    try {
      if (typeof window !== "undefined" && (window as any).__TAURI_INTERNALS__) {
        return await tauriInvoke(cmd, args);
      }
    } catch (error) {
      console.warn(`Tauri invoke failed for ${cmd}`, error);
    }

    if (cmd === "get_daily_challenge") {
      return {
        problems: [
          {
            id: "circle_area",
            title: "หาพื้นที่วงกลม",
            category: "Input / Output",
          },
          {
            id: "even_odd",
            title: "เลขคู่หรือเลขคี่",
            category: "Conditions",
          },
        ],
        category_progress: {
          "Input / Output": { completed: 1, required: 1 },
          Conditions: { completed: 0, required: 1 },
        },
        solved_problem_ids: ["circle_area"],
      } satisfies Challenge;
    }

    if (cmd === "get_problem_catalog") {
      return [];
    }

    if (cmd === "get_submissions") {
      return [];
    }

    return null;
  }

  let problems = $state<Problem[]>(appState.dailyChallenge?.problems ?? []);
  let problemCatalog = $state<Problem[]>([]);
  let categoryProgress = $state<Record<string, Progress>>(
    appState.dailyChallenge?.category_progress ?? {},
  );
  let solvedProblemIds = $state<string[]>(appState.dailyChallenge?.solved_problem_ids ?? []);
  let loading = $state(appState.dailyChallenge === null);
  let errorMsg = $state<string | null>(null);
  let teacherExpanded = $state(false);
  let loadedKey = $state("");

  let activeUser = $derived(appState.currentUser);
  let totalCompleted = $derived(
    Object.values(categoryProgress).reduce((total, progress) => total + progress.completed, 0),
  );
  let totalRequired = $derived(
    Object.values(categoryProgress).reduce((total, progress) => total + progress.required, 0),
  );
  let solvedSet = $derived(new Set(solvedProblemIds));
  let teacherProblems = $derived(
    problemCatalog.filter((problem) => problem.category === "Teacher Problems"),
  );
  let teacherSolvedCount = $derived(
    teacherProblems.filter((problem) => solvedSet.has(problem.id)).length,
  );
  let sortedProblems = $derived(
    [...problems].sort((a, b) => {
      const solvedA = solvedSet.has(a.id);
      const solvedB = solvedSet.has(b.id);
      if (solvedA !== solvedB) return solvedA ? 1 : -1;

      const progressA = categoryProgress[a.category];
      const progressB = categoryProgress[b.category];
      const categoryDoneA = progressA ? progressA.completed >= progressA.required : false;
      const categoryDoneB = progressB ? progressB.completed >= progressB.required : false;
      if (categoryDoneA !== categoryDoneB) return categoryDoneA ? 1 : -1;
      return 0;
    }),
  );
  let unsolvedProblems = $derived(sortedProblems.filter((problem) => !solvedSet.has(problem.id)));
  let solvedProblems = $derived(sortedProblems.filter((problem) => solvedSet.has(problem.id)));

  $effect(() => {
    const user = activeUser;
    const refresh = appState.needsRefresh;
    if (!user) return;

    const key = `${user}:${refresh}`;
    if (key === loadedKey) return;
    loadedKey = key;
    loadDailyChallenge();
  });

  async function loadDailyChallenge() {
    loading = true;
    errorMsg = null;

    try {
      await appState.prefetchDailyChallenge();

      const [catalogResult, submissionsResult] = await Promise.allSettled([
        invoke("get_problem_catalog"),
        invoke("get_submissions", { userId: appState.currentUser }),
      ]);
      const challenge = appState.dailyChallenge as Challenge | null;

      if (!challenge) {
        throw new Error("Daily challenge unavailable");
      }

      problems = challenge.problems ?? [];
      categoryProgress = challenge.category_progress ?? {};

      const catalog =
        catalogResult.status === "fulfilled" && Array.isArray(catalogResult.value)
          ? catalogResult.value
          : [];
      problemCatalog = catalog.length > 0 ? catalog : problems;

      const submissions =
        submissionsResult.status === "fulfilled" && Array.isArray(submissionsResult.value)
          ? submissionsResult.value
          : [];
      const acceptedProblemIds = submissions
        .filter((submission: any) => Number(submission.score) >= 100)
        .map((submission: any) => submission.problem_id)
        .filter(Boolean);

      solvedProblemIds = [
        ...new Set([...(challenge.solved_problem_ids ?? []), ...acceptedProblemIds]),
      ];
    } catch (error) {
      console.error("Failed to load problem list:", error);
      errorMsg = "โปรดซิงค์ข้อมูลกับ Supabase ในหน้าตั้งค่าเพื่อเริ่มต้นเป้าหมายประจำวัน";
    } finally {
      loading = false;
    }
  }

  function startRandomPractice() {
    const unsolved = problems.filter((problem) => !solvedSet.has(problem.id));
    const pool = unsolved.length > 0 ? unsolved : problems;
    const randomProblem = pool[Math.floor(Math.random() * pool.length)];
    if (randomProblem) openProblem(randomProblem.id);
  }

  function openProblem(problemId: string) {
    goto(`/daily?problem=${encodeURIComponent(problemId)}`);
  }

  function problemCategory(problem: Problem) {
    return categoryTranslations[problem.category] || problem.category;
  }

  function teacherTag(index: number) {
    return teacherTags[index % teacherTags.length];
  }
</script>

{#if loading}
  <div class="list-empty-state">
    <div class="list-loading-mark" aria-hidden="true"></div>
    <span>กำลังโหลดรายการโจทย์...</span>
  </div>
{:else if errorMsg}
  <div class="list-empty-state">
    <svg width="42" height="42" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" aria-hidden="true">
      <path d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
    </svg>
    <span>{errorMsg}</span>
    <a class="btn-run" href="/settings">ไปที่หน้าตั้งค่า</a>
  </div>
{:else if problems.length === 0}
  <div class="list-empty-state">
    <svg width="42" height="42" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" aria-hidden="true">
      <path d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
    </svg>
    <span>ไม่มีโจทย์อยู่ในแคชท้องถิ่น</span>
    <p>โปรดไปที่หน้า “ตั้งค่า” และกดซิงค์ฐานข้อมูลเพื่อดึงโจทย์ลงมา</p>
    <a class="btn-run" href="/settings">ไปที่หน้าตั้งค่า</a>
  </div>
{:else}
  <div class="screen list-screen">
    <header class="page-header">
      <div>
        <h1 class="page-title">ความท้าทายประจำวัน</h1>
        <p class="page-subtitle">ทำโจทย์ให้ครบทุกหมวดหมู่เพื่อจบเซสชันของวันนี้</p>
      </div>
      <button class="btn-submit list-action" type="button" onclick={startRandomPractice}>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" aria-hidden="true">
          <polygon points="5 3 19 12 5 21 5 3" />
        </svg>
        <span>เริ่มฝึกฝนทันที (สุ่มโจทย์)</span>
      </button>
    </header>

    <section class="teacher-featured" class:expanded={teacherExpanded} aria-labelledby="teacher-title">
      <div class="featured-head">
        <div class="featured-headline">
          <div class="featured-icon" aria-hidden="true">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M22 10 12 5 2 10l10 5 10-5Z" />
              <path d="M6 12v5c0 1.5 2.7 3 6 3s6-1.5 6-3v-5" />
            </svg>
          </div>
          <div>
            <h2 id="teacher-title" class="featured-title">โจทย์อาจารย์</h2>
            <p class="featured-subtitle">คลังโจทย์ทั้งหมดที่อาจารย์กำหนด</p>
          </div>
        </div>
        <span class="featured-count">{teacherSolvedCount} / {teacherProblems.length} ข้อ</span>
      </div>

      {#if teacherProblems.length > 0}
        <div class="featured-grid">
          {#each teacherProblems as problem, index (problem.id)}
            <button
              class="teacher-card"
              class:solved={solvedSet.has(problem.id)}
              class:extra={index >= PREVIEW_COUNT}
              type="button"
              onclick={() => openProblem(problem.id)}
              aria-label={`เปิดโจทย์อาจารย์ ${problem.title}`}
            >
              <span class="teacher-num">#{String(index + 1).padStart(2, "0")}</span>
              <span class="teacher-title">{problem.title}</span>
              <span class="teacher-tag">{teacherTag(index)}</span>
              {#if solvedSet.has(problem.id)}
                <span class="teacher-check">✓ แก้ไขแล้ว</span>
              {/if}
            </button>
          {/each}
        </div>
        <button class="featured-toggle" type="button" onclick={() => (teacherExpanded = !teacherExpanded)} aria-expanded={teacherExpanded}>
          <span>{teacherExpanded ? "ย่อรายการ" : `ดูโจทย์อาจารย์ทั้งหมด (${teacherProblems.length} ข้อ)`}</span>
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" aria-hidden="true">
            <polyline points="6 9 12 15 18 9" />
          </svg>
        </button>
      {:else}
        <p class="catalog-empty">ยังไม่มีโจทย์อาจารย์ใน catalog โปรดซิงค์ข้อมูลจากหน้าตั้งค่า</p>
      {/if}
    </section>

    <div class="progress-line">
      <div class="progress-bar-wrap">
        <div class="progress-bar-label">ความคืบหน้าวันนี้</div>
        <!-- Keep the existing animated ProgressBar effect. -->
        <ProgressBar value={totalCompleted} max={totalRequired} />
      </div>
      <div class="progress-text">เสร็จสิ้นแล้ว {totalCompleted} / {totalRequired} ข้อ</div>
    </div>

    <section class="daily-problems-section" aria-labelledby="daily-problems-title">
      <h2 id="daily-problems-title" class="section-label">ชุดโจทย์ปัญหาของวันนี้</h2>
      <div class="problem-groups">
        {#if unsolvedProblems.length > 0}
          <div class="problem-group">
            <div class="problem-group-head">
              <span class="problem-group-title">ยังไม่ทำ</span>
              <span class="problem-group-count">{unsolvedProblems.length} ข้อ</span>
            </div>
            <div class="problem-grid">
              {#each unsolvedProblems as problem (problem.id)}
                <button class="problem-card" type="button" onclick={() => openProblem(problem.id)} aria-label={`เปิดโจทย์ ${problem.title}`}>
                  <span class="problem-card-head">
                    <span class="problem-card-status"><span class="problem-status-dot"></span>ยังไม่ทำ</span>
                    <svg class="problem-card-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
                      <path d="m9 18 6-6-6-6" />
                    </svg>
                  </span>
                  <span class="problem-card-title">{problem.title}</span>
                  <span class="problem-card-footer">
                    <span class="problem-card-category">{problemCategory(problem)}</span>
                    <span class="problem-card-action">เปิดโจทย์ <span aria-hidden="true">→</span></span>
                  </span>
                </button>
              {/each}
            </div>
          </div>
        {/if}

        {#if solvedProblems.length > 0}
          <div class="problem-group problem-group-solved">
            <div class="problem-group-head">
              <span class="problem-group-title">ทำแล้ว</span>
              <span class="problem-group-count">{solvedProblems.length} ข้อ</span>
            </div>
            <div class="problem-grid">
              {#each solvedProblems as problem (problem.id)}
                <button class="problem-card problem-card-solved" type="button" onclick={() => openProblem(problem.id)} aria-label={`เปิดโจทย์ที่ทำแล้ว ${problem.title}`}>
                  <span class="problem-card-head">
                    <span class="problem-card-status"><span class="problem-status-dot problem-status-dot-solved">✓</span>แก้ไขแล้ว</span>
                    <svg class="problem-card-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
                      <path d="m9 18 6-6-6-6" />
                    </svg>
                  </span>
                  <span class="problem-card-title">{problem.title}</span>
                  <span class="problem-card-footer">
                    <span class="problem-card-category">{problemCategory(problem)}</span>
                    <span class="problem-card-action">เปิดโจทย์ <span aria-hidden="true">→</span></span>
                  </span>
                </button>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </section>
  </div>
{/if}

<style>
  .list-screen {
    gap: 26px;
  }

  .page-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 20px;
    flex-wrap: wrap;
  }

  .page-title {
    color: var(--text-primary);
    font-size: 18px;
    font-weight: 500;
    line-height: 1.4;
  }

  .page-subtitle {
    margin-top: 4px;
    color: var(--text-muted);
    font-size: 13px;
  }

  .list-action {
    padding: 8px 16px;
    border-radius: var(--radius-sm);
    font-size: 12px;
  }

  .list-action svg {
    width: 14px;
    height: 14px;
  }

  .teacher-featured {
    display: flex;
    flex-direction: column;
    gap: 14px;
    padding: 20px 22px;
    border: 1px solid var(--accent-blue);
    border-radius: var(--radius-lg);
    background: var(--bg-card);
  }

  .featured-head,
  .featured-headline {
    display: flex;
    align-items: center;
  }

  .featured-head {
    justify-content: space-between;
    gap: 12px;
    flex-wrap: wrap;
  }

  .featured-headline {
    gap: 10px;
  }

  .featured-icon {
    display: grid;
    place-items: center;
    width: 34px;
    height: 34px;
    flex: 0 0 34px;
    border: 1px solid var(--accent-blue-border);
    border-radius: 9px;
    background: var(--accent-blue-bg);
    color: var(--accent-blue);
  }

  .featured-icon svg {
    width: 18px;
    height: 18px;
  }

  .featured-title {
    color: var(--text-primary);
    font-size: 15px;
    font-weight: 600;
  }

  .featured-subtitle {
    margin-top: 1px;
    color: var(--text-muted);
    font-size: 12px;
  }

  .featured-count {
    padding: 5px 10px;
    border: 1px solid var(--accent-blue-border);
    border-radius: 999px;
    background: var(--accent-blue-bg);
    color: var(--accent-blue);
    font: 500 11px var(--font-mono);
    white-space: nowrap;
  }

  .featured-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 10px;
  }

  .teacher-card.extra {
    display: none;
  }

  .teacher-featured.expanded .teacher-card.extra {
    display: flex;
  }

  .teacher-card {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
    min-width: 0;
    padding: 13px 14px;
    border: 1px solid var(--accent-blue-border);
    border-radius: var(--radius-md);
    background: rgba(20, 20, 20, 0.5);
    color: inherit;
    text-align: left;
    cursor: pointer;
    transition: transform 0.15s, border-color 0.15s, background 0.15s;
  }

  .teacher-card:hover {
    transform: translateY(-2px);
    border-color: var(--accent-blue);
    background: #1c2733;
  }

  .teacher-card.solved {
    border-color: var(--accent-success-border);
  }

  .teacher-num {
    color: var(--accent-blue);
    font: 500 10px var(--font-mono);
  }

  .teacher-card.solved .teacher-num {
    color: var(--accent-success);
  }

  .teacher-title {
    color: var(--text-primary);
    font-size: 12.5px;
    font-weight: 500;
    line-height: 1.4;
  }

  .teacher-tag {
    color: var(--text-muted);
    font-size: 10px;
  }

  .teacher-check {
    color: var(--accent-success);
    font-size: 10.5px;
    font-weight: 500;
  }

  .featured-toggle {
    display: inline-flex;
    align-items: center;
    align-self: flex-start;
    gap: 6px;
    color: var(--accent-blue);
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
  }

  .featured-toggle svg {
    width: 13px;
    height: 13px;
    transition: transform 0.2s;
  }

  .teacher-featured.expanded .featured-toggle svg {
    transform: rotate(180deg);
  }

  .catalog-empty {
    color: var(--text-muted);
    font-size: 12px;
  }

  .progress-line {
    display: flex;
    align-items: flex-end;
    gap: 14px;
  }

  .progress-bar-wrap {
    display: flex;
    flex: 1;
    flex-direction: column;
    gap: 6px;
  }

  .progress-bar-label {
    color: var(--text-muted);
    font-size: 11px;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .daily-problems-section {
    display: flex;
    flex-direction: column;
  }

  .section-label {
    margin-bottom: 12px;
    color: var(--text-muted);
    font-size: 11px;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .problem-groups {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .problem-group {
    display: flex;
    flex-direction: column;
    gap: 9px;
  }

  .problem-group-solved {
    padding-top: 4px;
  }

  .problem-group-head {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 12px;
    padding: 0 2px;
  }

  .problem-group-title {
    color: var(--text-secondary);
    font-size: 12px;
    font-weight: 500;
  }

  .problem-group-count {
    color: var(--text-muted);
    font: 11px var(--font-mono);
  }

  .problem-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 10px;
  }

  .problem-card {
    position: relative;
    display: flex;
    min-height: 126px;
    flex-direction: column;
    align-items: flex-start;
    padding: 14px 16px 13px;
    overflow: hidden;
    border: 1px solid var(--border-muted);
    border-radius: 7px;
    background: var(--bg-card);
    color: inherit;
    text-align: left;
    cursor: pointer;
    transform: translateZ(0);
    transition:
      transform 180ms var(--ease-out-quart),
      background-color 180ms var(--ease-out-quart),
      border-color 180ms var(--ease-out-quart),
      color 180ms var(--ease-out-quart);
  }

  .problem-card:hover {
    z-index: 1;
    border-color: var(--accent-blue);
    background: #232b34;
    transform: translateY(-2px) scale(1.015);
  }

  .problem-card:active {
    transform: translateY(0) scale(0.985);
    transition-duration: 90ms;
  }

  .problem-card:focus-visible {
    z-index: 2;
    outline: 2px solid var(--accent-blue);
    outline-offset: 2px;
    border-color: var(--accent-blue);
  }

  .problem-card-solved {
    border-color: var(--accent-success-border);
    background: #1d241e;
  }

  .problem-card-solved:hover {
    border-color: var(--accent-success);
    background: #222d25;
  }

  .problem-card-head {
    display: flex;
    width: 100%;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
  }

  .problem-card-status {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    color: var(--text-muted);
    font-size: 10.5px;
  }

  .problem-card-solved .problem-card-status {
    color: var(--accent-success);
  }

  .problem-status-dot {
    display: inline-grid;
    place-items: center;
    width: 7px;
    height: 7px;
    flex: 0 0 7px;
    border: 1px solid var(--accent-blue);
    border-radius: 50%;
    color: transparent;
  }

  .problem-status-dot-solved {
    width: 14px;
    height: 14px;
    flex-basis: 14px;
    border-color: var(--accent-success);
    border-radius: 4px;
    color: var(--accent-success);
    font-size: 9px;
    font-weight: 700;
  }

  .problem-card-chevron {
    width: 14px;
    height: 14px;
    flex: 0 0 14px;
    color: var(--text-muted);
    transform: translateX(-2px);
    transition: color 180ms var(--ease-out-quart), transform 180ms var(--ease-out-quart);
  }

  .problem-card:hover .problem-card-chevron,
  .problem-card:focus-visible .problem-card-chevron {
    color: var(--accent-blue);
    transform: translateX(2px);
  }

  .problem-card-solved:hover .problem-card-chevron {
    color: var(--accent-success);
  }

  .problem-card-title {
    display: block;
    max-width: 46ch;
    margin-top: 18px;
    color: var(--text-primary);
    font-size: 13px;
    font-weight: 500;
    line-height: 1.45;
  }

  .problem-card-solved .problem-card-title {
    color: var(--text-secondary);
  }

  .problem-card-footer {
    display: flex;
    width: 100%;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    margin-top: auto;
    padding-top: 14px;
  }

  .problem-card-category {
    max-width: 70%;
    overflow: hidden;
    padding: 2px 7px;
    border: 1px solid var(--accent-blue-border);
    border-radius: 4px;
    background: var(--accent-blue-bg);
    color: var(--accent-blue);
    font-size: 10px;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .problem-card-solved .problem-card-category {
    border-color: var(--accent-success-border);
    background: var(--accent-success-bg);
    color: var(--accent-success);
  }

  .problem-card-action {
    flex: 0 0 auto;
    color: var(--text-muted);
    font-size: 10.5px;
    transition: color 180ms var(--ease-out-quart), transform 180ms var(--ease-out-quart);
  }

  .problem-card:hover .problem-card-action,
  .problem-card:focus-visible .problem-card-action {
    color: var(--accent-blue);
    transform: translateX(2px);
  }

  .problem-card-solved:hover .problem-card-action {
    color: var(--accent-success);
  }

  .list-empty-state {
    display: flex;
    min-height: 100%;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 14px;
    padding: 32px;
    color: var(--text-muted);
    text-align: center;
  }

  .list-empty-state p {
    max-width: 400px;
    font-size: 13px;
    line-height: 1.6;
  }

  .list-empty-state .btn-run {
    margin-top: 4px;
    padding: 8px 14px;
    text-decoration: none;
  }

  .list-loading-mark {
    width: 28px;
    height: 28px;
    border: 1.5px solid var(--border);
    border-top-color: var(--accent-blue);
    border-radius: 50%;
    animation: list-spin 0.9s linear infinite;
  }

  @keyframes list-spin {
    to { transform: rotate(360deg); }
  }

  @media (max-width: 760px) {
    .list-screen {
      gap: 22px;
      padding: 24px 20px;
    }

    .progress-line {
      align-items: flex-start;
      flex-direction: column;
      gap: 8px;
    }

    .progress-bar-wrap {
      width: 100%;
    }

    .problem-grid {
      grid-template-columns: 1fr;
    }
  }

  @media (max-width: 440px) {
    .page-header {
      display: block;
    }

    .list-action {
      margin-top: 16px;
    }
  }

</style>
