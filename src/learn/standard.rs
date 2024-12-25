
#![no_std]
#![no_main]

use core::ptr::write_volatile;

use cortex_m::{self as _, asm::nop};
use cortex_m_rt::entry;
use panic_halt as _;
// use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    // the * is used because it is a raw pointer
    const GPIO_PIN_CNF_21_ROW1_ADDR: *mut u32 = 0x5000_0754 as *mut u32;
    const GPIO_PIN_CNF_28_COL1_ADRR: *mut u32 = 0x5000_0770 as *mut u32;
    // Identify the bit field we are using
    const DIR_OUTPUT_POS: u32= 0;
    const PINCNF_DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS;
    unsafe {
        // *GPIO_PIN_CNF_21_ROW1_ADDR = PINCNF_DRIVE_LED;
        // *GPIO_PIN_CNF_28_COL1_ADRR = PINCNF_DRIVE_LED;
        // To prevent the compiler from optimizing the writes above
        write_volatile(GPIO_PIN_CNF_21_ROW1_ADDR, PINCNF_DRIVE_LED);
        write_volatile(GPIO_PIN_CNF_28_COL1_ADRR, PINCNF_DRIVE_LED);
    }
    // We then need to write to the output register
    const GPIO_OUT_REG: *mut u32 = 0x5000_0504 as *mut u32;
    
    const GPIO_OUT_ROW1_POS: u32 = 21;
    let mut is_on = false;
    loop {
        unsafe  {
            write_volatile(GPIO_OUT_REG, (is_on as u32) << GPIO_OUT_ROW1_POS);
        }
        for _ in 0..400_000 {
            nop();
        }
        is_on = !is_on;
    }
}


