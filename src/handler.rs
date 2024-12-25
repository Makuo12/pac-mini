use core::sync::atomic::Ordering;

use cortex_m::interrupt::free;
use microbit::pac::interrupt;
use rtt_target::rprintln;
// use rtt_target::rprintln;

use crate::{random::Xorshift64, time::Ticker, BALL, COL, GPIO, PRICE_COL, PRICE_ROW, ROW, TIMER};

pub fn get_position() -> (usize, usize) {
    free(|cs| {
        let mut column = 0;
        let mut row = 0;
        if let Some(ball) = BALL.borrow(cs).borrow().as_ref() {
            column = ball.column;
            row = ball.row;
        }
        return (column, row)
    })
}

pub fn update_position() {
    free(|cs| {
        let mut ball_ref = BALL.borrow(cs).borrow_mut();
        if let Some(ball) = ball_ref.as_mut() {
            ball.update_position();
            // after we update the ball position we'll try to check if it is a match with the fruit.
            ball.update_score();
        }
    })
}


#[interrupt]
fn GPIOTE() {
    free(|cs| {
        if let Some(gpiote) = GPIO.borrow(cs).borrow().as_ref() {
            let a = gpiote.channel0().is_event_triggered();
            let b = gpiote.channel1().is_event_triggered();
            let mut ball_ref = BALL.borrow(cs).borrow_mut();
            if let Some(ball) = ball_ref.as_mut() {
                ball.update_direction(a, b);
            }
            gpiote.channel0().reset_events();
            gpiote.channel1().reset_events();
            gpiote.reset_events();
        }
    })
}

#[interrupt]
fn TIMER1() {
    free(|cs| {
        let mut timer_ref = TIMER.borrow(cs).borrow_mut();
        if let Some(timer) = timer_ref.as_mut() {
            let current_col = PRICE_COL.load(Ordering::SeqCst);
            if current_col == 6 {
                rprintln!("{}", Ticker::now().ticks() / 32_768);
                let mut rand = Xorshift64::new(Ticker::now().ticks() / 32_768);
                let col_main = COL.load(Ordering::SeqCst);
                let row_main = ROW.load(Ordering::SeqCst);
                let (row, col) = rand.next_value(row_main, col_main);
                PRICE_COL.store(col, Ordering::SeqCst);
                PRICE_ROW.store(row, Ordering::SeqCst);
            } else {
                PRICE_COL.store(6, Ordering::SeqCst);
                PRICE_ROW.store(6, Ordering::SeqCst);
            } 
            timer.start(10_000_000u32);
            timer.reset_event();
        }
    })
}

