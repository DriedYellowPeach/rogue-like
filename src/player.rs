use crate::ViewShed;
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;

use super::{Map, Player, Position, State, TileType, HEIGHT, WIDTH};

// failed to move when destination is out of range or destination is a wall
pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<ViewShed>();
    // let map = ecs.fetch::<Vec<TileType>>();
    let map = ecs.fetch::<Map>();

    for (viewshed, _player, pos) in (&mut viewsheds, &mut players, &mut positions).join() {
        let dest_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        if map.tiles[dest_idx] != TileType::Wall {
            pos.x = (pos.x + delta_x).clamp(0, (WIDTH - 1) as i32);
            pos.y = (pos.y + delta_y).clamp(0, (HEIGHT - 1) as i32);

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(k) => match k {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}
