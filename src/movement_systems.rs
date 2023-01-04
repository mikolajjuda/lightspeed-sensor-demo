use bracket_lib::geometry::Point;
use specs::prelude::*;

use super::components::*;

pub struct Mover;

impl<'a> System<'a> for Mover {
    type SystemData = (WriteStorage<'a, Point>, ReadStorage<'a, Moving>);

    fn run(&mut self, (mut positions, movements): Self::SystemData) {
        for (pos, mov) in (&mut positions, &movements).join() {
            *pos += mov.velocity;
        }
    }
}
