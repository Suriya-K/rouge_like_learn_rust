use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

struct State {
    ecs: World,
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

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

// if not use #[derive(Component)] marco then we will have to write manual component
// imple Component for Position {
//      type Storage = VecStorage<Self>;
// }
#[derive(Component)]
struct Renderer {
    glyph: FontCharType,
    foreground: RGB,
    background: RGB,
}

#[derive(Component)]
struct Movement {}

#[derive(Component, Debug)]
struct Player {}

fn move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut position = ecs.write_storage::<Position>();
    let mut player = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut player, &mut position).join() {
        let des_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[des_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

#[derive(PartialEq, Clone, Copy)]
enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }
    let mut rng = RandomNumberGenerator::new();
    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }
    map
}

fn draw_map(map: &[TileType], ctx: &mut BTerm) {
    let mut x = 0;
    let mut y = 0;

    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(x, y, GRAY, BLACK, to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
            }
        }
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
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
