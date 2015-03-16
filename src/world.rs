
use std::f32;
use std::num::Float;

use math::{LineSeg, Vec2, V2_ORIGIN};


pub struct World {
    _walls: Vec<Wall>,
}

#[derive(Copy)]
pub struct Wall {
    pub seg: LineSeg,
    pub height :f32,
}

pub struct RayCastResult {
    pub dist: f32,
    pub pos: Vec2,
    pub along: f32,
    pub height: f32,
}


pub fn temp() -> World {
    World {
        _walls: vec![
            Wall::new(-40.0, -40.0,  40.0, -40.0, 10.0),
            Wall::new( 40.0, -40.0,  40.0,  40.0, 10.0),
            Wall::new( 40.0,  40.0, -40.0,  40.0, 10.0),
            Wall::new(-40.0,  40.0, -40.0, -40.0, 10.0),
            Wall::new(-15.0, -15.0, -10.0, -15.0, 20.0),
            Wall::new(-10.0, -15.0, -10.0, -10.0, 20.0),
            Wall::new(-10.0, -10.0, -15.0, -10.0, 20.0),
            Wall::new(-15.0, -10.0, -15.0, -15.0, 20.0),
            Wall::new( 15.0,  15.0,  10.0,  15.0, 20.0),
            Wall::new( 10.0,  15.0,  10.0,  10.0, 20.0),
            Wall::new( 10.0,  10.0,  15.0,  10.0, 20.0),
            Wall::new( 15.0,  10.0,  15.0,  15.0, 20.0)
        ]
    }
}


pub static W_ZERO: Wall = Wall {
    seg: LineSeg {
        a: Vec2 { x: 0.0, y: 0.0 },
        b: Vec2 { x: 0.0, y: 0.0 },
    },
    height: 0.0,
};


impl Wall {
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32, height: f32) -> Wall {
        Wall {
            seg: LineSeg::new(x0, y0, x1, y1),
            height: height,
        }
    }
}


impl World {
    pub fn get_walls(&self) -> &[Wall] {
        self._walls.as_slice()
    }

    pub fn cast_ray(&self, pos: Vec2, angle: f32) -> Option<RayCastResult> {
        let ray = LineSeg::new(
            pos.x, pos.y,
            pos.x + 1000.0*Float::sin(angle),
            pos.y - 1000.0*Float::cos(angle)
        );

        let (d2, wall, t) = self._walls.iter()
            .filter_map(|&wall| ray.intersects_at(wall.seg).map(|t| (wall, t)))
            .fold((f32::MAX, W_ZERO, 0.0), |(s_d2, s_wall, s_t), (wall, t)| {
                let d2 = (pos - wall.seg.at(t)).get_length_sqr();
                if d2 < s_d2 { (d2, wall, t) } else { (s_d2, s_wall, s_t) }
            });

        if d2 < f32::MAX {
            Some(RayCastResult {
                dist: (pos - wall.seg.at(t)).get_length(),
                pos: wall.seg.at(t),
                along: wall.seg.get_length()*t,
                height: wall.height
            })
        } else {
            None
        }
    }
}
