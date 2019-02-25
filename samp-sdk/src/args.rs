use crate::amx::Amx;
use crate::cell::AmxCell;

/// List of arguments of a native function.
pub struct Args<'a> {
    amx: &'a Amx,
    args: *const i32,
    offset: usize,
}

impl<'a> Args<'a> {
    pub fn new(amx: &'a Amx, args: *const i32) -> Args<'a> {
        Args {
            amx,
            args,
            offset: 0,
        }
    }

    pub fn next<T: AmxCell<'a> + 'a>(&mut self) -> Option<T> {
        let result = self.get(self.offset);
        self.offset += 1;
        return result;
    }

    pub fn get<T: AmxCell<'a> + 'a>(&self, offset: usize) -> Option<T> {
        if offset > self.count() {
            return None;
        }

        unsafe {
            T::from_raw(self.amx, self.args.offset((offset + 1) as isize).read())
                .ok()
        }
    }
    
    pub fn count(&self) -> usize {
        unsafe {
            (self.args.read() / 4) as usize
        }
    }
}