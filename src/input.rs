
use sdl::event::{Event, Key};

pub struct InputState {
    _keys_down: Vec<Key>,
    _mouse_dx: f32,
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            _keys_down: vec![],
            _mouse_dx: 0.0,
        }
    }

    pub fn check_event(&mut self, event: &Event) {
        match *event {
            Event::Key(key, down, _, _) => {
                if down {
                    self._keys_down.push(key);
                    self._keys_down.dedup();
                } else {
                    self._keys_down.retain(|&k| k != key);
                }
            }
            Event::MouseMotion(_, _, _, dx, _) => {
                self._mouse_dx = dx as f32;
                println!("{}", dx);
            }
            _ => {}
        }
    }

    pub fn clear_mouse(&mut self) {
        self._mouse_dx = 0.0
    }

    pub fn mouse_dx(&self) -> f32 {
        self._mouse_dx
    }

    pub fn has_key(&self, key: Key) -> bool {
        self._keys_down.iter().find(|&k| *k == key).is_some()
    }
}
