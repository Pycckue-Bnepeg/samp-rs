use crate::MagePlugin;
use crate::error::CastResult;
use crate::abilities::Ability;

use samp::native;
use samp::amx::Amx;
use samp::error::AmxResult;
use samp::cell::AmxString;
use samp::args::Args;
use samp::exec_public;

macro_rules! try_cast {
    ($e:expr, $err:expr) => {
        match $e {
            Some(__t) => __t,
            None => return Ok($err),
        };
    };
}

impl MagePlugin {
    #[native(name = "CreateAbility")]
    pub fn create_ability(&mut self, _: &Amx, creater_id: usize, ability_name: AmxString, damage: usize, mana_cost: usize) -> AmxResult<bool> {
        let ability_name = ability_name.to_string();
        let list = self.abilities.entry(creater_id).or_insert(vec![]);

        list.push(Ability {
            mana_cost,
            damage,
            name: ability_name,
        });

        Ok(true)
    }

    #[native(name = "CastAbility")]
    pub fn cast_ability(&mut self, _: &Amx, caster_id: usize, ability_idx: usize, target_id: usize) -> AmxResult<CastResult> {
        let caster = try_cast!(self.mages.get(&caster_id), CastResult::NoCaster);
        let target = try_cast!(self.mages.get(&target_id), CastResult::NoTarget);

        let caster_abilities = try_cast!(self.abilities.get(&caster_id), CastResult::NoAbility);
        let ability = try_cast!(caster_abilities.get(ability_idx), CastResult::NoAbility);
        
        Ok(ability.cast(caster, target))
    }

    #[native(name = "Testique", raw)]
    pub fn testique(&mut self, amx: &Amx, mut args: Args) -> AmxResult<i32> {
        let from_amx: AmxString = args.next().unwrap();
        let somevalues = vec![1, 2, 3, 4, 5];
        let what_to_say = String::from("hello?");

        exec_public!(amx, "SomePublic", &from_amx, &somevalues => array, &what_to_say => string)?;

        Ok(0)
    }
}
