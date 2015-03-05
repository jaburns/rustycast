extern crate sdl;
extern crate rand;

use self::sdl::video::{SurfaceFlag, VideoFlag};
use self::sdl::event::{Event};

static WIDTH:  usize = 320;
static HEIGHT: usize = 240;

fn get_color() -> (u8, u8, u8) {
    (rand::random::<u8>(),
     rand::random::<u8>(),
     rand::random::<u8>())
}

pub fn real_main() {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("RustyCast", "RustyCast");

    let screen = match sdl::video::set_video_mode(WIDTH as isize, HEIGHT as isize, 32,
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
            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    let (r, g, b) = get_color();
                    pixels[4*(WIDTH*y+x) + 0] = 0xFF;
                    pixels[4*(WIDTH*y+x) + 1] = r;
                    pixels[4*(WIDTH*y+x) + 2] = g;
                    pixels[4*(WIDTH*y+x) + 3] = b;
                }
            }
            true
        });

        t = t + 1;

        screen.flip();
    }

    sdl::quit();
}
