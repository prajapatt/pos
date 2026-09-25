use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

fn settings_path() -> PathBuf {
    env::var_os("CHUT_OS_SETTINGS_FILE").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("chut-settings.conf"))
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let path = settings_path();
    match args.next().as_deref() {
        Some("get") => {
            let key = args.next().unwrap_or_default();
            let text = fs::read_to_string(&path).unwrap_or_default();
            for line in text.lines() {
                if line.strip_prefix(&format!("{key}=")).is_some() { println!("{line}"); return ExitCode::SUCCESS; }
            }
            eprintln!("setting not found: {key}"); ExitCode::from(1)
        }
        Some("set") => {
            let key = args.next().unwrap_or_default();
            let value = args.next().unwrap_or_default();
            if key.is_empty() || key.contains(['=', '\\', '\n']) { eprintln!("invalid setting key"); return ExitCode::from(2); }
            let mut lines: Vec<String> = fs::read_to_string(&path).unwrap_or_default().lines().map(str::to_owned).filter(|line| !line.starts_with(&format!("{key}=")).to_owned()).collect();
            lines.push(format!("{key}={value}"));
            if let Err(error) = fs::write(&path, lines.join("\n") + "\n") { eprintln!("settings write failed: {error}"); return ExitCode::from(1); }
            ExitCode::SUCCESS
        }
        _ => { eprintln!("usage: chut-settings get <key> | set <key> <value>"); ExitCode::from(2) }
    }
}
