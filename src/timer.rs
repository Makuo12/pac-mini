// use microbit::{hal::Rtc, pac::RTC0};
// use fugit::{Instant, Duration};

// type TickInstant = Instant<u64, 1, 32768>;
// type TickDuration = Duration<u64, 1, 32768>;


// // The Timer type it what our Task would use schedule events
// pub struct Timer<'a> {
//     end_time: TickInstant,
//     ticker: &'a Ticker,
// }

// impl<'a> Timer<'a> {
//     pub fn new(duration: TickDuration, ticker: &'a Ticker) -> Self {
        
//         Self {
//             end_time: ticker.now() + duration, 
//             ticker
//         }
//     }

//     // Lastly we add a method to check to see if the timer is ready
//     pub fn is_ready(&self) -> bool {
//         self.ticker.now() >= self.end_time
//     }
// }

// pub struct Ticker {
//     rtc: Rtc<RTC0>
// }

// impl Ticker {
//     pub fn new(rtc0: RTC0) -> Self {
//         // We leave the prescaler as 0 to keep its native freqency
//         let rtc = Rtc::new(rtc0, 0).unwrap();
//         // rtc as a register that increments on each cycle to the low power oscallator
//         // Enable/start the Real Time Counter.
//         rtc.enable_counter(); // 
//         Self {rtc} 
//     }
//     pub fn now(&self) -> TickInstant {
//         return TickInstant::from_ticks(self.rtc.get_counter() as u64);
//     }
// }