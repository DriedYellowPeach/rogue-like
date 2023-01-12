use specs::prelude::*;
use super::{ViewShed, Position, Monster, Name};
use rltk::{console, Point};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( 
                        ReadExpect<'a, Point>,
                        ReadStorage<'a, ViewShed>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, Monster>,
                        ReadStorage<'a, Name>);

    fn run(&mut self, data: Self::SystemData) {
        let (player_pos, viewsheds, poses, monsters, names) = data;

        for (viewshed, _pos, _monster, name) in (&viewsheds, &poses, &monsters, &names).join() {
            if viewshed.visible_tiles.contains(&*player_pos){
                console::log(format!("{} shouts insults", name.name));
            }
        }
    }
}
