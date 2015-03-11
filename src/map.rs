
use math::{LineSeg};

pub struct Map {
    pub walls: Vec<LineSeg>
}

pub fn temp_map() -> Map {
    Map {
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

