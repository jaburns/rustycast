
use std::f32;
use std::num::Float;

use math::{LineSeg, Vec2, V2_ORIGIN};


pub struct World {
    _sectors: Vec<Sector>,
}

pub struct Sector {
    pub info: SectorInfo,
    pub walls: Vec<Wall>,
}

#[derive(Copy)]
pub struct SectorInfo {
    pub floor_elev: f32,
    pub ceiling_elev: f32,
}

#[derive(Copy)]
pub struct Wall {
    pub seg: LineSeg,
    pub portal: Option<(usize,usize)>, // (sector index, wall index) for matching wall in another sector.
}

#[derive(Copy)]
pub struct RayCastResult {
    pub dist: f32,
    pub along: f32,
    pub hit_pos: Vec2,
    pub in_info: SectorInfo,
    pub out_info: Option<SectorInfo>,
}


pub static W_ZERO: Wall = Wall {
    seg: LineSeg {
        a: Vec2 { x: 0.0, y: 0.0 },
        b: Vec2 { x: 0.0, y: 0.0 },
    },
    portal: None
};


impl Wall {
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32, portal: Option<(usize,usize)>) -> Wall {
        Wall {
            seg: LineSeg::new(x0, y0, x1, y1),
            portal: portal,
        }
    }
}


impl World {
    pub fn get_walls(&self) -> &[Wall] {
        self._sectors[0].walls.as_slice()
    }

    pub fn cast_ray(&self, sector: usize, pos: Vec2, angle: f32) -> Vec<RayCastResult> {
        let mut result = vec![];
        self._cast_ray(sector, None, pos, angle, &mut result);
        result
    }

    fn _cast_ray(&self, sector: usize, source_wall: Option<usize>, pos: Vec2, angle: f32, results: &mut Vec<RayCastResult>) {
        let ray = LineSeg::new(
            pos.x, pos.y,
            pos.x + 1000.0*Float::sin(angle),
            pos.y - 1000.0*Float::cos(angle)
        );

        let (d2, wall, t) = self._sectors[sector].walls.iter()
            .filter_map(|&wall| ray.intersects_at(wall.seg).map(|t| (wall, t)))
            .fold((f32::MAX, W_ZERO, 0.0), |(s_d2, s_wall, s_t), (wall, t)| {
                let d2 = (pos - wall.seg.at(t)).get_length_sqr();
                if d2 < s_d2 { (d2, wall, t) } else { (s_d2, s_wall, s_t) }
            });

        if d2 < f32::MAX {
            results.push(RayCastResult {
                dist: (pos - wall.seg.at(t)).get_length(),
                along: wall.seg.get_length()*t,
                hit_pos: wall.seg.at(t),
                in_info: self._sectors[sector].info,
                out_info: wall.portal.map(|(sec,wall)| self._sectors[sec].info),
            });
        }
    }
}



pub fn temp() -> World {
    World {
        _sectors: vec![
            Sector {
                info: SectorInfo {
                    floor_elev: 0.0,
                    ceiling_elev: 20.0,
                },
                walls: vec![
                    Wall::new(-40.0, -40.0,  40.0, -40.0, None),
                    Wall::new( 40.0, -40.0,  40.0,  40.0, None),
                    Wall::new( 40.0,  40.0, -40.0,  40.0, None),
                    Wall::new(-40.0,  40.0, -40.0, -40.0, None),
                    Wall::new(-15.0, -15.0, -10.0, -15.0, Some((1,0))),
                    Wall::new(-10.0, -15.0, -10.0, -10.0, Some((1,1))),
                    Wall::new(-10.0, -10.0, -15.0, -10.0, Some((1,2))),
                    Wall::new(-15.0, -10.0, -15.0, -15.0, Some((1,3))),
                    Wall::new( 15.0,  15.0,  10.0,  15.0, Some((2,0))),
                    Wall::new( 10.0,  15.0,  10.0,  10.0, Some((2,1))),
                    Wall::new( 10.0,  10.0,  15.0,  10.0, Some((2,2))),
                    Wall::new( 15.0,  10.0,  15.0,  15.0, Some((2,3)))
                ]
            },
            Sector {
                info: SectorInfo {
                    floor_elev: 5.0,
                    ceiling_elev: 20.0,
                },
                walls: vec![
                    Wall::new(-15.0, -15.0, -10.0, -15.0, Some((0,4))),
                    Wall::new(-10.0, -15.0, -10.0, -10.0, Some((0,5))),
                    Wall::new(-10.0, -10.0, -15.0, -10.0, Some((0,6))),
                    Wall::new(-15.0, -10.0, -15.0, -15.0, Some((0,7))),
                ]
            },
            Sector {
                info: SectorInfo {
                    floor_elev: 5.0,
                    ceiling_elev: 20.0,
                },
                walls: vec![
                    Wall::new(-15.0, -15.0, -10.0, -15.0, Some((0,8))),
                    Wall::new(-10.0, -15.0, -10.0, -10.0, Some((0,9))),
                    Wall::new(-10.0, -10.0, -15.0, -10.0, Some((0,10))),
                    Wall::new(-15.0, -10.0, -15.0, -15.0, Some((0,11))),
                ]
            }
        ]
    }
}


