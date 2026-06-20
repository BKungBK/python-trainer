use std::fs;

fn main() {
    // Attempt to load .env file from various locations at compile time
    let env_paths = [".env", "src-tauri/.env", "../.env"];
    let mut url = String::new();
    let mut key = String::new();

    for path in &env_paths {
        if let Ok(content) = fs::read_to_string(path) {
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
                break;
            }
        }
    }

    if !url.is_empty() {
        println!("cargo:rustc-env=SUPABASE_URL={}", url);
    }
    if !key.is_empty() {
        println!("cargo:rustc-env=SUPABASE_ANON_KEY={}", key);
    }

    tauri_build::build()
}
