
use std::num::Float;

use world::{World, RayCastResult};
use math::{LineSeg, Vec2, Mat3};
use game::{Game};


const MAP_SCALE: f32 = 2.0;
const FOV_DIV: f32 = 300.0;
const VISPLANE_DIST: f32 = 300.0;
const PERSON_HEIGHT: f32 = 5.0;


struct RenderContext<'a> {
    pub pixels: &'a mut [u8],
    pub width: usize,
    pub height: usize,
}


impl<'a> Game<'a> {
    pub fn render(&self, pixels: &mut [u8], w :usize, h: usize) {
        let mut ctx = RenderContext {
            pixels: pixels,
            width: w,
            height: h,
        };

        if self.show_map {
            self.render_map(&mut ctx);
        } else {
            self.render_game(&mut ctx);
        }
    }

    fn render_map(&self, ctx: &mut RenderContext) {
        let trans = Mat3::rotation(-self.face_angle)
                  * Mat3::translation(-self.pos * MAP_SCALE)
                  * Mat3::scale(Vec2::new(MAP_SCALE, MAP_SCALE));

        ctx.clear();
        let (player_x, player_y) = (ctx.width / 2, ctx.height / 2);
        ctx.put_px(player_x, player_y, 0xFF, 0x00, 0x00);

        for wall in self.world.get_walls().iter() {
            ctx.draw_seg(wall.seg.transform(trans), 0xFF, 0x00, 0x00);
        }
    }

    fn render_game(&self, ctx: &mut RenderContext) {
        let person_height = PERSON_HEIGHT + Float::abs(Float::sin(self.t)) * 10.0;
        let w = ctx.width;
        let h = ctx.height;

        for x in 0..w {
            let offset = ((x as f32) - (w as f32) / 2.0) / FOV_DIV;

            let cast = self.world.cast_ray(0, self.pos, self.face_angle + offset);
            if cast.is_empty() { continue; }
            let RayCastResult {dist, along, hit_pos, in_info, out_info} = cast[0];

            let height = match out_info {
                Some(i) => i.floor_elev - in_info.floor_elev,
                None    => in_info.ceiling_elev - in_info.floor_elev,
            };

            let cast_dist = dist * Float::cos(offset);
            let pxheight = (VISPLANE_DIST * height / cast_dist) as isize;

            let bottompx = h as isize/2 + (VISPLANE_DIST * person_height / cast_dist) as isize;
            let toppx_ = bottompx - pxheight;
            let toppx = if toppx_ >= h as isize { h as isize - 1 } else { toppx_ };

            for y in 0..toppx {
                ctx.put_px(x, y as usize, 0x00, 0x00, 0x00);
            }

            if toppx >= h as isize || bottompx < 0 {
                continue;
            }

            let top = if toppx >= 0 { toppx as usize } else { 0 };
            let bottom = if bottompx < h as isize { bottompx as usize } else { h };

            let brightness = brightness_from_dist(cast_dist);

            for y in top..bottom {
                let yy = (bottompx as usize - y) as f32 / pxheight as f32 * height / 15.0;
                let tex_lookup = (along * 25.0) as u8 ^ (512.0*yy) as u8;
                let color = ((tex_lookup as f32) * brightness) as u8;
                ctx.put_px(x, y, color / 2, color / 2, color);
            }

            ctx.draw_floor(x, bottom, h, person_height, self.pos, hit_pos, cast_dist);
        }
    }
}


impl<'a> RenderContext<'a> {
    pub fn put_px(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        self.pixels[3*(self.width*y+x) + 0] = r;
        self.pixels[3*(self.width*y+x) + 1] = g;
        self.pixels[3*(self.width*y+x) + 2] = b;
    }

    pub fn clear(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.put_px(x, y, 0x00, 0x00, 0x00);
            }
        }
    }

    pub fn draw_seg(&mut self, seg: LineSeg, r: u8, g: u8, b: u8) {
        let len = seg.get_length();

        for t in 0..(len as usize) {
            let pt = seg.at(t as f32 / len);
            let ux = (pt.x + (self.width /2) as f32) as usize;
            let uy = (pt.y + (self.height/2) as f32) as usize;

            if ux < self.width && uy < self.height {
                self.put_px(ux, uy, r, g, b);
            }
        }
    }

    pub fn draw_floor(&mut self, x: usize, top: usize, bottom: usize, elevation: f32, pos: Vec2, hit_pos: Vec2, cast_dist: f32) {
        for y in top..bottom {
            let dist_floor = VISPLANE_DIST * elevation / (y as f32 - self.height as f32 / 2.0);
            let floor_pos = pos + (hit_pos - pos) * dist_floor / cast_dist;
            let tex_lookup = (floor_pos.x * 10.0) as u8 ^ (floor_pos.y * 10.0) as u8;
            let color = (tex_lookup as f32 * brightness_from_dist(dist_floor)) as u8;
            self.put_px(x, y, 0x00, color, 0x00);
        }
    }
}


fn brightness_from_dist(dist: f32) -> f32 {
    (20.0 / dist).min(1.0).max(0.0)
}

