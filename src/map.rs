
use math::{LineSeg};

pub struct Map {
    pub walls: Vec<LineSeg>
}

pub fn temp_map() -> Map {
    Map {
        walls: vec![
            LineSeg::new(-100.0, -100.0,  100.0, -100.0),
            LineSeg::new( 100.0, -100.0,  100.0,  100.0),
            LineSeg::new( 100.0,  100.0, -100.0,  100.0),
            LineSeg::new(-100.0,  100.0, -100.0, -100.0)
        ]
    }
}

