use bracket_lib::prelude::*;
use specs::prelude::*;

mod components;
mod map;
mod player;
pub use components::*;
pub use map::*;
pub use player::*;

pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let position = self.ecs.read_storage::<Position>();
        let renderer = self.ecs.read_storage::<Renderer>();
        for (pos, render) in (&position, &renderer).join() {
            ctx.set(
                pos.x,
                pos.y,
                render.foreground,
                render.background,
                render.glyph,
            );
        }

        let player = self.ecs.read_storage::<Player>();
        for (_player, pos) in (&player, &position).join() {
            let (pos_x, pos_y) = pos.get_pos();
            ctx.print_color_right(40, 49, GREEN, BLACK, pos_x);
            ctx.print_color_right(43, 49, GREEN, BLACK, pos_y);
        }
    }
}

fn main() -> BError {
    unsafe {
        std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    }

    let context = BTermBuilder::simple80x50()
        .with_title("Hello Terminal")
        .build();
    let is_context = match context {
        Ok(terminal) => terminal,
        Err(e) => {
            eprint!("Error Happen please fix {}", e);
            std::process::exit(1);
        }
    };

    let mut game_state: State = State { ecs: World::new() };
    game_state.ecs.insert(new_map());
    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderer>();
    game_state.ecs.register::<Movement>();
    game_state.ecs.register::<Player>();
    game_state
        .ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderer {
            glyph: to_cp437('@'),
            foreground: RGB::named(WHITE),
            background: RGB::named(BLACK),
        })
        .with(Player {})
        .build();
    main_loop(is_context, game_state)
}
