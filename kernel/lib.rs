#![no_std]

pub mod arch {
    pub mod x86_64 {
        pub mod apic;
        pub mod cpu;
        pub mod gdt;
        pub mod idt;
        pub mod paging;
        pub mod serial;
    }
}

pub mod core {
    pub mod cpu;
    pub mod interrupt;
    pub mod kernel;
    pub mod panic;
    pub mod scheduler;
    pub mod syscall;
    pub mod task;
    pub mod thread;
    pub mod timer;
}

pub mod process { pub mod process; pub mod thread; pub mod address_space; pub mod capabilities; pub mod elf_loader; }
pub mod ipc { pub mod channel; pub mod message; pub mod event; pub mod shared_memory; }
pub mod fs {
    pub mod directory;
    pub mod file;
    pub mod inode;
    pub mod mount;
    pub mod path;
    pub mod vfs;
}
pub mod interrupt { pub mod interrupt; pub mod exception; pub mod irq; }
pub mod security { pub mod capability; }
pub mod drivers { pub mod driver; }
pub mod memory {
    pub mod allocator;
    pub mod page;
    pub mod physical;
    pub mod r#virtual;
    pub mod heap;
}

mod entry;