#![allow(dead_code)]

extern crate core;
extern crate sdl2;
extern crate sdl2_image;

mod math;
mod world;
mod game;
mod input;
mod render;

use std::thread;
use std::time::{Instant, Duration};
use std::path::Path;

use sdl2::render::{BlendMode};
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2_image::LoadSurface;


const WINDOW_WIDTH  :u32 = 1 * 320;
const WINDOW_HEIGHT :u32 = 1 * 240;

const W :usize = 320;
const H :usize = 240;

const FRAME_TIME_MS :u64 = 17;


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let res_path = Path::new("/Users/jaburns/dev/rustycast/res");

    let window = video_subsystem.window("RustyCast", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    sdl_context.mouse().set_relative_mouse_mode(true);

    let mut renderer = window.renderer().build().unwrap();

    let mut sky = LoadSurface::from_file(&res_path.join("sky.png")).unwrap();

    let mut texture = renderer.create_texture_streaming(PixelFormatEnum::ARGB8888, W as u32, H as u32).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut inputs = input::InputState::new();
    let mut game = game::Game {
        sector: world::SectorIndex(0),
        pos: math::V2_ORIGIN,
        face_angle: 0.0,
        look_angle: 0.0,
        world: &world::temp(),
        show_map: false,
        t: 0.0
    };

    texture.set_blend_mode(BlendMode::None);

    // Wasted mode
    // texture.set_alpha_mod(0x22);
    // texture.set_blend_mode(BlendMode::Blend);

    'main : loop {
        let last_time = Instant::now();

        for event in event_pump.poll_iter() {
            inputs.check_event(&event);
            if let Event::Quit {..} = event { break 'main; }
        }

        if inputs.has_key(input::Key::Quit) {
            break 'main;
        }

        game.step(&inputs);
        texture.with_lock(None, |buffer, _| {
            game.render(&mut sky, buffer, W, H);
        }).unwrap();

        renderer.copy(&texture, None, None);
        renderer.present();

        let delta_time = ((Instant::now() - last_time).subsec_nanos() / 1000000) as u64;

        if delta_time < FRAME_TIME_MS {
            thread::sleep(Duration::from_millis(FRAME_TIME_MS - delta_time));
        }
    }
}

