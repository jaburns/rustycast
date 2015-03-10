
use std::num::Float;
use sdl::video::{Surface};
use sdl::event::{Key};

use map::{Map};
use math::{LineSeg, Vec2, Mat3};
use input::{InputState};


pub struct Game<'a> {
    pub pos: Vec2,
    pub face_angle: f32,
    pub map: &'a Map,
}

impl<'a> Game<'a> {
    pub fn step(&mut self, input: &InputState) {
        if input.has_key(Key::Left) {
            self.face_angle -= 0.02;
        }
        if input.has_key(Key::Right) {
            self.face_angle += 0.02;
        }
        if input.has_key(Key::Up) {
            self.pos.x += Float::cos(self.face_angle - 3.1415926535 / 2.0);
            self.pos.y += Float::sin(self.face_angle - 3.1415926535 / 2.0);
        }
    }

    pub fn render(&self, surf: &Surface) {
        let trans = Mat3::rotation(-self.face_angle)
                  * Mat3::translation(-self.pos);

        surf.clear();
        for wall in self.map.walls.iter() {
            draw_seg(&surf, &wall.transform(trans));
        }
    }
}

fn draw_seg(surf: &Surface, seg: &LineSeg) {
    let w = surf.get_width() as usize;
    let h = surf.get_height() as usize;
    let len = seg.get_length();

    surf.with_lock(|pixels| {
        for t in 0..(len as usize) {
            let pt = seg.at(t as f32 / len);
            let ux = (pt.x + (w/2) as f32) as usize;
            let uy = (pt.y + (h/2) as f32) as usize;

            if ux < w && uy < h {
                pixels[3*(w*uy+ux) + 0] = 0xFF;
                pixels[3*(w*uy+ux) + 1] = 0xFF;
                pixels[3*(w*uy+ux) + 2] = 0xFF;
            }
        };
        true
    });
}
