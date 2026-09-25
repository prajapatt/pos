use std::env;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let mut arguments = env::args().skip(1);
    let Some(program) = arguments.next() else {
        eprintln!("usage: chut-terminal <program> [arguments...]");
        return ExitCode::from(2);
    };
    let status = match Command::new(&program).args(arguments).status() {
        Ok(status) => status,
        Err(error) => {
            eprintln!("chut-terminal: failed to start {program}: {error}");
            return ExitCode::from(1);
        }
    };
    ExitCode::from(status.code().unwrap_or(1).try_into().unwrap_or(1))
}
