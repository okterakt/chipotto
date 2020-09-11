use std::ops::Range;

// 4096B
const MEM_SIZE: u16 = 0x1000;
const MEM_START: u16 = 0x200;
const LEGAL_MEM_RANGE: Range<u16> = MEM_START..MEM_SIZE;

pub struct Memory {
    bytes: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            bytes: vec![0; MEM_SIZE as usize],
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        check_legal_mem_access(address, 1);
        self.bytes[address as usize]
    }

    pub fn read_word(&mut self, address: u16) -> u16 {
        check_legal_mem_access(address, 2);
        ((self.bytes[address as usize] as u16) << 8) | self.bytes[(address + 1) as usize] as u16
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        check_legal_mem_access(address, 1);
        self.bytes[address as usize] = byte;
    }

    pub fn write_word(&mut self, address: u16, word: u16) {
        check_legal_mem_access(address, 2);
        self.bytes[address as usize] = (word >> 8) as u8;
        self.bytes[(address + 1) as usize] = word as u8;
    }
}

fn check_legal_mem_access(address: u16, num_bytes: u16) {
    if !LEGAL_MEM_RANGE.contains(&(address))
        || !LEGAL_MEM_RANGE.contains(&(address + num_bytes - 1))
    {
        panic!(
            "illegal memory access at address {} for {} bytes",
            address, num_bytes
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::*;

    #[test]
    fn test_read_write_valid() {
        let mut mem = Memory::new();
        mem.write_byte(0x200, 0xff);
        assert_eq!(0xff, mem.read_byte(0x200));
        mem.write_word(0x400, 0xf1f3);
        assert_eq!(0xf1f3, mem.read_word(0x400));
        assert_eq!(0xf1, mem.read_byte(0x400));
        assert_eq!(0xf3, mem.read_byte(0x401));
    }

    #[test]
    #[should_panic(expected = "illegal memory access at address")]
    fn test_read_byte_panic() {
        let mut mem = Memory::new();
        mem.read_byte(0x1000);
    }

    #[test]
    #[should_panic(expected = "illegal memory access at address")]
    fn test_write_word_panic() {
        let mut mem = Memory::new();
        mem.write_word(0x30, 0x12);
    }
}
