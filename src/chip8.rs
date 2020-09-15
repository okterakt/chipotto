use crate::cpu::Cpu;
use crate::framebuffer::FrameBuffer;
use crate::instr::Instr;
use std::borrow::BorrowMut;
use std::fs;
use std::ops::Add;
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub(crate) struct Chip8 {
    paused: bool,
    clock_hz: u32,
    pub cpu: Cpu,
    pub frame_buffer: FrameBuffer,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            paused: false,
            clock_hz: 500,
            cpu: Cpu::new(),
            frame_buffer: FrameBuffer::default(),
        }
    }

    pub fn load_rom(&mut self, contents: &[u8]) {
        self.cpu.load_rom(contents);
    }

    pub fn cpu_cycle(&mut self) {
        self.cpu.cycle(&mut self.frame_buffer);
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
