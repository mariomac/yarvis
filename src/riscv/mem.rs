use std::io::Write;
use std::ops;

// Little endian memory

pub struct Mem(Vec<u8>);

impl Mem {
    pub fn new(length: usize) -> Self {
        Self(vec![0; length])
    }

    pub fn read_u8(&self, addr: usize) -> u8 {
        self.0[addr]
    }

    pub fn write_u8(&mut self, addr: usize, val: u8) {
        self.0[addr] = val;
    }

    pub fn read_u16(&self, addr: usize) -> u16 {
        u16::from_le_bytes([self.0[addr], self.0[addr + 1]])
    }

    pub fn write_u16(&mut self, addr: usize, val: u16) {
        self.0[addr] = val as u8;
        self.0[addr + 1] = (val >> 8) as u8;
    }

    pub fn read_u32(&self, addr: usize) -> u32 {
        u32::from_le_bytes([
            self.0[addr],
            self.0[addr + 1],
            self.0[addr + 2],
            self.0[addr + 3],
        ])
    }

    pub fn write_u32(&mut self, addr: usize, val: u32) {
        self.0[addr] = val as u8;
        self.0[addr + 1] = (val >> 8) as u8;
        self.0[addr + 2] = (val >> 16) as u8;
        self.0[addr + 3] = (val >> 24) as u8;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_creation() {
        let m = super::Mem::new(33);
        assert_eq!(33, m.0.len())
    }

    #[test]
    fn test_read_write_u8() {
        let mut m = super::Mem::new(10);
        assert_eq!(0, m.read_u8(0));
        m.write_u8(0, 123);
        assert_eq!(123, m.read_u8(0));
    }

    #[test]
    fn test_read_write_u16() {
        let mut m = super::Mem::new(10);
        assert_eq!(0, m.read_u16(1));
        m.write_u16(1, 12345);
        assert_eq!(12345, m.read_u16(1));
    }

    #[test]
    fn test_read_write_u32() {
        let mut m = super::Mem::new(10);
        assert_eq!(0, m.read_u32(2));
        m.write_u32(2, 0x12345678);
        assert_eq!(0x12345678, m.read_u32(2));
    }
}
