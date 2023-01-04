use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::Component;

use super::state::Turn;

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub hidden: bool,
}

impl Default for Renderable {
    fn default() -> Self {
        Self {
            glyph: to_cp437('@'),
            fg: RGB::from(WHITE),
            bg: RGB::from(BLACK),
            hidden: false,
        }
    }
}

#[derive(Component)]
pub struct SensorGhost {
    pub emitter: Entity,
    pub turn: u64,
}

#[derive(Component)]
pub struct Detectable {
    //object can be detected by sensors
}

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Sensor {
    //object can detect stuff
    pub max_range: u32,
}

#[derive(Debug)]
pub struct DetectionInfo {
    pub position: Point,
    pub turn: Turn,
}

#[derive(Component, Default)]
pub struct SensorStorage {
    //object can remember sensor readings
    pub detections: Vec<DetectionInfo>,
}

#[derive(Component)]
#[storage(HashMapStorage)]
pub struct Player {}