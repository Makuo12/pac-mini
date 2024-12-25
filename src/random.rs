use core::num::Wrapping;

pub struct Xorshift64 {
    state: Wrapping<u64>,
}

impl Xorshift64 {
    // Initialize with a non-zero seed
    pub fn new(seed: u64) -> Self {
        assert!(seed != 0, "Seed must be non-zero");
        Self {
            state: Wrapping(seed),
        }
    }
    pub fn next_value(&mut self, row_main: usize, col_main: usize) -> (usize, usize) {
        let row = self.next() as usize;
        let col = self.next() as usize;
        if row_main == row ||  col_main == col {
            self.next_value(row_main, col_main);
        }
        return (row, col);
    }
    // Generate the next random number
    fn next(&mut self) -> u8 {
        let mut x = self.state;
        x ^= x << 7;
        x ^= x >> 9;
        self.state = x;
        (x.0 % 5) as u8// Return the inner u64 value
        

    }
}