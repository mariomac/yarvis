#![allow(dead_code)]

pub struct RType;
pub struct R4Type;
pub struct IType;
pub struct BType;
pub struct LType;
pub struct JType;

pub enum Format {
    R(RType),
    R4(R4Type),
    I(IType),
    B(BType),
    L(LType),
    J(JType),
}

pub struct Instr {
    pub bits: u32,
    pub format: Format,
}

const RD_MASK: u32 = 0b11111000000000000000000000000000;
const RS1_MASK: u32 = 0b00000111110000000000000000000000;
const RS2_MASK: u32 = 0b00000000001111100000000000000000;
const RS3_MASK: u32 = 0b00000000000000011111000000000000;
const IMM_I_MASK: u32 = 0b00000000001111111111110000000000;
const IMM_B117_MASK: u32 = 0b11111000000000000000000000000000;
const IMM_B60_MASK: u32 = 0b00000000000000011111110000000000;
const IMM_LUI_MASK: u32 = 0b00000111111111111111111110000000;
const FUNCT3_MASK: u32 = 0b00000000000000000000001110000000;
const FUNCT5_MASK: u32 = 0b00000000000000000000111110000000;
const FUNCT10_MASK: u32 = 0b00000000000000011111111110000000;
const JUMP_MASK: u32 = 0b11111111111111111111111110000000;
const OPCODE_MASK: u32 = 0b00000000000000000000000001111111;
const RD_SHIFT: u32 = 27;
const RS1_SHIFT: u32 = 22;
const RS2_SHIFT: u32 = 17;
const RS3_SHIFT: u32 = 12;
const IMM_I_SHIFT: u32 = 10;
const IMM_B117_SHIFT: u32 = 20; // must leave room for bits 6..0
const IMM_B60_SHIFT: u32 = 10;
const IMM_LUI_SHIFT: u32 = 7;
const FUNCT3_SHIFT: u32 = 7;
const FUNCT5_SHIFT: u32 = 7;
const FUNCT10_SHIFT: u32 = 7;
const JUMP_SHIFT: u32 = 7;

pub fn opcode(i: u32) -> u8 {
    (i & OPCODE_MASK) as u8
}

fn rd(i: u32) -> u8 {
    ((i & RD_MASK) >> RD_SHIFT) as u8
}

fn rs1(i: u32) -> u8 {
    ((i & RS1_MASK) >> RS1_SHIFT) as u8
}

fn rs2(i: u32) -> u8 {
    ((i & RS2_MASK) >> RS2_SHIFT) as u8
}

fn rs3(i: u32) -> u8 {
    ((i & RS3_MASK) >> RS3_SHIFT) as u8
}

fn funct10(i: u32) -> u16 {
    ((i & FUNCT10_MASK) >> FUNCT10_SHIFT) as u16
}

fn funct5(i: u32) -> u8 {
    ((i & FUNCT5_MASK) >> FUNCT5_SHIFT) as u8
}

fn funct3(i: u32) -> u8 {
    ((i & FUNCT3_MASK) >> FUNCT3_SHIFT) as u8
}

fn imm_i(i: u32) -> u16 {
    ((i & IMM_I_MASK) >> IMM_I_SHIFT) as u16
}

fn imm_b(i: u32) -> u16 {
    (((i & IMM_B117_MASK) >> IMM_B117_SHIFT) | ((i & IMM_B60_MASK) >> IMM_B60_SHIFT)) as u16
}

fn imm_lui(i: u32) -> u32 {
    (i & IMM_LUI_MASK) >> IMM_LUI_SHIFT
}

fn jump_offset(i: u32) -> u32 {
    (i & JUMP_MASK) >> JUMP_SHIFT
}

impl RType {
    pub fn rd(&self, bits: u32) -> u8 {
        rd(bits)
    }
    pub fn rs1(&self, bits: u32) -> u8 {
        rs1(bits)
    }
    pub fn rs2(&self, bits: u32) -> u8 {
        rs2(bits)
    }
    pub fn funct10(&self, bits: u32) -> u16 {
        funct10(bits)
    }
}

impl R4Type {
    pub fn rd(&self, bits: u32) -> u8 {
        rd(bits)
    }
    pub fn rs1(&self, bits: u32) -> u8 {
        rs1(bits)
    }
    pub fn rs2(&self, bits: u32) -> u8 {
        rs2(bits)
    }
    pub fn rs3(&self, bits: u32) -> u8 {
        rs3(bits)
    }
    pub fn funct5(&self, bits: u32) -> u8 {
        funct5(bits)
    }
}

