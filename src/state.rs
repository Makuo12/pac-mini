use core::sync::atomic::Ordering;

use rtt_target::rprintln;

use crate::{PRICE_COL, PRICE_ROW};


pub enum Direction {
    Left,
    Right,
    Down,
    Top
}

pub struct Ball {
    pub row: usize,
    pub column: usize,
    pub max_row_len: usize,
    pub max_col_len: usize,
    pub score: u32,
    // update_col and update_row is only used when
    // we updated the column and row in the update_direction
    // we do this so that only update_position can update the leds
    pub update_col: usize,
    pub update_row: usize,
    pub direction: Direction
}

impl Ball {
    pub fn new(max_row_len: usize, max_col_len: usize) -> Ball {
        return Ball { row: 0, column: 0, max_row_len, max_col_len, score: 0, update_col: 0, update_row: 0, direction: Direction::Right }
    } 
    pub fn update_position(&mut self) {
        self.direction.update_position(&mut self.row, &mut self.column, &self.max_row_len,
            &self.max_col_len, &mut self.update_row, &mut self.update_col);
    }
    pub fn update_score(&mut self) {
        let price_col = PRICE_COL.load(Ordering::SeqCst);
        let price_row = PRICE_ROW.load(Ordering::SeqCst);
        if price_col == self.column && price_row == self.row {

            self.score += 1;
            PRICE_COL.store(6, Ordering::SeqCst);
            PRICE_ROW.store(6, Ordering::SeqCst);
            rprintln!("score!!!")
        }
    }
    pub fn update_direction(&mut self, a: bool, b: bool) {
        if a {
            let (d, col, row) = self.direction.button_a(self);
            self.direction = d;
            self.update_col = col;
            self.update_row = row;
        }
        if b {
            let (d, col, row) = self.direction.button_b(self);
            self.direction = d;
            self.update_col = col;
            self.update_row = row;
        }
    }
    // pub fn tell_direction(&self) {
    //     match self.direction {
    //         Direction::Down => rprintln!("down"),
    //         Direction::Top => rprintln!("top"),
    //         Direction::Left => rprintln!("left"),
    //         Direction::Right => rprintln!("right")
    //     }
    // }

}

impl Direction {
    /// Gets the new direction
    /// Actioned by a button in micro bit v2
    /// # Parameters
    /// - `self`: Current Direction
    /// - `ball`: a reference of the ball object
    /// # Returns
    /// Direction 
    /// updated column: usize
    /// update row: usize
    /// format (Direction, column, row)
    pub fn button_a(&self, ball: &Ball) -> (Direction, usize, usize) {
        let direction: Direction;
        let mut column = 0;
        let mut row = 0;
        match self {
            Direction::Down => {
                if ball.column == 0 {
                    column = ball.max_col_len - 1
                }
                direction = Direction::Left;
            },
            Direction::Top => {
                if ball.column == 0 {
                    column = ball.max_col_len - 1
                }
                direction = Direction::Left;
            },
            Direction::Left => {
                if ball.row == 0 {
                    row = ball.max_row_len -1
                }
                direction = Direction::Top;
            },
            Direction::Right => {
                if ball.row == 0 {
                    row = ball.max_row_len -1
                }
                direction = Direction::Top;
            },
        }
        return (direction, column, row)
    }
    /// Gets the new direction
    /// Actioned by b button in micro bit v2
    /// # Parameters
    /// - `self`: Current Direction
    /// - `ball`: a reference of the ball object
    /// # Returns
    /// Direction 
    /// updated column: usize
    /// update row: usize
    /// format (Direction, column, row)
    pub fn button_b(&self, ball: &Ball) -> (Direction, usize, usize) {
        let direction: Direction;
        let mut column = 0;
        let mut row = 0;
        match self {
            Direction::Down => {
                if ball.column == ball.max_col_len - 1{
                    column = 0
                }
                direction = Direction::Right;
            },
            Direction::Top => {
                if ball.column == ball.max_col_len - 1{
                    column = 0
                }
                direction = Direction::Right;
            },
            Direction::Left => {
                if ball.row == ball.max_row_len - 1{
                    row = 0;
                }
                direction = Direction::Down;
            },
            Direction::Right => {
                if ball.row == ball.max_row_len - 1{
                    row = 0;
                }
                direction = Direction::Down;
            },
        }
        return (direction, column, row)
    }
    // Base on the position we would know how to 
    // Increment or decrement the row
    fn update_position(&self, row: &mut usize, column: &mut usize, max_row: &usize,
        max_col: &usize, update_row: &mut usize, update_col: &mut usize) {
        if *update_col > 0 {
            *column = *update_col;
            *update_col = 0;
        }
        if *update_row > 0 {
            *row = *update_row;
            *update_row = 0;
        }
        match self {
            Direction::Down => {
                if *row < *max_row - 1 {
                    *row = *row+1;
                } else {
                    *row = 0
                }
            },
            Direction::Top => {
                if *row > 0 {
                    *row = *row-1;
                } else {
                    *row = *max_row - 1
                }
            },
            Direction::Left => {
                if *column > 0 {
                    *column = *column - 1;
                } else {
                    *column = *max_col - 1;
                }
            },
            Direction::Right => {
                if *column < *max_col - 1 {
                    *column = *column + 1
                } else {
                    *column = 0
                }
            }
        }
    }
}