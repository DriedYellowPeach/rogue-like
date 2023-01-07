use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::Component;

const WIDTH: usize = 80;
const HEIGHT: usize = 50;

struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component, Debug)]
struct Player {}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * WIDTH) + x as usize
}

pub fn idx_xy(idx: i32) -> (i32, i32) {
    (idx % WIDTH as i32, idx / WIDTH as i32)
}

fn new_map() -> Vec<TileType> {
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

fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    for (idx, tile) in map.iter().enumerate() {
        let (x, y) = idx_xy(idx as i32);
        match tile {
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(0., 1.0, 0.), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            },
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
            }
        }
    }
}

// failed to move when destination is out of range or destination is a wall
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let dest_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);

        if map[dest_idx] != TileType::Wall {
            pos.x = (pos.x + delta_x).clamp(0, (WIDTH-1) as i32);
            pos.y = (pos.y + delta_y).clamp(0, (HEIGHT-1) as i32);
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {},
        Some(k) => match k {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {},
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    // a simple terminal monitor with shape 80X50
    let monitor = RltkBuilder::simple80x50()
        .with_title("Hello Rogue")
        .build()?;

    // gs ticks every tick, this will set the monitor terminal
    let mut gs = State {
        ecs: World::new()
    };
    
    // let ecs register component
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    //let ecs register resource
    gs.ecs.insert(new_map());
    
    // before main loop, create all the entity
    gs.ecs
        .create_entity()
        .with(Position{x: 40, y: 25})
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();
    
    for i in 1..=10 {
        gs.ecs
            .create_entity()
            .with(Position{x: i * 7, y: 10})
            .with(Renderable{
                glyph: rltk::to_cp437('&'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
    }

    // game main loop, inside monitor, rendering by calling tick
    rltk::main_loop(monitor, gs)
}
