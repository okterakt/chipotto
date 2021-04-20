# Chipotto
![CI](https://github.com/okterakt/chipotto/workflows/CI/badge.svg?branch=master)
[![GitHub license](https://img.shields.io/github/license/okterakt/chipotto?color=007af2)](https://github.com/okterakt/chipotto/blob/master/LICENSE)

Chipotto is a simple CHIP-8 emulator written in Rust as a learning project to get myself familiar with the language and the world of emulation. You can find more information about CHIP-8 [here](https://en.wikipedia.org/wiki/CHIP-8).

Some screenshots of the emulator:

![IBM Logo](/pics/screen_ibm.png)

*IBM Logo*

![Space Invaders](/pics/screen_space_invaders.png)

*Space Invaders*

## Structure

The project structure can be divided in two parts: the core emulator logic, which can be found in the `src/core/` directory, and the two files `main.rs` and `app.rs`, which can be found in `src/`.

The `main.rs` file is the entry point to the program and is responsible for parsing the command line arguments and executing the application. `app.rs` takes care of creating the window, drawing the frame buffer to the screen, managing the keypad and handling the timing.
Among the core components, `chip8.rs` takes the role of a central component which coordinates the tasks of and allows communication between the cpu, the frame buffer, the keypad, and the memory.

The following is the tree view of the `src/` directory:

```sh
.
├── app.rs
├── core
│   ├── chip8.rs
│   ├── cpu.rs
│   ├── framebuffer.rs
│   ├── instr.rs
│   ├── keypad.rs
│   ├── memory.rs
│   └── mod.rs
└── main.rs
```

## Usage

### How to install
First of all you will need to install the [Cargo and Rust tools](https://rustup.rs/). Since Chipotto depends on the `minifb` crate for window creation, rendering, and keyboard management, you will also need to install the following dependencies:
```sh
sudo apt install libxkbcommon-dev libwayland-cursor0 libwayland-dev
```

Then simply clone the project from GitHub:
```sh
git clone https://github.com/okterakt/chipotto.git
```

### How to run
To run the project with one of the example roms, simply cd into the main directory and execute the following:
```sh
cargo run roms/IBM\ Logo.ch8
```

Below you can find a more thorough explanation of the options and features available for execution:
```
Chipotto 0.1
okterakt
Simple CHIP-8 emulator developed in Rust as a learning project.

USAGE:
    chipotto [OPTIONS] <ROM_FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cpu-clock <CLOCK_HZ>    CPU clock in HZ
        --color1 <COLOR_1>        screen color 1
        --color2 <COLOR_2>        screen color 2

ARGS:
    <ROM_FILE>    ROM file containing program to run
```

The CPU clock specifies the number of operations per second that will be executed by the emulator. The default, and usually optimal, value here is 500.
Color 1 and color 2 are by default #000 (black) and #fff (white); they can be changed by specifying the hex code of a valid rgb color.
Here's an example including the options:
```sh
cargo run roms/IBM\ Logo.ch8 -c 700 --color1 123412 --color2 f1a4c8
```

### Keypad

The keypad mapping is as follows:

**Original**
|   |   |   |   |
|---|---|---|---|
| 1 | 2 | 3 | C |
| 4 | 5 | 5 | D |
| 7 | 8 | 9 | E |
| A | 0 | B | F |

**Mapping**
|   |   |   |   |
|---|---|---|---|
| 1 | 2 | 3 | 4 |
| Q | W | E | R |
| A | S | D | F |
| Z | X | C | V |

## Acknowledgements
The following resources were of great help during development:
* http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
* https://github.com/cbeust/chip-8
* https://github.com/pwmarcz/chiprs
* https://github.com/AlexEne/rust-chip8
* https://github.com/novoselov-ab/chip8-rust
* https://www.reddit.com/r/EmuDev/comments/ij1tx7/trying_to_make_a_chip8_emulator_a_little_lost/
* https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#instructions
