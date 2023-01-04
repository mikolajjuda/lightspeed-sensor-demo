use bracket_lib::prelude::*;
use specs::prelude::*;

use super::components::*;
pub struct Spawner;

impl<'a> System<'a> for Spawner {
    type SystemData = (Entities<'a>, Read<'a, LazyUpdate>);
    fn run(&mut self, (entities, updater): Self::SystemData) {
        for i in 100..200 {
            let e = entities.create();
            updater.insert(e, Point { x: i, y: 10 });
            updater.insert(e, Detectable {});
            updater.insert(
                e,
                Renderable {
                    fg: RGB::from(RED),
                    ..Default::default()
                },
            );
            updater.insert(
                e,
                Moving {
                    velocity: Point { x: 0, y: 5 },
                },
            );
        }
    }
}
