use super::{BlocksTile, Map, Position};
use specs::prelude::*;

pub struct MapIndexingSystem {}

/// the 'a lifetime parameter here is always confusing
/// for me, why this boilertype needed?
/// the reason is one function take self and another reference
/// 'a can be more flexible, not need them to have same liftime parameter.
impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, poses, blkers, ents) = data;

        map.populates_blocked();
        map.clear_all_content();

        for (ent, position) in (&ents, &poses).join() {
            let idx = map.xy_idx(position.x, position.y);

            // do populate blocker things
            if let Some(_p) = blkers.get(ent) {
                map.blocked[idx] = true;
            }

            map.tile_content[idx].push(ent);
        }
    }
}
