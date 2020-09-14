use crate::instr::Instr;
use crate::memory::Memory;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::borrow::BorrowMut;
use std::fs;
use std::path::PathBuf;

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
    dt: u8,          // delay timer
    st: u8,          // sound timer
    stack: Vec<u16>, // stack
    mem: Memory,
    rng: ThreadRng,
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
            rng: rand::thread_rng(),
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
        self.skip(); // we read two bytes from memory so we need to increment pc by 2
        let instr = self.decode(opcode);
        self.exec(instr);
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
                self.stack.push(self.pc);
                self.pc = nnn;
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
                if self.v[x] != self.v[y] {
                    self.skip();
                }
            }
            Instr::LdVxKK(x, kk) => {
                // Set Vx = kk.
                self.v[x] = kk
            }
            Instr::AddVxKK(x, kk) => {
                // Set Vx = Vx + kk.
                self.v[x] += kk
            }
            Instr::LdVxVy(x, y) => {
                // Set Vx = Vy.
                self.v[x] = self.v[y]
            }
            Instr::OrVxVy(x, y) => {
                // Set Vx = Vx OR Vy.
                self.v[x] |= self.v[y]
            }
            Instr::AndVxVy(x, y) => {
                // Set Vx = Vx AND Vy.
                self.v[x] &= self.v[y]
            }
            Instr::XorVxVy(x, y) => {
                // Set Vx = Vx XOR Vy.
                self.v[x] ^= self.v[y]
            }
            Instr::AddVxVy(x, y) => {
                // Set Vx = Vx + Vy, set VF = carry.
                let sum = (self.v[x] as u16) + (self.v[y] as u16);
                if sum > 255 {
                    self.v[0xF] = 1;
                }
                self.v[x] = sum as u8;
            }
            Instr::SubVxVy(x, y) => {
                // Set Vx = Vx - Vy, set VF = NOT borrow.
                if self.v[x] > self.v[y] {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x] -= self.v[y];
            }
            Instr::SubnVxVy(x, y) => {
                // Set Vx = Vy - Vx, set VF = NOT borrow.
                if self.v[y] > self.v[x] {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x] = self.v[y] - self.v[x];
            }
            Instr::ShrVx(x) => {
                // Set Vx = Vx SHR 1.
                self.v[0xF] = x as u8 & 1;
                self.v[x] >>= 1;
            }
            Instr::ShlVx(x) => {
                // Set Vx = Vx SHL 1.
                self.v[0xF] = (x as u8 & 0x80) >> 7;
                self.v[x] <<= 1;
            }
            Instr::LdI(nnn) => {
                // Set I = nnn.
                self.i = nnn
            }
            Instr::JpV0(nnn) => {
                // Jump to location nnn + V0.
                self.pc = nnn + (self.v[0] as u16);
            }
            Instr::RndVxKK(x, kk) => {
                // Set Vx = random byte AND kk.
                let rand_byte = self.rng.gen::<u8>();
                self.v[x] = kk & rand_byte;
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
                self.i += self.v[x] as u16;
            }
            Instr::LdFVx(x) => {
                // Set I = location of sprite for digit Vx.
                self.i = (self.v[x] * 5) as u16;
            }
            Instr::LdBVx(x) => {
                // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                let mut num = self.v[x];
                let hundreds = num / 100;
                let tens = (num % 100) / 10;
                let digits = num % 10;
                self.mem.write_byte(self.i, hundreds);
                self.mem.write_byte(self.i + 1, tens);
                self.mem.write_byte(self.i + 2, digits);
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

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;
    use crate::instr::Instr;

    #[test]
    fn test_exec_LdBVx() {
        let mut cpu = Cpu::new();
        cpu.i = 0x210;
        cpu.v[0] = 139;
        let instr = Instr::LdBVx(0);
        cpu.exec(instr);
        assert_eq!(1, cpu.mem.read_byte(cpu.i));
        assert_eq!(3, cpu.mem.read_byte(cpu.i + 1));
        assert_eq!(9, cpu.mem.read_byte(cpu.i + 2))
    }
}
