use bracket_lib::prelude::*;
use specs::prelude::*;

mod components;
mod enemy_ai_system;
mod map;
mod map_indexing_system;
mod player;
mod rect;
mod visibility_system;
pub use components::*;
pub use map::*;
pub use player::*;
pub use rect::Rect;
use visibility_system::VisibilitySystem;

use crate::{enemy_ai_system::MonsterAI, map_indexing_system::MapIndexingSystem};

const MAP_SIZE: usize = 80 * 50;

#[derive(PartialEq, Clone, Copy)]
pub enum SystemState {
    Running,
    Paused,
}

pub struct State {
    ecs: World,
    system_state: SystemState,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis_sys = VisibilitySystem {};
        let mut monster_ai_sys = MonsterAI {};
        let mut map_indexing_sys = MapIndexingSystem {};
        monster_ai_sys.run_now(&self.ecs);
        vis_sys.run_now(&self.ecs);
        map_indexing_sys.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        if self.system_state == SystemState::Running {
            self.run_systems();
            self.system_state = SystemState::Paused;
        } else {
            self.system_state = player_input(self, ctx);
        }

        draw_map(&self.ecs, ctx);

        let position = self.ecs.read_storage::<Position>();
        let renderer = self.ecs.read_storage::<Renderer>();
        let map = self.ecs.fetch::<Map>();
        for (pos, render) in (&position, &renderer).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(
                    pos.x,
                    pos.y,
                    render.foreground,
                    render.background,
                    render.glyph,
                );
            }
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

    let mut game_state: State = State {
        ecs: World::new(),
        system_state: SystemState::Running,
    };
    let mut map: Map = Map {
        tiles: vec![TileType::Wall; MAP_SIZE],
        rooms: Vec::new(),
        width: 80,
        height: 50,
        revealed_tiles: vec![false; MAP_SIZE],
        visible_tiles: vec![false; MAP_SIZE],
        blocked_tiles: vec![false; MAP_SIZE],
    };
    map.new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderer>();
    game_state.ecs.register::<Movement>();
    game_state.ecs.register::<Player>();
    game_state.ecs.register::<FieldOfView>();
    game_state.ecs.register::<Monster>();
    game_state.ecs.register::<EntityName>();
    game_state.ecs.register::<BlocksTile>();

    let mut rng = RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let random_monster = rng.roll_dice(1, 2);
        let (glyph, foreground, name) = match random_monster {
            1 => (to_cp437('g'), RGB::named(GREEN1), "Goblin".to_string()),
            _ => (to_cp437('o'), RGB::named(CYAN1), "Orc".to_string()),
        };
        game_state
            .ecs
            .create_entity()
            .with(EntityName {
                name: format!("{} #{}", &name, i),
            })
            .with(Monster {})
            .with(Position { x, y })
            .with(Renderer {
                glyph,
                foreground,
                background: RGB::named(BLACK),
            })
            .with(FieldOfView {
                visuble_tiles: Vec::new(),
                range: 4,
                dirty: true,
            })
            .with(BlocksTile {})
            .build();
    }

    game_state
        .ecs
        .create_entity()
        .with(EntityName {
            name: "Player".to_string(),
        })
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
            dirty: true,
        })
        .build();

    game_state.ecs.insert(map);
    game_state.ecs.insert(Point::new(player_x, player_y));
    main_loop(is_context, game_state)
}
