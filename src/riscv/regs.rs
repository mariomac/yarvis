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

    pub fn new() -> Self {
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
    pub fn pc_inc(&mut self) {
        self.pc += Self::XPRLEN_BYTES as u32;
    }
}

#[test]
fn test_creation() {
    let r: Regs<u32> = Regs::new();
    assert_eq!(0, r.zero());
    assert_eq!(0, r.ra());
    assert_eq!(32, Regs::XPRLEN);
    for n in 0..32 {
        assert_eq!(0, r.r(n));
    }
}

#[test]
fn test_x_regs() {
    let mut r: Regs<u32> = Regs::new();
    for n in 0..32 {
        r.r_set(31 - n, n as u32);
    }
    // zero reg is always zero
    assert_eq!(0, r.r(0));
    for n in 0..31 {
        assert_eq!(n as u32, r.r(31 - n));
    }
    assert_eq!(0, r.zero());
    assert_eq!(30, r.ra());
    r.ra_set(123);
    assert_eq!(123, r.ra());
}

#[test]
fn test_pc() {
    let mut r: Regs<u32> = Regs::new();
    r.pc = 16;
    r.pc_inc();
    assert_eq!(20, r.pc);
}
