pub mod db;
pub mod judge;
pub mod supabase;
pub mod discord;

use db::{DbManager, Problem, Submission, UserStatus};
use judge::{execute_python_code, normalize_output};
use supabase::SupabaseClient;
use discord::DiscordPresenceManager;
use std::sync::Mutex;
use std::time::Duration;
use tauri::{State, Manager};
use serde::{Serialize, Deserialize};
use chrono::Local;

pub struct AppState {
    pub db: DbManager,
    pub supabase: SupabaseClient,
    /// Server-side cache for DailyChallengeInfo.
    /// Populated on first get_daily_challenge call and invalidated after submit.
    /// Heartbeat reads from here — no recomputation every 15 seconds.
    pub daily_cache: Mutex<Option<DailyChallengeInfo>>,
    pub discord: DiscordPresenceManager,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SampleResult {
    pub tc_id: String,
    pub input: String,
    pub expected: String,
    pub actual: String,
    pub verdict: String,
    pub error: String,
    pub duration_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionResult {
    pub submission_id: String,
    pub problem_id: String,
    pub score: i32,
    pub passed_count: i32,
    pub total_count: i32,
    pub verdict: String,
    pub details: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyChallengeInfo {
    pub date: String,
    pub problems: Vec<Problem>,
    pub category_progress: std::collections::HashMap<String, CategoryProgress>,
    pub solved_problem_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryProgress {
    pub completed: i32,
    pub required: i32,
}

// --- Commands ---

#[tauri::command]
fn detect_python() -> Result<Option<String>, String> {
    Ok(judge::auto_detect_python())
}

#[tauri::command]
fn select_user(state: State<'_, AppState>, user_id: String) -> Result<(), String> {
    if user_id != "NG" && user_id != "MR3" {
        return Err("Invalid user identity. Must be 'NG' or 'MR3'.".to_string());
    }
    state.db.set_setting("active_user", &user_id)
        .map_err(|e| format!("Failed to save user identity: {}", e))?;
    // Invalidate daily cache when switching users
    *state.daily_cache.lock().unwrap() = None;
    Ok(())
}

#[tauri::command]
fn get_active_user(state: State<'_, AppState>) -> Result<Option<String>, String> {
    state.db.get_setting("active_user")
        .map_err(|e| format!("Failed to read active user: {}", e))
}

#[tauri::command]
async fn sync_from_supabase(state: State<'_, AppState>) -> Result<(), String> {
    let result = state.supabase.pull_and_sync_problems(&state.db).await;
    // Invalidate daily cache after sync so fresh data is used
    *state.daily_cache.lock().unwrap() = None;
    result
}

#[tauri::command]
fn get_setting(state: State<'_, AppState>, key: String) -> Result<Option<String>, String> {
    state.db.get_setting(&key)
        .map_err(|e| format!("Failed to fetch setting: {}", e))
}

#[tauri::command]
fn save_setting(state: State<'_, AppState>, key: String, value: String) -> Result<(), String> {
    state.db.set_setting(&key, &value)
        .map_err(|e| format!("Failed to save setting: {}", e))?;
    // Invalidate daily cache when category config settings change
    if key.starts_with("req_") {
        *state.daily_cache.lock().unwrap() = None;
    }
    Ok(())
}

#[tauri::command]
fn update_discord_presence(
    state: State<'_, AppState>,
    pathname: String,
    problem_id: Option<String>,
    user: String,
    is_idle: Option<bool>,
) -> Result<(), String> {
    let details = if is_idle.unwrap_or(false) {
        "ไม่ได้ใช้งาน".to_string()
    } else if pathname.starts_with("/daily") && problem_id.is_some() {
        let pid = problem_id.unwrap();
        if let Ok(Some(prob)) = state.db.get_problem(&pid) {
            format!("กำลังแก้โจทย์: {}", prob.title)
        } else {
            "กำลังแก้โจทย์".to_string()
        }
    } else {
        "กำลังเลือกโจทย์".to_string()
    };
    
    let state_str = format!("ผู้เรียน: {}", user);
    state.discord.update(details, state_str);
    Ok(())
}

#[tauri::command]
fn get_submissions(state: State<'_, AppState>, user_id: String) -> Result<Vec<db::SubmissionWithProblem>, String> {
    state.db.get_submissions_with_problems(&user_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_peer_submissions(state: State<'_, AppState>) -> Result<Vec<Submission>, String> {
    let user_id = state.db.get_setting("active_user")
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "No active user".to_string())?;
    let peer_id = if user_id == "NG" { "MR3" } else { "NG" };
    state.supabase.fetch_user_submissions(&peer_id).await
}

#[tauri::command]
fn get_supabase_config(state: State<'_, AppState>) -> Result<(String, String), String> {
    Ok(state.supabase.get_config())
}

#[tauri::command]
fn get_public_test_cases(state: State<'_, AppState>, problem_id: String) -> Result<Vec<db::TestCase>, String> {
    state.db.get_public_test_cases(&problem_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn run_sample(
    state: State<'_, AppState>,
    problem_id: String,
    code: String,
) -> Result<Vec<SampleResult>, String> {
    let public_tcs = if problem_id == "dummy" {
        vec![db::TestCase {
            id: "dummy_tc".to_string(),
            problem_id: "dummy".to_string(),
            input: "dummy_input".to_string(),
            expected_output: "OK\n".to_string(),
            visible: true,
        }]
    } else {
        state.db.get_public_test_cases(&problem_id)
            .map_err(|e| format!("Failed to load public examples: {}", e))?
    };

    let py_path = state.db.get_setting("python_path").unwrap_or(None);
    let timeout_ms = state.db.get_setting("timeout_ms")
        .unwrap_or(None)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(15000);

    let mut results = Vec::new();
    for tc in public_tcs {
        let res = execute_python_code(&code, &tc.input, py_path.as_deref(), Duration::from_millis(timeout_ms));
        let verdict = if !res.success {
            res.verdict.clone()
        } else {
            let norm_actual = normalize_output(&res.output);
            let norm_expected = normalize_output(&tc.expected_output);
            if norm_actual == norm_expected { "Passed".to_string() } else { "Wrong Answer".to_string() }
        };
        results.push(SampleResult {
            tc_id: tc.id,
            input: tc.input,
            expected: tc.expected_output,
            actual: res.output,
            verdict,
            error: res.error,
            duration_ms: res.duration_ms,
        });
    }
    Ok(results)
}

#[tauri::command]
async fn submit_solution(
    state: State<'_, AppState>,
    problem_id: String,
    code: String,
) -> Result<SubmissionResult, String> {
    let user_id = state.db.get_setting("active_user")
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "No active user profile selected".to_string())?;

    let private_tcs = state.db.get_private_test_cases(&problem_id)
        .map_err(|e| format!("Failed to load private judging data: {}", e))?;

    if private_tcs.is_empty() {
        return Err("No test cases found for this problem".to_string());
    }

    let py_path = state.db.get_setting("python_path").unwrap_or(None);
    let timeout_ms = state.db.get_setting("timeout_ms")
        .unwrap_or(None)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(15000);

    let mut passed_count = 0;
    let total_count = private_tcs.len() as i32;
    let mut details = Vec::new();
    let mut final_verdict = "Accepted".to_string();

    for tc in &private_tcs {
        let res = execute_python_code(&code, &tc.input, py_path.as_deref(), Duration::from_millis(timeout_ms));
        let verdict = if !res.success {
            res.verdict.clone()
        } else {
            let norm_actual = normalize_output(&res.output);
            let norm_expected = normalize_output(&tc.expected_output);
            if norm_actual == norm_expected { "Passed".to_string() } else { "Wrong Answer".to_string() }
        };
        if verdict == "Passed" {
            passed_count += 1;
        } else if final_verdict == "Accepted" {
            final_verdict = verdict.clone();
        }
        details.push(verdict);
    }

    let score = if total_count > 0 { (passed_count * 100) / total_count } else { 0 };
    if score < 100 && final_verdict == "Accepted" {
        final_verdict = "Wrong Answer".to_string();
    }

    let submission_id = format!("sub_{}", chrono::Utc::now().timestamp_micros());
    let submitted_at = chrono::Local::now().to_rfc3339();

    let mut sub = Submission {
        id: submission_id.clone(),
        user_id: user_id.clone(),
        problem_id: problem_id.clone(),
        code,
        score,
        passed_count,
        total_count,
        verdict: final_verdict.clone(),
        submitted_at: submitted_at.clone(),
        synced: false,
    };

    if state.supabase.push_submission(&sub).await.is_ok() {
        sub.synced = true;
    }

    state.db.insert_submission(&sub)
        .map_err(|e| format!("Failed to record submission: {}", e))?;

    // Invalidate daily cache so next get_daily_challenge reflects new progress
    *state.daily_cache.lock().unwrap() = None;

    // Update heartbeat in background (non-blocking)
    let _ = update_daily_progress_after_submit(&state.db, &state.supabase, &user_id, &problem_id, score >= 100).await;

    Ok(SubmissionResult {
        submission_id,
        problem_id,
        score,
        passed_count,
        total_count,
        verdict: final_verdict,
        details,
    })
}

/// Core computation for daily challenge — called only on cache miss.
async fn get_daily_challenge_inner(db: &DbManager, supabase: &SupabaseClient) -> Result<DailyChallengeInfo, String> {
    let date_str = Local::now().format("%Y-%m-%d").to_string();
    let problems = db.get_problems().map_err(|e| e.to_string())?;

    if problems.is_empty() {
        return Ok(DailyChallengeInfo {
            date: date_str,
            problems: Vec::new(),
            category_progress: std::collections::HashMap::new(),
            solved_problem_ids: Vec::new(),
        });
    }

    // Fetch all category requirement settings in a SINGLE batch query
    let settings_keys = [
        "req_io", "req_cond", "req_loops", "req_functions", "req_lists",
        "req_implementation", "req_math", "req_string", "req_sorting",
        "req_searching", "req_greedy", "req_recursion", "req_data_structures",
        "req_graph", "req_dynamic_programming", "req_matrix_operations",
        "req_numerical_methods", "req_simulation", "req_optimization",
        "req_data_parsing", "req_statistics", "req_signal_processing",
    ];
    let settings_map = db.get_settings_map(&settings_keys);

    let get_req = |key: &str, default: i32| -> i32 {
        settings_map.get(key).and_then(|s| s.parse::<i32>().ok()).unwrap_or(default)
    };

    let categories_config = vec![
        ("Input / Output",      get_req("req_io", 2)),
        ("Conditions",          get_req("req_cond", 3)),
        ("Loops",               get_req("req_loops", 5)),
        ("Functions",           get_req("req_functions", 2)),
        ("Lists",               get_req("req_lists", 2)),
        ("Implementation",      get_req("req_implementation", 0)),
        ("Math",                get_req("req_math", 0)),
        ("String",              get_req("req_string", 0)),
        ("Sorting",             get_req("req_sorting", 0)),
        ("Searching",           get_req("req_searching", 0)),
        ("Greedy",              get_req("req_greedy", 0)),
        ("Recursion",           get_req("req_recursion", 0)),
        ("Data Structures",     get_req("req_data_structures", 0)),
        ("Graph",               get_req("req_graph", 0)),
        ("Dynamic Programming", get_req("req_dynamic_programming", 0)),
        ("Matrix Operations",   get_req("req_matrix_operations", 0)),
        ("Numerical Methods",   get_req("req_numerical_methods", 0)),
        ("Simulation",          get_req("req_simulation", 0)),
        ("Optimization",        get_req("req_optimization", 0)),
        ("Data Parsing",        get_req("req_data_parsing", 0)),
        ("Statistics",          get_req("req_statistics", 0)),
        ("Signal Processing",   get_req("req_signal_processing", 0)),
    ];

    let challenge_uuids = if supabase.is_configured() {
        match supabase.fetch_daily_challenge(&date_str).await {
            Ok(Some(ids)) => {
                let _ = db.save_daily_challenge(&date_str, &ids);
                ids
            }
            Ok(None) => {
                let seed = (chrono::Utc::now().timestamp_nanos_opt().unwrap_or(12345) & 0xFFFFFFFF) as u32;
                let assigned_ids = db.get_all_assigned_problem_ids().unwrap_or_default();
                let ids = generate_daily_problems_with_seed(seed, &problems, &categories_config, &assigned_ids);
                
                let _ = db.save_daily_challenge(&date_str, &ids);
                let _ = supabase.push_daily_challenge(&date_str, &ids).await;
                ids
            }
            Err(e) => {
                eprintln!("Failed to fetch daily challenge from Supabase: {}. Falling back to local cache.", e);
                match db.get_daily_challenge(&date_str) {
                    Ok(Some(ids)) => ids,
                    _ => {
                        let seed = (chrono::Utc::now().timestamp_nanos_opt().unwrap_or(12345) & 0xFFFFFFFF) as u32;
                        let assigned_ids = db.get_all_assigned_problem_ids().unwrap_or_default();
                        let ids = generate_daily_problems_with_seed(seed, &problems, &categories_config, &assigned_ids);
                        let _ = db.save_daily_challenge(&date_str, &ids);
                        ids
                    }
                }
            }
        }
    } else {
        match db.get_daily_challenge(&date_str) {
            Ok(Some(ids)) => ids,
            _ => {
                let seed = (chrono::Utc::now().timestamp_nanos_opt().unwrap_or(12345) & 0xFFFFFFFF) as u32;
                let assigned_ids = db.get_all_assigned_problem_ids().unwrap_or_default();
                let ids = generate_daily_problems_with_seed(seed, &problems, &categories_config, &assigned_ids);
                let _ = db.save_daily_challenge(&date_str, &ids);
                ids
            }
        }
    };

    let mut challenge_problems: Vec<Problem> = Vec::new();
    for id in &challenge_uuids {
        if let Ok(Some(p)) = db.get_problem(id) {
            challenge_problems.push(p);
        }
    }
    if challenge_problems.is_empty() {
        challenge_problems = problems.iter()
            .filter(|p| challenge_uuids.contains(&p.id))
            .cloned()
            .collect();
    }

    let user_id = db.get_setting("active_user").unwrap_or(None).unwrap_or_default();
    let mut daily_solved_problems = std::collections::HashSet::new();
    if !user_id.is_empty() {
        if let Ok(subs) = db.get_submissions(&user_id) {
            for sub in subs {
                if sub.score >= 100 && sub.submitted_at.starts_with(&date_str) {
                    daily_solved_problems.insert(sub.problem_id);
                }
            }
        }
    }

    let mut category_progress = std::collections::HashMap::new();
    for (cat, req) in &categories_config {
        let mut completed = 0;
        for prob in &challenge_problems {
            if prob.category == *cat && daily_solved_problems.contains(&prob.id) {
                completed += 1;
            }
        }
        category_progress.insert(cat.to_string(), CategoryProgress { completed, required: *req });
    }

    let solved_problem_ids: Vec<String> = daily_solved_problems.into_iter().collect();

    Ok(DailyChallengeInfo {
        date: date_str,
        problems: challenge_problems,
        category_progress,
        solved_problem_ids,
    })
}

#[tauri::command]
async fn get_daily_challenge(state: State<'_, AppState>) -> Result<DailyChallengeInfo, String> {
    // Return cached result if available — avoids 25+ SQLite queries on repeated calls
    {
        let cache = state.daily_cache.lock().unwrap();
        if let Some(ref cached) = *cache {
            return Ok(cached.clone());
        }
    }
    // Cache miss — compute fresh, then store
    let result = get_daily_challenge_inner(&state.db, &state.supabase).await?;
    *state.daily_cache.lock().unwrap() = Some(result.clone());
    Ok(result)
}

/// Heartbeat: reads progress from Rust cache (no SQLite recomputation).
async fn perform_heartbeat_inner(
    db: &DbManager,
    supabase: &SupabaseClient,
    daily_cache: &Mutex<Option<DailyChallengeInfo>>,
    status: String,
    current_problem_id: Option<String>,
) -> Result<(), String> {
    let user_id = db.get_setting("active_user")
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "No active user".to_string())?;

    // Use cached challenge to compute progress — no DB round-trip
    let (daily_completed, progress_json) = {
        let cache = daily_cache.lock().unwrap();
        if let Some(ref info) = *cache {
            let completed = info.category_progress.values()
                .all(|p| p.completed >= p.required);
            let json = serde_json::to_string(&info.category_progress)
                .unwrap_or_else(|_| "{}".to_string());
            (completed, json)
        } else {
            (false, "{}".to_string())
        }
    };

    let user_stat = UserStatus {
        user_id: user_id.clone(),
        status,
        current_problem_id,
        daily_progress: progress_json,
        daily_completed,
        last_active: chrono::Utc::now().to_rfc3339(),
    };

    let _ = db.save_user_status(&user_stat);
    let _ = supabase.update_status(&user_stat).await;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeartbeatResponse {
    pub peer_status: Option<UserStatus>,
    pub peer_submissions: Vec<Submission>,
}

#[tauri::command]
async fn send_heartbeat(
    state: State<'_, AppState>,
    status: String,
    current_problem_id: Option<String>,
) -> Result<HeartbeatResponse, String> {
    // 1. Send our heartbeat
    perform_heartbeat_inner(&state.db, &state.supabase, &state.daily_cache, status, current_problem_id).await?;
    
    // 2. Fetch active user & peer ID
    let user_id = state.db.get_setting("active_user")
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "No active user".to_string())?;
    let peer_id = if user_id == "NG" { "MR3" } else { "NG" };
    
    // 3. Fetch peer status (using the same logic as get_peer_status)
    let peer_status = if let Ok(Some(status)) = state.supabase.fetch_peer_status(&peer_id).await {
        let _ = state.db.save_user_status(&status);
        Some(status)
    } else {
        state.db.get_user_status(&peer_id).unwrap_or(None)
    };
    
    // 4. Fetch peer submissions
    let peer_submissions = state.supabase.fetch_user_submissions(&peer_id).await.unwrap_or_default();
    
    Ok(HeartbeatResponse {
        peer_status,
        peer_submissions,
    })
}

#[tauri::command]
async fn get_peer_status(state: State<'_, AppState>) -> Result<Option<UserStatus>, String> {
    let user_id = state.db.get_setting("active_user")
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "No active user".to_string())?;

    let peer_id = if user_id == "NG" { "MR3" } else { "NG" };

    if let Ok(Some(status)) = state.supabase.fetch_peer_status(peer_id).await {
        let _ = state.db.save_user_status(&status);
        return Ok(Some(status));
    }

    state.db.get_user_status(peer_id)
        .map_err(|e| format!("Failed to read cached peer status: {}", e))
}

#[tauri::command]
async fn reroll_daily_challenge(state: State<'_, AppState>) -> Result<(), String> {
    let date_str = Local::now().format("%Y-%m-%d").to_string();

    state.db.delete_daily_challenge_by_date(&date_str)
        .map_err(|e| format!("Failed to delete local daily challenge: {}", e))?;

    if state.supabase.is_configured() {
        state.supabase.delete_daily_challenge(&date_str).await
            .map_err(|e| format!("Failed to delete Supabase daily challenge: {}", e))?;
    }

    // Invalidate cache so next get_daily_challenge regenerates
    *state.daily_cache.lock().unwrap() = None;

    Ok(())
}

// --- Helpers ---

async fn update_daily_progress_after_submit(
    db: &DbManager,
    supabase: &SupabaseClient,
    _user_id: &str,
    problem_id: &str,
    is_solved: bool,
) -> Result<(), String> {
    if !is_solved { return Ok(()); }
    // Trigger a lightweight heartbeat (uses Mutex::new(None) — cache was just invalidated)
    let temp_cache: Mutex<Option<DailyChallengeInfo>> = Mutex::new(None);
    let _ = perform_heartbeat_inner(db, supabase, &temp_cache, "Online".to_string(), Some(problem_id.to_string())).await;
    Ok(())
}

fn generate_daily_problems_with_seed(
    seed: u32,
    problems: &[Problem],
    categories_config: &[(&str, i32)],
    assigned_ids: &std::collections::HashSet<String>,
) -> Vec<String> {
    let mut current_seed = seed;
    if current_seed == 0 { current_seed = 12345; }

    let mut selected_ids = Vec::new();

    for &(cat, count) in categories_config {
        let cat_probs: Vec<&Problem> = problems.iter().filter(|p| p.category == cat).collect();
        if cat_probs.is_empty() { continue; }

        let mut unassigned: Vec<&Problem> = Vec::new();
        let mut assigned: Vec<&Problem> = Vec::new();

        for p in cat_probs {
            if assigned_ids.contains(&p.id) { assigned.push(p); } else { unassigned.push(p); }
        }

        unassigned.sort_by_key(|p| &p.id);
        assigned.sort_by_key(|p| &p.id);
        deterministic_shuffle(&mut unassigned, current_seed);
        deterministic_shuffle(&mut assigned, current_seed);

        let target_count = count as usize;
        let pick_unassigned = std::cmp::min(target_count, unassigned.len());
        for i in 0..pick_unassigned {
            selected_ids.push(unassigned[i].id.clone());
        }
        let picked = pick_unassigned;
        if picked < target_count {
            let needed = target_count - picked;
            let pick_assigned = std::cmp::min(needed, assigned.len());
            for i in 0..pick_assigned {
                selected_ids.push(assigned[i].id.clone());
            }
        }
    }

    selected_ids
}

fn deterministic_shuffle<T>(vec: &mut [T], seed: u32) {
    let mut current_seed = seed;
    let n = vec.len();
    if n <= 1 { return; }
    for i in (1..n).rev() {
        current_seed = current_seed.wrapping_mul(1103515245).wrapping_add(12345);
        let j = (current_seed as usize) % (i + 1);
        vec.swap(i, j);
    }
}

// --- Entry point ---

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().expect("Failed to get app data directory");
            let db = DbManager::new(app_data_dir);
            let supabase = SupabaseClient::new();
            let discord = DiscordPresenceManager::new("1518711095651471490");
            app.manage(AppState {
                db,
                supabase,
                daily_cache: Mutex::new(None),
                discord,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            detect_python,
            select_user,
            get_active_user,
            sync_from_supabase,
            get_public_test_cases,
            run_sample,
            submit_solution,
            get_daily_challenge,
            send_heartbeat,
            get_peer_status,
            save_setting,
            get_setting,
            reroll_daily_challenge,
            update_discord_presence,
            get_submissions,
            get_peer_submissions,
            get_supabase_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
