pub struct KeyState {
    // 0-4 = l u r d
    pub directions: [bool; 4],
}

impl KeyState {
    pub fn new() -> KeyState {
        KeyState {
            directions: [false; 4],
        }
    }

    pub fn proc_key(&mut self, key_id: i8, updown: bool) {
        self.directions[key_id as usize] = updown;
    }
}
