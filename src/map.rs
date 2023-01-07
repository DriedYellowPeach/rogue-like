use rltk::{RandomNumberGenerator, Rltk, RGB};
// use super::{Rect};

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 50;

pub struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * WIDTH) + x as usize
}

pub fn idx_xy(idx: i32) -> (i32, i32) {
    (idx % WIDTH as i32, idx / WIDTH as i32)
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; WIDTH * HEIGHT];

    // build boundries for map
    // set first and last columns to wall
    for y in 0..HEIGHT {
        map[xy_idx(0, y as i32)] = TileType::Wall;
        map[xy_idx((WIDTH - 1) as i32, y as i32)] = TileType::Wall;
    }

    // set first and last row to wall
    for x in 0..WIDTH {
        map[xy_idx(x as i32, 0)] = TileType::Wall;
        map[xy_idx(x as i32, (HEIGHT - 1) as i32)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, (WIDTH - 1) as i32);
        let y = rng.roll_dice(1, (HEIGHT - 1) as i32);
        let idx = xy_idx(x, y);

        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

pub fn new_map_rooms_and_corridors() -> Vec<TileType> {
    let mut map = vec![TileType::Wall; WIDTH * HEIGHT];

    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        // x max is 80 - 1 - 1 - w
        let x = rng.roll_dice(1, 80 - 1 - w) - 1;
        let y = rng.roll_dice(1, 50 - 1 - h) - 1;
        let new_room = Rect::new(x, y, w, h);
        if !rooms.iter().any(|other| new_room.intersect(other)) {
            apply_room_to_map(&new_room, &mut map);
            if !rooms.is_empty() {
                let (x1, y1) = new_room.center();
                let (x2, y2) = rooms.last().unwrap().center();

                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, x1, x2, y1);
                    apply_vertical_tunnel(&mut map, y1, y2, x2);
                } else {
                    apply_vertical_tunnel(&mut map, y1, y2, x1);
                    apply_horizontal_tunnel(&mut map, x1, x2, y2);
                }
            }
            rooms.push(new_room);
        }
    }

    map
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    for (idx, tile) in map.iter().enumerate() {
        let (x, y) = idx_xy(idx as i32);
        match tile {
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0., 1.0, 0.),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('#'),
                );
            }
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.'),
                );
            }
        }
    }
}

pub fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

use std::cmp::{max, min};
pub fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < WIDTH * HEIGHT {
            map[idx] = TileType::Floor;
        }
    }
}

pub fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < WIDTH * HEIGHT {
            map[idx] = TileType::Floor;
        }
    }
}
