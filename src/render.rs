
use std::num::Float;
use sdl::video::{Surface};

use map::{Map};
use math::{LineSeg, Vec2, Mat3};


pub fn render_map(surf: &Surface, map: &Map) {
    surf.clear();
    for wall in map.walls.iter() {
        draw_seg(&surf, &wall);
    }
}

fn draw_seg(surf: &Surface, seg: &LineSeg) {
    let w = surf.get_width() as usize;
    let h = surf.get_height() as usize;
    let len = seg.get_length();
    let ulen = len as usize;

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

