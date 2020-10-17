// W: memory word (initially u32)
pub struct Regs<W> {
    // general purpose
    x: [W; 32],

    // program counter
    pub pc: W,
    pub fsr: W,
    // TODO: floating point
    //f: [f64; regs_s],
}

impl Regs<u32> {
    pub const XPRLEN_BYTES: usize = 4;
    pub const XPRLEN: usize = 32;

    pub fn new_32() -> Self {
        Self {
            x: [0; 32],
            pc: 0,
            fsr: 0,
        }
    }

    pub fn r(&self, n: usize) -> u32 {
        if n == 0 {
            0
        } else {
            self.x[n]
        }
    }
    pub fn r_set(&mut self, n: usize, val: u32) {
        if n > 0 {
            self.x[n] = val;
        }
    }
    // aliases for registers
    pub fn zero(&self) -> u32 {
        0
    }
    pub fn ra(&self) -> u32 {
        self.x[1]
    }
    pub fn ra_set(&mut self, val: u32) {
        self.x[1] = val;
    }
}

#[test]
fn test_creation() {
    let r: Regs<u32> = Regs::new_32();
    assert_eq!(0, r.zero());
    assert_eq!(32, Regs::XPRLEN);
}
