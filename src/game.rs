
use std::num::Float;
use sdl::video::{Surface};
use sdl::event::{Key};

use map::{Map};
use math::{LineSeg, Vec2, Mat3};
use input::{InputState};
use math;


pub struct Game<'a> {
    pub pos: Vec2,
    pub face_angle: f32,
    pub map: &'a Map,
    pub show_map: bool,
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
            self.pos.x += Float::sin(self.face_angle);
            self.pos.y -= Float::cos(self.face_angle);
        }
        if input.has_key(Key::Down) {
            self.pos.x -= Float::sin(self.face_angle);
            self.pos.y += Float::cos(self.face_angle);
        }
        self.show_map = input.has_key(Key::Tab);
    }

    pub fn render(&self, surf: &Surface) {
        if self.show_map {
            self.render_lines(&surf);
        } else {
            self.render_cast(&surf);
        }
    }

    fn render_lines(&self, surf: &Surface) {
        let trans = Mat3::rotation(-self.face_angle)
                  * Mat3::translation(-self.pos);

        surf.clear();
        for wall in self.map.walls.iter() {
            draw_seg(&surf, &wall.transform(trans), 0xFF, 0x00, 0x00);
        }


        let w = surf.get_width() as usize;
        for x in 0..w {
            let offset = ((x as f32) - (w as f32) / 2.0) / 600.0;
            let (q1,q2) = self.cast_ray(offset);
            match q1 {
                Some(dist) => {
                    let hit = Vec2::new(
                         Float::sin(self.face_angle),
                        -Float::cos(self.face_angle)
                    ) * dist;
                    let draw_sg = LineSeg::new(self.pos.x, self.pos.y, q2.x, q2.y);
                    draw_seg(&surf, &draw_sg.transform(trans), 0x00, 0xFF, 0x00);
                }
                None => {}
            }
        }
    }

    fn render_cast(&self, surf: &Surface) {
        let w = surf.get_width() as usize;
        let h = surf.get_height() as usize;

        surf.clear();
        surf.with_lock(|pixels| {
            for x in 0..w {
                let offset = ((x as f32) - (w as f32) / 2.0) / 600.0;
                let (q1,q2) = self.cast_ray(offset);
                match q1 {
                    Some(dist) => {
                        let ray_height = (3000.0 / dist) as usize;
                        let top = if h > ray_height { (h - ray_height) / 2 } else { 0 };
                        let bottom = h - top;
                        let proto_color = (0xFF-(h/2) + ray_height/2);
                        let color = if proto_color > 0xFF { 0xFF } else { proto_color as u8 };
                        for y in 0..top {
                            put_px(pixels, w, x, y, 0, 0x66, 0xFF);
                        }
                        for y in top..bottom {
                            put_px(pixels, w, x, y, color, 0, 0);
                        }
                        for y in bottom..h {
                            put_px(pixels, w, x, y, 0, 0xBB, 0);
                        }
                    }
                    None => {
                        for y in 0..(h/2) {
                            put_px(pixels, w, x, y, 0, 0x66, 0xFF);
                        }
                        for y in (h/2)..h {
                            put_px(pixels, w, x, y, 0, 0xBB, 0);
                        }
                    }
                }
            }
            true
        });
    }

    fn cast_ray(&self, offset: f32) -> (Option<f32>, Vec2) {
        let ray = LineSeg::new(
            self.pos.x,
            self.pos.y,
            self.pos.x + 1000.0*Float::sin(self.face_angle + offset),
            self.pos.y - 1000.0*Float::cos(self.face_angle + offset)
        );

        let face_vec = Vec2::new(
             Float::sin(self.face_angle),
            -Float::cos(self.face_angle)
        );

        let mut shortest: Option<f32> = None;
        let mut vv: Vec2 = math::V2_ORIGIN;

        for wall in self.map.walls.iter() {
            match ray.intersects_at(*wall) {
                Some(t) => {
                    let dist = (wall.at(t) - self.pos).project(face_vec).get_length();
                    shortest = match shortest {
                        Some(d) => { if dist < d { vv = wall.at(t); Some(dist) } else { shortest } }
                        None    => { vv = wall.at(t); Some(dist) }
                    }
                }
                None => {}
            };
        }

        (shortest, vv)
    }
}

fn put_px(pixels: &mut [u8], w: usize, x: usize, y: usize, r: u8, g: u8, b: u8) {
    pixels[3*(w*y+x) + 0] = b;
    pixels[3*(w*y+x) + 1] = g;
    pixels[3*(w*y+x) + 2] = r;
}

fn draw_seg(surf: &Surface, seg: &LineSeg, r: u8, g: u8, b: u8) {
    let w = surf.get_width() as usize;
    let h = surf.get_height() as usize;
    let len = seg.get_length();

    surf.with_lock(|pixels| {
        for t in 0..(len as usize) {
            let pt = seg.at(t as f32 / len);
            let ux = (pt.x + (w/2) as f32) as usize;
            let uy = (pt.y + (h/2) as f32) as usize;

            if ux < w && uy < h {
                put_px(pixels, w, ux, uy, r, g, b);
            }
        };
        true
    });
}
