
use sdl::video::{Surface};
use std::num::Float;

pub struct State {
    pub t: usize
}

impl State {
    pub fn step(&mut self) {
        self.t += 1;
    }

    pub fn draw(&self, surf: &Surface) {
        let w = surf.get_width() as usize;
        let h = surf.get_height() as usize;
        surf.with_lock(|pixels| {
            for x in 0..w {
                for y in 0..h {
                    let (r, g, b) = self.get_color(x, y);
                    pixels[3*(w*y+x) + 0] = b;
                    pixels[3*(w*y+x) + 1] = g;
                    pixels[3*(w*y+x) + 2] = r;
                }
            }
            true
        });
    }

    fn get_color(&self, x: usize, y: usize) -> (u8, u8, u8) {
        let fx = x as f32;
        let fy = y as f32;
        let dist = Float::sqrt(fx*fx + fy*fy) as u8;

        (dist,
            ((3*y+10*self.t) % 0x100) as u8,
            ((7*self.t) % 0x100) as u8)
    }
}

