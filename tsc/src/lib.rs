#![no_std]

pub fn start() -> u64 {
    unsafe {
        // This memory fence guarantees that previous instructions have finished.
        let _ = core::arch::x86_64::__cpuid(0);
        // The timestamp counter increments every clock cycle.
        core::arch::x86_64::_rdtsc()
    }
}

pub fn stop() -> u64 {
    unsafe {
        let mut core: u32 = 0;
        // Like rtdsc but it waits for all previous instructions to finish
        let r = core::arch::x86_64::__rdtscp(&mut core as *mut _) as u64;
        // This memory fence guarantees that previous instructions have finished.
        let _ = core::arch::x86_64::__cpuid(0);
        r
    }
}
