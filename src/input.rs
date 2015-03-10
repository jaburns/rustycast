
use sdl::event::{Event, Key};

pub struct InputState {
    pub keys_down: Vec<Key>
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            keys_down: vec![]
        }
    }

    pub fn check_event(&mut self, event: &Event) {
        match *event {
            Event::Key(key, down, _, _) => {
                if down {
                    self.keys_down.push(key);
                    self.keys_down.dedup();
                } else {
                    self.keys_down.retain(|&k| k != key);
                }
                println!("{:?}", self.keys_down);
            }
            _ => {}
        }
    }
}
