extern crate sdl;
extern crate sdl_image;

use self::sdl::video::{Surface, SurfaceFlag, VideoFlag};
use self::sdl::event::{Event};

use self::sdl_image::{InitFlag};

use state;

static WIDTH:  usize = 320;
static HEIGHT: usize = 240;

pub fn real_main() {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("RustyCast", "RustyCast");

    let screen = match sdl::video::set_video_mode(WIDTH as isize, HEIGHT as isize, 24,
                                                  &[SurfaceFlag::HWSurface],
                                                  &[VideoFlag::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };

    sdl_image::init(&[InitFlag::PNG]);

    let img = match sdl_image::load(&Path::new("thing.png")) {
        Ok(img) => img,
        Err(err) => panic!("Failed to load image")
    };

    let mut game_state = state::State { t: 0 };

    'main : loop {
        'event : loop {
            match sdl::event::poll_event() {
                Event::Quit => break 'main,
                Event::None => break 'event,
                _ => {}
            }
        }

        game_state.step();

        if (game_state.t / 100) % 2 == 0 {
            game_state.draw(&screen);
        } else {
            screen.blit(&img);
        }

        screen.flip();
    }

    sdl::quit();
}
