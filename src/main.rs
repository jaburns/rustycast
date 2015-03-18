#![allow(dead_code)]
#![feature(core)]

extern crate sdl2;
extern crate time;

mod math;
mod world;
mod game;
mod input;

use std::time::Duration;
use std::path::Path;

use time::PreciseTime;

use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::mouse;
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keycode::KeyCode;

use std::rand;

const WINDOW_WIDTH  :i32 = 320;
const WINDOW_HEIGHT :i32 = 240;

const W :usize = 320;
const H :usize = 240;

pub fn main() {
    let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();

    let window = match Window::new("RustyCast", WindowPos::PosCentered, WindowPos::PosCentered, WINDOW_WIDTH, WINDOW_HEIGHT, OPENGL) {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    window.set_grab(true);
    mouse::set_relative_mouse_mode(true);

    let renderer = match Renderer::from_window(window, RenderDriverIndex::Auto, ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, (W as i32, H as i32)).unwrap();
    let mut drawer = renderer.drawer();
    let mut event_pump = sdl_context.event_pump();


    let mut inputs = input::InputState::new();
    let mut game = game::Game {
        pos: math::V2_ORIGIN,
        face_angle: 0.0,
        world: &world::temp(),
        show_map: false,
        t: 0.0
    };

    'main : loop {
        let last_time = PreciseTime::now();

        for event in event_pump.poll_iter() {
            inputs.check_event(&event);
            match event {
                Event::Quit {..} => { break 'main; },
                _ => {}
            }
        }

        if (inputs.has_key(input::Key::Quit)) {
            break 'main;
        }

        game.step(&inputs);
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            game.render(buffer, W, H);
        }).unwrap();

        drawer.copy(&texture, None, Some(Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT)));
        drawer.present();

        println!("{}", last_time.to(PreciseTime::now()).num_milliseconds());
    }
}

