pub struct MMU {
    memory: [u8; 0x10000],  // 64KB of memory
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            memory: [0; 0x10000],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}
