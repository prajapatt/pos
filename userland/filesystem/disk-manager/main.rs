use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
}

fn scan_devices(root: &str) -> Vec<DiskInfo> {
    let mut devices = Vec::new();

    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy().to_string();
            if name.starts_with("sd") || name.starts_with("nvme") || name.starts_with("vd") {
                let size = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                devices.push(DiskInfo {
                    name: name.clone(),
                    path: path.to_string_lossy().to_string(),
                    size_bytes: size,
                });
            }
        }
    }

    devices
}

fn main() -> ExitCode {
    let root = std::env::args().nth(1).unwrap_or_else(|| "/dev".to_string());
    let devices = scan_devices(&root);

    if devices.is_empty() {
        eprintln!("disk-manager: no block devices found under {root}");
        return ExitCode::from(1);
    }

    for disk in devices {
        println!("{}\t{}\t{} bytes", disk.name, disk.path, disk.size_bytes);
    }

    ExitCode::SUCCESS
}
