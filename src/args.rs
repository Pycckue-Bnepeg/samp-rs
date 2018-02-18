use types::Cell;

pub struct Parser {
    params: *mut Cell,
    index: isize,
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
        
        unsafe {
            self.params.offset(self.index)
        }
    }
}