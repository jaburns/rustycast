
use std::num::Float;

use sdl2::surface::Surface;

use world::{RayCastResult};
use math::{LineSeg, Vec2, Mat3};
use game::{Game};


const MAP_SCALE: f32 = 2.0;
const FOV_DIV: f32 = 300.0;
const VISPLANE_DIST: f32 = 300.0;
const PERSON_HEIGHT: f32 = 5.0;


struct RenderContext<'a> {
    pub pixels: &'a mut [u8],
    pub width: isize,
    pub height: isize,
}


impl<'a> Game<'a> {
    pub fn render(&self, sky: &mut Surface, pixels: &mut [u8], w :usize, h: usize) {
        let mut ctx = RenderContext {
            pixels: pixels,
            width: w as isize,
            height: h as isize,
        };

        if self.show_map {
            self.render_map(&mut ctx);
        } else {
            self.render_game(sky, &mut ctx);
        }
    }

    fn render_map(&self, ctx: &mut RenderContext) {
        let trans = Mat3::rotation(-self.face_angle)
                  * Mat3::translation(-self.pos * MAP_SCALE)
                  * Mat3::scale(Vec2::new(MAP_SCALE, MAP_SCALE));

        ctx.clear();
        let (player_x, player_y) = (ctx.width / 2, ctx.height / 2);
        ctx.put_px(player_x as usize, player_y as usize, 0xFF, 0x00, 0x00);

        for wall in self.world.get_walls().iter() {
            ctx.draw_seg(wall.seg.transform(trans), 0xFF, 0x00, 0x00);
        }
    }

    fn render_game(&self, sky: &mut Surface, ctx: &mut RenderContext) {
        let person_height = PERSON_HEIGHT + 5.0; //+ Float::abs(Float::sin(self.t * 3.0)) * 10.0;
        let looking_offset = -self.look_angle as isize;
        let w = ctx.width as usize;
        let h = ctx.height as usize;

        for x in 0..w {
            let offset = ((x as f32) - (w as f32) / 2.0) / FOV_DIV;

            let mut render_bottom = h as isize;

            for RayCastResult {along, hit_pos, in_info, out_info}
            in self.world.cast_ray(0, self.pos, self.face_angle + offset) {
                let dist = (hit_pos - self.pos).get_length();

                let wall_seg_height = match out_info {
                    Some(i) => i.floor_elev - in_info.floor_elev,
                    None    => in_info.ceiling_elev - in_info.floor_elev,
                };

                let cast_dist = dist * Float::cos(offset);
                let pxheight = if wall_seg_height > 0.0 {
                    (VISPLANE_DIST * wall_seg_height / cast_dist) as isize
                } else {
                    0
                };

                let wall_bottom = h as isize / 2 + looking_offset + (VISPLANE_DIST * (person_height - in_info.floor_elev) / cast_dist) as isize;
                let wall_top = wall_bottom - pxheight;

                let mut wall_offset_bottom = wall_bottom - render_bottom;
                if wall_offset_bottom < 0 { wall_offset_bottom = 0; }
                let draw_wall_bottom = if wall_bottom > render_bottom { render_bottom } else { wall_bottom };

                ctx.draw_wall(x, wall_top, draw_wall_bottom, wall_offset_bottom, along, cast_dist);
                ctx.draw_floor(x, draw_wall_bottom, render_bottom, person_height - in_info.floor_elev, self.pos, hit_pos, offset, -looking_offset);

                render_bottom = wall_top;
            }
        }

        ctx.draw_seg(LineSeg::new(0.0, -3.0, 0.0, 4.0), 0xff, 0xff, 0xff);
        ctx.draw_seg(LineSeg::new(-3.0, 0.0, 4.0, 0.0), 0xff, 0xff, 0xff);
        ctx.put_px(160, 120, 0x00, 0x00, 0x00);
    }
}

/*

   //  sky
                if top > 0 && top < ctx.height {
                    for y in 0..top as usize {
                        sky.with_lock(|buffer| {
                            let (r,g,b) = get_px(buffer, x, y, w);
                            ctx.put_px(x, y, r, g, b);
                        });
                    }
                }
*/


pub fn get_px(tex: &mut [u8], x: usize, y: usize, w: usize) -> (u8,u8,u8) {
    (tex[3*(w*y+x)+0], tex[3*(w*y+x)+1], tex[3*(w*y+x)+2])
}

impl<'a> RenderContext<'a> {
    pub fn put_px(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        self.pixels[3*(self.width as usize*y+x) + 0] = r;
        self.pixels[3*(self.width as usize*y+x) + 1] = g;
        self.pixels[3*(self.width as usize*y+x) + 2] = b;
    }

    pub fn clear(&mut self) {
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
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

            if ux < self.width as usize && uy < self.height as usize {
                self.put_px(ux, uy, r, g, b);
            }
        }
    }

    pub fn draw_wall(&mut self, x: usize, top: isize, bottom: isize, y_offset: isize, along:f32, cast_dist: f32) {
        let cut_top = if top < 0 { 0 } else { top } as usize;
        let cut_bottom = if bottom > self.height { self.height } else { bottom } as usize;

        for y in cut_top..cut_bottom {
            let yy = ((bottom + y_offset) as usize - y) as f32 * cast_dist / 5000.0;
            let tex_lookup = (along * 25.0) as u8 ^ (512.0*yy) as u8;
            let color = ((tex_lookup as f32) * brightness_from_dist(cast_dist)) as u8;
            self.put_px(x, y, color / 2, color / 2, color);
        }
    }

    pub fn draw_floor(&mut self, x: usize, top: isize, bottom: isize, elevation: f32, pos: Vec2, hit_pos: Vec2, angle: f32, look: isize) {
        let cut_top = if top < self.height / 2 - look { self.height / 2 - look } else { top } as usize;
        let cut_bottom = if bottom > self.height { self.height } else { bottom } as usize;

        for y in cut_top..cut_bottom {
            let dist_floor = VISPLANE_DIST * elevation / ((y as isize + look) as f32 - self.height as f32 / 2.0);
            let floor_pos = pos + (hit_pos - pos).normalize() * dist_floor / Float::cos(angle);
            let tex_lookup = (floor_pos.x * 10.0) as u8 ^ (floor_pos.y * 10.0) as u8;
            let color = (tex_lookup as f32 * brightness_from_dist(dist_floor)) as u8;
            self.put_px(x, y, 0x00, color, 0x00);
        }
    }
}


fn brightness_from_dist(dist: f32) -> f32 {
    (20.0 / dist).min(1.0).max(0.0)
}

