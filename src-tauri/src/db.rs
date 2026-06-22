use rusqlite::{params, Connection, Result};
use std::path::PathBuf;
use std::fs;
use std::sync::Mutex;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    pub id: String,
    pub title: String,
    pub category: String,
    pub description: String,
    pub input_specification: String,
    pub output_specification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub id: String,
    pub problem_id: String,
    pub input: String,
    pub expected_output: String,
    #[serde(default)]
    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub id: String,
    pub user_id: String,
    pub problem_id: String,
    pub code: String,
    pub score: i32,
    pub passed_count: i32,
    pub total_count: i32,
    pub verdict: String,
    pub submitted_at: String,
    #[serde(skip_serializing, default)]
    pub synced: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionWithProblem {
    pub id: String,
    pub user_id: String,
    pub problem_id: String,
    pub problem_title: String,
    pub problem_category: String,
    pub code: String,
    pub score: i32,
    pub passed_count: i32,
    pub total_count: i32,
    pub verdict: String,
    pub submitted_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatus {
    pub user_id: String,
    pub status: String,
    pub current_problem_id: Option<String>,
    pub daily_progress: String, // JSON string
    pub daily_completed: bool,
    pub last_active: String,
}

/// DbManager holds a single persistent SQLite connection behind a Mutex.
/// This avoids the overhead of opening/closing a file handle on every query.
pub struct DbManager {
    conn: Mutex<Connection>,
}

impl DbManager {
    pub fn new(app_data_dir: PathBuf) -> Self {
        if !app_data_dir.exists() {
            let _ = fs::create_dir_all(&app_data_dir);
        }
        let db_path = app_data_dir.join("trainer.db");
        println!("DATABASE PATH: {:?}", db_path);

        let conn = Connection::open(&db_path).expect("Failed to open SQLite database");

        // Apply performance pragmas once at startup
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA cache_size = 10000;
             PRAGMA temp_store = MEMORY;
             PRAGMA foreign_keys = ON;",
        )
        .expect("Failed to set SQLite pragmas");

        let manager = DbManager {
            conn: Mutex::new(conn),
        };
        manager.init_db().expect("Failed to initialize SQLite database");
        manager.seed_if_empty().expect("Failed to seed mock problems");
        manager
    }

    // ---- Schema Init ----

    fn init_db(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS problems (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                category TEXT NOT NULL,
                description TEXT NOT NULL,
                input_specification TEXT NOT NULL,
                output_specification TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS public_test_cases (
                id TEXT PRIMARY KEY,
                problem_id TEXT NOT NULL REFERENCES problems(id) ON DELETE CASCADE,
                input TEXT NOT NULL,
                expected_output TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS private_test_cases (
                id TEXT PRIMARY KEY,
                problem_id TEXT NOT NULL REFERENCES problems(id) ON DELETE CASCADE,
                input TEXT NOT NULL,
                expected_output TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS user_status (
                user_id TEXT PRIMARY KEY,
                status TEXT NOT NULL,
                current_problem_id TEXT REFERENCES problems(id) ON DELETE SET NULL,
                daily_progress TEXT NOT NULL DEFAULT '{}',
                daily_completed INTEGER NOT NULL DEFAULT 0,
                last_active TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS submissions (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                problem_id TEXT NOT NULL REFERENCES problems(id) ON DELETE CASCADE,
                code TEXT NOT NULL,
                score INTEGER NOT NULL,
                passed_count INTEGER NOT NULL,
                total_count INTEGER NOT NULL,
                verdict TEXT NOT NULL,
                submitted_at TEXT NOT NULL,
                synced INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS daily_challenges (
                date TEXT PRIMARY KEY,
                problem_ids TEXT NOT NULL
            );",
        )?;

        // Migration: add synced column if missing
        let _ = conn.execute(
            "ALTER TABLE submissions ADD COLUMN synced INTEGER NOT NULL DEFAULT 0",
            [],
        );

        Ok(())
    }

    // ---- Settings ----

    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?")?;
        let mut rows = stmt.query(params![key])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)",
            params![key, value],
        )?;
        Ok(())
    }

    /// Fetch multiple settings in a single query — drastically reduces overhead
    /// compared to calling get_setting() for each key individually.
    pub fn get_settings_map(&self, keys: &[&str]) -> HashMap<String, String> {
        if keys.is_empty() {
            return HashMap::new();
        }
        let conn = self.conn.lock().unwrap();
        let placeholders = keys.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query = format!(
            "SELECT key, value FROM settings WHERE key IN ({})",
            placeholders
        );
        let mut stmt = match conn.prepare(&query) {
            Ok(s) => s,
            Err(_) => return HashMap::new(),
        };
        let mut map = HashMap::new();
        let rows = stmt.query_map(rusqlite::params_from_iter(keys.iter()), |row| {
            let k: String = row.get(0)?;
            let v: String = row.get(1)?;
            Ok((k, v))
        });
        if let Ok(rows) = rows {
            for row in rows.flatten() {
                map.insert(row.0, row.1);
            }
        }
        map
    }

    // ---- Problems ----

    pub fn insert_problem(&self, prob: &Problem) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO problems (id, title, category, description, input_specification, output_specification)
             VALUES (?, ?, ?, ?, ?, ?)",
            params![prob.id, prob.title, prob.category, prob.description, prob.input_specification, prob.output_specification],
        )?;
        Ok(())
    }

    pub fn insert_problems_batch(&self, problems: &[Problem]) -> Result<()> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT OR REPLACE INTO problems (id, title, category, description, input_specification, output_specification)
                 VALUES (?, ?, ?, ?, ?, ?)",
            )?;
            for prob in problems {
                stmt.execute(params![
                    prob.id, prob.title, prob.category,
                    prob.description, prob.input_specification, prob.output_specification
                ])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_problems(&self) -> Result<Vec<Problem>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, category, description, input_specification, output_specification FROM problems",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Problem {
                id: row.get(0)?,
                title: row.get(1)?,
                category: row.get(2)?,
                description: row.get(3)?,
                input_specification: row.get(4)?,
                output_specification: row.get(5)?,
            })
        })?;
        rows.collect::<Result<Vec<_>>>()
    }

    pub fn get_problem(&self, problem_id: &str) -> Result<Option<Problem>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, category, description, input_specification, output_specification
             FROM problems WHERE id = ?",
        )?;
        let mut rows = stmt.query_map(params![problem_id], |row| {
            Ok(Problem {
                id: row.get(0)?,
                title: row.get(1)?,
                category: row.get(2)?,
                description: row.get(3)?,
                input_specification: row.get(4)?,
                output_specification: row.get(5)?,
            })
        })?;
        if let Some(res) = rows.next() { Ok(Some(res?)) } else { Ok(None) }
    }

    pub fn delete_problem(&self, problem_id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM problems WHERE id = ?", params![problem_id])?;
        Ok(())
    }

    // ---- Test Cases ----

    pub fn insert_public_test_case(&self, tc: &TestCase) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO public_test_cases (id, problem_id, input, expected_output) VALUES (?, ?, ?, ?)",
            params![tc.id, tc.problem_id, tc.input, tc.expected_output],
        )?;
        Ok(())
    }

    pub fn insert_public_test_cases_batch(&self, tcs: &[TestCase]) -> Result<()> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT OR REPLACE INTO public_test_cases (id, problem_id, input, expected_output) VALUES (?, ?, ?, ?)",
            )?;
            for tc in tcs {
                stmt.execute(params![tc.id, tc.problem_id, tc.input, tc.expected_output])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn insert_private_test_case(&self, tc: &TestCase) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO private_test_cases (id, problem_id, input, expected_output) VALUES (?, ?, ?, ?)",
            params![tc.id, tc.problem_id, tc.input, tc.expected_output],
        )?;
        Ok(())
    }

    pub fn insert_private_test_cases_batch(&self, tcs: &[TestCase]) -> Result<()> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn.transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT OR REPLACE INTO private_test_cases (id, problem_id, input, expected_output) VALUES (?, ?, ?, ?)",
            )?;
            for tc in tcs {
                stmt.execute(params![tc.id, tc.problem_id, tc.input, tc.expected_output])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_public_test_cases(&self, problem_id: &str) -> Result<Vec<TestCase>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, problem_id, input, expected_output FROM public_test_cases WHERE problem_id = ?",
        )?;
        let rows = stmt.query_map(params![problem_id], |row| {
            Ok(TestCase {
                id: row.get(0)?,
                problem_id: row.get(1)?,
                input: row.get(2)?,
                expected_output: row.get(3)?,
                visible: true,
            })
        })?;
        rows.collect::<Result<Vec<_>>>()
    }

    pub fn get_private_test_cases(&self, problem_id: &str) -> Result<Vec<TestCase>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, problem_id, input, expected_output FROM private_test_cases WHERE problem_id = ?",
        )?;
        let rows = stmt.query_map(params![problem_id], |row| {
            Ok(TestCase {
                id: row.get(0)?,
                problem_id: row.get(1)?,
                input: row.get(2)?,
                expected_output: row.get(3)?,
                visible: false,
            })
        })?;
        rows.collect::<Result<Vec<_>>>()
    }

    pub fn get_all_public_test_cases(&self) -> Result<Vec<TestCase>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, problem_id, input, expected_output FROM public_test_cases",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(TestCase {
                id: row.get(0)?,
                problem_id: row.get(1)?,
                input: row.get(2)?,
                expected_output: row.get(3)?,
                visible: true,
            })
        })?;
        rows.collect::<Result<Vec<_>>>()
    }

    pub fn get_all_private_test_cases(&self) -> Result<Vec<TestCase>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, problem_id, input, expected_output FROM private_test_cases",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(TestCase {
                id: row.get(0)?,
                problem_id: row.get(1)?,
                input: row.get(2)?,
                expected_output: row.get(3)?,
                visible: false,
            })
        })?;
        rows.collect::<Result<Vec<_>>>()
    }

    pub fn delete_public_test_case(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM public_test_cases WHERE id = ?", params![id])?;
        Ok(())
    }

    pub fn delete_private_test_case(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM private_test_cases WHERE id = ?", params![id])?;
        Ok(())
    }

    // ---- Submissions ----

    pub fn insert_submission(&self, sub: &Submission) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let synced_int = if sub.synced { 1 } else { 0 };
        conn.execute(
            "INSERT OR REPLACE INTO submissions
             (id, user_id, problem_id, code, score, passed_count, total_count, verdict, submitted_at, synced)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                sub.id, sub.user_id, sub.problem_id, sub.code,
                sub.score, sub.passed_count, sub.total_count,
                sub.verdict, sub.submitted_at, synced_int
            ],
        )?;
        Ok(())
    }

    pub fn get_submissions(&self, user_id: &str) -> Result<Vec<Submission>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, problem_id, code, score, passed_count, total_count, verdict, submitted_at, synced
             FROM submissions WHERE user_id = ? ORDER BY submitted_at DESC",
        )?;
        let rows = stmt.query_map(params![user_id], |row| {
            let synced_int: i32 = row.get(9)?;
            Ok(Submission {
                id: row.get(0)?,
                user_id: row.get(1)?,
                problem_id: row.get(2)?,
                code: row.get(3)?,
                score: row.get(4)?,
                passed_count: row.get(5)?,
                total_count: row.get(6)?,
                verdict: row.get(7)?,
                submitted_at: row.get(8)?,
                synced: synced_int != 0,
            })
        })?;
        rows.collect::<Result<Vec<_>>>()
    }

    pub fn get_submissions_with_problems(&self, user_id: &str) -> Result<Vec<SubmissionWithProblem>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT s.id, s.user_id, s.problem_id, p.title, p.category, s.code, s.score, s.passed_count, s.total_count, s.verdict, s.submitted_at
             FROM submissions s
             LEFT JOIN problems p ON s.problem_id = p.id
             WHERE s.user_id = ? ORDER BY s.submitted_at DESC",
        )?;
        let rows = stmt.query_map(params![user_id], |row| {
            let title: String = row.get(3).unwrap_or_else(|_| "Unknown Problem".to_string());
            let category: String = row.get(4).unwrap_or_else(|_| "Unknown Category".to_string());
            Ok(SubmissionWithProblem {
                id: row.get(0)?,
                user_id: row.get(1)?,
                problem_id: row.get(2)?,
                problem_title: title,
                problem_category: category,
                code: row.get(5)?,
                score: row.get(6)?,
                passed_count: row.get(7)?,
                total_count: row.get(8)?,
                verdict: row.get(9)?,
                submitted_at: row.get(10)?,
            })
        })?;
        rows.collect::<Result<Vec<_>>>()
    }

    pub fn get_unsynced_submissions(&self) -> Result<Vec<Submission>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, problem_id, code, score, passed_count, total_count, verdict, submitted_at, synced
             FROM submissions WHERE synced = 0",
        )?;
        let rows = stmt.query_map([], |row| {
            let synced_int: i32 = row.get(9)?;
            Ok(Submission {
                id: row.get(0)?,
                user_id: row.get(1)?,
                problem_id: row.get(2)?,
                code: row.get(3)?,
                score: row.get(4)?,
                passed_count: row.get(5)?,
                total_count: row.get(6)?,
                verdict: row.get(7)?,
                submitted_at: row.get(8)?,
                synced: synced_int != 0,
            })
        })?;
        rows.collect::<Result<Vec<_>>>()
    }

    pub fn mark_submission_synced(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE submissions SET synced = 1 WHERE id = ?", params![id])?;
        Ok(())
    }

    pub fn get_best_score(&self, user_id: &str, problem_id: &str) -> Result<i32> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT MAX(score) FROM submissions WHERE user_id = ? AND problem_id = ?",
        )?;
        let mut rows = stmt.query(params![user_id, problem_id])?;
        if let Some(row) = rows.next()? {
            let score: Option<i32> = row.get(0)?;
            Ok(score.unwrap_or(0))
        } else {
            Ok(0)
        }
    }

    // ---- Daily Challenges ----

    pub fn get_daily_challenge(&self, date_str: &str) -> Result<Option<Vec<String>>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT problem_ids FROM daily_challenges WHERE date = ?")?;
        let mut rows = stmt.query(params![date_str])?;
        if let Some(row) = rows.next()? {
            let raw: String = row.get(0)?;
            let ids: Vec<String> = serde_json::from_str(&raw).unwrap_or_default();
            Ok(Some(ids))
        } else {
            Ok(None)
        }
    }

    pub fn get_all_assigned_problem_ids(&self) -> Result<std::collections::HashSet<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT problem_ids FROM daily_challenges")?;
        let rows = stmt.query_map([], |row| {
            let raw: String = row.get(0)?;
            let ids: Vec<String> = serde_json::from_str(&raw).unwrap_or_default();
            Ok(ids)
        })?;
        let mut all_ids = std::collections::HashSet::new();
        for r in rows.flatten() {
            for id in r {
                all_ids.insert(id);
            }
        }
        Ok(all_ids)
    }

    pub fn save_daily_challenge(&self, date_str: &str, problem_ids: &[String]) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let raw = serde_json::to_string(problem_ids).unwrap_or_else(|_| "[]".to_string());
        conn.execute(
            "INSERT OR REPLACE INTO daily_challenges (date, problem_ids) VALUES (?, ?)",
            params![date_str, raw],
        )?;
        Ok(())
    }

    pub fn delete_daily_challenge_by_date(&self, date_str: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM daily_challenges WHERE date = ?", params![date_str])?;
        Ok(())
    }

    // ---- User Status ----

    pub fn save_user_status(&self, status: &UserStatus) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let completed_int = if status.daily_completed { 1 } else { 0 };
        conn.execute(
            "INSERT OR REPLACE INTO user_status
             (user_id, status, current_problem_id, daily_progress, daily_completed, last_active)
             VALUES (?, ?, ?, ?, ?, ?)",
            params![
                status.user_id, status.status, status.current_problem_id,
                status.daily_progress, completed_int, status.last_active
            ],
        )?;
        Ok(())
    }

    pub fn get_user_status(&self, user_id: &str) -> Result<Option<UserStatus>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT user_id, status, current_problem_id, daily_progress, daily_completed, last_active
             FROM user_status WHERE user_id = ?",
        )?;
        let mut rows = stmt.query_map(params![user_id], |row| {
            let completed_int: i32 = row.get(4)?;
            Ok(UserStatus {
                user_id: row.get(0)?,
                status: row.get(1)?,
                current_problem_id: row.get(2)?,
                daily_progress: row.get(3)?,
                daily_completed: completed_int != 0,
                last_active: row.get(5)?,
            })
        })?;
        if let Some(res) = rows.next() { Ok(Some(res?)) } else { Ok(None) }
    }

    // ---- Seed ----

    pub fn seed_if_empty(&self) -> Result<()> {
        let problems = self.get_problems()?;
        if !problems.is_empty() {
            return Ok(());
        }

        let mock_probs = vec![
            Problem {
                id: "prob_hello".to_string(),
                title: "สวัสดีชาวโลก (Hello World)".to_string(),
                category: "Input / Output".to_string(),
                description: "เขียนโปรแกรมที่แสดงข้อความ 'Hello, World!' ออกทางจอภาพหลัก (standard output)".to_string(),
                input_specification: "ไม่มีข้อมูลเข้าสำหรับโจทย์ข้อนี้".to_string(),
                output_specification: "แสดงข้อความ 'Hello, World!' บนบรรทัดเดียว".to_string(),
            },
            Problem {
                id: "prob_echo".to_string(),
                title: "สะท้อนข้อมูลเข้า (Echo Input)".to_string(),
                category: "Input / Output".to_string(),
                description: "อ่านข้อความ 1 บรรทัดจากอุปกรณ์อินพุตหลัก (standard input) แล้วแสดงผลข้อความดังกล่าวออกไปเหมือนเดิมทุกประการ".to_string(),
                input_specification: "ข้อความ 1 บรรทัด".to_string(),
                output_specification: "แสดงข้อความเดิมที่รับเข้ามา".to_string(),
            },
            Problem {
                id: "prob_square".to_string(),
                title: "กำลังสองของตัวเลข (Square of Number)".to_string(),
                category: "Conditions".to_string(),
                description: "รับจำนวนเต็ม N หาก N เป็นจำนวนเต็มบวก ให้แสดงกำลังสองของ N หาก N มีค่าน้อยกว่าหรือเท่ากับศูนย์ ให้แสดง 0".to_string(),
                input_specification: "จำนวนเต็ม N จำนวน 1 ตัว".to_string(),
                output_specification: "แสดงกำลังสองของ N หรือเลข 0".to_string(),
            },
            Problem {
                id: "prob_sum".to_string(),
                title: "ผลรวมของตัวเลข 1 ถึง N (Sum of N Numbers)".to_string(),
                category: "Loops".to_string(),
                description: "เขียนโปรแกรมที่คำนวณหาผลรวมของจำนวนเต็มทั้งหมดตั้งแต่ 1 ถึง N (รวม N ด้วย)".to_string(),
                input_specification: "จำนวนเต็มบวก N จำนวน 1 ตัว".to_string(),
                output_specification: "แสดงจำนวนเต็ม 1 ตัวซึ่งเป็นผลรวมที่คำนวณได้".to_string(),
            },
            Problem {
                id: "prob_double".to_string(),
                title: "เพิ่มค่าสองเท่า (Double it)".to_string(),
                category: "Functions".to_string(),
                description: "เขียนโปรแกรมที่รับตัวเลข N และแสดงผลลัพธ์เป็นสองเท่าของ N".to_string(),
                input_specification: "ตัวเลข N จำนวน 1 ตัว".to_string(),
                output_specification: "แสดงผลลัพธ์ N คูณด้วย 2".to_string(),
            },
            Problem {
                id: "prob_max".to_string(),
                title: "ค้นหาค่าสูงสุด (Find Maximum)".to_string(),
                category: "Lists".to_string(),
                description: "รับชุดตัวเลขที่คั่นด้วยช่องว่าง ค้นหาและแสดงผลลัพธ์ที่มีค่ามากที่สุดในชุดตัวเลขนั้น".to_string(),
                input_specification: "ชุดจำนวนเต็มคั่นด้วยช่องว่างในบรรทัดเดียว".to_string(),
                output_specification: "แสดงค่าที่มากที่สุด".to_string(),
            },
        ];

        for prob in mock_probs {
            self.insert_problem(&prob)?;
        }

        let tcs = vec![
            ("tc_hello_pub",  "prob_hello",  "",             "Hello, World!\n", true),
            ("tc_hello_priv", "prob_hello",  "",             "Hello, World!\n", false),
            ("tc_echo_pub",   "prob_echo",   "antigravity",  "antigravity\n",   true),
            ("tc_echo_priv",  "prob_echo",   "test_run",     "test_run\n",      false),
            ("tc_sq_pub1",    "prob_square", "5",            "25\n",            true),
            ("tc_sq_pub2",    "prob_square", "-3",           "0\n",             true),
            ("tc_sq_priv1",   "prob_square", "10",           "100\n",           false),
            ("tc_sq_priv2",   "prob_square", "0",            "0\n",             false),
            ("tc_sum_pub",    "prob_sum",    "5",            "15\n",            true),
            ("tc_sum_priv1",  "prob_sum",    "10",           "55\n",            false),
            ("tc_sum_priv2",  "prob_sum",    "1",            "1\n",             false),
            ("tc_db_pub",     "prob_double", "4.5",          "9.0\n",           true),
            ("tc_db_priv",    "prob_double", "7",            "14.0\n",          false),
            ("tc_max_pub",    "prob_max",    "3 7 2 9 1",    "9\n",             true),
            ("tc_max_priv1",  "prob_max",    "-5 -10 -2 -8", "-2\n",            false),
            ("tc_max_priv2",  "prob_max",    "42",           "42\n",            false),
        ];

        for (id, prob_id, input, expected, visible) in tcs {
            let tc = TestCase {
                id: id.to_string(),
                problem_id: prob_id.to_string(),
                input: input.to_string(),
                expected_output: expected.to_string(),
                visible,
            };
            if visible {
                self.insert_public_test_case(&tc)?;
            } else {
                self.insert_private_test_case(&tc)?;
            }
        }

        Ok(())
    }
}
