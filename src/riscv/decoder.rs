use crate::riscv::instr;
use crate::riscv::instr::Format;
use crate::riscv::instr::Instr;
use crate::riscv::instr::opcode;

// index: 6..2 opcode bits (1 and 0 are always 1)
const formats: [Format; 1] = [
    instr::Format::RType, //0000011 - LOAD
];

const OP_LOAD: u8 = 0000011;

pub fn from(bits: u32) -> Instr {
    let opcode = opcode(bits);
    if opcode & 0b11 != 0b11 {
        panic!("Not supported opcodes[1:0] != 11")
    }
    let f = formats[(opcode >> 2) as usize];
    f.0=bits;
    Instr {
        opcode: opcode,
        Format: 
    }
    match opcode {
        OP_LOAD => Instr {
            opcode: opcode,
            
        }
    }
}

