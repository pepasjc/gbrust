use std::fs::File;
use std::io::Read;

pub struct MMU {
    memory: [u8; 0x10000],  // 64KB of memory
    rom: Vec<u8>,
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            memory: [0; 0x10000],
            rom: Vec::new(),
        }
    }

    pub fn load_rom(&mut self, filename: &str) -> std::io::Result<()> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        
        // Copy ROM to memory
        for (i, &byte) in buffer.iter().enumerate() {
            if i < 0x8000 {  // ROM space is 0x0000-0x7FFF
                self.memory[i] = byte;
            }
        }
        self.rom = buffer;
        Ok(())
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}
