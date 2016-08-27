use world::{World, SectorIndex};
use math::{Vec2};
use input::{InputState, Key};


const SPEED: f32 = 0.3;
const TURN: f32 = 0.03;


pub struct Game<'a> {
    pub sector: SectorIndex,
    pub pos: Vec2,
    pub face_angle: f32,
    pub look_angle: f32,
    pub world: &'a World,
    pub show_map: bool,
    pub t: f32
}


impl<'a> Game<'a> {
    pub fn step(&mut self, input: &InputState) {
        self.face_angle += input.mouse_dx() / 500.0;
        self.look_angle += input.mouse_dy() / 2.0;

        self.look_angle = self.look_angle.min(120.0).max(-120.0);

        if input.has_key(Key::Forward) { self.do_move( 1.0,  0.0); }
        if input.has_key(Key::Back)    { self.do_move(-1.0,  0.0); }
        if input.has_key(Key::Left)    { self.do_move( 0.0, -1.0); }
        if input.has_key(Key::Right)   { self.do_move( 0.0,  1.0); }

        self.show_map = input.has_key(Key::ShowMap);
        self.t += 0.02;
    }

    fn do_move(&mut self, para: f32, perp: f32) {
        let sin = SPEED*self.face_angle.sin();
        let cos = SPEED*self.face_angle.cos();

        let new_pos = self.pos + Vec2::new(
             sin*para + cos*perp,
            -cos*para + sin*perp
        );

        self.sector = self.world.move_object(self.sector, self.pos, new_pos);
        self.pos = new_pos;
    }
}
