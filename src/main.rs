use rltk::{GameState, Point, Rltk, RGB};
use specs::prelude::*;

mod component;
pub use component::*;

mod map;
pub use map::*;

mod player;
pub use player::*;

mod visibility_system;
pub use visibility_system::*;

mod monster_ai_system;
pub use monster_ai_system::*;

#[derive(PartialEq)]
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
    pub ecs: World,
    pub run_state: RunState,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);

        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        // if Paused, wait for player input
        // after input, the next tick will run once
        if self.run_state == RunState::Paused {
            self.run_state = player_input(self, ctx)
        } else {
            self.run_systems();
            self.run_state = RunState::Paused;
        }

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let players = self.ecs.read_storage::<Player>();
        let viewsheds = self.ecs.read_storage::<ViewShed>();

        for (pos, render) in (&positions, &renderables).join() {
            let pt = Point::new(pos.x, pos.y);
            for (_player, viewshed) in (&players, &viewsheds).join() {
                if viewshed.visible_tiles.contains(&pt) {
                    ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
                }
            }
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
    let mut gs = State { ecs: World::new(), run_state: RunState::Running};

    // let ecs register component
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<ViewShed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    let map = new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms.first().unwrap().center();

    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();
        let roll = rng.roll_dice(1, 2);
        let (glyph,name) = match roll {
            1 => (rltk::to_cp437('g'), "Goblin".to_string()),
            _ => (rltk::to_cp437('o'), "Orc".to_string()),
        };
        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(ViewShed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
            .with(Name{name: format!("{} #{}", name, i)})
            .build();
    }

    //let ecs register resource
    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));

    // before main loop, create all the entity
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(ViewShed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    // game main loop, inside monitor, rendering by calling tick
    rltk::main_loop(monitor, gs)
}
