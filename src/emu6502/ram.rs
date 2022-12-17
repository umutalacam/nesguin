pub struct RAM {
    pub mem_array: [u8; 0xFFFF]
}

impl RAM {
    pub fn new() -> Self {
        RAM { 
            mem_array: [0; 0xFFFF]
        }
    }

    pub fn write_byte(&mut self, index:u16, data:u8) {
        // Check address
        // Put into the address
        self.mem_array[index as usize] = data;
    }

    pub fn read_byte(&mut self, index:u16) -> u8 {
        // load from memory
        return self.mem_array[index as usize];
    }

    pub fn write_word(&mut self, index:u16, data:u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0x00FF) as u8;
        self.write_byte(index, lo);
        self.write_byte(index + 1, hi);
    }

    pub fn read_word(&mut self, index:u16) -> u16 {
        let lo = self.read_byte(index) as u16;
        let hi = self.read_byte(index + 1) as u16;
        return (hi << 8) | lo;
    }

}
