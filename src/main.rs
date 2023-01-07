use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod component;
pub use component::*;

mod map;
pub use map::*;

mod player;
pub use player::*;

pub struct State {
    pub ecs: World,
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

    //let ecs register resource
    gs.ecs.insert(new_map_rooms_and_corridors());

    // before main loop, create all the entity
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
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
