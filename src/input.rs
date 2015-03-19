
use sdl2::event::Event;
use sdl2::keycode::KeyCode;


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


fn map_sdl_key(key: KeyCode) -> Option<Key> {
    match key {
        KeyCode::W      => Some(Key::Forward),
        KeyCode::S      => Some(Key::Back),
        KeyCode::A      => Some(Key::Left),
        KeyCode::D      => Some(Key::Right),
        KeyCode::Tab    => Some(Key::ShowMap),
        KeyCode::Escape => Some(Key::Quit),
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
                match map_sdl_key(keycode) {
                    Some(key) => {
                        self._keys_down.push(key);
                        self._keys_down.dedup();
                    }
                    None => {}
                }
            }
            Event::KeyUp { keycode, .. } => {
                match map_sdl_key(keycode) {
                    Some(key) => {
                        self._keys_down.retain(|&k| k != key);
                    }
                    None => {}
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
