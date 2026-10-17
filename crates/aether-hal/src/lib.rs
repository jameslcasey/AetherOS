#![no_std]

pub fn early_init() {
    // Architecture-specific serial, GDT/IDT or VBAR, etc.
    // Prototype: no-op that future phases will replace.
}

pub mod mm {
    pub fn init_physical_allocator(_entries: &[&'static [u8]], _offset: u64) {
        // Phase 2: bitmap + buddy
    }
}
