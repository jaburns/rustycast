
use std::num::Float;
use sdl::video::{Surface};
use sdl::event::{Key};

use world::{World, RayCastResult};
use math::{LineSeg, Vec2, Mat3};
use input::{InputState};


const SPEED: f32 = 0.4;
const TURN: f32 = 0.03;

const FOV_DIV: f32 = 300.0;
const VISPLANE_DIST: f32 = 300.0;
const BASE_WALL_HEIGHT: f32 = 10.0;
const PERSON_HEIGHT: f32 = 5.0;


pub struct Game<'a> {
    pub pos: Vec2,
    pub face_angle: f32,
    pub world: &'a World,
    pub show_map: bool,
    pub t: f32
}


impl<'a> Game<'a> {
    pub fn step(&mut self, input: &InputState) {
        if input.mouse_dx() < -1.0 { self.face_angle -= TURN; }
        if input.mouse_dx() >  1.0 { self.face_angle += TURN; }

        if input.has_key(Key::W) { self.do_move( 1.0,  0.0); }
        if input.has_key(Key::S) { self.do_move(-1.0,  0.0); }
        if input.has_key(Key::A) { self.do_move( 0.0, -1.0); }
        if input.has_key(Key::D) { self.do_move( 0.0,  1.0); }

        self.show_map = input.has_key(Key::Tab);
        self.t += 0.02;
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
        for wall in self.world.get_walls().iter().map(|x| x.seg) {
            draw_seg(&surf, wall.transform(trans), 0xFF, 0x00, 0x00);
        }

        let w = surf.get_width() as usize;
        for x in 0..w {
            let offset = ((x as f32) - (w as f32) / 2.0) / FOV_DIV;

            self.world.cast_ray(self.pos, self.face_angle + offset).map(|RayCastResult {pos, ..}| {
                let draw_sg = LineSeg::new(self.pos.x, self.pos.y, pos.x, pos.y);
                draw_seg(&surf, draw_sg.transform(trans), 0x00, 0xFF, 0x00);
            });
        }
    }

    fn render_game(&self, surf: &Surface) {
        let w = surf.get_width() as usize;
        let h = surf.get_height() as usize;

        let person_height = PERSON_HEIGHT;// + Float::abs(Float::sin(self.t * 3.0)) * 10.0;

        surf.with_lock(|pixels| {
            for x in 0..w {
                let offset = ((x as f32) - (w as f32) / 2.0) / FOV_DIV;

                let cast = self.world.cast_ray(self.pos, self.face_angle + offset);
                if !cast.is_some() { continue; }
                let RayCastResult {dist, pos: hit_pos, along, height} = cast.unwrap();

                let cast_dist = dist * Float::cos(offset);

                let pxheight = (VISPLANE_DIST * height / cast_dist) as isize;
                let bottompx = h as isize/2 + (VISPLANE_DIST * person_height / cast_dist) as isize;
                let toppx = bottompx - pxheight;

                if toppx > h as isize/2 {
                    let topfloor = if toppx >= h as isize { h as isize } else { toppx };
                    for y in (h as isize/2)..topfloor {
                        let dist_floor = VISPLANE_DIST * (person_height - height) / ((y as f32) - (h as f32)/2.0);
                        let brightness = (20.0 / dist_floor).min(1.0).max(0.0);
                        let floor_pos = self.pos + (hit_pos - self.pos) * dist_floor / cast_dist;
                        let tex_lookup = ((floor_pos.x * 10.0) as u8) ^ ((floor_pos.y * 10.0) as u8);
                        let color = ((tex_lookup as f32) * brightness) as u8;
                        put_px(pixels, w, x, y as usize, 0x00, color, 0x00);
                    }
                }

                if toppx >= h as isize || bottompx < 0 {
                    continue;
                }

                let top = if toppx >= 0 { toppx as usize } else { 0 };
                let bottom = if bottompx < h as isize { bottompx as usize } else { h };

                let brightness = (20.0 / cast_dist).min(1.0).max(0.0);


                for y in top..bottom {
                    let yy = (bottompx as usize - y) as f32 / pxheight as f32 * height / 15.0;
                    let tex_lookup = (along * 25.0) as u8 ^ (512.0*yy) as u8;
                    let color = ((tex_lookup as f32) * brightness) as u8;
                    put_px(pixels, w, x, y, color / 2, color / 2, color);
                }

                for y in bottom..h {
                    let dist_floor = VISPLANE_DIST * person_height / ((y as f32) - (h as f32)/2.0);
                    let brightness = (20.0 / dist_floor).min(1.0).max(0.0);
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
