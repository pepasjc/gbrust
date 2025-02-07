pub struct CPU {
    // CPU registers
    pub a: u8,    // Accumulator
    pub f: u8,    // Flags
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,  // Stack pointer
    pub pc: u16,  // Program counter
    pub debug_mode: bool,
    pub mmu: Option<crate::mmu::MMU>,
}

// Flag bit positions
const ZERO_FLAG: u8 = 7;
const SUBTRACT_FLAG: u8 = 6;
const HALF_CARRY_FLAG: u8 = 5;
const CARRY_FLAG: u8 = 4;

#[derive(Debug, thiserror::Error)]
pub enum CPUError {
    #[error("No MMU connected")]
    NoMMU,
    #[error("Unknown opcode: {0:#04x}")]
    UnknownOpcode(u8),
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            debug_mode: false,
            mmu: None,
        }
    }

    pub fn initialize(&mut self) {
        // Initialize CPU to Game Boy boot state
        self.a = 0x01;
        self.f = 0xB0;
        self.b = 0x00;
        self.c = 0x13;
        self.d = 0x00;
        self.e = 0xD8;
        self.h = 0x01;
        self.l = 0x4D;
        self.sp = 0xFFFE;
        self.pc = 0x0000;
    }

    // Flag helpers
    pub fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.f |= 1 << flag;
        } else {
            self.f &= !(1 << flag);
        }
    }

    pub fn get_flag(&self, flag: u8) -> bool {
        (self.f & (1 << flag)) != 0
    }

    // region: 8-bit Load Instructions
    /// LD B,n - Load immediate value into B
    /// Opcode: 0x06
    /// Length: 2 bytes
    /// Flags: None affected
    /// Cycles: 8
    pub fn ld_b_n(&mut self, n: u8) {
        self.b = n;
    }

    /// LD C,n - Load immediate value into C
    /// Opcode: 0x0E
    /// Length: 2 bytes
    /// Flags: None affected
    /// Cycles: 8
    pub fn ld_c_n(&mut self, n: u8) {
        self.c = n;
    }

    /// LD (HL-),A - Load A into memory at address HL and decrement HL
    /// Opcode: 0x32
    /// Length: 1 byte
    /// Flags: None affected
    /// Cycles: 8
    pub fn ld_hl_dec_a(&mut self) -> Result<(), CPUError> {
        let hl = ((self.h as u16) << 8) | (self.l as u16);
        if let Some(mmu) = &mut self.mmu {
            mmu.write_byte(hl, self.a);
            // Decrement HL
            let new_hl = hl.wrapping_sub(1);
            self.h = (new_hl >> 8) as u8;
            self.l = new_hl as u8;
            Ok(())
        } else {
            Err(CPUError::NoMMU)
        }
    }
    // endregion

    // region: 8-bit Arithmetic Instructions
    /// INC B - Increment register B
    /// Opcode: 0x04
    /// Length: 1 byte
    /// Flags: Z 0 H -
    ///   Z: Set if result is zero
    ///   N: Reset
    ///   H: Set if carry from bit 3
    ///   C: Not affected
    pub fn inc_b(&mut self) {
        self.b = self.b.wrapping_add(1);
        self.set_flag(ZERO_FLAG, self.b == 0);
        self.set_flag(SUBTRACT_FLAG, false);
        self.set_flag(HALF_CARRY_FLAG, (self.b & 0x0F) == 0);
    }

    /// DEC B - Decrement register B
    /// Opcode: 0x05
    /// Length: 1 byte
    /// Flags: Z 1 H -
    ///   Z: Set if result is zero
    ///   N: Set
    ///   H: Set if no borrow from bit 4
    ///   C: Not affected
    pub fn dec_b(&mut self) {
        self.b = self.b.wrapping_sub(1);
        self.set_flag(ZERO_FLAG, self.b == 0);
        self.set_flag(SUBTRACT_FLAG, true);
        self.set_flag(HALF_CARRY_FLAG, (self.b & 0x0F) == 0x0F);
    }

    /// XOR A - Exclusive OR register A with A (zeros A)
    /// Opcode: 0xAF
    /// Length: 1 byte
    /// Flags: Z 0 0 0
    ///   Z: Set if result is zero (always in this case)
    ///   N: Reset
    ///   H: Reset
    ///   C: Reset
    pub fn xor_a(&mut self) {
        self.a ^= self.a; // A XOR A always equals 0
        self.set_flag(ZERO_FLAG, true);     // Result is always 0
        self.set_flag(SUBTRACT_FLAG, false); // Reset
        self.set_flag(HALF_CARRY_FLAG, false); // Reset
        self.set_flag(CARRY_FLAG, false);    // Reset
    }
    // endregion

    // region: 8-bit Rotation/Shift Instructions
    /// RRA - Rotate A right through carry
    /// Opcode: 0x1F
    /// Length: 1 byte
    /// Flags: 0 0 0 C
    ///   Z: Reset
    ///   N: Reset
    ///   H: Reset
    ///   C: Set to bit 0 of A before rotation
    pub fn rra(&mut self) {
        let carry = if self.get_flag(CARRY_FLAG) { 1 } else { 0 };
        let new_carry = self.a & 0x01;  // Get bit 0 before rotation
        
        self.a = (self.a >> 1) | (carry << 7);
        
        self.set_flag(ZERO_FLAG, false);
        self.set_flag(SUBTRACT_FLAG, false);
        self.set_flag(HALF_CARRY_FLAG, false);
        self.set_flag(CARRY_FLAG, new_carry == 1);
    }
    // endregion

    // region: 16-bit Load Instructions
    /// LD HL,nn - Load 16-bit immediate value into HL
    /// Opcode: 0x21
    /// Length: 3 bytes
    /// Flags: None affected
    /// Cycles: 12
    pub fn ld_hl_nn(&mut self, nn: u16) {
        self.l = (nn & 0xFF) as u8;
        self.h = (nn >> 8) as u8;
    }

    /// LD SP,nn - Load 16-bit immediate value into SP
    /// Opcode: 0x31
    /// Length: 3 bytes
    /// Flags: None affected
    /// Cycles: 12
    pub fn ld_sp_nn(&mut self, nn: u16) {
       self.sp = nn;
    }
    // endregion

    // region: 16-bit Jump Instructions
    /// JP nn - Jump to address nn
    /// Opcode: 0xC3
    /// Length: 3 bytes
    /// Flags: None affected
    /// Cycles: 16
    pub fn jp(&mut self, addr: u16) {
        self.pc = addr;
    }
    /// JR Nz n - Jump to address PC+n if Z flag is reset
    /// Opcode: 0x20
    /// Length: 2 bytes
    /// Flags: None affected
    /// Cycles: 12/8
    pub fn jr_nz_n(&mut self, n: u8) {
        if !self.get_flag(ZERO_FLAG) {
            self.pc = self.pc.wrapping_add(n as u16);
        }
    }

    // endregion

    // region: CPU Control Instructions
    /// NOP - No operation
    /// Opcode: 0x00
    /// Length: 1 byte
    /// Flags: None affected
    /// Cycles: 4
    pub fn nop(&mut self) {
        // Do nothing
    }
    // endregion

    // region: CPU Operation Functions
    pub fn step(&mut self) -> Result<(), CPUError> {
        if self.debug_mode {
            self.print_state();
            
            // Print next instruction
            if let Some(mmu) = &self.mmu {
                let opcode = mmu.read_byte(self.pc);
                println!("Next instruction at {:04X}: {:02X}", self.pc, opcode);
            }
        }
        
        let opcode = self.fetch_byte()?;
        self.execute(opcode)
    }

    pub fn execute(&mut self, opcode: u8) -> Result<(), CPUError> {
        match opcode {
            0x00 => {
                self.nop();
                Ok(())
            },
            0x06 => {
                let n = self.fetch_byte()?;
                self.ld_b_n(n);
                Ok(())
            },
            0x04 => {
                self.inc_b();
                Ok(())
            },
            0x05 => {
                self.dec_b();
                Ok(())
            },
            0x0E => {
                let n = self.fetch_byte()?;
                self.ld_c_n(n);
                Ok(())
            },
            0x1F => {
                if self.debug_mode {
                    println!("RRA");
                }
                self.rra();
                Ok(())
            },
            0x20 => {
                let n = self.fetch_byte()?;
                if self.debug_mode {
                    println!("JR NZ,${:02X}", n);
                }
                self.jr_nz_n(n);
                Ok(())
            }
            0x21 => {
                let nn = self.fetch_word()?;
                if self.debug_mode {
                    println!("LD HL,${:04X}", nn);
                }
                self.ld_hl_nn(nn);
                Ok(())
            },
            0x31 => {
                let nn = self.fetch_word()?;
                if self.debug_mode {
                    println!("LD SP,${:04X}", nn);
                }
                self.ld_sp_nn(nn);
                Ok(())
            },
            0x32 => {
                if self.debug_mode {
                    let hl = ((self.h as u16) << 8) | (self.l as u16);
                    println!("LD (HL-),A [HL=${:04X}, A=${:02X}]", hl, self.a);
                }
                self.ld_hl_dec_a()
            },
            0xAF => {
                if self.debug_mode {
                    println!("XOR A,A");
                }
                self.xor_a();
                Ok(())
            },
            0xC3 => {
                let addr = self.fetch_word()?;
                if self.debug_mode {
                    println!("JP ${:04X}", addr);
                }
                self.jp(addr);
                Ok(())
            },
            _ => Err(CPUError::UnknownOpcode(opcode)),
        }
    }
    // endregion

    // region: Helper Functions
    pub fn fetch_byte(&mut self) -> Result<u8, CPUError> {
        if let Some(mmu) = &self.mmu {
            let byte = mmu.read_byte(self.pc);
            self.pc = self.pc.wrapping_add(1);
            Ok(byte)
        } else {
            Err(CPUError::NoMMU)
        }
    }

    pub fn fetch_word(&mut self) -> Result<u16, CPUError> {
        if let Some(mmu) = &self.mmu {
            let low_byte = mmu.read_byte(self.pc);
            let high_byte = mmu.read_byte(self.pc.wrapping_add(1));
            self.pc = self.pc.wrapping_add(2);
            Ok(((high_byte as u16) << 8) | (low_byte as u16))
        } else {
            Err(CPUError::NoMMU)
        }
    }

    pub fn set_mmu(&mut self, mmu: crate::mmu::MMU) {
        self.mmu = Some(mmu);
    }

    pub fn print_state(&self) {
        println!("\nCPU State:");
        println!("AF: {:02X}{:02X} BC: {:02X}{:02X} DE: {:02X}{:02X} HL: {:02X}{:02X}",
            self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l);
        println!("PC: {:04X} SP: {:04X}", self.pc, self.sp);
        println!("Flags: Z:{} N:{} H:{} C:{}\n",
            self.get_flag(ZERO_FLAG) as u8,
            self.get_flag(SUBTRACT_FLAG) as u8,
            self.get_flag(HALF_CARRY_FLAG) as u8,
            self.get_flag(CARRY_FLAG) as u8);
    }
    // endregion
}
