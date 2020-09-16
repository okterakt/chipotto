use crate::app::Chip8App;
use clap::{App, Arg, ArgMatches};
use std::path::PathBuf;
use std::str::FromStr;
use std::{error, process};

mod chip8;

mod app;

pub struct Config {
    rom_file_path: PathBuf,
    clock_hz: u64,
    color1: (u8, u8, u8),
    color2: (u8, u8, u8),
}

impl Config {
    pub fn new(rom_file_path: PathBuf) -> Self {
        Config {
            rom_file_path,
            clock_hz: 500,
            color1: (0x00, 0x00, 0x00),
            color2: (0xFF, 0xFF, 0xFF),
        }
    }

    pub fn clock_hz(mut self, clock: u64) -> Self {
        self.clock_hz = clock;
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
    let args = App::new("Chipotto")
        .version("0.1")
        .author("okterakt")
        .about("Simple CHIP-8 emulator developed in Rust as a learning project.")
        .arg(
            Arg::with_name("ROM_FILE")
                .required(true)
                .help("ROM file containing program to run")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("CLOCK_HZ")
                .short("c")
                .long("cpu-clock")
                .help("CPU clock in HZ")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("COLOR_1")
                .long("color1")
                .help("screen color 1")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("COLOR_2")
                .long("color2")
                .help("screen color 2")
                .takes_value(true),
        )
        .get_matches();

    let config = parse_args(args).unwrap_or_else(|err| {
        eprintln!("Command line arguments parsing error: {}", err);
        process::exit(1);
    });

    if let Err(err) = Chip8App::new(config).run() {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}

fn parse_args(matches: ArgMatches) -> Result<Config, Box<dyn error::Error>> {
    let rom_file_path = PathBuf::from(matches.value_of("ROM_FILE").unwrap());
    let mut config = Config::new(rom_file_path);

    if let Some(clock_hz) = matches.value_of("CLOCK_HZ") {
        config = config.clock_hz(u64::from_str(clock_hz)?);
    }
    if let Some(col1) = matches.value_of("COLOR_1") {
        config = config.color1(rgb_from_hex(col1)?);
    }
    if let Some(col2) = matches.value_of("COLOR_2") {
        config = config.color2(rgb_from_hex(col2)?);
    }

    Ok(config)
}

fn rgb_from_hex(hex: &str) -> Result<(u8, u8, u8), Box<dyn error::Error>> {
    let mut hex_trimmed = hex.trim_start_matches("#");
    hex_trimmed = hex_trimmed.trim_start_matches("0x");
    let r: u8 = u8::from_str_radix(&hex_trimmed[0..2], 16)?;
    let g: u8 = u8::from_str_radix(&hex_trimmed[2..4], 16)?;
    let b: u8 = u8::from_str_radix(&hex_trimmed[4..6], 16)?;
    Ok((r, g, b))
}
