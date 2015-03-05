//!
//! Run `cargo build -v`. When it fails to compile main, get the failed command and append
//! the following to the failed rustc command:
//!
//!     -C link-args="-lSDLmain -lSDL -Wl,-framework,Cocoa"
//!
#![no_main]

extern crate sdl;

use sdl::video::{SurfaceFlag, VideoFlag};
use sdl::event::{Event};

use std::num::Float;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn SDL_main() {
    real_main()
}

pub fn real_main() {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("RustyCast", "RustyCast");

    let screen = match sdl::video::set_video_mode(320, 240, 32,
                                                  &[SurfaceFlag::HWSurface],
                                                  &[VideoFlag::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };

    let mut t = 0;

    'main : loop {
        'event : loop {
            match sdl::event::poll_event() {
                Event::Quit => break 'main,
                Event::None => break 'event,
                _ => {}
            }
        }

        screen.with_lock(|pixels| {
            for x in 0..320 {
                for y in 0..240 {
                    let fx = x as f32;
                    let fy = y as f32;
                    let dist = Float::sqrt(fx*fx + fy*fy) as u8;

                    let r = dist;
                    let g = 3*(y as u8)-0xFF-10*t;
                    let b = 7*t;

                    pixels[4*(320*y+x) + 0] = 0xFF;
                    pixels[4*(320*y+x) + 1] = r;
                    pixels[4*(320*y+x) + 2] = g;
                    pixels[4*(320*y+x) + 3] = b;
                }
            }
            true
        });

        t = t + 1;

        screen.flip();
    }

    sdl::quit();
}
