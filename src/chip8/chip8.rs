use super::cpu::Cpu;
use super::framebuffer::FrameBuffer;
use super::keypad::Keypad;
use super::memory::Memory;

pub struct Chip8 {
    paused: bool,
    pub cpu: Cpu,
    pub frame_buffer: FrameBuffer,
    pub memory: Memory,
    pub keypad: Keypad,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            paused: false,
            cpu: Cpu::new(),
            frame_buffer: FrameBuffer::default(),
            memory: Memory::new(),
            keypad: Keypad::default(),
        }
    }

    pub fn load_rom(&mut self, contents: &[u8]) {
        self.memory.load_rom(contents);
    }

    pub fn cpu_cycle(&mut self) {
        self.cpu
            .cycle(&mut self.frame_buffer, &mut self.memory, &mut self.keypad);
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
