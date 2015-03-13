
use std::num::Float;
use sdl::video::{Surface};
use sdl::event::{Key};

use world::{World};
use math::{LineSeg, Vec2, Mat3};
use input::{InputState};


const SPEED: f32 = 0.4;
const TURN: f32 = 0.03;

const FOV_DIV: f32 = 600.0;
const VISPLANE_DIST: f32 = 600.0;
const WALL_HEIGHT: f32 = 10.0;
const PERSON_HEIGHT: f32 = 5.0;


pub struct Game<'a> {
    pub pos: Vec2,
    pub face_angle: f32,
    pub world: &'a World,
    pub show_map: bool,
}


impl<'a> Game<'a> {
    pub fn step(&mut self, input: &InputState) {
        if input.has_key(Key::Left)  { self.face_angle -= TURN; }
        if input.has_key(Key::Right) { self.face_angle += TURN; }

        if input.has_key(Key::W) { self.do_move( 1.0,  0.0); }
        if input.has_key(Key::S) { self.do_move(-1.0,  0.0); }
        if input.has_key(Key::A) { self.do_move( 0.0, -1.0); }
        if input.has_key(Key::D) { self.do_move( 0.0,  1.0); }

        self.show_map = input.has_key(Key::Tab);
    }

    fn do_move(&mut self, para: f32, perp: f32) {
        let sin = SPEED*Float::sin(self.face_angle);
        let cos = SPEED*Float::cos(self.face_angle);
        self.pos.x +=  sin*para + cos*perp;
        self.pos.y += -cos*para + sin*perp;
    }

    pub fn render(&self, surf: &Surface) {
        if self.show_map {
            self.render_map(&surf);
        } else {
            self.render_game(&surf);
        }
    }

    fn render_map(&self, surf: &Surface) {
        let trans = Mat3::rotation(-self.face_angle)
                  * Mat3::translation(-self.pos)
                  * Mat3::scale(Vec2::new(2.0, 2.0));

        surf.clear();
        for wall in self.world.walls.iter() {
            draw_seg(&surf, wall.transform(trans), 0xFF, 0x00, 0x00);
        }

        let w = surf.get_width() as usize;
        for x in 0..w {
            let offset = ((x as f32) - (w as f32) / 2.0) / FOV_DIV;

            self.world.cast_ray(self.pos, self.face_angle + offset).map(|(_, pos, _)| {
                let draw_sg = LineSeg::new(self.pos.x, self.pos.y, pos.x, pos.y);
                draw_seg(&surf, draw_sg.transform(trans), 0x00, 0xFF, 0x00);
            });
        }
    }

    fn render_game(&self, surf: &Surface) {
        let w = surf.get_width() as usize;
        let h = surf.get_height() as usize;

        surf.with_lock(|pixels| {
            for x in 0..w {
                let offset = ((x as f32) - (w as f32) / 2.0) / FOV_DIV;

                let cast = self.world.cast_ray(self.pos, self.face_angle + offset);
                if !cast.is_some() { continue; }
                let (dist, hit_pos, along) = cast.unwrap();

                let cast_dist = dist * Float::cos(offset);
                let px_height = (VISPLANE_DIST * WALL_HEIGHT / cast_dist) as usize;
                let top = if h > px_height { (h - px_height) / 2 } else { 0 };
                let bottom = h - top;

                let brightness = ((0xFF - top) as f32) / 255.0;
                for y in top..bottom {
                    let yy = (y as f32 - (h as f32 - px_height as f32) / 2.0) / px_height as f32;
                    let tex_lookup = (along * 25.0) as u8 ^ (255.0*yy) as u8;
                    let color = ((tex_lookup as f32) * brightness) as u8;
                    put_px(pixels, w, x, y, color / 2, color / 2, color);
                }

                for y in bottom..h {
                    let dist_floor = VISPLANE_DIST * PERSON_HEIGHT / ((y as f32) - (h as f32)/2.0);
                    let brightness = ((0xFF - (h-y)) as f32) / 255.0;
                    let floor_pos = self.pos + (hit_pos - self.pos) * dist_floor / cast_dist;
                    let tex_lookup = ((floor_pos.x * 10.0) as u8) ^ ((floor_pos.y * 10.0) as u8);
                    let color = ((tex_lookup as f32) * brightness) as u8;
                    put_px(pixels, w, x, y, 0x00, color, 0x00);
                }
            }
            true
        });
    }
}


fn put_px(pixels: &mut [u8], w: usize, x: usize, y: usize, r: u8, g: u8, b: u8) {
    pixels[3*(w*y+x) + 0] = b;
    pixels[3*(w*y+x) + 1] = g;
    pixels[3*(w*y+x) + 2] = r;
}

fn draw_seg(surf: &Surface, seg: LineSeg, r: u8, g: u8, b: u8) {
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
