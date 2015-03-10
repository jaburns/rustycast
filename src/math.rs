
use std::ops::{Add, Sub, Mul, Neg, Div};
use std::num::Float;

// ---------------------------------------------------------------------------

#[derive(Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy)]
pub struct Mat3 {
    pub a00: f32, pub a01: f32, pub a02: f32,
    pub a10: f32, pub a11: f32, pub a12: f32,
    pub a20: f32, pub a21: f32, pub a22: f32,
}

#[derive(Copy)]
pub struct LineSeg {
    pub a: Vec2,
    pub b: Vec2,
}

pub const V2_ORIGIN: Vec2 = Vec2 {x:0.0, y:0.0};

pub const M3_IDENTITY: Mat3 = Mat3 {
    a00: 0.0, a01: 0.0, a02: 0.0,
    a10: 0.0, a11: 0.0, a12: 0.0,
    a20: 0.0, a21: 0.0, a22: 0.0,
};

// ----- Operator overloading ------------------------------------------------

impl Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Vec2 {
        Vec2 {x: self.x * rhs, y: self.y * rhs}
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Vec2 {
        Vec2 {x: self.x / rhs, y: self.y / rhs}
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 {x: -self.x, y: -self.y}
    }
}

impl Mul<Mat3> for Mat3 {
    type Output = Mat3;
    fn mul(self, rhs: Mat3) -> Mat3 {
        M3_IDENTITY //TODO
    }
}

impl Mul<Vec2> for Mat3 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        V2_ORIGIN //TODO
    }
}

// ---------------------------------------------------------------------------

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 {x: x, y: y}
    }

    pub fn get_length_sqr(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn get_length(self) -> f32 {
        Float::sqrt(self.get_length_sqr())
    }

    pub fn dot(self, rhs: Vec2) -> f32 {
        0.0 //TODO
    }

    pub fn cross(self, rhs: Vec2) -> f32 {
        0.0 //TODO
    }

    pub fn normalize(self) -> Vec2 {
        self / self.get_length()
    }
}

impl Mat3 {
    pub fn new(a00: f32, a01: f32, a02: f32,
               a10: f32, a11: f32, a12: f32,
               a20: f32, a21: f32, a22: f32) -> Mat3 {
        Mat3 { a00: a00, a01: a01, a02: a02,
                   a10: a10, a11: a11, a12: a12,
                   a20: a20, a21: a21, a22: a22 }
    }
}

impl LineSeg {
    pub fn intersects_at(self, rhs: LineSeg) -> Option<f32> {
        None // TODO
    }

    pub fn point_at(self, t: f32) -> Vec2 {
        V2_ORIGIN // TODO
    }
}

// ---------------------------------------------------------------------------

fn main() {
    let a = Vec2 {x: -50.0, y:-50.0};
    let b = Vec2 {x:  1.0, y: 2.0};
    let c = a + b;
    println!("{}, {}", c.x, c.y);
    //println!("{}, {}", (c / 2).x, -b.y);
}

