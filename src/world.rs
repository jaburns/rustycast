
use std::f32;
use std::num::Float;

use math::{LineSeg, Vec2, V2_ORIGIN};


pub struct World {
    _walls: Vec<LineSeg>
}

pub struct RayCastResult {
    pub dist: f32,
    pub pos: Vec2,
    pub along: f32,
}


pub fn temp() -> World {
    World {
        _walls: vec![
            LineSeg::new(-40.0, -40.0,  40.0, -40.0),
            LineSeg::new( 40.0, -40.0,  40.0,  40.0),
            LineSeg::new( 40.0,  40.0, -40.0,  40.0),
            LineSeg::new(-40.0,  40.0, -40.0, -40.0),
            LineSeg::new(-15.0, -15.0, -10.0, -15.0),
            LineSeg::new(-10.0, -15.0, -10.0, -10.0),
            LineSeg::new(-10.0, -10.0, -15.0, -10.0),
            LineSeg::new(-15.0, -10.0, -15.0, -15.0),
            LineSeg::new( 15.0,  15.0,  10.0,  15.0),
            LineSeg::new( 10.0,  15.0,  10.0,  10.0),
            LineSeg::new( 10.0,  10.0,  15.0,  10.0),
            LineSeg::new( 15.0,  10.0,  15.0,  15.0)
        ]
    }
}


impl World {
    pub fn get_walls(&self) -> &[LineSeg] {
        self._walls.as_slice()
    }

    pub fn cast_ray(&self, pos: Vec2, angle: f32) -> Option<RayCastResult> {
        let ray = LineSeg::new(
            pos.x, pos.y,
            pos.x + 1000.0*Float::sin(angle),
            pos.y - 1000.0*Float::cos(angle)
        );

        let (dist, pos, along) = self._walls.iter()
            .filter_map(|&wall| ray.intersects_at(wall).map(|t| (wall.at(t), wall.get_length()*t)))
            .map(|(int, along)| ((int - pos).get_length(), int, along))
            .fold((f32::MAX, V2_ORIGIN, 0.0), |(short_dist, short_pos, short_along), (dist, pos, along)| {
                if dist < short_dist { (dist, pos, along) } else { (short_dist, short_pos, short_along) }
            });

        if dist < f32::MAX {
            Some(RayCastResult {
                dist: dist,
                pos: pos,
                along: along
            })
        } else {
            None
        }
    }
}
