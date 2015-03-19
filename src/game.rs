
use std::num::Float;

use world::{World};
use math::{Vec2};
use input::{InputState, Key};


const SPEED: f32 = 0.4;
const TURN: f32 = 0.03;


pub struct Game<'a> {
    pub pos: Vec2,
    pub face_angle: f32,
    pub world: &'a World,
    pub show_map: bool,
    pub t: f32
}


impl<'a> Game<'a> {
    pub fn step(&mut self, input: &InputState) {
        self.face_angle += input.mouse_dx() / 1000.0;

        if input.has_key(Key::Forward) { self.do_move( 1.0,  0.0); }
        if input.has_key(Key::Back)    { self.do_move(-1.0,  0.0); }
        if input.has_key(Key::Left)    { self.do_move( 0.0, -1.0); }
        if input.has_key(Key::Right)   { self.do_move( 0.0,  1.0); }

        self.show_map = input.has_key(Key::ShowMap);
        self.t += 0.02;
    }

    fn do_move(&mut self, para: f32, perp: f32) {
        let sin = SPEED*Float::sin(self.face_angle);
        let cos = SPEED*Float::cos(self.face_angle);
        self.pos.x +=  sin*para + cos*perp;
        self.pos.y += -cos*para + sin*perp;
    }
}
