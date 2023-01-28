use super::{CombatStats, Player, SufferDamage};
use rltk::console;
use specs::prelude::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
        // WriteStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut all_stats, mut damage) = data;

        for (one_stats, damage) in (&mut all_stats, &damage).join() {
            one_stats.hp -= damage.amounts.iter().sum::<i32>();
        }
        damage.clear();
    }
}

pub fn delete_the_dead(ecs: &mut World) {
    let mut dead = Vec::new();

    {
        let all_combat_stats = ecs.read_storage::<CombatStats>();
        let entities = ecs.entities();

        for (ent, one_combat_stats) in (&entities, &all_combat_stats).join() {
            if one_combat_stats.hp < 1 {
                if is_player(ecs, ent) {
                    console::log("You are killed");
                    continue;
                }
                dead.push(ent);
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete");
    }
}

fn is_player(ecs: &World, ent: Entity) -> bool {
    let players = ecs.read_storage::<Player>();
    return players.get(ent).is_some();
}
