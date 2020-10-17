#![allow(dead_code)]

pub struct RType(u32);
pub struct R4Type(u32);
pub struct IType(u32);
pub struct BType(u32);
pub struct LType(u32);
pub struct JType(u32);

pub enum Format {
    RType,
    R4Type,
    IType,
    BType,
    LType,
    JType,
}

pub struct Instr {
    pub opcode: u8,
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

fn opcode(i: u32) -> u8 {
    (i & OPCODE_MASK) as u8
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
    pub fn rd(&self) -> u8 {
        rd(self.0)
    }
    pub fn rs1(&self) -> u8 {
        rs1(self.0)
    }
    pub fn rs2(&self) -> u8 {
        rs2(self.0)
    }
    pub fn funct10(&self) -> u16 {
        funct10(self.0)
    }
}

impl R4Type {
    pub fn rd(&self) -> u8 {
        rd(self.0)
    }
    pub fn rs1(&self) -> u8 {
        rs1(self.0)
    }
    pub fn rs2(&self) -> u8 {
        rs2(self.0)
    }
    pub fn rs3(&self) -> u8 {
        rs3(self.0)
    }
    pub fn funct5(&self) -> u8 {
        funct5(self.0)
    }
}

impl IType {
    pub fn rd(&self) -> u8 {
        rd(self.0)
    }
    pub fn rs1(&self) -> u8 {
        rs1(self.0)
    }
    pub fn imm(&self) -> u16 {
        imm_i(self.0)
    }
    pub fn funct3(&self) -> u8 {
        funct3(self.0)
    }
}

impl BType {
    pub fn rs1(&self) -> u8 {
        rs1(self.0)
    }
    pub fn rs2(&self) -> u8 {
        rs2(self.0)
    }
    pub fn imm(&self) -> u16 {
        imm_b(self.0)
    }
    pub fn funct3(&self) -> u8 {
        funct3(self.0)
    }
}

impl LType {
    pub fn rd(&self) -> u8 {
        rd(self.0)
    }
    pub fn imm(&self) -> u32 {
        imm_lui(self.0)
    }
}

impl JType {
    pub fn offset(&self) -> u32 {
        jump_offset(self.0)
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
    let i = RType(0b10101010101100111001100110000000);
    assert_eq!(0b10101, i.rd());
    assert_eq!(0b01010, i.rs1());
    assert_eq!(0b11001, i.rs2());
    assert_eq!(0b1100110011, i.funct10());
}

#[test]
fn test_r4_type() {
    let i = R4Type(0b10101010101100111011100110000000);
    assert_eq!(0b10101, i.rd());
    assert_eq!(0b01010, i.rs1());
    assert_eq!(0b11001, i.rs2());
    assert_eq!(0b11011, i.rs3());
    assert_eq!(0b10011, i.funct5());
}

#[test]
fn test_i_type() {
    let i = IType(0b10101_01010_110011101110_011_0000000);
    assert_eq!(0b10101, i.rd());
    assert_eq!(0b01010, i.rs1());
    assert_eq!(0b110011101110, i.imm());
    assert_eq!(0b011, i.funct3());
}

#[test]
fn test_b_type() {
    let i = BType(0b10101_01010_11001_1101110_011_0000000);
    assert_eq!(0b01010, i.rs1());
    assert_eq!(0b11001, i.rs2());
    assert_eq!(0b101011101110, i.imm());
    assert_eq!(0b011, i.funct3());
}

#[test]
fn test_l_type() {
    let i = LType(0b10101_01010110011101110011_0000000);
    assert_eq!(0b10101, i.rd());
    assert_eq!(0b01010110011101110011, i.imm());
}

#[test]
fn test_j_type() {
    let i = JType(0b1010101010110011101110011_0000000);
    assert_eq!(0b1010101010110011101110011, i.offset());
}
