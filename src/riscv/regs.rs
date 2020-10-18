#![allow(dead_code)]

pub const ZERO: usize = 0;
pub const RA: usize = 1;   // return address
pub const SP: usize = 2; // stack pointer
pub const GP: usize = 3; // global pointer
pub const TP: usize = 4; // thread pointer
pub const T0: usize = 5; // temporary registers
pub const T1: usize = 6;
pub const T2: usize = 7;
pub const S0: usize = 8; // saved register 0
pub const FP: usize = 8; // frame pointer (same as s0)
pub const S1: usize = 9; // saved register 1
pub const A0: usize = 10; // return value or function argument 0
pub const A1: usize = 11; // return value or function argument 1
pub const A2: usize = 12; // function arguments
pub const A3: usize = 13;
pub const A4: usize = 14;
pub const A5: usize = 15;
pub const A6: usize = 16;
pub const A7: usize = 17;
pub const S2: usize = 18; // saved registers
pub const S3: usize = 19;
pub const S4: usize = 20;
pub const S5: usize = 21;
pub const S6: usize = 22;
pub const S7: usize = 23;
pub const S8: usize = 24;
pub const S9: usize = 25;
pub const S10: usize = 26;
pub const S11: usize = 27;
pub const T3: usize = 28;  // temporary registers
pub const T4: usize = 29;
pub const T5: usize = 30;
pub const T6: usize = 31;


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

    pub fn x(&self, n: usize) -> u32 {
        if n == 0 {
            0
        } else {
            self.x[n]
        }
    }
    pub fn x_set(&mut self, n: usize, val: u32) {
        if n > 0 {
            self.x[n] = val;
        }
    }
    // aliases for registers
    pub fn pc_inc(&mut self) {
        self.pc += Self::XPRLEN_BYTES as u32;
    }
}

#[test]
fn test_creation() {
    let r: Regs<u32> = Regs::new();
    assert_eq!(0, r.x(ZERO));
    assert_eq!(0, r.x(RA));
    assert_eq!(32, Regs::XPRLEN);
    for n in 0..32 {
        assert_eq!(0, r.x(n));
    }
}

#[test]
fn test_x_regs() {
    let mut r: Regs<u32> = Regs::new();
    for n in 0..32 {
        r.x_set(31 - n, n as u32);
    }
    // zero reg is always zero
    assert_eq!(0, r.x(0));
    for n in 0..31 {
        assert_eq!(n as u32, r.x(31 - n));
    }
    assert_eq!(0, r.x(ZERO));
    assert_eq!(30, r.x(RA));
    r.x_set(RA, 123);
    assert_eq!(123, r.x(RA));
}

#[test]
fn test_pc() {
    let mut r: Regs<u32> = Regs::new();
    r.pc = 16;
    r.pc_inc();
    assert_eq!(20, r.pc);
}
