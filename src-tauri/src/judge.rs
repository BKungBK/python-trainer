use std::process::{Command, Stdio};
use std::io::Write;
use std::time::{Instant, Duration};
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JudgeResult {
    pub success: bool,
    pub output: String,
    pub error: String,
    pub verdict: String, // "Passed", "Wrong Answer", "Runtime Error", "Time Limit Exceeded"
    pub duration_ms: u64,
}

pub fn execute_python_code(
    code: &str,
    input: &str,
    python_path: Option<&str>,
    timeout_duration: Duration,
) -> JudgeResult {
    let py_exec = python_path
        .filter(|p| !p.trim().is_empty())
        .map(|s| s.to_string())
        .or_else(auto_detect_python)
        .unwrap_or_else(|| "python".to_string());

    // Write to a temporary file
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join(format!("trainer_temp_{}.py", uuid_like_id()));
    
    if let Err(e) = fs::write(&temp_file_path, code) {
        return JudgeResult {
            success: false,
            output: String::new(),
            error: format!("Failed to write temporary solution file: {}", e),
            verdict: "Runtime Error".to_string(),
            duration_ms: 0,
        };
    }

    let start_time = Instant::now();

    // Spawn Python subprocess
    let mut cmd = Command::new(&py_exec);
    cmd.arg(&temp_file_path)
       .stdin(Stdio::piped())
       .stdout(Stdio::piped())
       .stderr(Stdio::piped());

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }

    let child_res = cmd.spawn();

    let mut child = match child_res {
        Ok(c) => c,
        Err(e) => {
            let _ = fs::remove_file(&temp_file_path);
            return JudgeResult {
                success: false,
                output: String::new(),
                error: format!("Failed to spawn Python process (executable: '{}'): {}. Ensure Python is installed and added to PATH.", py_exec, e),
                verdict: "Runtime Error".to_string(),
                duration_ms: 0,
            };
        }
    };

    // Write input to stdin
    if let Some(mut stdin) = child.stdin.take() {
        let input_bytes = input.as_bytes().to_vec();
        // Spawning a thread to write to stdin prevents blocking if input is large
        std::thread::spawn(move || {
            let _ = stdin.write_all(&input_bytes);
            let _ = stdin.flush();
        });
    }

    // Wait for the process to finish with a timeout
    let (status, duration) = match wait_timeout(&mut child, timeout_duration) {
        Ok(Some((st, dur))) => (Ok(st), dur),
        Ok(None) => {
            let _ = child.kill();
            let _ = fs::remove_file(&temp_file_path);
            return JudgeResult {
                success: false,
                output: String::new(),
                error: "Time Limit Exceeded (TLE)".to_string(),
                verdict: "Time Limit Exceeded".to_string(),
                duration_ms: timeout_duration.as_millis() as u64,
            };
        }
        Err(e) => {
            let _ = child.kill();
            (Err(e), start_time.elapsed())
        }
    };

    let _ = fs::remove_file(&temp_file_path);

    let duration_ms = duration.as_millis() as u64;

    match status {
        Ok(exit_status) => {
            let output = child.wait_with_output().unwrap();
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            if exit_status.success() {
                JudgeResult {
                    success: true,
                    output: stdout,
                    error: stderr,
                    verdict: "Passed".to_string(),
                    duration_ms,
                }
            } else {
                JudgeResult {
                    success: false,
                    output: stdout,
                    error: stderr,
                    verdict: "Runtime Error".to_string(),
                    duration_ms,
                }
            }
        }
        Err(e) => JudgeResult {
            success: false,
            output: String::new(),
            error: format!("Execution failed: {}", e),
            verdict: "Runtime Error".to_string(),
            duration_ms,
        },
    }
}

// Helper: Wait with timeout in a simple blocking thread way since std::process doesn't have async timeout
fn wait_timeout(
    child: &mut std::process::Child,
    limit: Duration,
) -> std::io::Result<Option<(std::process::ExitStatus, Duration)>> {
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Ok(Some((status, start.elapsed()))),
            Ok(None) => {
                if start.elapsed() >= limit {
                    return Ok(None);
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(e) => return Err(e),
        }
    }
}

// Generate simple uuid-like strings without full uuid crate
fn uuid_like_id() -> String {
    use std::time::SystemTime;
    let since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0));
    format!("{}_{}", since_epoch.as_micros(), rand_number())
}

fn rand_number() -> u32 {
    // Basic LCG generator for temp files
    use std::cell::Cell;
    thread_local! {
        static SEED: Cell<u32> = Cell::new(12345);
    }
    SEED.with(|s| {
        let val = s.get().wrapping_mul(1103515245).wrapping_add(12345);
        s.set(val);
        val
    })
}

// Normalize outputs for comparisons (newlines, trailing spaces)
pub fn normalize_output(out: &str) -> String {
    out.replace("\r\n", "\n")
       .trim_end()
       .lines()
       .map(|l| l.trim_end())
       .collect::<Vec<&str>>()
       .join("\n")
}

pub fn auto_detect_python() -> Option<String> {
    let mut candidates = Vec::new();

    // 1. Try PATH commands
    candidates.push("python".to_string());
    candidates.push("python3".to_string());
    candidates.push("py".to_string());

    // 2. Try LOCALAPPDATA standard Python installer directories
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        let programs_path = std::path::Path::new(&local_app_data).join("Programs").join("Python");
        if programs_path.exists() {
            if let Ok(entries) = fs::read_dir(programs_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let exec_path = path.join("python.exe");
                        if exec_path.exists() {
                            if let Some(s) = exec_path.to_str() {
                                candidates.push(s.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. Try System Program Files directories
    for program_files_var in &["ProgramFiles", "ProgramFiles(x86)"] {
        if let Ok(prog_files) = std::env::var(program_files_var) {
            let py_dirs = vec![
                std::path::Path::new(&prog_files).to_path_buf(),
                std::path::Path::new(&prog_files).join("Python"),
            ];
            for parent in py_dirs {
                if parent.exists() {
                    if let Ok(entries) = fs::read_dir(parent) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if path.is_dir() {
                                let name = path.file_name().unwrap_or_default().to_string_lossy();
                                if name.starts_with("Python") {
                                    let exec_path = path.join("python.exe");
                                    if exec_path.exists() {
                                        if let Some(s) = exec_path.to_str() {
                                            candidates.push(s.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Check each candidate and return the first one that runs successfully
    for candidate in candidates {
        if verify_python_executable(&candidate) {
            return Some(candidate);
        }
    }

    None
}

fn verify_python_executable(exec: &str) -> bool {
    let mut cmd = Command::new(exec);
    cmd.arg("-c")
       .arg("print('OK')")
       .stdout(Stdio::piped())
       .stderr(Stdio::piped());

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }

    let child_res = cmd.spawn();

    if let Ok(mut child) = child_res {
        let limit = Duration::from_secs(1);
        let start = Instant::now();
        loop {
            match child.try_wait() {
                Ok(Some(status)) => {
                    if status.success() {
                        if let Ok(output) = child.wait_with_output() {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            if stdout.trim() == "OK" {
                                return true;
                            }
                        }
                    }
                    return false;
                }
                Ok(None) => {
                    if start.elapsed() >= limit {
                        let _ = child.kill();
                        return false;
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
                Err(_) => return false,
            }
        }
    }
    false
}
