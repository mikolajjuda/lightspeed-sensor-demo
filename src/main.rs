use std::time::{Duration, Instant};

use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::Component;

const PYTHAGORAS: DistanceAlg = DistanceAlg::Pythagoras;

const CONSOLE_WIDTH: u32 = 160;
const CONSOLE_HEIGHT: u32 = 90;
const MAP_VIEW_WIDTH: u32 = 150;
const MAP_VIEW_HEIGHT: u32 = 80;
const LIGHTSPEED: u32 = 10;

#[derive(Default, Debug)]
struct Turn(u64);

#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
    hidden: bool,
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
struct SensorGhost {
    emitter: Entity,
    turn: u64,
}

#[derive(Component)]
struct Detectable {
    //object can be detected by sensors
}

#[derive(Component)]
#[storage(HashMapStorage)]
struct Sensor {
    //object can detect stuff
    max_range: u32,
}

#[derive(Debug)]
struct DetectionInfo {
    position: Point,
    turn: Turn,
}

#[derive(Component, Default)]
struct SensorStorage {
    //object can remember sensor readings
    detections: Vec<DetectionInfo>,
}

#[derive(Component)]
#[storage(HashMapStorage)]
struct Player {}

struct GhostPlacer;

impl<'a> System<'a> for GhostPlacer {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Point>,
        ReadStorage<'a, Detectable>,
        Read<'a, LazyUpdate>,
        Read<'a, Turn>,
    );
    fn run(
        &mut self,
        (entities, positions, detectability, updater, current_turn): Self::SystemData,
    ) {
        (&entities, &positions, &detectability)
            .par_join()
            .for_each(|(e, pos, _d)| {
                let g = entities.create();
                updater.insert(g, pos.clone());
                updater.insert(
                    g,
                    SensorGhost {
                        emitter: e,
                        turn: current_turn.0,
                    },
                );
            });
    }
}

struct Detector;

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
                    let lightturns_distance = PYTHAGORAS.distance2d(*pos, *gp) as u32 / LIGHTSPEED;
                    if s.max_range < lightturns_distance {
                        continue;
                    }
                    let time_distance = current_turn.0 - g.turn;
                    if lightturns_distance as u64 == time_distance {
                        if lightturns_distance == 0 && e == g.emitter {
                            continue;
                        }
                        if let Some(ref mut sensor_storage) = ss {
                            (*sensor_storage).detections.push(DetectionInfo {
                                //depending on gameplay needs more complicated logic for sensors detection possible
                                position: gp.clone(),
                                turn: Turn(current_turn.0),
                            });
                        }
                    }
                }
            });
    }
}

struct State {
    ecs: World,
    view_translation: Point,
    time_last_turn: Instant,
    turn_compute_time: Duration,
}

impl State {
    fn new() -> Self {
        let mut w = World::new();
        w.register::<Point>();
        w.register::<Renderable>();
        w.register::<Player>();
        w.register::<SensorGhost>();
        w.register::<Sensor>();
        w.register::<SensorStorage>();
        w.register::<Detectable>();

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
                self.ecs.maintain();
                let mut detector = Detector {};
                detector.run_now(&self.ecs);

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
                ctx.set(
                    pos.x - self.view_translation.x + 1,
                    pos.y + self.view_translation.y + 1,
                    render.fg,
                    render.bg,
                    render.glyph,
                );
            }

            let current_turn = self.ecs.read_resource::<Turn>().0;

            //rendering detection info probably shoud be done using renderable component but I'm too lazy to write more systems right now
            let players = self.ecs.read_storage::<Player>();
            let sensor_storages = self.ecs.read_storage::<SensorStorage>();
            for (_p, s) in (&players, &sensor_storages).join() {
                let d = &s.detections;
                for di in d.iter().rev() {
                    if di.turn.0 != current_turn {
                        break;
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
fn main() -> BError {
    let context = BTermBuilder::simple(CONSOLE_WIDTH, CONSOLE_HEIGHT)
        .unwrap()
        .with_title("Lightspeed Sensor Demo")
        .build()?;

    let mut gs = State::new();

    gs.ecs
        .create_entity()
        .with(Point { x: 50, y: 50 })
        .with(Renderable {
            fg: RGB::from(BLUE),
            ..Default::default()
        })
        .with(Player {})
        .with(Sensor { max_range: 100 })
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
        .build();

    gs.time_last_turn = Instant::now();
    main_loop(context, gs)
}
