use std::u8;

use crate::emu6502::ram::RAM;
use crate::emu6502::op_codes::OpCode;
use crate::emu6502::op_codes::OpCodeMap;
use crate::emu6502::op_codes::AddressingMode;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum CPUFlag {
    // NVss DIZC
    Carry,
    Zero,
    InterruptDisable,
    Decimal,
    B0,
    B1,
    Overflow,
    Negative
}

pub struct CPU {
    pub register_a:u8,
    pub register_x:u8,
    pub register_y:u8,
    pub program_counter:u16,
    pub stack_pointer:u8,
    pub status:u8,
    pub memory:RAM,
    pub op_codes:OpCodeMap
}

/* Core CPU functionality */
impl CPU {
    /**
     * CPU constructor, create a new CPU object
     */
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            program_counter: 0,
            stack_pointer: 0,
            status:0, 
            memory: RAM::new(),
            op_codes: OpCodeMap::new()
        }
    }

    /**
     * Resets CPU to initial state
     */
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;
        self.program_counter = self.memory.read_word(0xFFFC);
        self.stack_pointer = 0xff;
    }

    /**
     * Loads program into the memory
     */
    pub fn load_program(&mut self, program:Vec<u8>) {
        self.memory.mem_array[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        // set program counter
        self.program_counter = 0x8000;
        self.memory.write_word(0xFFFC, 0x8000)
    }

    /**
     * Start execution of the program in the memory. 
     * Emulates fetch decode execute cycle.
     */
    pub fn run(&mut self) {
        println!("Execution started.");
        // Fetch Decode Execution cycle
        loop {
            if self.program_counter >= 0xFFFF {
                break;
            }
            // Fetch
            let instruction = self.fetch_instruction();
            self.program_counter += 1;
            // Decode
            let op_code = self.decode_instruction(instruction);
            // Execute 
            // TODO: Cycles from opcode
            self.execute_instruction(instruction);
        }
        println!("Execution completed.");
    }

    /**
     * Fetch next instruction from memory using program counter.
     */
    fn fetch_instruction(&mut self) -> u8 {
        self.memory.read_byte(self.program_counter)
    }

    /**
     * Decode instruction and get OpCode details.
     */
    fn decode_instruction(&mut self, instruction: u8) -> &OpCode {
        self.op_codes.get_op_code(&instruction)
    }
    
    /**
     * Execute the instruction logic
     */
    fn execute_instruction(&mut self, instruction: u8) {
        // decode
        match instruction {
            0x00 => self.op_nop(),
            // LDA
            0xA9 => self.op_lda(&AddressingMode::Immediate),
            0xA5 => self.op_lda(&AddressingMode::ZeroPage),
            0xB5 => self.op_lda(&AddressingMode::ZeroPage_X),
            0xAD => self.op_lda(&AddressingMode::Absolute),
            0xBD => self.op_lda(&AddressingMode::Absolute_X),
            0xB9 => self.op_lda(&AddressingMode::Absolute_Y),
            0xA1 => self.op_lda(&AddressingMode::Indirect_X),
            0xB1 => self.op_lda(&AddressingMode::Indirect_Y),

            // STA
            0x85 => self.op_sta(&AddressingMode::ZeroPage),
            0x95 => self.op_sta(&AddressingMode::ZeroPage_X),
            0x8D => self.op_sta(&AddressingMode::Absolute),
            0x9D => self.op_sta(&AddressingMode::Absolute_X),
            0x99 => self.op_sta(&AddressingMode::Absolute_Y),
            0x81 => self.op_sta(&AddressingMode::Indirect_X),
            0x91 => self.op_sta(&AddressingMode::Indirect_Y),
            
            0xAA => self.op_tax(),
            0xE8 => self.op_inx(),

            // Stack instructions
            0x9A => self.op_txs(),
            0xBA => self.op_tsx(),
            0x48 => self.op_pha(),
            0x08 => self.op_php(),
            0x68 => self.op_pla(),
            0x28 => self.op_plp(),

            _ => {
                println!("Reaching address {} and no instruction exists: {}", self.program_counter, instruction);
                self.op_nop();
            }
        }
    }

}

// Helper methods for CPU operations
impl CPU {

