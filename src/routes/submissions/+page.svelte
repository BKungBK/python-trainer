<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { appState } from "$lib/state.svelte";
  import { goto } from "$app/navigation";
  import { fade, scale } from "svelte/transition";
  import { smoothScroll } from "$lib/actions/smoothScroll";

  let submissions = $state<any[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);

  let searchQuery = $state("");
  let categoryFilter = $state("All");
  let verdictFilter = $state("All");

  let selectedSub = $state<any | null>(null);
  let isModalOpen = $state(false);

  let activeUser = $derived(appState.currentUser);

  onMount(async () => {
    await fetchSubmissions();
  });

  // Fetch when activeUser changes reactively
  $effect(() => {
    if (activeUser) {
      fetchSubmissions();
    }
  });

  async function fetchSubmissions() {
    if (!activeUser) return;
    loading = true;
    error = null;
    try {
      const res: any[] = await invoke("get_submissions", { userId: activeUser });
      submissions = res || [];
    } catch (e) {
      console.error(e);
      error = String(e);
    } finally {
      loading = false;
    }
  }

  // Get unique categories for filters
  let categories = $derived([
    "All",
    ...new Set(submissions.map((s) => s.problem_category).filter(Boolean))
  ]);

  // Filtered submissions
  let filteredSubmissions = $derived(
    submissions.filter((s) => {
      const matchesSearch = s.problem_title?.toLowerCase().includes(searchQuery.toLowerCase()) || 
                            s.problem_id?.toLowerCase().includes(searchQuery.toLowerCase());
      const matchesCategory = categoryFilter === "All" || s.problem_category === categoryFilter;
      const matchesVerdict = verdictFilter === "All" || 
                            (verdictFilter === "Passed" && (s.verdict === "Passed" || s.verdict === "Accepted")) ||
                            (verdictFilter === "Failed" && s.verdict !== "Passed" && s.verdict !== "Accepted");
      return matchesSearch && matchesCategory && matchesVerdict;
    })
  );

  function openSubDetails(sub: any) {
    selectedSub = sub;
    isModalOpen = true;
  }

  function closeModal() {
    isModalOpen = false;
    selectedSub = null;
  }

  function restoreCode(sub: any) {
    if (!activeUser) return;
    const key = `code_save_${activeUser}_${sub.problem_id}`;
    localStorage.setItem(key, sub.code);
    closeModal();
    goto(`/daily?problem=${sub.problem_id}`);
  }

  function formatTime(isoString: string) {
    try {
      const date = new Date(isoString);
      return date.toLocaleString("th-TH", {
        year: "numeric",
        month: "short",
        day: "numeric",
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit"
      });
    } catch (e) {
      return isoString;
    }
  }

  function getVerdictClass(verdict: string) {
    if (verdict === "Passed" || verdict === "Accepted") return "t-pass";
    return "t-fail";
  }

  // Category translation map (matching daily page)
  const categoryTranslations: Record<string, string> = {
    "Input / Output": "ข้อมูลเข้า / ข้อมูลออก",
    "Conditions": "เงื่อนไข",
    "Loops": "การวนซ้ำ",
    "Functions": "ฟังก์ชัน",
    "Lists": "ลิสต์ / อาร์เรย์",
    "Implementation": "การใช้งานจริง",
    "Math": "คณิตศาสตร์",
    "String": "การจัดการข้อความ",
    "Sorting": "การเรียงลำดับ",
    "Searching": "การค้นหาข้อมูล",
    "Greedy": "ขั้นตอนวิธีแบบโลภ",
    "Recursion": "การเรียกซ้ำ",
    "Data Structures": "โครงสร้างข้อมูล",
    "Graph": "กราฟ",
    "Dynamic Programming": "การโปรแกรมพลวัต",
    "Matrix Operations": "การดำเนินการเมทริกซ์",
    "Numerical Methods": "ระเบียบวิธีเชิงตัวเลข",
    "Simulation": "การจำลองสถานการณ์",
    "Optimization": "การเพิ่มประสิทธิภาพ",
    "Data Parsing": "การแจกแจงข้อมูล",
    "Statistics": "สถิติ",
    "Signal Processing": "การประมวลผลสัญญาณ"
  };
</script>

