const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

pub struct FrameBuffer {
    pub buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Default for FrameBuffer {
    fn default() -> Self {
        FrameBuffer {
            buffer: [0u8; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }
}

impl FrameBuffer {
    pub fn clear(&mut self) {
        self.buffer.iter_mut().for_each(|pixel| *pixel = 0)
    }

    fn set_pixel(&mut self, x: usize, y: usize, v: u8) {
        self.buffer[y * SCREEN_WIDTH + x] = v;
    }

    fn get_pixel(&self, x: usize, y: usize) -> u8 {
        self.buffer[y * SCREEN_WIDTH + x]
    }

    pub fn draw(&mut self, x: u8, y: u8, data: &[u8]) -> bool {
        // each byte will represent a pixel on the screen; this means that when we get a byte
        // in input, we first need to transform each bit in a byte with values 0 or 1 (on/off).
        // one byte in input is equivalent to 8 bytes in the buffer.
        let mut collided = false;
        let mut row = 0;
        for byte in data.iter() {
            for col in 0..8 {
                let new_val = (byte >> (7 - col)) & 0x01;
                if new_val == 1 {
                    let x_idx = ((x + col) as usize) % SCREEN_WIDTH;
                    let y_idx = ((y + row) as usize) % SCREEN_HEIGHT;
                    let old_val = self.get_pixel(x_idx, y_idx);
                    if old_val == 1 {
                        collided = true;
                    }
                    self.set_pixel(x_idx, y_idx, new_val ^ old_val);
                }
            }
            row += 1;
        }
        collided
    }

    pub fn dump(&self) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let val = self.buffer[y * SCREEN_WIDTH + x];
                if val == 1 {
                    print!(".");
                } else {
                    print!("X");
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_pixel() {
        let mut frame_buffer = FrameBuffer::default();
        assert_eq!(0x00, frame_buffer.get_pixel(54, 13));
        frame_buffer.set_pixel(54, 13, 0x01);
        assert_eq!(0x01, frame_buffer.get_pixel(54, 13))
    }

    #[test]
    fn test_draw_and_collision() {
        let mut frame_buffer = FrameBuffer::default();
        let collided = frame_buffer.draw(60, 2, &[0b1001_0111]);
        assert_eq!(false, collided);
        assert_eq!(0x01, frame_buffer.get_pixel(60, 2));
        assert_eq!(0x00, frame_buffer.get_pixel(61, 2));
        assert_eq!(0x00, frame_buffer.get_pixel(62, 2));
        assert_eq!(0x01, frame_buffer.get_pixel(63, 2));
        assert_eq!(0x00, frame_buffer.get_pixel(0, 2));
        assert_eq!(0x01, frame_buffer.get_pixel(1, 2));
        assert_eq!(0x01, frame_buffer.get_pixel(2, 2));
        assert_eq!(0x01, frame_buffer.get_pixel(3, 2));

        let collided = frame_buffer.draw(62, 2, &[0b1001_0110]);
        assert_eq!(true, collided);
        assert_eq!(0x01, frame_buffer.get_pixel(62, 2));
        assert_eq!(0x01, frame_buffer.get_pixel(63, 2));
        assert_eq!(0x00, frame_buffer.get_pixel(0, 2));
        assert_eq!(0x00, frame_buffer.get_pixel(1, 2));
        assert_eq!(0x01, frame_buffer.get_pixel(2, 2));
        assert_eq!(0x00, frame_buffer.get_pixel(3, 2));
        assert_eq!(0x01, frame_buffer.get_pixel(4, 2));
        assert_eq!(0x00, frame_buffer.get_pixel(5, 2));
    }
}
