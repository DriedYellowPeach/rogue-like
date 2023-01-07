use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
use specs_derive::Component;

struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

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
    
    // before main loop, create all the entity
    gs.ecs
        .create_entity()
        .with(Position{x: 40, y: 25})
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
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
