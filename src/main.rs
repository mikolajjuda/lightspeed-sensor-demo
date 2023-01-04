use std::time::Instant;

use bracket_lib::prelude::*;
use specs::prelude::*;

mod constants;
use constants::*;
mod state;
use state::*;
mod components;
use components::*;
mod movement_system;
mod sensor_systems;
mod spawner_system;

fn main() -> BError {
    let context = BTermBuilder::simple(CONSOLE_WIDTH, CONSOLE_HEIGHT)
        .unwrap()
        .with_title("Lightspeed Sensor Demo")
        .build()?;

    let mut gs = State::new();

    gs.ecs
        .create_entity()
        .with(Point { x: 45, y: 40 })
        .with(Renderable {
            fg: RGB::from(BLUE),
            ..Default::default()
        })
        .with(Player {})
        .with(Sensor {
            max_range: 100000000,
        })
        .with(SensorStorage::default())
        .with(Detectable {})
        .build();

    gs.ecs
        .create_entity()
        .with(Point { x: 100, y: 10 })
        .with(Renderable {
            fg: RGB::from(RED),
            ..Default::default()
        })
        .with(Detectable {})
        .with(Moving {
            velocity: Point { x: -2, y: 1 },
        })
        .build();
    gs.ecs
        .create_entity()
        .with(Point { x: 20, y: 10 })
        .with(Renderable {
            fg: RGB::from(RED),
            ..Default::default()
        })
        .with(Detectable {})
        .with(Moving {
            velocity: Point { x: 2, y: 10 },
        })
        .build();

    gs.time_last_turn = Instant::now();
    main_loop(context, gs)
}
