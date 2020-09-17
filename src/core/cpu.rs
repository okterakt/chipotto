use super::framebuffer::FrameBuffer;
use super::instr::Instr;
use super::keypad::Keypad;
use super::memory::Memory;
use rand::prelude::ThreadRng;
use rand::Rng;

const PC_START: u16 = 0x200;
const STACK_SIZE: usize = 16;

pub struct Cpu {
    pc: u16,         // program counter
    v: [u8; 16],     // Vx registers
    i: u16,          // I register
    dt: u8,          // delay timer
    st: u8,          // sound timer
    stack: Vec<u16>, // stack
    rng: ThreadRng,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: PC_START,
            v: [0; 16],
            i: 0,
            dt: 0,
            st: 0,
            stack: Vec::with_capacity(STACK_SIZE),
            rng: rand::thread_rng(),
        }
    }

    pub fn update_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }

    pub fn cycle(&mut self, frame_buffer: &mut FrameBuffer, mem: &mut Memory, keypad: &mut Keypad) {
        let opcode = self.fetch(mem);
        self.skip(); // we read two bytes from memory so we need to increment pc by 2
        let instr = self.decode(opcode);
        self.exec(instr, frame_buffer, mem, keypad);
    }

    fn fetch(&self, mem: &Memory) -> u16 {
        mem.read_word(self.pc)
    }

    fn decode(&self, opcode: u16) -> Instr {
        Instr::from(opcode)
    }

    fn exec(
        &mut self,
        instr: Instr,
        frame_buffer: &mut FrameBuffer,
        mem: &mut Memory,
        keypad: &mut Keypad,
    ) {
        match instr {
            Instr::Cls => {
                // Clear the display.
                frame_buffer.clear();
                frame_buffer.set_changed(true);
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
                self.v[x] = self.v[x].wrapping_add(kk);
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
                let (res, overflow) = self.v[x].overflowing_add(self.v[y]);
                self.v[0xF] = overflow as u8;
                self.v[x] = res;
            }
            Instr::SubVxVy(x, y) => {
                // Set Vx = Vx - Vy, set VF = NOT borrow.
                let (res, overflow) = self.v[x].overflowing_sub(self.v[y]);
                self.v[0xF] = !overflow as u8;
                self.v[x] = res;
            }
            Instr::SubnVxVy(x, y) => {
                // Set Vx = Vy - Vx, set VF = NOT borrow.
                let (res, overflow) = self.v[y].overflowing_sub(self.v[x]);
                self.v[0xF] = !overflow as u8;
                self.v[x] = res;
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
                let coll = frame_buffer.draw(
                    self.v[x] as u8,
                    self.v[y] as u8,
                    mem.read_data(self.i, n as u16).as_slice(),
                );
                frame_buffer.set_changed(true);
                self.v[0x0F] = coll as u8;
            }
            Instr::SkpVx(x) => {
                // Skip next instruction if key with the value of Vx is pressed.
                if keypad.is_down(self.v[x]) {
                    self.skip();
                }
            }
            Instr::SknpVx(x) => {
                // Skip next instruction if key with the value of Vx is not pressed.
                if !keypad.is_down(self.v[x]) {
                    self.skip();
                }
            }
            Instr::LdVxDT(x) => {
                // Set Vx = delay timer value.
                self.v[x] = self.dt;
            }
            Instr::LdVxK(x) => {
                // Wait for a key press, store the value of the key in Vx.
                if let Some(k) = keypad.get_down_key() {
                    self.v[x] = k;
                } else {
                    self.pc -= 2;
                }
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
                let num = self.v[x];
                let hundreds = num / 100;
                let tens = (num % 100) / 10;
                let digits = num % 10;
                mem.write_byte(self.i, hundreds);
                mem.write_byte(self.i + 1, tens);
                mem.write_byte(self.i + 2, digits);
            }
            Instr::LdIVx(x) => {
                // Store registers V0 through Vx in memory starting at location I.
                mem.write_data(self.i, &self.v[0..=x])
            }
            Instr::LdVxI(x) => {
                // Read registers V0 through Vx from memory starting at location I.
                mem.copy_into(&mut self.v, self.i, (x + 1) as u16);
            }
            _ => {}
        }
    }

    fn skip(&mut self) {
        self.pc += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec_LdBVx() {
        // TODO: create frame buffer, memory and keypad only, not entire chip8
        let mut frame_buffer = FrameBuffer::default();
        let mut cpu = Cpu::new();
        let mut mem = Memory::new();
        let mut keypad = Keypad::default();
        cpu.i = 0x210;
        cpu.v[0] = 139;
        let instr = Instr::LdBVx(0);
        cpu.exec(instr, &mut frame_buffer, &mut mem, &mut keypad);
        assert_eq!(1, mem.read_byte(cpu.i));
        assert_eq!(3, mem.read_byte(cpu.i + 1));
        assert_eq!(9, mem.read_byte(cpu.i + 2))
    }
}
