use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::Component;
use std::{
    cmp::{max, min},
    collections::btree_map::Keys,
};

struct State {
    ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.run_systems();
        player_input(self, ctx);

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
        let mut leftmove = LeftMove {};
        leftmove.run_now(&self.ecs);
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

struct LeftMove {}

impl<'a> System<'a> for LeftMove {
    type SystemData = (ReadStorage<'a, Movement>, WriteStorage<'a, Position>);

    fn run(&mut self, (mov, mut pos): Self::SystemData) {
        for (_mov, pos) in (&mov, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

#[derive(Component, Debug)]
struct Player {}

fn move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut position = ecs.write_storage::<Position>();
    let mut player = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut player, &mut position).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
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

fn main() -> BError {
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
    for i in 0..10 {
        game_state
            .ecs
            .create_entity()
            .with(Position { x: i * 5, y: 5 })
            .with(Renderer {
                glyph: to_cp437(char::from_digit(i as u32, 10).unwrap()),
                foreground: RGB::named(RED),
                background: RGB::named(BLACK),
            })
            .with(Movement {})
            .build();
    }
    main_loop(is_context, game_state)
}
