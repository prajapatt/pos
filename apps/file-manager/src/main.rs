use std::env;
use std::fs;
use std::process::ExitCode;

fn main() -> ExitCode {
    let path = env::args().nth(1).unwrap_or_else(|| ".".into());
    let entries = match fs::read_dir(&path) {
        Ok(entries) => entries,
        Err(error) => {
            eprintln!("chut-file-manager: {path}: {error}");
            return ExitCode::from(1);
        }
    };
    let mut names = Vec::new();
    for entry in entries {
        match entry {
            Ok(entry) => names.push(entry.file_name().to_string_lossy().into_owned()),
            Err(error) => eprintln!("chut-file-manager: entry error: {error}"),
        }
    }
    names.sort_unstable();
    for name in names { println!("{name}"); }
    ExitCode::SUCCESS
}
