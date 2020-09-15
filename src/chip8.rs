use crate::cpu::Cpu;
use crate::framebuffer::FrameBuffer;
use crate::instr::Instr;
use std::fs;
use std::ops::Add;
use std::path::PathBuf;
use std::time::{Duration, Instant};

const TIMERS_INTERVAL_MICROS: u64 = 1_000_000 / 60;

struct Chip8 {
    paused: bool,
    clock_hz: u32,
    cpu: Cpu,
    frame_buffer: FrameBuffer,
    // other config options: bg and fg color (or maybe not part of this backend), different clock speed
}

impl Chip8 {
    pub fn new(file_path: &PathBuf) -> Self {
        Chip8 {
            paused: false,
            clock_hz: 500,
            cpu: Cpu::new(),
            frame_buffer: FrameBuffer::default(),
        }
    }

    pub fn load_rom(&mut self, filename: &PathBuf) {
        // TODO: return error to main
        let contents = fs::read(filename).expect("could not read the file");
        self.cpu.load_rom(contents.as_slice());
    }

    pub fn run(&mut self) {
        let mut last_cycle_update = Instant::now();
        let mut last_timers_update = Instant::now();
        let cycle_duration = Duration::from_micros((1_000_000 / self.clock_hz) as u64);
        let timers_duration = Duration::from_micros(TIMERS_INTERVAL_MICROS);
        while !self.paused {
            if last_cycle_update.elapsed() >= cycle_duration {
                self.cpu.cycle();
                last_cycle_update = Instant::now();
            }
            if last_timers_update.elapsed() >= timers_duration {
                self.cpu.update_timers();
                last_timers_update = Instant::now();
            }
        }
    }

    pub fn pause() {
        unimplemented!()
    }

    pub fn resume() {
        unimplemented!()
    }
}
