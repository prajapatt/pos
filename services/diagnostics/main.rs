mod cpu;
mod gpu;
mod logs;
mod memory;
mod storage;

use cpu::CpuDiagnostics;
use gpu::GpuDiagnostics;
use logs::LogCollector;
use memory::MemoryDiagnostics;
use storage::StorageDiagnostics;

#[derive(Clone, Debug, Default)]
pub struct DiagnosticsReport {
    pub cpu: CpuDiagnostics,
    pub gpu: GpuDiagnostics,
    pub memory: MemoryDiagnostics,
    pub storage: StorageDiagnostics,
    pub log_summary: String,
}

#[derive(Clone, Debug, Default)]
pub struct DiagnosticsRunner {
    cpu: CpuDiagnostics,
    gpu: GpuDiagnostics,
    memory: MemoryDiagnostics,
    storage: StorageDiagnostics,
    logs: LogCollector,
}

impl DiagnosticsRunner {
    pub fn run(&mut self) -> DiagnosticsReport {
        let cpu = self.cpu.sample();
        let gpu = self.gpu.sample();
        let memory = self.memory.sample();
        let storage = self.storage.sample();
        let log_summary = self.logs.collect_summary();
        DiagnosticsReport {
            cpu,
            gpu,
            memory,
            storage,
            log_summary,
        }
    }
}

fn main() {
    let mut runner = DiagnosticsRunner::default();
    let report = runner.run();
    println!("cpu load: {}%", report.cpu.load_percent);
}