    /**
     * Resolves the memory location of the operand by using the addressing mode.
     */
    fn resolve_operand_addr(&mut self, mode: & AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => {
                let addr = self.program_counter;
                self.program_counter += 1;
                addr
            },
            AddressingMode::ZeroPage => {
                let addr = self.memory.read_byte(self.program_counter) as u16;
                self.program_counter += 1;
                addr
            },
            AddressingMode::Absolute => {
                let addr = self.memory.read_word(self.program_counter);
                self.program_counter += 2;
                addr
            },
            AddressingMode::ZeroPage_X => {
                let pos = self.memory.read_byte(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                self.program_counter += 1;
                addr
            },
            AddressingMode::ZeroPage_Y => {
                let pos = self.memory.read_byte(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                self.program_counter += 1;
                addr
            },
            AddressingMode::Absolute_X => {
                let base = self.memory.read_word(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                self.program_counter += 2;
                addr
            },
            AddressingMode::Absolute_Y => {
                let base = self.memory.read_word(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                self.program_counter += 2;
                addr
            },
            AddressingMode::Indirect_X => {
                let base = self.memory.read_byte(self.program_counter);
                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.memory.read_byte(ptr as u16);
                let hi = self.memory.read_byte(ptr.wrapping_add(1) as u16);
                self.program_counter += 1;
                (hi as u16) << 8 | (lo as u16)
            },
            AddressingMode::Indirect_Y => {
                let base = self.memory.read_byte(self.program_counter);
                let lo = self.memory.read_byte(base as u16);
                let hi = self.memory.read_byte((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                self.program_counter += 1;
                deref
            },
            AddressingMode::NoneAddressing => {
                // No parameter is used for these kind of instructions, pass program counter.
                return self.program_counter;
            }
        }
    }

    /**
     * Returns the actual memory address of an index on CPU stack.
     */
    pub fn resolve_stack_addr(&mut self, index: u8) -> u16 {
        let stack_base:u16 = 0x0100;
        return stack_base + index as u16;
    }

    /**
     *  Pushes a byte to the stack and decrements the stack pointer
     */
    fn stack_push_byte(&mut self, data:u8) {
        let addr = self.resolve_stack_addr(self.stack_pointer);
        self.memory.write_byte(addr, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1)
    }

    /**
     *  Pops a byte from the stack and increments stack pointer
     */
    fn stack_pop_byte(&mut self) -> u8{
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let addr = self.resolve_stack_addr(self.stack_pointer);
        return self.memory.read_byte(addr);
    }

    /**
     *  Sets particular CPU flag
     */
    pub fn set_cpu_flag(&mut self, flag: CPUFlag, val: bool) {
        // Map flag bits
        let mask:u8 = match flag {
            CPUFlag::Carry =>               0b0000_0001,
            CPUFlag::Zero =>                0b0000_0010,
            CPUFlag::InterruptDisable =>    0b0000_0100,
            CPUFlag::Decimal =>             0b0000_1000,
            CPUFlag::B0 =>                  0b0001_0000,
            CPUFlag::B1 =>                  0b0010_0000,
            CPUFlag::Overflow =>            0b0100_0000,
            CPUFlag::Negative =>            0b1000_0000
        };
        // invert mask bits 00000001 -> 11111110
        let inverted_mask = 0b1111_1111 - mask;
        // set cpu flag to zero 1010_1011 & 1111_1110 -> 1010_1010   
        self.status = self.status & inverted_mask;
        if val {
            // set cpu flag to 1
            // 1010_1010 | 0000_0001 -> 1010_1010  
            self.status = self.status | mask;
        }
    }

    /**
     * Update zero and negative flags regarding to the arithmetic result.
     */
    fn update_zn_flags(&mut self, result: u8) {
        let val = result == 0;
        self.set_cpu_flag(CPUFlag::Zero, val);
        self.set_cpu_flag(CPUFlag::Negative, result & 0b1000_0000 != 0);
    }

}

// OpCodes implementations
impl CPU {
    fn op_nop(&mut self) {}

    fn op_lda(&mut self, mode: & AddressingMode) {
        let addr = self.resolve_operand_addr(mode);
        let value = self.memory.read_byte(addr);
        // Update register
        self.register_a = value;
        // Update flags
        self.update_zn_flags(value);
        println!("lda")
    }

    fn op_sta(&mut self, mode: & AddressingMode) {
        let addr = self.resolve_operand_addr(mode);
        self.memory.write_byte(addr, self.register_a);
    }

    fn op_tax(&mut self) {
        // get register value a
        let val_a = self.register_a;
        self.register_x = val_a;
        self.update_zn_flags(val_a);
        println!("tax")
    }

    fn op_inx(&mut self) {
        // increment register a
        self.register_x += 1;
        self.update_zn_flags(self.register_x);
        println!("inx")
    }

    fn op_txs(&mut self) {
        // Transfer X to Stack ptr
        self.stack_pointer = self.register_x;
        println!("txs");
    }

    fn op_tsx(&mut self) {
        self.register_x = self.stack_pointer;
        println!("tsx");
    }

    fn op_pha(&mut self) {
        // push a to stack
        self.stack_push_byte(self.register_a);
        // set flags
        self.update_zn_flags(self.register_a);
        println!("pha");
    }

    fn op_pla(&mut self) {
        // Pull from stack to resiter a
        let value = self.stack_pop_byte();
        self.register_a = value;
        // set flags
        self.update_zn_flags(value);
        println!("pla");
    }
    
    fn op_php(&mut self) {
        // Pushes a copy of the status flags on to the stack.
        let status_flag = self.status;
        self.stack_push_byte(status_flag);
        println!("php");
    }

    fn op_plp(&mut self) {
        // Pulls an 8 bit value from the stack and into the processor flags.
        let status = self.stack_pop_byte();
        self.status = status;
        println!("plp");
    }

}

