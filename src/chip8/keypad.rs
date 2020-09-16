const KEY_COUNT: usize = 16;

pub struct Keypad {
    keys: [bool; KEY_COUNT],
}

impl Default for Keypad {
    fn default() -> Self {
        Keypad {
            keys: [false; KEY_COUNT],
        }
    }
}

impl Keypad {
    pub fn is_down(&self, idx: u8) -> bool {
        self.keys[idx as usize]
    }

    pub fn set_down(&mut self, idx: u8, down: bool) {
        self.keys[idx as usize] = down;
    }

    pub fn get_down_key(&self) -> Option<u8> {
        for (i, b) in self.keys.iter().enumerate() {
            if *b {
                return Some(i as u8);
            }
        }
        None
    }
}
