use bracket_lib::prelude::*;
use specs::prelude::*;

mod components;
mod map;
mod player;
mod rect;
mod visibility_system;
pub use components::*;
pub use map::*;
pub use player::*;
pub use rect::Rect;
use visibility_system::VisibilitySystem;

pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis_sys = VisibilitySystem {};
        vis_sys.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs, ctx);

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
    let mut map: Map = Map {
        tiles: vec![TileType::Wall; 80 * 50],
        rooms: Vec::new(),
        width: 80,
        height: 50,
        revealed_tiles: vec![false; 80 * 50],
    };
    map.new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    game_state.ecs.insert(map);

    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderer>();
    game_state.ecs.register::<Movement>();
    game_state.ecs.register::<Player>();
    game_state.ecs.register::<FieldOfView>();

    game_state
        .ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderer {
            glyph: to_cp437('@'),
            foreground: RGB::named(WHITE),
            background: RGB::named(BLACK),
        })
        .with(Player {})
        .with(FieldOfView {
            visuble_tiles: Vec::new(),
            range: 6,
        })
        .build();
    main_loop(is_context, game_state)
}
