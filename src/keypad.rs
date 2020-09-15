const KEY_COUNT: usize = 16;

struct Keypad {
    keys: [bool; KEY_COUNT]
}

impl Keypad {
    pub fn is_pressed(&self, idx: u8) -> bool {
        self.keys[idx as usize]
    }

    pub fn set_pressed(&mut self, idx: u8, pressed: bool) {
        self.keys[idx as usize] = pressed;
    }

    pub fn get_pressed_key(&self) -> Option<u8> {
        for (i, b) in self.keys.iter().enumerate() {
            if b {
                Some(i as u8)
            }
        }
        None
    }
}
