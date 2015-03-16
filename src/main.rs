#![allow(dead_code)]
#![feature(core)]

extern crate sdl;
extern crate time;
extern crate sdl_image;

mod math;
mod world;
mod game;
mod input;

use std::time::Duration;

use time::PreciseTime;

use sdl::video::{SurfaceFlag, VideoFlag};
use sdl::event::{Event, Key};
use sdl::wm::{GrabMode};
use std::path::{Path};
use sdl_image::{InitFlag};


const WIDTH:  usize = 320;
const HEIGHT: usize = 240;


fn main() {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("RustyCast", "RustyCast");

    let screen = match sdl::video::set_video_mode(WIDTH as isize, HEIGHT as isize, 24,
                                                  &[SurfaceFlag::HWSurface],
                                                  &[
                                                   // VideoFlag::Fullscreen,
                                                    VideoFlag::DoubleBuf]) {
        //VideoFlag::Fullscreen,
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };

    sdl::wm::grab_input(GrabMode::On);
    sdl::mouse::set_cursor_visible(false);

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
        show_map: false,
        t: 0.0
    };


    'main : loop {
        //let mut last_time = PreciseTime::now();

        inputs.clear_mouse();
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

        //println!("{}", last_time.to(PreciseTime::now()).num_milliseconds());
    }

    sdl::quit();
}


