use core::arch::asm;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CpuFeatures { pub has_cpuid: bool, pub has_long_mode: bool, pub has_sse2: bool }

#[cfg(target_arch = "x86_64")]
pub fn features() -> CpuFeatures {
	let (max_extended, _) = unsafe { cpuid(0x8000_0000) };
	let (_, _, _, basic_edx) = unsafe { cpuid(1) };
	let (_, _, _, extended_edx) = unsafe { cpuid(0x8000_0001) };
	detect_cpu_features(max_extended, basic_edx, extended_edx)
}

#[cfg(target_arch = "x86_64")]
pub(crate) fn detect_cpu_features(max_extended: u32, basic_edx: u32, extended_edx: u32) -> CpuFeatures {
	let has_long_mode = max_extended >= 0x8000_0001 && (extended_edx & (1 << 29)) != 0;
	let has_sse2 = (basic_edx & (1 << 26)) != 0;
	CpuFeatures { has_cpuid: true, has_long_mode, has_sse2 }
}

#[cfg(not(target_arch = "x86_64"))]
pub const fn features() -> CpuFeatures { CpuFeatures { has_cpuid: false, has_long_mode: false, has_sse2: false } }

#[cfg(target_arch = "x86_64")]
unsafe fn cpuid(leaf: u32) -> (u32, u32, u32, u32) {
	let mut eax = leaf; let mut ebx: u32; let mut ecx = 0; let mut edx: u32;
	asm!("cpuid", inout("eax") eax, lateout("ebx") ebx, inout("ecx") ecx, lateout("edx") edx, options(nomem, nostack));
	(eax, ebx, ecx, edx)
}

#[cfg(test)]
mod tests {
	use super::detect_cpu_features;

	#[test]
	fn detects_long_mode_and_sse2_from_cpuid_bits() {
		let features = detect_cpu_features(0x8000_0001, 1 << 26, 1 << 29);
		assert!(features.has_cpuid);
		assert!(features.has_long_mode);
		assert!(features.has_sse2);
	}

	#[test]
	fn rejects_missing_long_mode_and_sse2() {
		let features = detect_cpu_features(0x8000_0000, 0, 0);
		assert!(features.has_cpuid);
		assert!(!features.has_long_mode);
		assert!(!features.has_sse2);
	}
}
