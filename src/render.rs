
use sdl::video::{Surface};

use map::{Map};
use math::{LineSeg, Mat3};


pub fn render_map(surf: &Surface, map: &Map, theta: f32) {
    surf.clear();
    for wall in map.walls.iter() {
        let wall1 = wall.transform(Mat3::rotation(theta));
        draw_seg(&surf, &wall1);
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

