use std::cell::Cell;

pub struct Player {
    pub mana: Cell<usize>,
    pub health: Cell<usize>,
}