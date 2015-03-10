#![feature(old_path)]
#![feature(core)]
#![allow(dead_code)]

extern crate sdl;
extern crate sdl_image;

mod math;
mod map;
mod render;

use sdl::video::{SurfaceFlag, VideoFlag};
use sdl::event::{Event};
use sdl_image::{InitFlag};


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

    let map = map::temp_map();
    let mut theta = 0.0;

    'main : loop {
        'event : loop {
            match sdl::event::poll_event() {
                Event::Quit => break 'main,
                Event::None => break 'event,
                _ => {}
            }
        }

        theta += 0.01;
        render::render_map(&screen, &map, theta);

        screen.flip();
    }

    sdl::quit();
}

//    sdl_image::init(&[InitFlag::PNG]);
//    let img = match sdl_image::load(&Path::new("res/thing.png")) {
//        Ok(img) => img,
//        Err(err) => panic!("Failed to load image: {}", err)
//    };

