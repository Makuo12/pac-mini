
#![no_std]
#![no_main]

// use core::ptr::write_volatile;

use core::cell::RefCell;

use cortex_m::{self as _, interrupt::Mutex, interrupt::free};
use cortex_m_rt::entry;
use microbit::pac::interrupt;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use microbit::hal::gpiote::Gpiote;
use microbit::{
    board::Buttons,
    pac::{self, GPIOTE}
};
use microbit::board::Board;
use microbit::hal::timer::Timer;
use panic_halt as _;
use rtt_target::rprintln;
// use rtt_target::{rprintln, rtt_init_print};

static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));
static STATE: Mutex<RefCell<Option<bool>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    // To get all five rows and five columns
    let (mut col, mut row) = board.display_pins.degrade();
    // Because we would only be using row1 we would set it high
    row[0].set_high().ok();
    // Now `button` is a generic GPIO pin without button-specific functionality.
    init_buttons(board.GPIOTE, board.buttons);
    let active_col = 0;
    col[active_col].set_low().ok();
    loop {
        if toggle_button() {
            col[active_col].toggle().ok();
        }
        row[0].toggle().ok();
        timer.delay_ms(300);
    }
}

fn toggle_button() -> bool {
    free(|cs| {
        let mut state = false;
        if let Some(result) = STATE.borrow(cs).borrow_mut().take() {
            if result {
                state = true;
            }
        }
        return state;
    })
}

fn init_buttons(board_gpiote: GPIOTE, buttons: Buttons) {
    let gpiote = Gpiote::new(board_gpiote);
    let channel0 = gpiote.channel0();
    channel0
    .input_pin(&buttons.button_a.degrade())
    .hi_to_lo()
    .enable_interrupt();
    channel0.reset_events();
    let channel1 = gpiote.channel1();

    channel1
    .input_pin(&buttons.button_b.degrade())
    .hi_to_lo()
    .enable_interrupt();

    channel1.reset_events();
    free(move |cs| {
            *GPIO.borrow(cs).borrow_mut() = Some(gpiote);
            unsafe {
                pac::NVIC::unmask(pac::interrupt::GPIOTE);
            }
            pac::NVIC::unpend(pac::interrupt::GPIOTE);
        }

    )
}


#[interrupt]
fn GPIOTE() {
    free(|cs| {
        if let Some(gpiote) = GPIO.borrow(cs).borrow_mut().as_ref() {
            let chan_one = gpiote.channel0().is_event_triggered();
            rprintln!("State came through 1");
            let chan_two = gpiote.channel1().is_event_triggered();
            if chan_one {
                *STATE.borrow(cs).borrow_mut() = Some(true);
            };
            if chan_two {
                *STATE.borrow(cs).borrow_mut() = Some(true);
            };
            gpiote.channel0().reset_events();
            gpiote.channel1().reset_events(); 
            gpiote.reset_events();
        }
    })

}