mod animations;
mod core;
mod events;
mod physics;
mod renderer;
mod utils;

use core::delta_time::DeltaTime;
use core::direction::Direction;
use std::time::Duration;

use animations::components::animation::Animation;
use animations::components::movement_animations::MovementAnimations;
use core::components::keyboard_controlled::KeyboardControlled;
use core::components::position::Position;
use events::movement_event::MovementEvent;
use physics::components::speed::Speed;
use physics::systems::movement_event_handler::MovementEventHandler;
use physics::systems::position_updater::PositionUpdater;
use animations::systems::animator::Animator;
use animations::systems::movement_animation_updater::MovementAnimationUpdater;
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
        .resizable()
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build()
        .expect("Could not make the canvas!");

    let mut world = World::new();
    world.insert(DeltaTime(0f32));
    world.insert(None as Option<MovementEvent>);

    let mut dispatcher = DispatcherBuilder::new()
        .with(MovementAnimationUpdater, "Movement Animation Updater", &[])
        .with(Animator, "Animator", &["Movement Animation Updater"])
        .with(MovementEventHandler, "Movement Event Handler", &[])
        .with(PositionUpdater, "Position Updater", &["Movement Event Handler"])
        .build();
    dispatcher.setup(&mut world);

    let texture_creator = canvas.texture_creator();
    let textures = [
        texture_creator.load_texture("assets/Background/Background.png")?,
        texture_creator.load_texture("assets/Character/Idle/Idle-Sheet.png")?,
        texture_creator.load_texture("assets/Character/Run/Run-Sheet.png")?,
    ];

    let idle_frames = divide_sheet_to_sprites(64, 256, 80, 1);
    let movement_frames = divide_sheet_to_sprites(80, 640, 80, 2);

    world.create_entity()
        .with(Position::new(120f32, 500f32))
        .with(KeyboardControlled)
        .with(MovementAnimations::new(movement_frames, idle_frames.clone(), Direction::Stop))
        .with(Speed::new(0f32, 0f32))
        .with(idle_frames[0].clone())
        .with(Animation::new(idle_frames, 0, 0f32))
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
