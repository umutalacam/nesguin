use nesguin::emu6502::cpu::CPU;

fn main() {
    println!("Starting emulation...");
    println!("Loading program...");
    let program:Vec<u8> = vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00];
    let mut cpu:CPU = CPU::new();
    cpu.load_program(program);
    cpu.run();
}
