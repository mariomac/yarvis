use std::ops;

// Little endian memory

pub struct Mem(Vec<u8>);

impl Mem {
    pub fn new(length: usize) -> Self {
        Self(vec![0; length])
    }
}

impl ops::Index<usize> for Mem {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::IndexMut<usize> for Mem {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
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
        assert_eq!(0, m[0]);
        m[0] = 123;
        assert_eq!(123, m[0]);
    }
}
