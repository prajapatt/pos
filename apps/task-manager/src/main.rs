use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    #[cfg(windows)]
    let result = Command::new("tasklist").args(["/FO", "TABLE"]).status();
    #[cfg(unix)]
    let result = Command::new("ps").args(["-eo", "pid,comm,%cpu,%mem"]).status();
    match result {
        Ok(status) if status.success() => ExitCode::SUCCESS,
        Ok(status) => { eprintln!("process query exited with {status}"); ExitCode::from(1) }
        Err(error) => { eprintln!("process query failed: {error}"); ExitCode::from(1) }
    }
}
