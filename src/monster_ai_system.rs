use super::{Map, Monster, Name, Position, ViewShed, WantsToMelee, RunState};
use rltk::{console, Point};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        // resource of type Point is player position
        ReadExpect<'a, Point>,
        // resource of type Entity is player
        ReadExpect<'a, Entity>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, ViewShed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, WantsToMelee>,
        // ReadStorage<'a, BlocksTile>
    );

    fn run(&mut self, data: Self::SystemData) {
        // let (mut map, player_pos, player_entity, run_state, entities, mut viewsheds, mut poses, monsters, names, mut wants_to_melee) = data;
        let (mut map, player_pos, player_entity, run_state, entities,  mut viewsheds, mut poses, monsters, names, mut wants_to_melee) = data;

        if *run_state != RunState::MonsterTurn {return}

        for (ent, viewshed, pos, _monster, name) in
            (&entities, &mut viewsheds, &mut poses, &monsters, &names).join() {
            // player_pos is not a reference type, here is a explicitly deref coercion
            // no deref coercion here, because deref coercion take ref and give another ref
            let distance =
                rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
            if distance < 1.5 {
                // console::log(format!("{} attack", name.name));
                wants_to_melee.insert(ent, WantsToMelee { target: *player_entity }).expect("unable to insert attach");
            }
            // bug here, two monster search path and will get overlap
            // how to avoid that?
            else if viewshed.visible_tiles.contains(&*player_pos) {
                let start = map.xy_idx(pos.x, pos.y);
                console::log(format!("{} pos: {},{}", name.name, pos.x, pos.y));
                let end = map.xy_idx(player_pos.x, player_pos.y);
                // &*map is a explicitly deref, map is of type FetchMut, what we want is struct Map
                let path = rltk::a_star_search(start, end, &*map);
                if path.success && path.steps.len() > 1 {
                    let step_idx = path.steps[1];
                    if map.blocked[step_idx] {
                        continue
                    }
                    (pos.x, pos.y) = map.idx_xy(step_idx);
                    map.blocked[step_idx] = true;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
