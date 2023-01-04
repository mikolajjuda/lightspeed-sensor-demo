use bracket_lib::prelude::*;
use specs::prelude::*;

use crate::state::NumGhosts;

use super::components::*;
use super::constants::*;
use super::state::Turn;
pub struct GhostPlacer;

impl<'a> System<'a> for GhostPlacer {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Point>,
        ReadStorage<'a, Detectable>,
        Read<'a, LazyUpdate>,
        Read<'a, Turn>,
        Write<'a, NumGhosts>,
    );
    fn run(
        &mut self,
        (entities, positions, detectability, updater, current_turn, mut num_ghosts): Self::SystemData,
    ) {
        for (e, pos, _d) in (&entities, &positions, &detectability).join() {
            let g = entities.create();
            updater.insert(g, pos.clone());
            updater.insert(
                g,
                SensorGhost {
                    emitter: e,
                    turn: current_turn.0,
                },
            );
            num_ghosts.0 += 1;
        }
    }
}

pub struct Detector;

impl<'a> System<'a> for Detector {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Point>,
        ReadStorage<'a, Sensor>,
        WriteStorage<'a, SensorStorage>,
        ReadStorage<'a, SensorGhost>,
        Read<'a, Turn>,
    );

    fn run(
        &mut self,
        (entities, positions, sensors, mut sensor_storages, ghosts, current_turn): Self::SystemData,
    ) {
        (
            &entities,
            &positions,
            &sensors,
            (&mut sensor_storages).maybe(),
        )
            .par_join()
            .for_each(|(e, pos, s, mut ss)| {
                for (gp, g) in (&positions, &ghosts).join() {
                    let distance = PYTHAGORAS.distance2d(*pos, *gp);
                    let lightturns_distance = (distance / LIGHTSPEED as f32) as u32;
                    if (s.max_range as f32) < distance {
                        continue;
                    }
                    let time_distance = current_turn.0 - g.turn;
                    if lightturns_distance as u64 == time_distance {
                        if lightturns_distance == 0 && e == g.emitter {
                            continue;
                        }
                        if let Some(ref mut sensor_storage) = ss {
                            (*sensor_storage).detections.push(DetectionInfo {
                                //depending on gameplay needs more complicated logic for sensors detection is possible
                                position: gp.clone(),
                                turn: Turn(current_turn.0),
                            });
                        }
                    }
                }
            });
    }
}
