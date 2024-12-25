use cortex_m::interrupt::free;
use microbit::{
    board::Buttons,
    pac::{self, GPIOTE, TIMER1}
};
use microbit::hal::timer::Timer;
use microbit::hal::gpiote::Gpiote;
use crate::{state::Ball, BALL, GPIO, TIMER};


pub fn init_ball(max_row_len: usize, max_col_len: usize) {
    free(|cs| {
        *BALL.borrow(cs).borrow_mut() = Some(Ball::new(max_row_len, max_col_len));
    })
}


pub fn init_timer1(board_timer: TIMER1) {
    let mut timer = Timer::new(board_timer);
    timer.start(10_000_000u32);
    timer.enable_interrupt();
    timer.reset_event();
    free(move |cs| {
        *TIMER.borrow(cs).borrow_mut() = Some(timer);
        unsafe {
            pac::NVIC::unmask(pac::interrupt::TIMER1);
        }
        pac::NVIC::unpend(pac::interrupt::TIMER1);            
    })

}
pub fn init_buttons(board_gpiote: GPIOTE, buttons: Buttons) {
    let gpiote = Gpiote::new(board_gpiote);
    let chan0 = gpiote.channel0();

    chan0
    .input_pin(&buttons.button_a.degrade())
    .hi_to_lo()
    .enable_interrupt();

    chan0.reset_events();

    
    let chan1 = gpiote.channel1();

    chan1
    .input_pin(&buttons.button_b.degrade())
    .hi_to_lo()
    .enable_interrupt();

    chan1.reset_events();

    free(move |cs| {
        *GPIO.borrow(cs).borrow_mut() = Some(gpiote);

        unsafe {
            pac::NVIC::unmask(pac::interrupt::GPIOTE);
        }
        pac::NVIC::unpend(pac::interrupt::GPIOTE);            
    })

}