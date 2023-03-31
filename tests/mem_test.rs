use nesguin::emu6502::ram::RAM;

#[test] 
fn read_byte_test() {
    const DATA:u8 = 26;
    const ADDRESS:u16 = 0x0A11;
    let mut memory = RAM::new();
    // write a byte
    memory.write_byte(ADDRESS, DATA);
    // read byte
    let read_data = memory.read_byte(ADDRESS);
    assert_eq!(DATA, read_data)
}

#[test] 
fn read_word_test() {
    // write word 
    const ADDRESS:u16 = 0xFFFD;
    const DATA:u16 = 0xABC1;
    let mut memory = RAM::new();
    memory.write_word(ADDRESS, DATA);
    let read_data = memory.read_word(ADDRESS);
    assert_eq!(DATA, read_data);
}

