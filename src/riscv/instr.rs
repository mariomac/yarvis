pub struct RType(u32);
pub struct R4Type(u32);


pub enum Format {
    RType,
    R4Type,
}

pub struct Instr {
    pub opcode: u8,
    pub format: Format,
}

const RD_MASK: u32      = 0b11111000000000000000000000000000;
const RS1_MASK: u32     = 0b00000111110000000000000000000000;
const RS2_MASK: u32     = 0b00000000001111100000000000000000;
const RS3_MASK: u32     = 0b00000000000000011111000000000000;
const FUNCT5_MASK: u32  = 0b00000000000000000000111110000000;
const FUNCT10_MASK: u32 = 0b00000000000000011111111110000000;
const OPCODE_MASK: u32  = 0b00000000000000000000000001111111;
const RD_SHIFT: u32     = 27;
const RS1_SHIFT: u32    = 22;
const RS2_SHIFT: u32    = 17;
const RS3_SHIFT: u32    = 12;
const FUNCT5_SHIFT: u32 = 7;
const FUNCT10_SHIFT: u32 = 7;

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

fn opcode(i: u32) -> u8 {
    (i & OPCODE_MASK) as u8
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