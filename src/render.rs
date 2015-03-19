
use std::num::Float;

use world::{World, RayCastResult};
use math::{LineSeg, Vec2, Mat3};
use game::{Game};


const FOV_DIV: f32 = 300.0;
const VISPLANE_DIST: f32 = 300.0;
const PERSON_HEIGHT: f32 = 5.0;


impl<'a> Game<'a> {
    pub fn render(&self, pixels: &mut [u8], w :usize, h: usize) {
        if self.show_map {
            self.render_map(pixels, w, h);
        } else {
            self.render_game(pixels, w, h);
        }
    }

    fn render_map(&self, pixels: &mut [u8], w: usize, h: usize) {
        let trans = Mat3::rotation(-self.face_angle)
                  * Mat3::translation(-self.pos * 2.0)
                  * Mat3::scale(Vec2::new(2.0, 2.0));

        for x in 0..w {
            for y in 0..h {
                put_px(pixels, w ,x, y, 0x00, 0x00, 0x00);
            }
        }
        put_px(pixels, w, w/2, h/2, 0xFF, 0x00, 0x00);

        for wall in self.world.get_walls().iter().map(|x| x.seg) {
            draw_seg(pixels, w, h, wall.transform(trans), 0xFF, 0x00, 0x00);
        }
    }

    fn render_game(&self, pixels: &mut [u8], w: usize, h: usize) {
        let person_height = PERSON_HEIGHT + Float::abs(Float::sin(self.t)) * 10.0;

        for x in 0..w {
            let offset = ((x as f32) - (w as f32) / 2.0) / FOV_DIV;

            let cast = self.world.cast_ray(0, self.pos, self.face_angle + offset);
            if cast.is_empty() { continue; }
            let RayCastResult {dist, along, hit_pos, info} = cast[0];
            let height = 20.0;

            let cast_dist = dist * Float::cos(offset);

            let pxheight = (VISPLANE_DIST * height / cast_dist) as isize;
            let bottompx = h as isize/2 + (VISPLANE_DIST * person_height / cast_dist) as isize;
            let toppx_ = bottompx - pxheight;
            let toppx = if toppx_ >= h as isize { h as isize - 1 } else { toppx_ };

            for y in 0..toppx {
                put_px(pixels, w, x, y as usize, 0x00, 0x00, 0x00);
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
    }
}

fn put_px(pixels: &mut [u8], w: usize, x: usize, y: usize, r: u8, g: u8, b: u8) {
    pixels[3*(w*y+x) + 0] = r;
    pixels[3*(w*y+x) + 1] = g;
    pixels[3*(w*y+x) + 2] = b;
}

fn draw_seg(pixels: &mut [u8], w: usize, h: usize, seg: LineSeg, r: u8, g: u8, b: u8) {
    let len = seg.get_length();

    for t in 0..(len as usize) {
        let pt = seg.at(t as f32 / len);
        let ux = (pt.x + (w/2) as f32) as usize;
        let uy = (pt.y + (h/2) as f32) as usize;

        if ux < w && uy < h {
            put_px(pixels, w, ux, uy, r, g, b);
        }
    }
}

