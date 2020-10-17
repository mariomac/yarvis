mod riscv;

fn main() {
    //let l : riscv::Regs;

    let mut m = riscv::Mem::new(33);
    m.write_u8(1, 6);
    println!("Hello, {}", m.read_u8(1));
}
