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
        let t_bytes = &self.0[addr..addr+std::mem::size_of::<u16>()];
        u16::from_le_bytes(t_bytes.try_into())
    }

    pub fn write_u8(&mut self, addr: usize, val: u8) {
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
}
