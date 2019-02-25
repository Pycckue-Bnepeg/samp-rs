use samp::cell::Cell;

#[derive(Debug, Clone, Copy)]
pub enum CastResult {
    Success(i32), // Can't be less than zero.
    TargetDead,
    NoMana,
    NoTarget,
    NoCaster,
    NoAbility,
    NotEnoughMoney,
}

impl Cell<'_> for CastResult {
    fn as_cell(&self) -> i32 {
        use CastResult::*;

        match self {
            Success(damage) => *damage,
            TargetDead => 0,
            NoMana => -1,
            NoTarget => -2,
            NoCaster => -3,
            NoAbility => -4,
            NotEnoughMoney => -5,
        }
    }
}