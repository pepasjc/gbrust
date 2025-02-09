use std::fs::File;
use std::io::Read;

// LCD Register addresses
const LCDC: u16 = 0xFF40;  // LCD Control
const STAT: u16 = 0xFF41;  // LCD Status
const LY: u16   = 0xFF44;  // LCD Y-Coordinate
const LYC: u16  = 0xFF45;  // LY Compare

#[derive(Debug)]
pub struct CartridgeHeader {
    pub title: String,
    pub cartridge_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
}

pub struct MMU {
    // Memory regions
    rom_bank0: [u8; 0x4000],      // 0000-3FFF Fixed ROM bank
    rom_bankn: [u8; 0x4000],      // 4000-7FFF Switchable ROM bank
    vram: [u8; 0x2000],           // 8000-9FFF Video RAM
    ext_ram: [u8; 0x2000],        // A000-BFFF External RAM
    wram: [u8; 0x2000],           // C000-DFFF Work RAM
    oam: [u8; 0xA0],              // FE00-FE9F Sprite info
    io_regs: [u8; 0x80],          // FF00-FF7F I/O Registers
    hram: [u8; 0x7F],             // FF80-FFFE High RAM
    ie_register: u8,              // FFFF Interrupt Enable
    pub header: Option<CartridgeHeader>,

    // LCD timing
    pub cycles: u32,
    pub scanline: u8,
    pub mode: u8,
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            rom_bank0: [0; 0x4000],
            rom_bankn: [0; 0x4000],
            vram: [0; 0x2000],
            ext_ram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            io_regs: [0; 0x80],
            hram: [0; 0x7F],
            ie_register: 0,
            header: None,
            cycles: 0,
            scanline: 0,
            mode: 0,
        }
    }

    fn parse_header(&mut self) {
        // Read cartridge header from ROM bank 0
        let title = String::from_utf8_lossy(&self.rom_bank0[0x134..=0x143])
            .trim_matches(char::from(0))
            .to_string();
        
        let cartridge_type = self.rom_bank0[0x147];
        let rom_size = self.rom_bank0[0x148];
        let ram_size = self.rom_bank0[0x149];

        self.header = Some(CartridgeHeader {
            title,
            cartridge_type,
            rom_size,
            ram_size,
        });
    }

    pub fn load_rom(&mut self, filename: &str) -> std::io::Result<()> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        
        // Load first ROM bank (0x0000-0x3FFF)
        for (i, &byte) in buffer.iter().take(0x4000).enumerate() {
            self.rom_bank0[i] = byte;
        }
        
        // Load second ROM bank (0x4000-0x7FFF)
        if buffer.len() > 0x4000 {
            for (i, &byte) in buffer[0x4000..].iter().take(0x4000).enumerate() {
                self.rom_bankn[i] = byte;
            }
        }

        // Parse cartridge header
        self.parse_header();
        
        if let Some(ref header) = self.header {
            println!("Loaded ROM: {}", header.title);
            println!("Cartridge type: 0x{:02X}", header.cartridge_type);
            println!("ROM size: 0x{:02X}", header.rom_size);
            println!("RAM size: 0x{:02X}", header.ram_size);
        }

        Ok(())
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom_bank0[address as usize],
            0x4000..=0x7FFF => self.rom_bankn[(address - 0x4000) as usize],
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xA000..=0xBFFF => self.ext_ram[(address - 0xA000) as usize],
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize], // Echo RAM
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            0xFF00..=0xFF7F => self.io_regs[(address - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.ie_register,
            _ => 0xFF, // Unmapped memory returns 0xFF
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => (), // ROM is read-only
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize] = value,
            0xA000..=0xBFFF => self.ext_ram[(address - 0xA000) as usize] = value,
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize] = value,
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize] = value, // Echo RAM
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = value,
            0xFF00..=0xFF7F => self.io_regs[(address - 0xFF00) as usize] = value,
            0xFF40 => {  // LCDC
                self.io_regs[(address - 0xFF00) as usize] = value;
            },
            0xFF41 => {  // STAT
                // Only bits 3-6 are writable
                let current = self.io_regs[(address - 0xFF00) as usize];
                self.io_regs[(address - 0xFF00) as usize] = (value & 0x78) | (current & 0x87);
            },
            0xFF44 => {  // LY is read-only
                // Reset LY when written to (this is Game Boy behavior)
                self.scanline = 0;
                self.io_regs[(address - 0xFF00) as usize] = 0;
            },
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.ie_register = value,
            _ => (), // Ignore writes to unmapped memory
        }
    }

    pub fn update_lcd(&mut self, cycles: u32) {
        self.cycles += cycles;

        if self.cycles >= 456 {  // One scanline takes 456 cycles
            self.cycles -= 456;
            self.scanline = (self.scanline + 1) % 154;
            self.write_byte(LY, self.scanline);

            // Mode 2: Scanning OAM - 80 cycles
            if self.cycles <= 80 {
                self.mode = 2;
            }
            // Mode 3: Drawing pixels - 172 cycles
            else if self.cycles <= 252 {
                self.mode = 3;
            }
            // Mode 0: HBlank - 204 cycles
            else {
                self.mode = 0;
            }

            // Mode 1: VBlank (lines 144-153)
            if self.scanline >= 144 {
                self.mode = 1;
            }

            // Update LCD status register
            let mut stat = self.read_byte(STAT) & 0xFC;  // Clear lower 2 bits
            stat |= self.mode;
            self.write_byte(STAT, stat);
        }
    }
}
