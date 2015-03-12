
use std::f32;
use std::num::Float;

use math::{LineSeg, Vec2, V2_ORIGIN};


pub struct World {
    pub walls: Vec<LineSeg>
}


pub fn temp() -> World {
    World {
        walls: vec![
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
    pub fn cast_ray(&self, pos: Vec2, angle: f32) -> Option<(f32, Vec2, f32)> {
        let ray = LineSeg::new(
            pos.x, pos.y,
            pos.x + 1000.0*Float::sin(angle),
            pos.y - 1000.0*Float::cos(angle)
        );

        let (dist, pos, along) = self.walls.iter()
            .filter_map(|&wall| ray.intersects_at(wall).map(|t| (wall.at(t), wall.get_length()*t)))
            .map(|(int, along)| ((int - pos).get_length(), int, along))
            .fold((f32::MAX, V2_ORIGIN, 0.0), |(short_dist, short_pos, short_along), (dist, pos, along)| {
                if dist < short_dist { (dist, pos, along) } else { (short_dist, short_pos, short_along) }
            });

        if dist < f32::MAX { Some((dist, pos, along)) } else { None }
    }
}
