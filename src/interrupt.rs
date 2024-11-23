use core::arch::asm;
use bw_r_drivers_tc37x::scu::wdt_call::call_without_endinit;

use tc375_pac as pac;

extern "C"{
    static __INTERRUPT_TABLE : u8;
}

pub fn load_interrupt_table() {
    call_without_endinit(|| unsafe {
        let interrupt_table = &__INTERRUPT_TABLE as *const u8 as u32;
        asm!("mtcr	$biv, {0}", in(reg32) interrupt_table);
        asm!("isync");
    });
}

pub fn enable(){
    unsafe {
        asm!("enable");
    }
}
