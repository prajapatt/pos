use core::arch::asm;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalApic {
    pub base: u64,
    pub enabled: bool,
}

impl LocalApic {
    pub const fn new() -> Self {
        Self { base: 0xFEE0_0000, enabled: false }
    }

    pub fn enable(&mut self) {
        let mut eax: u64;
        let mut edx: u64;
        unsafe {
            asm!(
                "mov {0:e}, eax",
                out(reg) eax,
                options(nomem, nostack, preserves_flags)
            );
            asm!(
                "mov {0:e}, edx",
                out(reg) edx,
                options(nomem, nostack, preserves_flags)
            );
        }
        let msr = 0x1B;
        let old = self.read_msr(msr);
        self.enabled = true;
        self.write_msr(msr, old | (1 << 11));
    }

    pub fn read_reg(&self, offset: u32) -> u32 {
        let ptr = (self.base + offset as u64) as *const u32;
        unsafe { core::ptr::read_volatile(ptr) }
    }

    pub fn write_reg(&self, offset: u32, value: u32) {
        let ptr = (self.base + offset as u64) as *mut u32;
        unsafe { core::ptr::write_volatile(ptr, value) }
    }

    fn read_msr(&self, msr: u32) -> u64 {
        let low: u32;
        let high: u32;
        unsafe {
            asm!("rdmsr", in("ecx") msr, out("eax") low, out("edx") high, options(nostack, preserves_flags));
        }
        ((high as u64) << 32) | low as u64
    }

    fn write_msr(&self, msr: u32, value: u64) {
        let low = value as u32;
        let high = (value >> 32) as u32;
        unsafe {
            asm!("wrmsr", in("ecx") msr, in("eax") low, in("edx") high, options(nostack, preserves_flags));
        }
    }
}

impl Default for LocalApic {
    fn default() -> Self {
        Self::new()
    }
}