impl IType {
    pub fn rd(&self, bits: u32) -> u8 {
        rd(bits)
    }
    pub fn rs1(&self, bits: u32) -> u8 {
        rs1(bits)
    }
    pub fn imm(&self, bits: u32) -> u16 {
        imm_i(bits)
    }
    pub fn funct3(&self, bits: u32) -> u8 {
        funct3(bits)
    }
}

impl BType {
    pub fn rs1(&self, bits: u32) -> u8 {
        rs1(bits)
    }
    pub fn rs2(&self, bits: u32) -> u8 {
        rs2(bits)
    }
    pub fn imm(&self, bits: u32) -> u16 {
        imm_b(bits)
    }
    pub fn funct3(&self, bits: u32) -> u8 {
        funct3(bits)
    }
}

impl LType {
    pub fn rd(&self, bits: u32) -> u8 {
        rd(bits)
    }
    pub fn imm(&self, bits: u32) -> u32 {
        imm_lui(bits)
    }
}

impl JType {
    pub fn offset(&self, bits: u32) -> u32 {
        jump_offset(bits)
    }
}

#[test]
fn test_opcode() {
    let i = 0b1111111111111111111111110001010;
    let op = opcode(i);
    assert_eq!(0b1010, op);
}
#[test]
fn test_r_type() {
    let i = Instr {
        bits: 0b10101010101100111001100110000000,
        format: Format::R(RType {}),
    };
    match i.format {
        Format::R(f) => {
            assert_eq!(0b10101, f.rd(i.bits));
            assert_eq!(0b01010, f.rs1(i.bits));
            assert_eq!(0b11001, f.rs2(i.bits));
            assert_eq!(0b1100110011, f.funct10(i.bits));
        }
        _ => panic!("Expecting RType format"),
    }
}

#[test]
fn test_r4_type() {
    let i = Instr {
        bits: 0b10101010101100111011100110000000,
        format: Format::R4(R4Type {}),
    };
    match i.format {
        Format::R4(f) => {
            assert_eq!(0b10101, f.rd(i.bits));
            assert_eq!(0b01010, f.rs1(i.bits));
            assert_eq!(0b11001, f.rs2(i.bits));
            assert_eq!(0b11011, f.rs3(i.bits));
            assert_eq!(0b10011, f.funct5(i.bits));
        }
        _ => panic!("Expecting R4Type format"),
    }
}

#[test]
fn test_i_type() {
    let i = Instr {
        bits: 0b10101_01010_110011101110_011_0000000,
        format: Format::I(IType {}),
    };
    match i.format {
        Format::I(f) => {
            assert_eq!(0b10101, f.rd(i.bits));
            assert_eq!(0b01010, f.rs1(i.bits));
            assert_eq!(0b110011101110, f.imm(i.bits));
            assert_eq!(0b011, f.funct3(i.bits));
        }
        _ => panic!("Expecting IType format"),
    }
}

#[test]
fn test_b_type() {
    let i = Instr {
        bits: 0b10101_01010_11001_1101110_011_0000000,
        format: Format::B(BType {}),
    };
    match i.format {
        Format::B(f) => {
            assert_eq!(0b01010, f.rs1(i.bits));
            assert_eq!(0b11001, f.rs2(i.bits));
            assert_eq!(0b101011101110, f.imm(i.bits));
            assert_eq!(0b011, f.funct3(i.bits));
        }
        _ => panic!("Expecting BType format"),
    }
}

#[test]
fn test_l_type() {
    let i = Instr {
        bits: 0b10101_01010110011101110011_0000000,
        format: Format::L(LType {}),
    };
    match i.format {
        Format::L(f) => {
            assert_eq!(0b10101, f.rd(i.bits));
            assert_eq!(0b01010110011101110011, f.imm(i.bits));
        }
        _ => panic!("Expecting LType format"),
    }
}

#[test]
fn test_j_type() {
    let i = Instr {
        bits: 0b1010101010110011101110011_0000000,
        format: Format::J(JType {}),
    };
    match i.format {
        Format::J(f) => {
            assert_eq!(0b1010101010110011101110011, f.offset(i.bits));
        }
        _ => panic!("Expecting JType format"),
    }
}
