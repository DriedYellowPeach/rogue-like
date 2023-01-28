use super::{Player, ViewShed};
use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator, Rltk, RGB};
use specs::prelude::*;
use specs::{World, WorldExt};
use std::cmp::{max, min};

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

#[derive(Default)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        (idx as i32 % self.width, idx as i32 / self.width)
    }

    pub fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    pub fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < WIDTH * HEIGHT {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    pub fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < WIDTH * HEIGHT {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        !self.blocked[self.xy_idx(x, y)]
    }

    pub fn clear_all_content(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; WIDTH * HEIGHT],
            rooms: Vec::new(),
            width: WIDTH as i32,
            height: HEIGHT as i32,
            revealed_tiles: vec![false; WIDTH * HEIGHT],
            blocked: vec![false; WIDTH * HEIGHT],
            tile_content: vec![Vec::new(); WIDTH * HEIGHT],
        };

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
            if !map.rooms.iter().any(|other| new_room.intersect(other)) {
                // apply_room_to_map(&new_room, &mut map);
                map.apply_room_to_map(&new_room);
                if !map.rooms.is_empty() {
                    let (x1, y1) = new_room.center();
                    let (x2, y2) = map.rooms.last().unwrap().center();

                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(x1, x2, y1);
                        map.apply_vertical_tunnel(y1, y2, x2);
                    } else {
                        map.apply_vertical_tunnel(y1, y2, x1);
                        map.apply_horizontal_tunnel(x1, x2, y2);
                    }
                }
                map.rooms.push(new_room);
            }
        }

        map
    }

    pub fn populates_blocked(&mut self) {
        for (i, tile) in self.tiles.iter().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let (x, y) = self.idx_xy(idx);

        let cardinal_move = [
            (1, 0, 1.0),
            (-1, 0, 1.0),
            (0, 1, 1.0),
            (0, -1, 1.0),
            (1, 1, 1.45),
            (1, -1, 1.45),
            (-1, 1, 1.45),
            (-1, -1, 1.45),
        ];

        // let cal_distance = |x: f32, y: f32| (x * x + y * y).sqrt();

        for (delta_x, delta_y, distance) in cardinal_move {
            let (e_x, e_y) = (x + delta_x, y + delta_y);
            if self.is_exit_valid(e_x, e_y) {
                let exit_idx = self.xy_idx(e_x, e_y);
                // exits.push((exit_idx, cal_distance(e_x as f32, e_y as f32)));
                exits.push((exit_idx, distance));
            }
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let (x1, y1) = self.idx_xy(idx1);
        let (x2, y2) = self.idx_xy(idx2);

        let p1 = Point::new(x1, y1);
        let p2 = Point::new(x2, y2);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let mut viewsheds = ecs.write_storage::<ViewShed>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, viewshed) in (&mut players, &mut viewsheds).join() {
        for (idx, tile) in map.tiles.iter().enumerate() {
            let (x, y) = map.idx_xy(idx);
            let pt = Point::new(x, y);

            if map.revealed_tiles[idx] {
                let glyph;
                let mut fg;
                match tile {
                    TileType::Wall => {
                        glyph = rltk::to_cp437('#');
                        fg = RGB::from_f32(0., 1.0, 0.);
                    }
                    TileType::Floor => {
                        glyph = rltk::to_cp437('.');
                        fg = RGB::from_f32(0., 0.5, 0.5);
                    }
                }

                if !viewshed.visible_tiles.contains(&pt) {
                    fg = fg.to_greyscale()
                }
                ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
            }
        }
    }
}
