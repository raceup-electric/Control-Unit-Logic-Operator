use core::time::Duration;
/// Wait for a number of cycles roughly calculated from a duration.
#[inline(always)]
pub fn wait_nop(period: Duration) {
    let ns: u32 = period.as_nanos() as u32;
    let n_cycles = ns / 920;
    for _ in 0..n_cycles {
        // SAFETY: nop is always safe
        unsafe { core::arch::asm!("nop") };
    }
}
