use crate::players::Player;
use crate::error::CastResult;

pub struct Ability {
    pub mana_cost: usize,
    pub name: String,
    pub damage: usize,
}

impl Ability {
    pub fn cast(&self, caster: &Player, target: &Player) -> CastResult {
        if caster.mana.get() < self.mana_cost {
            return CastResult::NoMana;
        }

        caster.mana.set(caster.mana.get() - self.mana_cost);
        target.health.set(target.mana.get() - self.damage);

        if target.health.get() <= 0 {
            target.health.set(0);

            CastResult::TargetDead
        } else {
            CastResult::Success(self.damage as i32)
        }
    }
}