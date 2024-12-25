use core::{cell::RefCell, sync::atomic::{AtomicU32, Ordering}};

use cortex_m::interrupt::{free, Mutex};
use fugit::Instant;
use microbit::{
    hal::{rtc::RtcInterrupt, Rtc},
    pac::{interrupt, NVIC, RTC0},
};
use rtt_target::rprintln;

use crate::TICKER;


type TickInstant = Instant<u64, 1, 32768>;
// type TickDuration = Duration<u64, 1, 32768>;

// Use a Timer add on

pub struct Ticker {
    pub ovr_count: AtomicU32,
    pub rtc: Mutex<RefCell<Option<Rtc<RTC0>>>>,
}


impl Ticker {
    pub fn init(rtc0: RTC0, nvic: &mut NVIC) {
        let mut rtc = Rtc::new(rtc0, 0).unwrap();
        rtc.enable_counter();
        #[cfg(feature = "trigger-overflow")]
        {
            rtc.trigger_overflow();
            while rtc.get_counter() == 0 {}
        }
        rtc.enable_event(RtcInterrupt::Overflow);
        rtc.enable_interrupt(RtcInterrupt::Overflow, Some(nvic));
        free(|cs| {
            *TICKER.rtc.borrow(cs).borrow_mut() = Some(rtc);
            // unsafe {
            //     pac::NVIC::unmask(pac::interrupt::RTC0);
            // }
            // pac::NVIC::unpend(pac::interrupt::RTC0);
        })
    }
    pub fn now() -> TickInstant {
        let ticks = {
            loop {
                let ovr_before = TICKER.ovr_count.load(Ordering::SeqCst);
                let mut counter_value = 0;
                free(|cs| {
                let ref_count =  TICKER.rtc.borrow(cs).borrow_mut();
                    if let Some(counter) = ref_count.as_ref() {
                        counter_value = counter.get_counter();
                    }
                });
                let ovr = TICKER.ovr_count.load(Ordering::SeqCst);
                if ovr == ovr_before {
                    break ((ovr << 24) | counter_value) as u64;
                }
            }
        };
        TickInstant::from_ticks(ticks)
    }
}

#[interrupt]
fn RTC0() {
    free(|cs| {
        let rtc_ref = TICKER.rtc.borrow(cs).borrow_mut();
        rprintln!("Overflow count {}");
        if let Some(rtc) = rtc_ref.as_ref() {
            if rtc.is_event_triggered(RtcInterrupt::Overflow) {
                rtc.reset_event(RtcInterrupt::Overflow);
                let value = TICKER.ovr_count.fetch_add(1, Ordering::Relaxed);
                rprintln!("Overflow count {}", value);
            }
            // Clearing the event flag can take up to 4 clock cycles:
            // (see nRF52833 Product Specification section 6.1.8)
            // this should do that...
            let _ = rtc.is_event_triggered(RtcInterrupt::Overflow);
        }
    })
}
