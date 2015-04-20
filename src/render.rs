
use std::num::Float;

use sdl2::surface::Surface;

use world::{RayCastResult};
use math::{LineSeg, Vec2, Mat3};
use game::{Game};
use core::ops::Range;

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
        let person_height = PERSON_HEIGHT + self.world.get_elevation(self.sector); //+ Float::abs(Float::sin(self.t * 3.0)) * 10.0;
        let looking_offset = -self.look_angle as isize;
        let w = ctx.width as usize;
        let h = ctx.height as usize;

        for x in 0..w {
            let offset = ((x as f32) - (w as f32) / 2.0) / FOV_DIV;
            let cos_offset = Float::cos(offset);

            let mut render_bottom = h as isize;
            let mut render_top = 0;

            for RayCastResult {along, hit_pos, in_info, out_info}
            in self.world.cast_ray(self.sector, self.pos, self.face_angle + offset) {
                let dist = (hit_pos - self.pos).get_length();
                let cast_dist = dist * cos_offset;

                let floor_wall_seg_height = match out_info {
                    Some(i) => i.floor_elev - in_info.floor_elev,
                    None    => in_info.ceiling_elev - in_info.floor_elev,
                };
                let floor_wall_seg_height_px = if floor_wall_seg_height > 0.0 {
                    (VISPLANE_DIST * floor_wall_seg_height / cast_dist) as isize
                } else {
                    0
                };

                let ceiling_wall_seg_height = match out_info {
                    Some(i) => in_info.ceiling_elev - i.ceiling_elev,
                    None    => 0.0
                };
                let ceiling_wall_seg_height_px = if ceiling_wall_seg_height > 0.0 {
                    (VISPLANE_DIST * ceiling_wall_seg_height / cast_dist) as isize
                } else {
                    0
                };

                let middle = h as isize / 2 + looking_offset;

                let floor_wall_bottom = middle + (VISPLANE_DIST * (person_height - in_info.floor_elev) / cast_dist) as isize;
                let floor_wall_top = floor_wall_bottom - floor_wall_seg_height_px;
                let floor_wall_offset_bottom = if floor_wall_bottom > render_bottom { floor_wall_bottom - render_bottom } else { 0 };

                let draw_floor_wall_top = if floor_wall_top < render_top { render_top } else { floor_wall_top };
                let draw_floor_wall_bottom = if floor_wall_bottom > render_bottom { render_bottom } else { floor_wall_bottom };

                ctx.draw_wall(x, draw_floor_wall_top, draw_floor_wall_bottom, floor_wall_offset_bottom, along, cast_dist);


                let ceiling_wall_top = middle + (VISPLANE_DIST * (person_height - in_info.ceiling_elev) / cast_dist) as isize;
                let ceiling_wall_bottom = ceiling_wall_top + ceiling_wall_seg_height_px;

                let draw_ceiling_wall_top = if ceiling_wall_top < render_top { render_top } else { ceiling_wall_top };
                let draw_ceiling_wall_bottom = if ceiling_wall_bottom > render_bottom { render_bottom } else { ceiling_wall_bottom };

                ctx.draw_wall(x, draw_ceiling_wall_top, draw_ceiling_wall_bottom, 0, along, cast_dist);
                ctx.draw_flat(x, draw_floor_wall_bottom, render_bottom, person_height - in_info.floor_elev, self.pos, hit_pos, cos_offset, -looking_offset);

                if in_info.ceiling_elev > 22.0 {
                    ctx.draw_sky(sky, x, render_top, draw_ceiling_wall_top);
                } else {
                    ctx.draw_flat(x, render_top, draw_ceiling_wall_top, person_height - in_info.ceiling_elev, self.pos, hit_pos, cos_offset, -looking_offset);
                }

                render_top = ceiling_wall_bottom;
                render_bottom = floor_wall_top;
            }
        }

        ctx.draw_seg(LineSeg::new(0.0, -3.0, 0.0, 4.0), 0xff, 0xff, 0xff);
        ctx.draw_seg(LineSeg::new(-3.0, 0.0, 4.0, 0.0), 0xff, 0xff, 0xff);
        ctx.put_px(160, 120, 0x00, 0x00, 0x00);
    }
}


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

    fn column_range(&mut self, top: isize, bottom: isize) ->Range<usize> {
        if bottom < 0 || top >= self.height {
            (0..0)
        } else {
            let cut_top = if top < 0 { 0 } else { top } as usize;
            let cut_bottom = if bottom > self.height { self.height } else { bottom } as usize;
            (cut_top..cut_bottom)
        }
    }

    pub fn draw_wall(&mut self, x: usize, top: isize, bottom: isize, y_offset: isize, along:f32, cast_dist: f32) {
        for y in self.column_range(top, bottom) {
            let yy = ((bottom + y_offset) as usize - y) as f32 * cast_dist / 5000.0;
            let tex_lookup = (along * 25.0) as u8 ^ (512.0*yy) as u8;
            let color = ((tex_lookup as f32) * brightness_from_dist(cast_dist)) as u8;
            self.put_px(x, y, color / 2, color / 2, color);
        }
    }

    pub fn draw_flat(&mut self, x: usize, top: isize, bottom: isize, elevation: f32, pos: Vec2, hit_pos: Vec2, cos_angle: f32, look: isize) {
        for y in self.column_range(top, bottom) {
            let dist_floor = VISPLANE_DIST * elevation / ((y as isize + look) as f32 - self.height as f32 / 2.0);
            let floor_pos = pos + (hit_pos - pos).normalize() * dist_floor / cos_angle;
            let tex_lookup = (floor_pos.x * 10.0) as u8 ^ (floor_pos.y * 10.0) as u8;
            let color = (tex_lookup as f32 * brightness_from_dist(dist_floor)) as u8;
            self.put_px(x, y, 0x00, color, 0x00);
        }
    }

    pub fn draw_sky(&mut self, sky: &mut Surface, x: usize, top: isize, bottom: isize) {
        for y in self.column_range(top, bottom) {
            sky.with_lock(|buffer| {
                let (r,g,b) = get_px(buffer, x, y, self.width as usize);
                self.put_px(x, y, r, g, b);
            });
        }
    }
}

fn brightness_from_dist(dist: f32) -> f32 {
    (20.0 / dist).min(1.0).max(0.0)
}

