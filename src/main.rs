use bracket_lib::prelude::*;

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "HELLO FROM RUST");
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

    let game_state: State = State {};
    main_loop(is_context, game_state)
}
