use super::{processes, system_info};

pub fn collect() -> Result<String, String> {
	let system = system_info::format(&system_info::collect()?);
	let process_list = processes::list()?;
	Ok(format!("{} process_count={}", system, process_list.len()))
}
