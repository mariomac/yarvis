pub struct RType(u32);
pub struct R4Type(u32);

pub enum Type {
    RType,
    R4Type,
}

pub struct Instr {
    pub opCode: u8,
    pub iType: Type,
}

const rd_mask: u32      = 0b11111000000000000000000000000000;
const rs1_mask: u32     = 0b00000111110000000000000000000000;
const rs2_mask: u32     = 0b00000000001111100000000000000000;
const funct10_mask: u32 = 0b00000000000000011111111110000000;
const opcode_mask: u32  = 0b00000000000000000000000001111111;
const rd_shift: u32     = 27;
const rs1_shift: u32    = 22;
const rs2_shift: u32    = 17;
const funct10_shift: u32 = 7;

fn rd(i: u32) -> u8 {
    ((i & rd_mask) >> rd_shift) as u8
}

fn rs1(i: u32) -> u8 {
    ((i & rs1_mask) >> rs1_shift) as u8
}

fn rs2(i: u32) -> u8 {
    ((i & rs2_mask) >> rs2_shift) as u8
}

fn funct10(i: u32) -> u16 {
    ((i & funct10_mask) >> funct10_shift) as u16
}

fn opcode(i: u32) -> u8 {
    (i & opcode_mask) as u8
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