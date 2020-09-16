use chip8::Chip8;
use clap::{App, Arg, ArgMatches};
use minifb::{Key, KeyRepeat, Scale, ScaleMode, Window, WindowOptions};
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{Duration, Instant};
use std::{error, fs};

mod chip8;
mod cpu;
mod framebuffer;
mod instr;
mod keypad;
mod memory;

const WINDOW_WIDTH: usize = 64;
const WINDOW_HEIGHT: usize = 32;

const TIMERS_INTERVAL_MICROS: u64 = 1_000_000 / 60;

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

struct Config {
    rom_file_path: PathBuf,
    clock_hz: u64,
    refresh_hz: u64,
    color1: (u8, u8, u8),
    color2: (u8, u8, u8),
}

impl Config {
    pub fn new(rom_file_path: PathBuf) -> Self {
        Config {
            rom_file_path,
            clock_hz: 500,
            refresh_hz: 60,
            color1: (0x00, 0x00, 0x00),
            color2: (0xFF, 0xFF, 0xFF),
        }
    }

    pub fn clock_hz(mut self, clock: u64) -> Self {
        self.clock_hz = clock;
        self
    }

    pub fn refresh_hz(mut self, refresh_rate_hz: u64) -> Self {
        self.refresh_hz = refresh_rate_hz;
        self
    }

    pub fn color1(mut self, color: (u8, u8, u8)) -> Self {
        self.color1 = color;
        self
    }

    pub fn color2(mut self, color: (u8, u8, u8)) -> Self {
        self.color2 = color;
        self
    }
}

fn main() {
    let matches = App::new("Chip-8")
        .version("0.1")
        .author("okterakt")
        .about("Simple chip-8 emulator")
        .arg(Arg::with_name("ROM_FILE").required(true).takes_value(true))
        .arg(
            Arg::with_name("clock")
                .short("c")
                .long("cpu-clock")
                .help("cpu clock in HZ")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("refresh rate")
                .short("r")
                .long("refresh-rate")
                .help("screen refresh rate in HZ")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("color 1")
                .long("color1")
                .help("screen color 1")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("color 2")
                .long("color2")
                .help("screen color 2")
                .takes_value(true),
        )
        .get_matches();

    let config = parse_args(matches).unwrap_or_else(|err| {
        eprintln!("could not parse command line arguments: {}", err);
        std::process::exit(1);
    });

    let mut chip8 = Chip8::new();
    let contents = fs::read(config.rom_file_path.clone()).unwrap_or_else(|err| {
        eprintln!("could not read file contents: {}", err);
        std::process::exit(1);
    });
    chip8.load_rom(&contents);

    // WINDOW CREATION
    let mut window = match minifb::Window::new(
        "Chip-8",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions {
            resize: true,
            scale: Scale::X8,
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

    // vars for main loop
    let mut last_cycle_update = Instant::now();
    let mut last_timers_update = Instant::now();
    let mut last_screen_refresh = Instant::now();
    let cycle_duration = Duration::from_micros((1_000_000 / config.clock_hz) as u64);
    let timers_duration = Duration::from_micros(TIMERS_INTERVAL_MICROS);
    let frame_duration = Duration::from_micros((1_000_000 / config.refresh_hz) as u64);

    // MAIN LOOP
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if last_cycle_update.elapsed() >= cycle_duration {
            handle_keypad(&window, &mut chip8);
            chip8.cpu_cycle();
            last_cycle_update = Instant::now();
        }
        if last_timers_update.elapsed() >= timers_duration {
            chip8.timers_tick();
            last_timers_update = Instant::now();
        }
        if last_screen_refresh.elapsed() >= frame_duration {
            refresh_screen(&mut window, &chip8, &config);
            last_screen_refresh = Instant::now();
        }
    }
}

fn handle_keypad(window: &Window, chip8: &mut Chip8) {
    for (i, k) in KEYS.iter().enumerate() {
        if window.is_key_down(*k) {
            chip8.keypad.set_down(i as u8, true);
        } else {
            chip8.keypad.set_down(i as u8, false);
        }
    }
}

fn parse_args(matches: ArgMatches) -> Result<Config, Box<dyn error::Error>> {
    let rom_file_path = PathBuf::from(matches.value_of("ROM_FILE").unwrap());
    let mut config = Config::new(rom_file_path);

    if let Some(clock_hz) = matches.value_of("clock") {
        config = config.clock_hz(u64::from_str(clock_hz)?);
    }
    if let Some(ref_rate) = matches.value_of("refresh rate") {
        config = config.refresh_hz(u64::from_str(ref_rate)?);
    }
    if let Some(col1) = matches.value_of("color 1") {
        config = config.color1(rgb_from_hex(col1)?);
    }
    if let Some(col2) = matches.value_of("color 2") {
        config = config.color2(rgb_from_hex(col2)?);
    }

    Ok(config)
}

fn refresh_screen(window: &mut Window, chip8: &Chip8, config: &Config) {
    let buffer: Vec<u32> = chip8
        .frame_buffer
        .buffer
        .to_vec()
        .iter()
        .map(|b| match b {
            0 => from_u8_rgb(config.color1.0, config.color1.1, config.color1.2),
            1 => from_u8_rgb(config.color2.0, config.color2.1, config.color2.2),
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

fn rgb_from_hex(hex: &str) -> Result<(u8, u8, u8), Box<dyn error::Error>> {
    let mut hex_trimmed = hex.trim_start_matches("#");
    hex_trimmed = hex_trimmed.trim_start_matches("0x");
    // TODO: error handling in case of invalid length and so on
    let r: u8 = u8::from_str_radix(&hex_trimmed[0..2], 16)?;
    let g: u8 = u8::from_str_radix(&hex_trimmed[2..4], 16)?;
    let b: u8 = u8::from_str_radix(&hex_trimmed[4..6], 16)?;
    Ok((r, g, b))
}
