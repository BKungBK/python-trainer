use std::fs;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

const MAX_OUTPUT_BYTES: usize = 1024 * 1024;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JudgeResult {
    pub success: bool,
    pub output: String,
    pub error: String,
    pub verdict: String, // "Passed", "Wrong Answer", "Runtime Error", "Time Limit Exceeded"
    pub duration_ms: u64,
}

fn read_capped<R: Read>(mut reader: R) -> std::io::Result<(Vec<u8>, bool)> {
    let mut output = Vec::new();
    let mut buffer = [0_u8; 8192];
    let mut truncated = false;

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let remaining = MAX_OUTPUT_BYTES.saturating_sub(output.len());
        let bytes_to_keep = bytes_read.min(remaining);
        output.extend_from_slice(&buffer[..bytes_to_keep]);
        truncated |= bytes_to_keep < bytes_read;
    }

    Ok((output, truncated))
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

    // Drain both pipes concurrently so a program with large output cannot block
    // while the parent waits for its process to finish.
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let stdout_reader =
        std::thread::spawn(move || stdout.map_or(Ok((Vec::new(), false)), read_capped));
    let stderr_reader =
        std::thread::spawn(move || stderr.map_or(Ok((Vec::new(), false)), read_capped));

    // Wait for the process to finish with a timeout
    let (exit_status, duration) = match wait_timeout(&mut child, timeout_duration) {
        Ok(Some((status, duration))) => (status, duration),
        Ok(None) => {
            let _ = child.kill();
            let _ = child.wait();
            let _ = stdout_reader.join();
            let _ = stderr_reader.join();
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
            let _ = child.wait();
            let _ = stdout_reader.join();
            let _ = stderr_reader.join();
            let _ = fs::remove_file(&temp_file_path);
            return JudgeResult {
                success: false,
                output: String::new(),
                error: format!("Failed while waiting for Python process: {}", e),
                verdict: "Runtime Error".to_string(),
                duration_ms: start_time.elapsed().as_millis() as u64,
            };
        }
    };

    let _ = child.wait();
    let _ = fs::remove_file(&temp_file_path);

    let duration_ms = duration.as_millis() as u64;

    let (stdout_bytes, stdout_truncated) = match stdout_reader.join() {
        Ok(Ok(result)) => result,
        Ok(Err(e)) => {
            return JudgeResult {
                success: false,
                output: String::new(),
                error: format!("Failed to read Python stdout: {}", e),
                verdict: "Runtime Error".to_string(),
                duration_ms,
            }
        }
        Err(_) => {
            return JudgeResult {
                success: false,
                output: String::new(),
                error: "Python stdout reader thread panicked".to_string(),
                verdict: "Runtime Error".to_string(),
                duration_ms,
            }
        }
    };
    let (stderr_bytes, stderr_truncated) = match stderr_reader.join() {
        Ok(Ok(result)) => result,
        Ok(Err(e)) => {
            return JudgeResult {
                success: false,
                output: String::from_utf8_lossy(&stdout_bytes).to_string(),
                error: format!("Failed to read Python stderr: {}", e),
                verdict: "Runtime Error".to_string(),
                duration_ms,
            }
        }
        Err(_) => {
            return JudgeResult {
                success: false,
                output: String::from_utf8_lossy(&stdout_bytes).to_string(),
                error: "Python stderr reader thread panicked".to_string(),
                verdict: "Runtime Error".to_string(),
                duration_ms,
            }
        }
    };

    let stdout = String::from_utf8_lossy(&stdout_bytes).to_string();
    let stderr = String::from_utf8_lossy(&stderr_bytes).to_string();

    if stdout_truncated || stderr_truncated {
        return JudgeResult {
            success: false,
            output: stdout,
            error: format!(
                "Python output exceeded the {} byte limit{}",
                MAX_OUTPUT_BYTES,
                if stderr.is_empty() {
                    String::new()
                } else {
                    format!("\n{}", stderr)
                }
            ),
            verdict: "Runtime Error".to_string(),
            duration_ms,
        };
    }

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
        static SEED: Cell<u32> = const { Cell::new(12345) };
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
        let programs_path = std::path::Path::new(&local_app_data)
            .join("Programs")
            .join("Python");
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
    candidates
        .into_iter()
        .find(|candidate| verify_python_executable(candidate))
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn normalize_output_should_ignore_line_endings_and_trailing_whitespace() {
        assert_eq!(normalize_output("a  \r\nb\t\r\n\r\n"), "a\nb");
    }

    #[test]
    fn read_capped_should_drain_but_only_retain_the_output_limit() {
        let input = vec![b'x'; MAX_OUTPUT_BYTES + 1];
        let (output, truncated) = read_capped(Cursor::new(input)).expect("reader should work");

        assert_eq!(output.len(), MAX_OUTPUT_BYTES);
        assert!(truncated);
    }
}
