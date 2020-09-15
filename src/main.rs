use chip8::Chip8;
use clap::{App, Arg};
use minifb::{Scale, ScaleMode, Window, WindowOptions};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

mod chip8;
mod cpu;
mod framebuffer;
mod instr;
mod memory;
mod keypad;

const WINDOW_WIDTH: usize = 64;
const WINDOW_HEIGHT: usize = 32;

const TIMERS_INTERVAL_MICROS: u64 = 1_000_000 / 60;

fn main() {
    let matches = App::new("Chip-8")
        .version("0.1")
        .author("okterakt")
        .about("Simple chip-8 emulator")
        .arg(Arg::with_name("ROM_FILE").required(true).takes_value(true))
        .get_matches();

    let rom_file_path = PathBuf::from(matches.value_of("ROM_FILE").unwrap());

    let mut chip8 = Chip8::new();
    let contents = fs::read(rom_file_path).unwrap_or_else(|err| {
        eprintln!("could not read file contents: {}", err);
        std::process::exit(1);
    });
    chip8.load_rom(&contents);

    let mut window = match minifb::Window::new(
        "Chip-8",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    ) {
        Ok(window) => window,
        Err(err) => {
            eprintln!("could not create window: {}", err);
            std::process::exit(1);
        }
    };

    let mut last_cycle_update = Instant::now();
    let mut last_timers_update = Instant::now();
    let cycle_duration = Duration::from_micros((1_000_000 / 500) as u64);
    let timers_duration = Duration::from_micros(TIMERS_INTERVAL_MICROS);
    loop {
        if last_cycle_update.elapsed() >= cycle_duration {
            chip8.cpu_cycle();
            last_cycle_update = Instant::now();
        }
        if last_timers_update.elapsed() >= timers_duration {
            chip8.timers_tick();
            update_window_buffer(&mut window, &chip8);
            last_timers_update = Instant::now();
        }
    }
}

fn update_window_buffer(window: &mut Window, chip8: &Chip8) {
    let buffer: Vec<u32> = chip8
        .frame_buffer
        .buffer
        .to_vec()
        .iter()
        .map(|b| match b {
            0 => from_u8_rgb(0xA1, 0x3F, 0xF6),
            1 => from_u8_rgb(0xFF, 0xFF, 0xFF),
            _ => unreachable!("no such value can exist in the frame buffer"),
        })
        .collect();
    window
        .update_with_buffer(buffer.as_slice(), WINDOW_WIDTH, WINDOW_HEIGHT)
        .unwrap();
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
