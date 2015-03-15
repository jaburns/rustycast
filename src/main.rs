#![allow(dead_code)]
#![feature(old_path)]
#![feature(core)]

extern crate sdl;
extern crate sdl_image;

mod math;
mod world;
mod game;
mod input;

use sdl::video::{SurfaceFlag, VideoFlag};
use sdl::event::{Event, Key};
use sdl_image::{InitFlag};


const WIDTH:  usize = 640;
const HEIGHT: usize = 480;


fn main() {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("RustyCast", "RustyCast");

    let screen = match sdl::video::set_video_mode(WIDTH as isize, HEIGHT as isize, 24,
                                                  &[SurfaceFlag::HWSurface],
                                                  &[ VideoFlag::DoubleBuf]) {
        //VideoFlag::Fullscreen,
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };

    sdl_image::init(&[InitFlag::PNG]);
    let sky = match sdl_image::load(&Path::new("res/sky.png")) {
        Ok(img) => img,
        Err(err) => panic!("Failed to load image: {}", err)
    };

    let mut inputs = input::InputState::new();

    let mut game = game::Game {
        pos: math::V2_ORIGIN,
        face_angle: 0.0,
        world: &world::temp(),
        show_map: false
    };

    'main : loop {
        'event : loop {
            let event = sdl::event::poll_event();
            inputs.check_event(&event);

            match event {
                Event::Quit => break 'main,
                Event::None => break 'event,
                _ => {}
            }
        }

        if inputs.has_key(Key::Escape) {
            break 'main;
        }

        screen.blit(&sky);

        game.step(&inputs);
        game.render(&screen);

        screen.flip();
    }

    sdl::quit();
}


