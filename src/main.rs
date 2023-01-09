use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod component;
pub use component::*;

mod map;
pub use map::*;

mod player;
pub use player::*;

mod visibility_system;
pub use visibility_system::*;

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
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

        draw_map(&self.ecs, ctx);

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
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
    let mut gs = State { ecs: World::new() };

    // let ecs register component
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<ViewShed>();

    let map = new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms.first().unwrap().center();
    //let ecs register resource
    gs.ecs.insert(map);

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

    for i in 1..=10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 10 })
            .with(Renderable {
                glyph: rltk::to_cp437('&'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
    }

    // game main loop, inside monitor, rendering by calling tick
    rltk::main_loop(monitor, gs)
}
