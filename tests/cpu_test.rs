use std::vec;

use nesguin::emu6502::cpu::{CPU, CPUFlag};

/***
 * Helper method for creating a CPU instance with program.
 */
fn load_test_program_to_cpu(program: Vec<u8>) -> CPU{
    // create cpu
    let mut cpu = CPU::new();
    cpu.reset();
    cpu.load_program(program);
    return cpu;
}

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
fn test_addressing_modes() {
    // TODO: To be implemented
}

#[test] 
fn test_op_lda() {
    // Immediate addressing
    let program = vec![0xa9, 0x00];
    let mut cpu = load_test_program_to_cpu(program);
    cpu.run();
    // zero flag must have set
    let flag = cpu.status;
    assert!((flag & 0b0000_0010) != 0);
    // a register must be 0
    assert!(cpu.register_a == 0)
}

#[test]
fn test_op_tax() {
    // create a test program
    let program = vec![0xA9, 0x02, 0xAA];
    // create cpu
    let mut cpu = load_test_program_to_cpu(program);
    cpu.run();
    // X should have the value of A which is 0x02
    assert_eq!(cpu.register_a, cpu.register_x);
}

#[test]
fn test_op_tsx() {
    // create a test program
    let program = vec![0xBA];
    // create cpu
    let mut cpu = load_test_program_to_cpu(program);
    // set stack pointer
    cpu.stack_pointer = 19;
    // run program
    cpu.run();
    // x should have the same value with stack pointer
    assert_eq!(cpu.register_x, cpu.stack_pointer);
}

#[test]
fn test_op_pha() {
    // create test program
    // LDA 0x01, PHA
    let program = vec![0xa9, 0x01, 0x48];
    let mut cpu = load_test_program_to_cpu(program);
    // run the program
    cpu.run();
    // stack should have the value 0x01
    let stack_address = cpu.resolve_stack_addr(cpu.stack_pointer + 1);
    let value = cpu.memory.read_byte(stack_address);
    assert_eq!(cpu.register_a, 0x01);
    assert_eq!(value, cpu.register_a);
}

#[test]
fn test_op_pla() {
    // create a test program
    // LDA $10, PHA, PLA
    let program = vec![0xa9, 0x01, 0x48, 0x68];
    // create cpu
    let mut cpu = load_test_program_to_cpu(program);
    // run the program
    cpu.run();
    assert_eq!(cpu.register_a, 0x01)

}

#[test]
fn test_op_php() {
    // LDA $21, PHA, PHP
    let program = vec![0xa9, 0x21, 0x48, 0x28];
    // create cpu
    let mut cpu = load_test_program_to_cpu(program);
    cpu.run();
    assert_eq!(cpu.status, 0x21);
}


