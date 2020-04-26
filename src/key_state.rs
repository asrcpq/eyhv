#[derive(Debug)]
pub struct KeyState {
    // 0-4 = l u r d
    pub directions: [bool; 4],
    pub slowdown: bool,
}

impl KeyState {
    pub fn new() -> KeyState {
        KeyState {
            directions: [false; 4],
            slowdown: false,
        }
    }

    pub fn proc_key(&mut self, key_id: i8, updown: bool) {
        if key_id < 4 {
            self.directions[key_id as usize] = updown;
        } else if key_id == 4 {
            self.slowdown = updown;
        }
    }
}
