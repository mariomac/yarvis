mod decoder;
mod instr;
mod mem;
mod regs;

pub type Instr = instr::Instr;
pub type Mem = mem::Mem;
pub type Regs = regs::Regs<u32>;
