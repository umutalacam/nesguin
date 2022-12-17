use std::vec;

use nesguin::emu6502::cpu::{CPU, CPUFlag};

#[test] 
fn test_flags() {
    // test bits 1,5,8
    let mut cpu = CPU::new();
    cpu.status = 0;
    cpu.set_cpu_flag(CPUFlag::B0, true);
    let flag = cpu.status;
    let expected:u8 = 0b0001_0000;
    assert_eq!(flag, expected);

    cpu.status = 0;
    cpu.set_cpu_flag(CPUFlag::Carry, true);
    let flag = cpu.status;
    let expected:u8 = 0b0000_0001;
    assert_eq!(flag, expected);

    cpu.status = 0;
    cpu.set_cpu_flag(CPUFlag::Negative, true);
    let flag = cpu.status;
    let expected:u8 = 0b1000_0000;
    assert_eq!(flag, expected);

    cpu.status = 0;
    cpu.set_cpu_flag(CPUFlag::Zero, true);
    let flag = cpu.status;
    let expected:u8 = 0b0000_0010;
    assert_eq!(flag, expected);
}

#[test] 
fn test_op_lda() {
    // create a test program
    let program = vec![0xa9, 0x00];
    // create cpu
    let mut cpu = CPU::new();
    cpu.load_program(program);
    cpu.run();
    // zero flag must have set
    let flag = cpu.status;
    assert!((flag & 0b0000_0010) != 0)
}




