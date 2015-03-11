
use sdl::event::{Event, Key};

pub struct InputState {
    _keys_down: Vec<Key>
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            _keys_down: vec![]
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
            _ => {}
        }
    }

    pub fn has_key(&self, key: Key) -> bool {
        self._keys_down.iter().find(|&k| *k == key).is_some()
    }
}
