
#![no_std]
#![no_main]

// use core::ptr::write_volatile;

use cortex_m::{self as _, asm::nop};
use cortex_m_rt::entry;
use embedded_hal::digital::{OutputPin, PinState};
use nrf52833_hal::{self as hal, gpio::Level};
use hal::pac;
use panic_halt as _;
// use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let _col1 = port0.p0_28.into_push_pull_output(Level::Low);
    let mut row1 = port0.p0_21.into_push_pull_output(Level::Low);
    let mut is_on = false;
    loop {
        let _ = row1.set_state(PinState::from(is_on));
        for _ in 0..4_000 {
            nop();
        }
        is_on = !is_on;
    }
}

