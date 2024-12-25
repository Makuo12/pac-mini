#![no_std]
#![no_main]

use cortex_m::interrupt::Mutex;
use microbit::display::blocking::Display;
// use core::ptr::write_volatile;
// use microbit::pac::interrupt;
// use random::Xorshift64;
use time::Ticker;
// use ::time::Time;
use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use cortex_m_rt::entry;
use handler::{get_position, update_position};
use initial::{init_ball, init_buttons, init_timer1};
use microbit::hal::gpiote::Gpiote;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use panic_halt as _;
use rtt_target::rtt_init_print;
use state::Ball;
use microbit::
    pac::TIMER1

;
// use rtt_target::{rprintln, rtt_init_print};
mod state;
mod initial;
mod handler;
mod random;
mod time;



pub static TICKER: Ticker = Ticker {
    ovr_count: AtomicU32::new(0),
    rtc: Mutex::new(RefCell::new(None))
};

static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));
static TIMER: Mutex<RefCell<Option<Timer<TIMER1>>>> = Mutex::new(RefCell::new(None));
static BALL: Mutex<RefCell<Option<Ball>>> = Mutex::new(RefCell::new(None));
static PRICE_COL: AtomicUsize = AtomicUsize::new(6);
static PRICE_ROW: AtomicUsize = AtomicUsize::new(6);
// static RTCTIMER: Mutex<RefCell<Option<RtcTimer>>>
// COL AND ROW helps use know the current coland row 
static COL: AtomicUsize = AtomicUsize::new(0);
static ROW: AtomicUsize = AtomicUsize::new(0);
// static TIME: AtomicU32 = AtomicU32::new(0);



#[entry]
fn main() -> ! {
    rtt_init_print!();
    if let Some(mut board) = Board::take() {
        Ticker::init(board.RTC0, &mut board.NVIC);
        let mut timer = Timer::new(board.TIMER0);
        let mut display = Display::new(board.display_pins);
        init_buttons(board.GPIOTE, board.buttons);
        init_ball(5, 5);
        init_timer1(board.TIMER1);
        let mut m: [[u8;5];5] = [[0;5];5]; 
        let mut previous_price_col = 6;
        let mut previous_price_row = 6;
        
        loop {
            let position = get_position();
            COL.store(position.0, Ordering::SeqCst);
            ROW.store(position.1, Ordering::SeqCst);
            let price_row = PRICE_ROW.load(Ordering::SeqCst);
            let price_col = PRICE_COL.load(Ordering::SeqCst);
            m[position.1][position.0] = 1;
            if price_col == 6 {
                // We either clear it or do nothing
                if previous_price_col != 6 {
                    m[previous_price_row][previous_price_col] = 0;
                    previous_price_col = 6;
                    previous_price_row = 6;
                }
            } else {
                m[price_row][price_col] = 1;
                previous_price_col = price_col;
                previous_price_row = price_row;
            }
            display.show(&mut timer, m, 100);
            m[position.1][position.0] = 0;
            display.show(&mut timer, m, 100);
            update_position();     
            // Check if 5 seconds have elapsed
        }
    }
    loop {}
}