<div class="screen" style="max-width: 1000px; margin: 0 auto; width: 100%; height: 100%; display: flex; flex-direction: column;" use:smoothScroll>
  <header class="pg-header">
    <h1 class="pg-title">ประวัติการส่งคำตอบ</h1>
    <p class="pg-sub">ประวัติผลคะแนนและชุดคำสั่ง Python ทั้งหมดของคุณ ({activeUser})</p>
  </header>

  <!-- Filter & Search Panel -->
  <div class="card" style="margin-bottom: 20px; display: flex; gap: 12px; flex-wrap: wrap; align-items: center; padding: 12px 16px;">
    <div style="flex: 1; min-width: 200px; display: flex; gap: 8px;">
      <input 
        type="text" 
        placeholder="ค้นหาชื่อโจทย์..." 
        bind:value={searchQuery}
        style="width: 100%; font-size: 13px;"
      />
    </div>
    
    <div style="display: flex; gap: 8px;">
      <select bind:value={categoryFilter} style="font-size: 13px; background: var(--bg-surface-raised); border: 1px solid var(--border); border-radius: var(--radius-sm); color: var(--text-primary); padding: 8px 12px;">
        <option value="All">ทุกหมวดหมู่</option>
        {#each categories.filter(c => c !== "All") as cat}
          <option value={cat}>{categoryTranslations[cat] || cat}</option>
        {/each}
      </select>

      <select bind:value={verdictFilter} style="font-size: 13px; background: var(--bg-surface-raised); border: 1px solid var(--border); border-radius: var(--radius-sm); color: var(--text-primary); padding: 8px 12px;">
        <option value="All">ทุกผลลัพธ์</option>
        <option value="Passed">ผ่าน (Passed/Accepted)</option>
        <option value="Failed">ไม่ผ่าน (Failed/WA/TLE)</option>
      </select>

      <button class="btn-run" onclick={fetchSubmissions} style="padding: 8px 14px;" aria-label="รีเฟรชประวัติการส่ง">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width: 12px; height: 12px;">
          <path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.57-8.38l5.67-5.67"/>
        </svg>
      </button>
    </div>
  </div>

  <!-- Submissions Table Area -->
  <div style="flex: 1; min-height: 0; overflow-y: auto;">
    {#if loading}
      <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 12px; padding: 60px 0;">
        <div class="spinner-ring" style="width: 24px; height: 24px; border-top-color: var(--accent-blue);"></div>
        <span style="font-size: 12px; color: var(--text-muted);">กำลังดึงประวัติการส่งโค้ด...</span>
      </div>
    {:else if error}
      <div class="prob-code text-error" style="margin: 20px 0; padding: 16px; border-left: 3px solid var(--accent-error); border-color: var(--accent-error-border); background: var(--accent-error-bg);">
        เกิดข้อผิดพลาด: {error}
      </div>
    {:else if filteredSubmissions.length === 0}
      <div class="card" style="text-align: center; padding: 48px; color: var(--text-muted); font-size: 13px;">
        ไม่พบประวัติการส่งคำตอบที่ตรงตามเงื่อนไข
      </div>
    {:else}
      <div class="card" style="padding: 0; overflow: hidden; border: 1px solid var(--border);">
        <table style="width: 100%; border-collapse: collapse; text-align: left; font-size: 13px;">
          <thead>
            <tr style="background: var(--bg-base); border-bottom: 1px solid var(--border); color: var(--text-secondary);">
              <th style="padding: 12px 16px;">วันเวลาที่ส่ง</th>
              <th style="padding: 12px 16px;">โจทย์</th>
              <th style="padding: 12px 16px;">หมวดหมู่</th>
              <th style="padding: 12px 16px; text-align: center;">คะแนน</th>
              <th style="padding: 12px 16px;">ผลลัพธ์</th>
            </tr>
          </thead>
          <tbody>
            {#each filteredSubmissions as sub}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
              <tr 
                onclick={() => openSubDetails(sub)}
                style="border-bottom: 1px solid var(--border); cursor: pointer; transition: background-color var(--duration-state) var(--ease-out-quart);"
                class="sub-row-hover"
              >
                <td style="padding: 12px 16px; color: var(--text-secondary); font-family: var(--font-mono); font-size: 12px;">
                  {formatTime(sub.submitted_at)}
                </td>
                <td style="padding: 12px 16px; font-weight: 500; color: var(--text-primary);">
                  {sub.problem_title || sub.problem_id}
                </td>
                <td style="padding: 12px 16px; color: var(--text-secondary);">
                  {categoryTranslations[sub.problem_category] || sub.problem_category || "-"}
                </td>
                <td style="padding: 12px 16px; text-align: center; font-weight: 600; color: {sub.score === 100 ? 'var(--accent-success)' : 'var(--accent-warning)'};">
                  {sub.score} / 100
                </td>
                <td style="padding: 12px 16px;">
                  <span class="test-badge {getVerdictClass(sub.verdict)}" style="padding: 2px 8px; font-size: 11px;">
                    {sub.verdict}
                  </span>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </div>
</div>

<!-- Submission Detail Modal -->
{#if isModalOpen && selectedSub}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-overlay" style="z-index: 10001;" onclick={closeModal} transition:fade={{ duration: 150 }}>
    <div 
      class="card modal-content" 
      onclick={(e) => e.stopPropagation()} 
      transition:scale={{ duration: 200, start: 0.96 }} 
      style="background: var(--bg-card); border: 1px solid var(--border); border-radius: var(--radius-md); width: 750px; max-width: 90vw; display: flex; flex-direction: column; max-height: 85vh;"
    >
      <header style="display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 16px; border-bottom: 1px solid var(--border); padding-bottom: 12px;">
        <div>
          <h2 style="font-weight: 500; font-size: 16px; color: var(--text-primary); margin-bottom: 4px;">
            {selectedSub.problem_title || selectedSub.problem_id}
          </h2>
          <p style="font-size: 12px; color: var(--text-muted);">
            ส่งเมื่อ: {formatTime(selectedSub.submitted_at)} &middot; หมวดหมู่: {categoryTranslations[selectedSub.problem_category] || selectedSub.problem_category}
          </p>
        </div>
        <button class="modal-close-btn" onclick={closeModal} style="position: static; padding: 6px;" aria-label="ปิด">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" style="width: 14px; height: 14px;">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </header>

      <div style="flex: 1; overflow-y: auto; display: flex; flex-direction: column; gap: 16px; min-height: 0; padding-right: 4px;">
        <!-- Verdict Summary Banner -->
        <div 
          style="
            display: flex; 
            align-items: center; 
            justify-content: space-between; 
            padding: 12px 16px; 
            background: var(--bg-base); 
            border: 1px solid var(--border); 
            border-radius: var(--radius-sm);
            border-left: 3px solid {selectedSub.score === 100 ? 'var(--accent-success)' : 'var(--accent-error)'};
          "
        >
          <div style="display: flex; align-items: center; gap: 12px;">
            <span class="test-badge {getVerdictClass(selectedSub.verdict)}" style="padding: 4px 10px; font-size: 12px;">
              {selectedSub.verdict}
            </span>
            <span style="font-size: 13px; color: var(--text-secondary);">
              ผ่าน {selectedSub.passed_count} / {selectedSub.total_count} กรณีทดสอบ
            </span>
          </div>
          <span style="font-size: 15px; font-weight: 600; color: {selectedSub.score === 100 ? 'var(--accent-success)' : 'var(--accent-warning)'};">
            คะแนน: {selectedSub.score} / 100
          </span>
        </div>

        <!-- Code Block -->
        <div style="display: flex; flex-direction: column; gap: 6px;">
          <span style="font-size: 12px; color: var(--text-secondary); font-weight: 500;">โค้ดที่คุณส่ง:</span>
          <pre 
            style="
              margin: 0; 
              padding: 16px; 
              background: #141414; 
              border: 1px solid var(--border); 
              border-radius: var(--radius-sm); 
              font-family: var(--font-mono); 
              font-size: 12px; 
              color: #f8f8f2; 
              overflow-x: auto;
              max-height: 350px;
              white-space: pre;
              line-height: 1.5;
            "
          ><code>{selectedSub.code}</code></pre>
        </div>
      </div>

      <!-- Action Buttons -->
      <footer style="display: flex; gap: 10px; justify-content: flex-end; margin-top: 20px; border-top: 1px solid var(--border); padding-top: 14px;">
        <button class="btn-run" onclick={closeModal} style="padding: 8px 18px;">
          ปิดหน้าต่าง
        </button>
        <button class="btn-submit" onclick={() => restoreCode(selectedSub)} style="padding: 8px 20px; display: flex; align-items: center; gap: 6px;">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" style="width: 12px; height: 12px;">
            <path d="M12 20h9"/>
            <path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"/>
          </svg>
          กู้คืนโค้ดนี้สู่อีดิตเตอร์
        </button>
      </footer>
    </div>
  </div>
{/if}

<style>
  .sub-row-hover:hover {
    background-color: var(--bg-surface-raised) !important;
  }
  .spinner-ring {
    border: 2px solid var(--border);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
</style>
