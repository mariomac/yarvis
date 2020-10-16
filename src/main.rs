mod riscv;

fn main() {
    let mut m = riscv::mem::Mem::new(33);
    m.write_u8(1, 6);
    println!("Hello, {}", m.read_u8(1));
}
