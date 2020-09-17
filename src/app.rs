use crate::core::chip8::Chip8;
use crate::Config;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::error::Error;
use std::fs;
use std::time::{Duration, Instant};

const WINDOW_WIDTH: usize = 64;
const WINDOW_HEIGHT: usize = 32;

const TIMERS_INTERVAL_MICROS: u64 = 1_000_000 / 60;
const REFRESH_INTERVAL_MICROS: u64 = 1_000_000 / 60;

const KEYS: [Key; 16] = [
    Key::X,
    Key::Key1,
    Key::Key2,
    Key::Key3,
    Key::Q,
    Key::W,
    Key::E,
    Key::A,
    Key::S,
    Key::D,
    Key::Z,
    Key::C,
    Key::Key4,
    Key::R,
    Key::F,
    Key::V,
];

pub struct Chip8App {
    config: Config,
    chip8: Chip8,
}

impl Chip8App {
    pub fn new(config: Config) -> Self {
        Chip8App {
            config,
            chip8: Chip8::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let contents = fs::read(self.config.rom_file_path.clone())?;
        self.chip8.load_rom(&contents);

        // WINDOW CREATION
        let mut window = minifb::Window::new(
            "Chipotto",
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            WindowOptions {
                resize: true,
                scale: Scale::X8,
                scale_mode: ScaleMode::AspectRatioStretch,
                ..WindowOptions::default()
            },
        )?;

        // vars for main loop
        let mut last_cycle_update = Instant::now();
        let mut last_timers_update = Instant::now();
        let mut last_screen_refresh = Instant::now();
        let cycle_duration = Duration::from_micros((1_000_000 / self.config.clock_hz) as u64);
        let timers_duration = Duration::from_micros(TIMERS_INTERVAL_MICROS);
        let frame_duration = Duration::from_micros(REFRESH_INTERVAL_MICROS);

        // MAIN LOOP
        while window.is_open() && !window.is_key_down(Key::Escape) {
            if last_cycle_update.elapsed() >= cycle_duration {
                self.handle_keypad(&window);
                self.chip8.cpu_cycle();
                last_cycle_update = Instant::now();
            }
            if last_timers_update.elapsed() >= timers_duration {
                self.chip8.timers_tick();
                last_timers_update = Instant::now();
            }
            if last_screen_refresh.elapsed() >= frame_duration {
                self.refresh_screen(&mut window);
                last_screen_refresh = Instant::now();
            }
        }

        Ok(())
    }

    fn handle_keypad(&mut self, window: &Window) {
        for (i, k) in KEYS.iter().enumerate() {
            if window.is_key_down(*k) {
                self.chip8.keypad.set_down(i as u8, true);
            } else {
                self.chip8.keypad.set_down(i as u8, false);
            }
        }
    }

    fn refresh_screen(&self, window: &mut Window) {
        let buffer: Vec<u32> = self
            .chip8
            .frame_buffer
            .get_buffer()
            .to_vec()
            .iter()
            .map(|b| match b {
                0 => from_u8_rgb(
                    self.config.color1.0,
                    self.config.color1.1,
                    self.config.color1.2,
                ),
                1 => from_u8_rgb(
                    self.config.color2.0,
                    self.config.color2.1,
                    self.config.color2.2,
                ),
                _ => unreachable!("no such value should exist in the frame buffer"),
            })
            .collect();
        window
            .update_with_buffer(buffer.as_slice(), WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
