
use super::{Point, RunState, ViewShed, CombatStats, Name, WantsToMelee};
use rltk::{Rltk, VirtualKeyCode, console};
use specs::prelude::*;

use super::{Map, Player, Position, State, HEIGHT, WIDTH};

// failed to move when destination is out of range or destination is a wall
pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<ViewShed>();
    let entities = ecs.entities();
    let map = ecs.fetch::<Map>();
    let mut player_pos = ecs.write_resource::<Point>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let names = ecs.read_storage::<Name>();
    let mut all_wants_to_melee = ecs.write_storage::<WantsToMelee>();

    for (entity, viewshed, _player, pos) in (&entities, &mut viewsheds, &mut players, &mut positions).join() {
        if pos.x + delta_x < 1 || pos.x + delta_x > map.width-1 || pos.y + delta_y < 1 || pos.y + delta_y > map.height-1 { return; }
        let dest_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        for potential_target in map.tile_content[dest_idx].iter() {
            match (combat_stats.get(*potential_target), names.get(*potential_target)) {
                (Some(_cs), Some(name)) => {
                    console::log(format!("Player Stab {}", name.name));
                    all_wants_to_melee.insert(entity, WantsToMelee { target: *potential_target }).expect("Add target failed");
                    return
                },
                (Some(_cs), None) => {
                    console::log(format!("Player Stab anonymous with id {}", potential_target.id()));
                    all_wants_to_melee.insert(entity, WantsToMelee { target: *potential_target }).expect("Add target failed");
                    return
                },
                _ => {}
            }
            // let target = combat_stats.get(*potential_target);
            // if let Some(_target) = target {
            //     wants_to_melee.insert(entity, WantsToMelee{ target: *potential_target }).expect("Add target failed");
            //     return;
            // }
        }

        if !map.blocked[dest_idx] {
            pos.x = (pos.x + delta_x).clamp(0, (WIDTH - 1) as i32);
            pos.y = (pos.y + delta_y).clamp(0, (HEIGHT - 1) as i32);
            player_pos.x = pos.x;
            player_pos.y = pos.y;

            viewshed.dirty = true;
        }
    }
}

/// On mac, Numpad is not working as usual, Numpad is mapping into Key
/// when not adding Key code, the Numpad won't working
use VirtualKeyCode::*;
pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => return RunState::AwaitingInput,
        Some(k) => match k {
            // cardinal
            Left | H | Numpad4 | Key4=> try_move_player(-1, 0, &mut gs.ecs),
            Right | L | Numpad6 | Key6 => try_move_player(1, 0, &mut gs.ecs),
            Up | K | Numpad8 | Key8 => try_move_player(0, -1, &mut gs.ecs),
            Down | J | Numpad2 | Key2 => try_move_player(0, 1, &mut gs.ecs),

            // diagonals
            Key9 | Numpad9 | U => try_move_player(1, -1, &mut gs.ecs),
            Key7 | Numpad7 | Y => try_move_player(-1, -1, &mut gs.ecs),
            Key3 | Numpad3 | N => try_move_player(1, 1, &mut gs.ecs),
            Key1 | Numpad1 | B => try_move_player(-1, 1, &mut gs.ecs),
            _ => return RunState::AwaitingInput,
        },
    }
    RunState::PlayerTurn
}
