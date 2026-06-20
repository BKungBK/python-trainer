use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::db::{Problem, TestCase, Submission, UserStatus, DbManager};

#[derive(Clone)]
pub struct SupabaseClient {
    url: String,
    anon_key: String,
    client: reqwest::Client,
}

impl SupabaseClient {
    pub fn new() -> Self {
        let (url, anon_key) = Self::load_credentials();
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_default();
        
        SupabaseClient { url, anon_key, client }
    }

    fn load_credentials() -> (String, String) {
        // 1. Try compile-time environment variables
        let url_opt = option_env!("SUPABASE_URL");
        let key_opt = option_env!("SUPABASE_ANON_KEY");
        
        if let (Some(url), Some(key)) = (url_opt, key_opt) {
            if !url.is_empty() && !key.is_empty() {
                return (url.to_string(), key.to_string());
            }
        }
        
        // 2. Try reading from a local .env file in the workspace root
        // (Vite and Tauri compile from the root, but dev server runs in root, so let's try root and src-tauri)
        for path in &[".env", "src-tauri/.env", "../.env"] {
            if let Ok(content) = std::fs::read_to_string(path) {
                let mut url = String::new();
                let mut key = String::new();
                for line in content.lines() {
                    let parts: Vec<&str> = line.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        let k = parts[0].trim();
                        let v = parts[1].trim().trim_matches('"').trim_matches('\'');
                        if k == "SUPABASE_URL" {
                            url = v.to_string();
                        } else if k == "SUPABASE_ANON_KEY" {
                            key = v.to_string();
                        }
                    }
                }
                if !url.is_empty() && !key.is_empty() {
                    return (url, key);
                }
            }
        }
        
