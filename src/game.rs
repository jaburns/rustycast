
use std::num::Float;
use sdl::video::{Surface};
use sdl::event::{Key};

use world::{World};
use math::{LineSeg, Vec2, Mat3};
use input::{InputState};


const SPEED: f32 = 0.4;
const TURN: f32 = 0.03;
const FOV_DIV: f32 = 500.0;
const WALL: f32 = 3000.0;


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
                  * Mat3::translation(-self.pos);

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

        surf.clear();
        surf.with_lock(|pixels| {
            for x in 0..w {
                let offset = ((x as f32) - (w as f32) / 2.0) / FOV_DIV;

                let (top, bottom, along) = match self.world.cast_ray(self.pos, self.face_angle + offset) {
                    Some((dist, _, along)) => {
                        let px_height = (WALL / (dist * Float::cos(offset))) as usize;
                        let top = if h > px_height { (h - px_height) / 2 } else { 0 };
                        (top, h-top, along)
                    }
                    None => { (h/2, h/2, 0.0) }
                };

                for y in 0..top      { put_px(pixels, w, x, y,  0x00, 0x66, 0xFF); }
                for y in bottom..h   { put_px(pixels, w, x, y,  0x00, 0xBB, 0x00); }

                let brightness = ((0xFF - top) as f32) / 255.0;
                for y in top..bottom {
                    let look_x = (along * 36.0) as u8;
                    let tex_lookup = ((along * 36.0) as u8)
                                   ^ ((255.0*(((y-top)as f32)/(bottom-top)as f32)) as u8);
                    let color = ((tex_lookup as f32) * brightness) as u8;
                    put_px(pixels, w, x, y, color, 0x00, 0x00);
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
