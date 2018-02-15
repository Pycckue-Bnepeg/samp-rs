use std::mem::size_of;
use types::Cell;

pub struct Parser {
    params: *mut Cell,
    index: usize,
}

impl Parser {
    pub fn new(params: *mut Cell) -> Parser {
        Parser {
            params,
            index: 0,
        }
    }

    pub fn next(&mut self) -> *mut Cell {
        self.index += 1;

        (self.params as usize + self.index * size_of::<usize>()) as *mut Cell
    }
}