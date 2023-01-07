use rltk::{GameState, Rltk};

struct State {
    height: i32,
    width: i32,
    welcome: String
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(self.width / 2 - self.welcome.len() as i32 / 2, self.height / 2, self.welcome.to_owned());
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    // a simple terminal monitor with shape 80X50
    let monitor = RltkBuilder::simple80x50()
        .with_title("Hello Rogue")
        .build()?;

    // gs ticks every tick, this will set the monitor terminal
    let gs = State{height:50, width: 80, welcome: String::from("Hello From Neil!!!")};

    // game main loop, inside monitor, rendering by calling tick
    rltk::main_loop(monitor, gs)
}
