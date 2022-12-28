use bracket_lib::prelude::*;

#[derive(Default)]
struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(90 / 2 - 1, "Lightspeed Sensor Demo");
    }
}
fn main() -> BError {
    let context = BTermBuilder::simple(160, 90)
        .unwrap()
        .with_title("Lightspeed Sensor Demo")
        .build()?;

    let gs = State::default();

    main_loop(context, gs)
}
