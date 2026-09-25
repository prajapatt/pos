#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessInfo {
	pub name: String,
	pub identifier: String,
}

pub fn list() -> Result<Vec<ProcessInfo>, String> {
	#[cfg(windows)]
	{
		return list_windows();
	}
	#[cfg(unix)]
	{
		return list_procfs();
	}
	#[allow(unreachable_code)]
	Err("process enumeration is unsupported on this target".into())
}

#[cfg(windows)]
fn list_windows() -> Result<Vec<ProcessInfo>, String> {
	use std::process::Command;

	let output = Command::new("tasklist")
		.args(["/FO", "CSV", "/NH"])
		.output()
		.map_err(|error| format!("tasklist failed to start: {error}"))?;
	if !output.status.success() {
		return Err(format!("tasklist exited with {}", output.status));
	}
	let text = String::from_utf8_lossy(&output.stdout);
	Ok(text
		.lines()
		.filter_map(parse_tasklist_line)
		.collect())
}

#[cfg(windows)]
fn parse_tasklist_line(line: &str) -> Option<ProcessInfo> {
	let mut fields = Vec::new();
	let mut field_start = 0;
	let mut quoted = false;
	for (index, character) in line.char_indices() {
		match character {
			'"' => quoted = !quoted,
			',' if !quoted => {
				fields.push(&line[field_start..index]);
				field_start = index + character.len_utf8();
			}
			_ => {}
		}
	}
	fields.push(&line[field_start..]);
	if fields.len() < 2 {
		return None;
	}
	Some(ProcessInfo {
		name: fields[0].trim_matches('"').to_string(),
		identifier: fields[1].trim_matches('"').to_string(),
	})
}

#[cfg(unix)]
fn list_procfs() -> Result<Vec<ProcessInfo>, String> {
	let entries = std::fs::read_dir("/proc").map_err(|error| format!("/proc unavailable: {error}"))?;
	let mut processes = Vec::new();
	for entry in entries {
		let entry = entry.map_err(|error| format!("/proc entry unavailable: {error}"))?;
		let name = entry.file_name().to_string_lossy().to_string();
		if name.chars().all(|character| character.is_ascii_digit()) {
			let pid = name;
			let comm_path = entry.path().join("comm");
			let process_name = std::fs::read_to_string(comm_path)
				.unwrap_or_else(|_| "unknown".into())
				.trim()
				.to_string();
			processes.push(ProcessInfo {
				name: process_name,
				identifier: pid,
			});
		}
	}
	Ok(processes)
}

pub fn format(processes: &[ProcessInfo]) -> String {
	processes
		.iter()
		.map(|process| format!("{}:{}", process.identifier, process.name))
		.collect::<Vec<_>>()
		.join(" ")
}
