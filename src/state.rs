use std::time::{Duration, Instant};

use bracket_lib::prelude::*;
use specs::prelude::*;

use crate::spawner_system::Spawner;

use super::components::*;
use super::constants::*;
use super::movement_system::*;
use super::sensor_systems::*;

#[derive(Default, Debug)]
pub struct Turn(pub u64);

pub struct State {
    pub ecs: World,
    pub view_translation: Point,
    pub time_last_turn: Instant,
    pub turn_compute_time: Duration,
}

impl State {
    pub fn new() -> Self {
        let mut w = World::new();
        w.register::<Point>();
        w.register::<Renderable>();
        w.register::<Player>();
        w.register::<SensorGhost>();
        w.register::<Sensor>();
        w.register::<SensorStorage>();
        w.register::<Detectable>();
        w.register::<Moving>();

        w.insert(Turn::default());

        State {
            ecs: w,
            view_translation: Point::zero(),
            time_last_turn: Instant::now(),
            turn_compute_time: Duration::ZERO,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        {
            //input
            match ctx.key {
                None => {}
                Some(key) => match key {
                    VirtualKeyCode::A => self.view_translation.x -= 1,
                    VirtualKeyCode::D => self.view_translation.x += 1,
                    VirtualKeyCode::W => self.view_translation.y += 1,
                    VirtualKeyCode::S => self.view_translation.y -= 1,
                    _ => {}
                },
            }
        }
        {
            //turn
            let since_last_turn = self.time_last_turn.elapsed();
            if since_last_turn > Duration::from_millis(1000) {
                ctx.print(0, CONSOLE_HEIGHT - 2, "TURN");
                let turn_timer = Instant::now();
                {
                    let mut current_turn = self.ecs.write_resource::<Turn>();
                    (*current_turn).0 += 1;
                }

                let mut ghost_placer = GhostPlacer {};
                ghost_placer.run_now(&self.ecs);
                let mut spawner = Spawner {};
                spawner.run_now(&self.ecs);
                self.ecs.maintain();
                let mut detector = Detector {};
                detector.run_now(&self.ecs);
                let mut mover = Mover {};
                mover.run_now(&self.ecs);

                self.turn_compute_time = turn_timer.elapsed();
                self.time_last_turn = Instant::now();
            }
        }
        self.ecs.maintain();
        {
            //render
            ctx.draw_hollow_box_double(0, 0, MAP_VIEW_WIDTH + 2, MAP_VIEW_HEIGHT + 2, WHITE, BLACK);

            let positions = self.ecs.read_storage::<Point>();
            let renderables = self.ecs.read_storage::<Renderable>();

            for (pos, render) in (&positions, &renderables).join() {
                if render.hidden {
                    continue;
                }
                let view_rect = Rect::with_size(
                    self.view_translation.x,
                    -self.view_translation.y,
                    MAP_VIEW_WIDTH as i32,
                    MAP_VIEW_HEIGHT as i32,
                );
                if !view_rect.point_in_rect(*pos) {
                    continue;
                }
                ctx.set(
                    pos.x - self.view_translation.x + 1,
                    pos.y + self.view_translation.y + 1,
                    render.fg,
                    render.bg,
                    render.glyph,
                );
            }

            let current_turn = self.ecs.read_resource::<Turn>().0;

            //rendering detection info probably shoud be done using renderable component but I'm to lazy to do that
            let players = self.ecs.read_storage::<Player>();
            let sensor_storages = self.ecs.read_storage::<SensorStorage>();
            for (_p, s) in (&players, &sensor_storages).join() {
                let d = &s.detections;
                for di in d.iter().rev() {
                    if di.turn.0 != current_turn {
                        break;
                    }
                    let view_rect = Rect::with_size(
                        self.view_translation.x,
                        -self.view_translation.y,
                        MAP_VIEW_WIDTH as i32,
                        MAP_VIEW_HEIGHT as i32,
                    );
                    if !view_rect.point_in_rect(di.position) {
                        continue;
                    }
                    ctx.set(
                        di.position.x - self.view_translation.x + 1,
                        di.position.y + self.view_translation.y + 1,
                        GREEN,
                        BLACK,
                        to_cp437('g'),
                    );
                }
            }

            ctx.print(
                0,
                CONSOLE_HEIGHT - 1,
                format!(
                    "view translation: ({}, {}); fps: {}; current turn: {}; turn compute time: {}",
                    self.view_translation.x,
                    self.view_translation.y,
                    ctx.fps,
                    current_turn,
                    self.turn_compute_time.as_secs_f32()
                ),
            );
        }
    }
}
