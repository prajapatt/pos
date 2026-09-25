use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let root = env::args().nth(1).unwrap_or_else(|| "apps".into());
    let entries = match fs::read_dir(&root) {
        Ok(entries) => entries,
        Err(error) => { eprintln!("chut-app-store: {root}: {error}"); return ExitCode::from(1); }
    };
    for entry in entries.flatten() {
        let manifest = entry.path().join("Cargo.toml");
        if Path::new(&manifest).is_file() {
            let name = entry.file_name().to_string_lossy().into_owned();
            let manifest_text = fs::read_to_string(manifest).unwrap_or_default();
            let version = manifest_text.lines().find_map(|line| line.strip_prefix("version = ")).unwrap_or("unknown");
            println!("{name} {version}");
        }
    }
    ExitCode::SUCCESS
}
