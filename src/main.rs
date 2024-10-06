mod components;
mod core;
mod events;
mod physics;
mod renderer;
mod systems;
mod utils;

use core::{DeltaTime, Position};
use std::time::Duration;

use components::moveable_entity::MoveableEntity;
use events::movement_event::{Direction, MovementEvent};
use physics::components::speed::Speed;
use physics::systems::movement_event_handler::MovementEventHandler;
use physics::systems::movement_system::MovementSystem;
use systems::animator::Animator;
use systems::movement_animator::MovementAnimator;
use crate::components::animation::Animation;
use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use specs::World;
use specs::prelude::*;
use utils::divide_sheet_to_sprites;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let timer_subsystem = sdl_context.timer()?;
    let mut event_pump = sdl_context.event_pump()?;

    let _image_context = image::init(InitFlag::PNG);

    let window = video_subsystem.window("Platformer", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build()
        .expect("Could not make the canvas!");

    let mut world = World::new();
    world.insert(DeltaTime(0f32));
    world.insert(None as Option<MovementEvent>);

    let mut dispatcher = DispatcherBuilder::new()
        .with(MovementAnimator, "Movement Animator", &[])
        .with(Animator, "Animator", &["Movement Animator"])
        .with(MovementEventHandler, "Movement Event Handler", &[])
        .with(MovementSystem, "Movement System", &["Movement Event Handler"])
        .build();
    dispatcher.setup(&mut world);

    let texture_creator = canvas.texture_creator();
    let textures = [
        texture_creator.load_texture("assets/Character/Idle/Idle-Sheet.png")?,
        texture_creator.load_texture("assets/Character/Run/Run-Sheet.png")?,
    ];

    let idle_frames = divide_sheet_to_sprites(64, 256, 80, 0);
    let movement_frames = divide_sheet_to_sprites(80, 640, 80, 1);

    // TODO: use the new pattern!
    world.create_entity()
        .with(Position { x: 120f32, y: 120f32 })
        .with(MoveableEntity { movement_frames, idle_frames: idle_frames.clone(), current_direction: Direction::Stop })
        .with(Speed { x: 0f32, y: 0f32 })
        .with(idle_frames[0].clone())
        .with(Animation { frames: idle_frames, current_frame: 0, elapsed_time: 0f32 })
        .build();

    let mut now = timer_subsystem.performance_counter();
    let mut last_frame_time: u64;
    let mut current_direction = Direction::Stop;

    'running: loop {
        let mut movement_event = None;

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::LEFT), .. } => {
                    current_direction = Direction::Left;
                    movement_event = Some(MovementEvent(current_direction.clone()));
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    current_direction = Direction::Right;
                    movement_event = Some(MovementEvent(current_direction.clone()));
                }
                Event::KeyUp { keycode: Some(Keycode::LEFT), .. } => {
                    if current_direction == Direction::Left {
                        movement_event = Some(MovementEvent(Direction::Stop));
                    }
                }
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    if current_direction == Direction::Right {
                        movement_event = Some(MovementEvent(Direction::Stop));
                    }
                }
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        last_frame_time = now;
        now = timer_subsystem.performance_counter();
        *world.write_resource() = DeltaTime(
            ((now - last_frame_time) as f32 * 1000f32 /
            timer_subsystem.performance_frequency() as f32) /
            1000f32
        );
        *world.write_resource() = movement_event;

        dispatcher.dispatch(&mut world);
        world.maintain();

        renderer::render(&mut canvas, &textures, world.system_data())?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }

    Ok(())
}
