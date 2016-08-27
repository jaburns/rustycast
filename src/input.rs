
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


#[derive(Copy,Clone,PartialEq)]
pub enum Key {
    Left,
    Right,
    Forward,
    Back,
    ShowMap,
    Quit,
}

pub struct InputState {
    _keys_down: Vec<Key>,
    _mouse_dx: f32,
    _mouse_dy: f32,
}


fn map_sdl_key(key: Keycode) -> Option<Key> {
    match key {
        Keycode::W      => Some(Key::Forward),
        Keycode::S      => Some(Key::Back),
        Keycode::A      => Some(Key::Left),
        Keycode::D      => Some(Key::Right),
        Keycode::Tab    => Some(Key::ShowMap),
        Keycode::Escape => Some(Key::Quit),
        _ => None
    }
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            _keys_down: vec![],
            _mouse_dx: 0.0,
            _mouse_dy: 0.0,
        }
    }

    pub fn check_event(&mut self, event: &Event) {
        match *event {
            Event::KeyDown { keycode, .. }  => {
                if let Some(key) = map_sdl_key(keycode.unwrap()) {
                    self._keys_down.push(key);
                    self._keys_down.dedup();
                }
            }
            Event::KeyUp { keycode, .. } => {
                if let Some(key) = map_sdl_key(keycode.unwrap()) {
                    self._keys_down.retain(|&k| k != key);
                }
            }
            Event::MouseMotion { xrel, yrel, .. } => {
                self._mouse_dx = xrel as f32;
                self._mouse_dy = yrel as f32;
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
    pub fn mouse_dy(&self) -> f32 {
        self._mouse_dy
    }

    pub fn has_key(&self, key: Key) -> bool {
        self._keys_down.iter().find(|&k| *k == key).is_some()
    }
}
