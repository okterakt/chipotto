use crate::cpu::Cpu;
use crate::framebuffer::FrameBuffer;
use crate::keypad::Keypad;

pub(crate) struct Chip8 {
    paused: bool,
    clock_hz: u32,
    pub cpu: Cpu,
    pub frame_buffer: FrameBuffer,
    pub keypad: Keypad,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            paused: false,
            clock_hz: 500,
            cpu: Cpu::new(),
            frame_buffer: FrameBuffer::default(),
            keypad: Keypad::default(),
        }
    }

    pub fn load_rom(&mut self, contents: &[u8]) {
        self.cpu.load_rom(contents);
    }

    pub fn cpu_cycle(&mut self) {
        self.cpu.cycle(&mut self.frame_buffer, &mut self.keypad);
    }

    pub fn timers_tick(&mut self) {
        self.cpu.update_timers();
    }

    pub fn pause() {
        unimplemented!()
    }

    pub fn resume() {
        unimplemented!()
    }
}
