
use sdl::event;
use sdl::event::{Event};


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
}

fn map_sdl_key(key: event::Key) -> Option<Key> {
    match key {
        event::Key::W      => Some(Key::Forward),
        event::Key::S      => Some(Key::Back),
        event::Key::A      => Some(Key::Left),
        event::Key::D      => Some(Key::Right),
        event::Key::Tab    => Some(Key::ShowMap),
        event::Key::Escape => Some(Key::Quit),
        _ => None
    }
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
                match map_sdl_key(key) {
                    Some(key) => {
                        if down {
                            self._keys_down.push(key);
                            self._keys_down.dedup();
                        } else {
                            self._keys_down.retain(|&k| k != key);
                        }
                    }
                    None => {}
                }
            }
            Event::MouseMotion(_, _, _, dx, _) => {
                self._mouse_dx = dx as f32;
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
