use rltk::{GameState, Point, Rltk, RGB}; use specs::prelude::*; mod component; pub use component::*; mod map; pub use map::*;

mod player;
pub use player::*;

mod visibility_system;
pub use visibility_system::*;

mod monster_ai_system;
pub use monster_ai_system::*;

mod map_indexing_system;
pub use map_indexing_system::*;

mod melee_combat_system;
pub use melee_combat_system::*;

mod damage_system;
pub use damage_system::*;

#[derive(Clone, Copy, PartialEq)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
}

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);

        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);

        let mut map_index = MapIndexingSystem {};
        map_index.run_now(&self.ecs);

        let mut melee = MeleeCombatSystem{};
        melee.run_now(&self.ecs);

        let mut damage = DamageSystem{};
        damage.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        let mut newrunstate = *self.ecs.fetch::<RunState>();

        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            },
            RunState::AwaitingInput => {
                newrunstate = player_input(self, ctx);
            },
            RunState::PlayerTurn => {
                self.run_systems();
                newrunstate = RunState::MonsterTurn;
            },
            RunState::MonsterTurn => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
        }

        {
            let mut run_writer = self.ecs.write_resource::<RunState>();
            *run_writer = newrunstate;
        }

        // if Paused, wait for player input
        // after input, the next tick will run once
        // if self.run_state == RunState::Paused {
        //     self.run_state = player_input(self, ctx)
        // } else {
        //     self.run_systems();
        //     self.run_state = RunState::Paused;
        // }

        damage_system::delete_the_dead(&mut self.ecs);
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
    let mut gs = State {
        ecs: World::new(),
    };

    // let ecs register component
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<ViewShed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();

    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms.first().unwrap().center();

    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();
        let roll = rng.roll_dice(1, 2);
        let (glyph, name) = match roll {
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
            .with(BlocksTile {})
            .with(Name {
                name: format!("{} #{}", name, i),
            })
            .with(CombatStats {max_hp: 16, hp: 16, defense: 1, power: 4})
            .build();
    }

    //let ecs register resource
    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(RunState::PreRun);

    // before main loop, create all the entity
    let player_ent = gs.ecs
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
        .with(Name {name: "Neil".to_string()})
        .with(ViewShed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(CombatStats { max_hp: 30, hp: 30, defense: 2, power: 5})
        .build();
    
    gs.ecs.insert(player_ent);
    // game main loop, inside monitor, rendering by calling tick
    rltk::main_loop(monitor, gs)
}
