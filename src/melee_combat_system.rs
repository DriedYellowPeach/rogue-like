use super::{CombatStats, Name, SufferDamage, WantsToMelee};
use rltk::console;
use specs::prelude::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToMelee>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut all_wants_melees, names, all_combat_stats, mut damages) = data;

        for (_ent, one_wants_melee, name, one_combat_stats) in
            (&entities, &all_wants_melees, &names, &all_combat_stats).join()
        {
            if one_combat_stats.hp > 0 {
                let target_combat_stats = all_combat_stats.get(one_wants_melee.target).unwrap();
                if target_combat_stats.hp > 0 {
                    let target_name = names.get(one_wants_melee.target).unwrap();
                    let real_dmg =
                        std::cmp::max(0, one_combat_stats.power - target_combat_stats.defense);

                    if real_dmg == 0 {
                        console::log(format!(
                            "{} is unable to hurt {}",
                            name.name, target_name.name
                        ));
                    } else {
                        console::log(format!(
                            "{} hits {}, for {} hp, {} hp left: {}",
                            name.name, target_name.name, real_dmg, target_name.name, target_combat_stats.hp
                        ));
                        SufferDamage::new_damage(&mut damages, one_wants_melee.target, real_dmg);
                    }
                }
            }
        }

        all_wants_melees.clear();
    }
}
