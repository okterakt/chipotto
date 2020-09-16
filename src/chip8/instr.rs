use std::fmt;
use std::fmt::{Display, Formatter};

pub enum Instr {
    Cls,
    Ret,
    Sys(u16),
    Jp(u16),
    Call(u16),
    SeVxKK(usize, u8),
    SneVxKK(usize, u8),
    SeVxVy(usize, usize),
    SneVxVy(usize, usize),
    LdVxKK(usize, u8),
    AddVxKK(usize, u8),
    LdVxVy(usize, usize),
    OrVxVy(usize, usize),
    AndVxVy(usize, usize),
    XorVxVy(usize, usize),
    AddVxVy(usize, usize),
    SubVxVy(usize, usize),
    SubnVxVy(usize, usize),
    ShrVx(usize),
    ShlVx(usize),
    LdI(u16),
    JpV0(u16),
    RndVxKK(usize, u8),
    DrwVxVyN(usize, usize, usize),
    SkpVx(usize),
    SknpVx(usize),
    LdVxDT(usize),
    LdVxK(usize),
    LdDTVx(usize),
    LdSTVx(usize),
    AddIVx(usize),
    LdFVx(usize),
    LdBVx(usize),
    LdIVx(usize),
    LdVxI(usize),
}

impl Instr {
    pub fn from(opcode: u16) -> Instr {
        let nibbles = (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            ((opcode & 0x000F) >> 0) as u8,
        );
        let nnn = opcode & 0x0FFF; // also called xyz
        let kk = (opcode & 0x00FF) as u8; // also called yz
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;

        match nibbles {
            (0, 0, 0xE, 0) => Instr::Cls,
            (0, 0, 0xE, 0xE) => Instr::Ret,
            (0, _, _, _) => Instr::Sys(nnn),
            (1, _, _, _) => Instr::Jp(nnn),
            (2, _, _, _) => Instr::Call(nnn),
            (3, _, _, _) => Instr::SeVxKK(x, kk),
            (4, _, _, _) => Instr::SneVxKK(x, kk),
            (5, _, _, _) => Instr::SeVxVy(x, y),
            (6, _, _, _) => Instr::LdVxKK(x, kk),
            (7, _, _, _) => Instr::AddVxKK(x, kk),
            (8, _, _, 0) => Instr::LdVxVy(x, y),
            (8, _, _, 1) => Instr::OrVxVy(x, y),
            (8, _, _, 2) => Instr::AndVxVy(x, y),
            (8, _, _, 3) => Instr::XorVxVy(x, y),
            (8, _, _, 4) => Instr::AddVxVy(x, y),
            (8, _, _, 5) => Instr::SubVxVy(x, y),
            (8, _, _, 6) => Instr::ShrVx(x),
            (8, _, _, 7) => Instr::SubnVxVy(x, y),
            (8, _, _, 0xE) => Instr::ShlVx(x),
            (9, _, _, 0) => Instr::SneVxVy(x, y),
            (0xA, _, _, _) => Instr::LdI(nnn),
            (0xB, _, _, _) => Instr::JpV0(nnn),
            (0xC, _, _, _) => Instr::RndVxKK(x, kk),
            (0xD, _, _, _) => Instr::DrwVxVyN(x, y, n),
            (0xE, _, 9, 0xE) => Instr::SkpVx(x),
            (0xE, _, 0xA, 1) => Instr::SknpVx(x),
            (0xF, _, 0, 7) => Instr::LdVxDT(x),
            (0xF, _, 0, 0xA) => Instr::LdVxK(x),
            (0xF, _, 1, 5) => Instr::LdDTVx(x),
            (0xF, _, 1, 8) => Instr::LdSTVx(x),
            (0xF, _, 1, 0xE) => Instr::AddIVx(x),
            (0xF, _, 2, 9) => Instr::LdFVx(x),
            (0xF, _, 3, 3) => Instr::LdBVx(x),
            (0xF, _, 5, 5) => Instr::LdIVx(x),
            (0xF, _, 6, 5) => Instr::LdVxI(x),
            _ => unreachable!("unknown instruction"),
        }
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Instr::Cls => write!(f, "CLS"),
            Instr::Ret => write!(f, "RET"),
            Instr::Jp(nnn) => write!(f, "JP {}", nnn),
            Instr::Call(nnn) => write!(f, "CALL {}", nnn),
            Instr::SeVxKK(x, kk) => write!(f, "SE V{}, {}", x, kk),
            Instr::SneVxKK(x, kk) => write!(f, "SNE V{}, {}", x, kk),
            Instr::SeVxVy(x, y) => write!(f, "SE V{}, V{}", x, y),
            Instr::LdVxKK(x, kk) => write!(f, "LD V{}, {}", x, kk),
            Instr::AddVxKK(x, kk) => write!(f, "ADD V{}, {}", x, kk),
            Instr::LdVxVy(x, y) => write!(f, "LD V{}, V{}", x, y),
            Instr::OrVxVy(x, y) => write!(f, "OR V{}, V{}", x, y),
            Instr::AndVxVy(x, y) => write!(f, "AND V{}, V{}", x, y),
            Instr::XorVxVy(x, y) => write!(f, "XOR V{}, V{}", x, y),
            Instr::AddVxVy(x, y) => write!(f, "ADD V{}, V{}", x, y),
            Instr::SubVxVy(x, y) => write!(f, "SUB V{}, V{}", x, y),
            Instr::ShrVx(x) => write!(f, "SHR V{}", x),
            Instr::SubnVxVy(x, y) => write!(f, "SUBN V{}, V{}", x, y),
            Instr::ShlVx(x) => write!(f, "SHR V{}", x),
            Instr::SneVxVy(x, y) => write!(f, "SNE V{}, V{}", x, y),
            Instr::LdI(nnn) => write!(f, "LD I, {}", nnn),
            Instr::JpV0(nnn) => write!(f, "JP V0, {}", nnn),
            Instr::RndVxKK(x, kk) => write!(f, "RND V{}, {}", x, kk),
            Instr::DrwVxVyN(x, y, n) => write!(f, "DRW V{}, V{}, {}", x, y, n),
            Instr::SkpVx(x) => write!(f, "SKP V{}", x),
            Instr::SknpVx(x) => write!(f, "SKPN V{}", x),
            Instr::LdVxDT(x) => write!(f, "LD V{}, DT", x),
            Instr::LdVxK(x) => write!(f, "LD V{}, K", x),
            Instr::LdDTVx(x) => write!(f, "LD DT, V{}", x),
            Instr::LdSTVx(x) => write!(f, "LD ST, V{}", x),
            Instr::AddIVx(x) => write!(f, "ADD I, V{}", x),
            Instr::LdFVx(x) => write!(f, "LD F, V{}", x),
            Instr::LdBVx(x) => write!(f, "LD B, V{}", x),
            Instr::LdIVx(x) => write!(f, "LD I, V{}", x),
            Instr::LdVxI(x) => write!(f, "LD V{}, I", x),
            _ => unreachable!("unknown instruction"),
        }
    }
}
