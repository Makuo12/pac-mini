
#![no_std]
#![no_main]

// use core::ptr::write_volatile;

use cortex_m::{self as _, asm::nop};
use cortex_m_rt::entry;
use nrf52833_pac::Peripherals;
use panic_halt as _;
// use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    p.P0.pin_cnf[21].write(|w| w.dir().output());
    p.P0.pin_cnf[28].write(|w| w.dir().output());
    let mut is_on = false;
    loop {
        p.P0.out.write(|w| w.pin21().bit(is_on));
        for _ in 0..4_000 {
            nop();
        }
        is_on = !is_on;
    }
}