        (String::new(), String::new())
    }

    pub fn is_configured(&self) -> bool {
        !self.url.is_empty() && !self.anon_key.is_empty()
    }

    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Ok(val) = HeaderValue::from_str(&self.anon_key) {
            headers.insert("apikey", val.clone());
            headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", self.anon_key)).unwrap_or(val));
        }
        headers
    }

    pub async fn pull_and_sync_problems(&self, db: &DbManager) -> Result<(), String> {
        if !self.is_configured() {
            return Err("Supabase is not configured. Run in offline mode or check credentials.".to_string());
        }

        let headers = self.get_headers();
        let client = &self.client;

        let prob_url = format!("{}/rest/v1/problems?select=*", self.url);
        let pub_tc_url = format!("{}/rest/v1/public_test_cases?select=*", self.url);
        let priv_tc_url = format!("{}/rest/v1/private_test_cases?select=*", self.url);
        let dc_url = format!("{}/rest/v1/daily_challenges?select=*", self.url);
        let cc_url = format!("{}/rest/v1/category_configs?select=*", self.url);

        let prob_fut = client.get(&prob_url).headers(headers.clone()).send();
        let pub_tc_fut = client.get(&pub_tc_url).headers(headers.clone()).send();
        let priv_tc_fut = client.get(&priv_tc_url).headers(headers.clone()).send();
        let dc_fut = client.get(&dc_url).headers(headers.clone()).send();
        let cc_fut = client.get(&cc_url).headers(headers).send();

        // Fetch all resources concurrently
        let (prob_res, pub_tc_res, priv_tc_res, dc_res, cc_res) = tokio::join!(
            prob_fut,
            pub_tc_fut,
            priv_tc_fut,
            dc_fut,
            cc_fut
        );

        // 1. Process Problems
        let prob_res = prob_res.map_err(|e| format!("Network error fetching problems: {}", e))?;
        if !prob_res.status().is_success() {
            return Err(format!("Supabase problems returned error code: {}", prob_res.status()));
        }
        let problems: Vec<Problem> = prob_res.json().await
            .map_err(|e| format!("Failed to parse problems: {}", e))?;

        // Batch insert problems
        db.insert_problems_batch(&problems)
            .map_err(|e| format!("Failed to cache problems: {}", e))?;

        let fetched_prob_ids: std::collections::HashSet<String> = problems.iter().map(|p| p.id.clone()).collect();
        // Delete local problems not in fetched list
        if let Ok(local_probs) = db.get_problems() {
            for p in local_probs {
                if !fetched_prob_ids.contains(&p.id) {
                    let _ = db.delete_problem(&p.id);
                }
            }
        }

        // 2. Process Public Test Cases
        if let Ok(pub_res) = pub_tc_res {
            if pub_res.status().is_success() {
                let public_tcs_res: Result<Vec<TestCase>, reqwest::Error> = pub_res.json().await;
                if let Ok(public_tcs) = public_tcs_res {
                    let _ = db.insert_public_test_cases_batch(&public_tcs);
                    let fetched_pub_tc_ids: std::collections::HashSet<String> = public_tcs.iter().map(|tc| tc.id.clone()).collect();
                    if let Ok(local_pub_tcs) = db.get_all_public_test_cases() {
                        for tc in local_pub_tcs {
                            if !fetched_pub_tc_ids.contains(&tc.id) {
                                let _ = db.delete_public_test_case(&tc.id);
                            }
                        }
                    }
                }
            }
        }

        // 3. Process Private Test Cases
        if let Ok(priv_res) = priv_tc_res {
            if priv_res.status().is_success() {
                let private_tcs_res: Result<Vec<TestCase>, reqwest::Error> = priv_res.json().await;
                if let Ok(private_tcs) = private_tcs_res {
                    let _ = db.insert_private_test_cases_batch(&private_tcs);
                    let fetched_priv_tc_ids: std::collections::HashSet<String> = private_tcs.iter().map(|tc| tc.id.clone()).collect();
                    if let Ok(local_priv_tcs) = db.get_all_private_test_cases() {
                        for tc in local_priv_tcs {
                            if !fetched_priv_tc_ids.contains(&tc.id) {
                                let _ = db.delete_private_test_case(&tc.id);
                            }
                        }
                    }
                }
            }
        }

        // 4. Process daily challenges registry
        if let Ok(dc_res) = dc_res {
            #[derive(Deserialize)]
            struct DailyChallengeRow {
                date: String,
                problem_ids: serde_json::Value,
            }
            if dc_res.status().is_success() {
                if let Ok(rows) = dc_res.json::<Vec<DailyChallengeRow>>().await {
                    for row in rows {
                        if let Ok(prob_ids) = serde_json::from_value::<Vec<String>>(row.problem_ids) {
                            let _ = db.save_daily_challenge(&row.date, &prob_ids);
                        }
                    }
                }
            }
        }

        // 4.5 Process target configurations
        if let Ok(cc_res) = cc_res {
            #[derive(Deserialize)]
            struct CategoryConfigRow {
                category: String,
                target_count: i32,
            }
            if cc_res.status().is_success() {
                if let Ok(rows) = cc_res.json::<Vec<CategoryConfigRow>>().await {
                    for row in rows {
                        let setting_key = match row.category.as_str() {
                            "Input / Output" => "req_io",
                            "Conditions" => "req_cond",
                            "Loops" => "req_loops",
                            "Functions" => "req_functions",
                            "Lists" => "req_lists",
                            "Implementation" => "req_implementation",
                            "Math" => "req_math",
                            "String" => "req_string",
                            "Sorting" => "req_sorting",
                            "Searching" => "req_searching",
                            "Greedy" => "req_greedy",
                            "Recursion" => "req_recursion",
                            "Data Structures" => "req_data_structures",
                            "Graph" => "req_graph",
                            "Dynamic Programming" => "req_dynamic_programming",
                            "Matrix Operations" => "req_matrix_operations",
                            "Numerical Methods" => "req_numerical_methods",
                            "Simulation" => "req_simulation",
                            "Optimization" => "req_optimization",
                            "Data Parsing" => "req_data_parsing",
                            "Statistics" => "req_statistics",
                            "Signal Processing" => "req_signal_processing",
                            _ => continue,
                        };
                        let _ = db.set_setting(setting_key, &row.target_count.to_string());
                    }
                }
            }
        }

        // 5. Sync user submissions from Supabase to prevent multi-device status reset
        if let Some(user_id) = db.get_setting("active_user").unwrap_or(None) {
            if let Ok(subs) = self.fetch_user_submissions(&user_id).await {
                for mut sub in subs {
                    sub.synced = true;
                    let _ = db.insert_submission(&sub);
                }
            }
        }

        // 6. Push local unsynced submissions to Supabase
        if let Ok(unsynced) = db.get_unsynced_submissions() {
            for sub in unsynced {
                if self.push_submission(&sub).await.is_ok() {
                    let _ = db.mark_submission_synced(&sub.id);
                }
            }
        }

        Ok(())
    }

    pub async fn fetch_user_submissions(&self, user_id: &str) -> Result<Vec<Submission>, String> {
        if !self.is_configured() {
            return Ok(Vec::new());
        }

        let sub_url = format!("{}/rest/v1/submissions?user_id=eq.{}", self.url, user_id);
        let res = self.client.get(&sub_url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Network error fetching user submissions: {}", e))?;

        if !res.status().is_success() {
            return Err(format!("Supabase user submissions fetch returned error: {}", res.status()));
        }

        let list: Vec<Submission> = res.json()
            .await
            .map_err(|e| format!("Failed to parse user submissions: {}", e))?;

        Ok(list)
    }

    pub async fn push_submission(&self, sub: &Submission) -> Result<(), String> {
        if !self.is_configured() {
            return Ok(()); // Silently skip sync when offline
        }

        let sub_url = format!("{}/rest/v1/submissions", self.url);
        let res = self.client.post(&sub_url)
            .headers(self.get_headers())
            .json(sub)
            .send()
            .await
            .map_err(|e| format!("Network error pushing submission: {}", e))?;

        if !res.status().is_success() {
            return Err(format!("Supabase submissions returned error: {}", res.status()));
        }

        Ok(())
    }

    pub async fn update_status(&self, status: &UserStatus) -> Result<(), String> {
        if !self.is_configured() {
            return Ok(());
        }

        // Use UPSERT by checking response / using postgrest upsert format
        // Upsert format: POST with Prefer: resolution=merge-duplicates
        let mut headers = self.get_headers();
        headers.insert("Prefer", HeaderValue::from_static("resolution=merge-duplicates"));
        
        let upsert_url = format!("{}/rest/v1/user_status", self.url);
        let res = self.client.post(&upsert_url)
            .headers(headers)
            .json(status)
            .send()
            .await
            .map_err(|e| format!("Network error updating status: {}", e))?;

        if !res.status().is_success() {
            return Err(format!("Supabase status update failed: {}", res.status()));
        }

        Ok(())
    }

    pub async fn fetch_peer_status(&self, peer_id: &str) -> Result<Option<UserStatus>, String> {
        if !self.is_configured() {
            return Ok(None);
        }

        let status_url = format!("{}/rest/v1/user_status?user_id=eq.{}", self.url, peer_id);
        let res = self.client.get(&status_url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Network error fetching peer status: {}", e))?;

        if !res.status().is_success() {
            return Err(format!("Supabase status fetch failed: {}", res.status()));
        }

        let mut list: Vec<UserStatus> = res.json()
            .await
            .map_err(|e| format!("Failed to parse peer status: {}", e))?;

        if !list.is_empty() {
            Ok(Some(list.remove(0)))
        } else {
            Ok(None)
        }
    }

    pub async fn push_daily_challenge(&self, date_str: &str, problem_ids: &[String]) -> Result<(), String> {
        if !self.is_configured() {
            return Ok(());
        }

        #[derive(Serialize)]
        struct DailyChallengeRow<'a> {
            date: &'a str,
            problem_ids: &'a [String],
        }

        let mut headers = self.get_headers();
        headers.insert("Prefer", HeaderValue::from_static("resolution=merge-duplicates"));
        
        let upsert_url = format!("{}/rest/v1/daily_challenges", self.url);
        let row = DailyChallengeRow { date: date_str, problem_ids };
        
        let _ = self.client.post(&upsert_url)
            .headers(headers)
            .json(&row)
            .send()
            .await;

        Ok(())
    }

    pub async fn fetch_daily_challenge(&self, date_str: &str) -> Result<Option<Vec<String>>, String> {
        if !self.is_configured() {
            return Ok(None);
        }

        let url = format!("{}/rest/v1/daily_challenges?date=eq.{}", self.url, date_str);
        let res = self.client.get(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Network error fetching daily challenge: {}", e))?;

        if !res.status().is_success() {
            return Err(format!("Supabase fetch daily challenge returned error: {}", res.status()));
        }

        #[derive(Deserialize)]
        struct DailyChallengeRow {
            problem_ids: serde_json::Value,
        }

        let mut list: Vec<DailyChallengeRow> = res.json()
            .await
            .map_err(|e| format!("Failed to parse daily challenge list: {}", e))?;

        if !list.is_empty() {
            let row = list.remove(0);
            let ids: Vec<String> = serde_json::from_value(row.problem_ids)
                .map_err(|e| format!("Failed to deserialize problem_ids JSON: {}", e))?;
            Ok(Some(ids))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_daily_challenge(&self, date_str: &str) -> Result<(), String> {
        if !self.is_configured() {
            return Ok(());
        }

        let url = format!("{}/rest/v1/daily_challenges?date=eq.{}", self.url, date_str);
        let res = self.client.delete(&url)
            .headers(self.get_headers())
            .send()
            .await
            .map_err(|e| format!("Network error deleting daily challenge: {}", e))?;

        if !res.status().is_success() && res.status() != reqwest::StatusCode::NOT_FOUND {
            return Err(format!("Supabase delete daily challenge returned error: {}", res.status()));
        }

        Ok(())
    }
}
