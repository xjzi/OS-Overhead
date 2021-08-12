#![no_std]

pub fn get_timestamp() -> u64 {
    unsafe {
        // This memory fence guarantees that previous instructions have finished.
        let _ = core::arch::x86_64::__cpuid(0);
        // The timestamp counter increments every clock cycle.
        core::arch::x86_64::_rdtsc()
    }
}