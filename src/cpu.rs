use crate::instr::Instr;
use crate::memory::Memory;
use std::fs;
use std::path::PathBuf;
use std::borrow::BorrowMut;

const PC_START: u16 = 0x200;
const STACK_SIZE: usize = 16;
const FONT_SPRITES: [u8; 80] = [
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 0
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 1
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 2
    0xF0, 0x10, 0x20, 0x40, 0x40, // 3
    0xF0, 0x90, 0xF0, 0x90, 0x90, // 4
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // 5
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // 6
    0xF0, 0x80, 0xF0, 0x80, 0x80, // 7
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 8
    0x20, 0x60, 0x20, 0x20, 0x70, // 9
    0x90, 0x90, 0xF0, 0x10, 0x10, // A
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // B
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // C
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // D
    0xF0, 0x80, 0x80, 0x80, 0xF0, // E
    0xE0, 0x90, 0x90, 0x90, 0xE0, // F
];

pub struct Cpu {
    pc: u16,         // program counter
    v: [u8; 16],     // Vx registers
    i: u16,          // I register
    dt: i32,         // delay timer
    st: i32,         // sound timer
    stack: Vec<u16>, // stack
    mem: Memory,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu {
            pc: PC_START,
            v: [0; 16],
            i: 0,
            dt: 0,
            st: 0,
            stack: Vec::with_capacity(STACK_SIZE),
            mem: Memory::new(),
        };

        // load font sprites; TODO: maybe move to Memory
        cpu.mem.write_data(0x0, &FONT_SPRITES);

        cpu
    }

    pub fn load_rom(&mut self, filename: &PathBuf) {
        let contents = fs::read(filename).expect("could not read the file");
        self.mem.write_data(PC_START, contents.as_slice());
    }

    pub fn cycle(&mut self) {
        let opcode = self.fetch();
        let instr = self.decode(opcode);
        self.exec(instr);
        self.step(); // TODO: check if correct
    }

    fn fetch(&self) -> u16 {
        self.mem.read_word(self.pc)
    }

    fn decode(&self, opcode: u16) -> Instr {
        Instr::from(opcode)
    }

    fn exec(&mut self, instr: Instr) {
        // TODO: maybe return Result
        match instr {
            Instr::Cls => {
                // Clear the display.
                // TODO: clear screen
            }
            Instr::Ret => {
                // Return from a subroutine.
                if let Some(address) = self.stack.pop() {
                    self.pc = address;
                }
            }
            Instr::Jp(nnn) => {
                // Jump to location nnn.
                self.pc = nnn;
            }
            Instr::Call(nnn) => {
                // Call subroutine at nnn.
                self.stack.push(pc);
                pc = nnn;
            }
            Instr::SeVxKK(x, kk) => {
                // Skip next instruction if Vx = kk.
                if self.v[x] == kk {
                    self.skip();
                }
            }
            Instr::SneVxKK(x, kk) => {
                // Skip next instruction if Vx != kk.
                if self.v[x] != kk {
                    self.skip();
                }
            }
            Instr::SeVxVy(x, y) => {
                // Skip next instruction if Vx = Vy.
                if self.v[x] == self.v[y] {
                    self.skip();
                }
            }
            Instr::SneVxVy(x, y) => {
                // Skip next instruction if Vx != Vy.
                if v[x] != v[y] {
                    skip();
                }
            }
            Instr::LdVxKK(x, kk) => self.v[x] == kk,
            Instr::AddVxKK(x, kk) => self.v[x] += kk,
            Instr::LdVxVy(x, y) => self.v[x] == self.v[y],
            Instr::OrVxVy(x, y) => self.v[x] | self.v[y],
            Instr::AndVxVy(x, y) => self.v[x] & self.v[y],
            Instr::XorVxVy(x, y) => self.v[x] ^ self.v[y],
            Instr::AddVxVy(x, y) => {
                // Set Vx = Vx + Vy, set VF = carry.
                let sum = (v[x] as u16) + v[y];
                if sum > 255 {
                    v[0xF] = 1;
                }
                v[x] = sum as u8;
            }
            Instr::SubVxVy(x, y) => {
                // Set Vx = Vx - Vy, set VF = NOT borrow.
                if v[x] > v[y] {
                    v[0xF] = 1;
                } else {
                    v[0xF] = 0;
                }
                v[x] -= v[y];
            }
            Instr::SubnVxVy(x, y) => {
                // Set Vx = Vy - Vx, set VF = NOT borrow.
                if v[y] > v[x] {
                    v[0xF] = 1;
                } else {
                    v[0xF] = 0;
                }
                v[x] = v[y] - v[x];
            }
            Instr::ShrVx(x) => {
                // Set Vx = Vx SHR 1.
                v[0xF] = x & 1;
                v[x] >>= 1;
            }
            Instr::ShlVx(x) => {
                // Set Vx = Vx SHL 1.
                v[0xF] = (x & 0x80) >> 7;
                v[x] <<= 1;
            }
            Instr::LdI(nnn) => {
                // Set I = nnn.
                self.i = nnn
            }
            Instr::JpV0(nnn) => {
                // Jump to location nnn + V0.
                self.pc = nnn + v[0];
            }
            Instr::RndVxKK(x, kk) => {
                // Set Vx = random byte AND kk.
                let rand_byte = 0x0; // TODO: get true random byte
                v[x] = kk & rand_byte;
            }
            Instr::DrwVxVyN(x, y, n) => {
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                // TODO: implement draw sprites
            }
            Instr::SkpVx(x) => {
                // Skip next instruction if key with the value of Vx is pressed.
                // TODO: Checks the keyboard, and if the key corresponding to the value of Vx
                // TODO: is currently in the down position, PC is increased by 2.
            }
            Instr::SknpVx(x) => {
                // Skip next instruction if key with the value of Vx is not pressed.
            }
            Instr::LdVxDT(x) => {
                // Set Vx = delay timer value.
                self.v[x] = self.dt;
            }
            Instr::LdVxK(x) => {
                // Wait for a key press, store the value of the key in Vx.
                // TODO: All execution stops until a key is pressed, then the value of that key is stored in Vx.
            }
            Instr::LdDTVx(x) => {
                // Set delay timer = Vx.
                self.dt = self.v[x];
            }
            Instr::LdSTVx(x) => {
                // Set sound timer = Vx.
                self.st = self.v[x];
            }
            Instr::AddIVx(x) => {
                // Set I = I + Vx.
                self.i += v[x];
            }
            Instr::LdFVx(x) => {
                // Set I = location of sprite for digit Vx.
                // TODO: The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
            }
            Instr::LdBVx(x) => {
                // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                // TODO: The interpreter takes the decimal value of Vx, and places the hundreds digit in
                // TODO: memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
            }
            Instr::LdIVx(x) => {
                // Store registers V0 through Vx in memory starting at location I.
                self.mem.write_data(self.i, &self.v[0..=x])
            }
            Instr::LdVxI(x) => {
                // Read registers V0 through Vx from memory starting at location I.
                // TODO: self.mem.copy_into(&self.v, self.i, self.x + 1);
                // fn copy_into(&mut self, dest: &mut [u8], address: u16, num_bytes: u16) {
                //     dest.copy_from_slice(&bytes[address..(address + num_bytes)]);
                // }
            }
            // TODO: finish implementing instructions
            _ => {}
        }
    }

    fn step(&mut self) {
        self.pc += 1;
    }

    fn skip(&mut self) {
        self.pc += 2;
    }
}
