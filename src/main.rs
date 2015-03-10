#![allow(dead_code)]

extern crate sdl;
extern crate sdl_image;

mod math;
mod map;
mod game;
mod input;

use sdl::video::{SurfaceFlag, VideoFlag};
use sdl::event::{Event};


const WIDTH:  usize = 320;
const HEIGHT: usize = 240;


fn main() {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("RustyCast", "RustyCast");

    let screen = match sdl::video::set_video_mode(WIDTH as isize, HEIGHT as isize, 24,
                                                  &[SurfaceFlag::HWSurface],
                                                  &[VideoFlag::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };

    let mut inputs = input::InputState::new();

    let mut game = game::Game {
        pos: math::V2_ORIGIN,
        face_angle: 0.0,
        map: &map::temp_map(),
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

        game.step(&inputs);
        game.render(&screen);

        screen.flip();
    }

    sdl::quit();
}

//    use sdl_image::{InitFlag};
//    sdl_image::init(&[InitFlag::PNG]);
//    let img = match sdl_image::load(&Path::new("res/thing.png")) {
//        Ok(img) => img,
//        Err(err) => panic!("Failed to load image: {}", err)
//    };

