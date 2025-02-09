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
    pub interrupt_enabled: bool,  // Add this new field
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
            interrupt_enabled: true,  // Add this line
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

    // LD A, D - Load register D into A
    /// Opcode: 0x7A
    /// Length: 1 byte
    /// Flags: None affected
    /// Cycles: 4
    pub fn ld_a_d(&mut self) {
        self.a = self.d;
    }

    /// LD A,n - Load immediate value into A
    /// Opcode: 0x3E
    /// Length: 2 bytes
    /// Flags: None affected
    /// Cycles: 8
    pub fn ld_a_n(&mut self, n: u8) {
        self.a = n;
    }

    /// LDH (n),A - Store A into high RAM address (FF00+n)
    /// Opcode: 0xE0
    /// Length: 2 bytes
    /// Flags: None affected
    /// Cycles: 12
    pub fn ldh_n_a(&mut self, n: u8) -> Result<(), CPUError> {
        let address = 0xFF00 | (n as u16);
        
        if let Some(mmu) = &mut self.mmu {
            mmu.write_byte(address, self.a);
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
    /// /// Cycles: 4
    pub fn inc_b(&mut self) {
        self.b = self.b.wrapping_add(1);
        self.set_flag(ZERO_FLAG, self.b == 0);
        self.set_flag(SUBTRACT_FLAG, false);
        self.set_flag(HALF_CARRY_FLAG, (self.b & 0x0F) == 0);
    }

    /// INC C - Increment register C
    /// Opcode: 0x0C
    /// Length: 1 byte
    /// Flags: Z 0 H -
    ///  Z: Set if result is zero
    /// N: Reset
    /// H: Set if carry from bit 3
    /// C: Not affected
    /// Cycles: 4
    pub fn inc_c(&mut self) {
        self.c = self.c.wrapping_add(1);
        self.set_flag(ZERO_FLAG, self.c == 0);
        self.set_flag(SUBTRACT_FLAG, false);
        self.set_flag(HALF_CARRY_FLAG, (self.c & 0x0F) == 0);
    }

    /// INC D - Increment register D
    /// Opcode: 0x14
    /// Length: 1 byte
    /// Flags: Z 0 H -
    ///  Z: Set if result is zero
    /// N: Reset
    /// H: Set if carry from bit 3
    /// C: Not affected
    /// Cycles: 4
    pub fn inc_d(&mut self) {
        self.d = self.d.wrapping_add(1);
        self.set_flag(ZERO_FLAG, self.d == 0);
        self.set_flag(SUBTRACT_FLAG, false);
        self.set_flag(HALF_CARRY_FLAG, (self.d & 0x0F) == 0);
    }

    /// DEC B - Decrement register B
    /// Opcode: 0x05
    /// Length: 1 byte
    /// Flags: Z 1 H -
    ///   Z: Set if result is zero
    ///   N: Set
    ///   H: Set if no borrow from bit 4
    ///   C: Not affected
    /// Cycles: 4
    pub fn dec_b(&mut self) {
        self.b = self.b.wrapping_sub(1);
        self.set_flag(ZERO_FLAG, self.b == 0);
        self.set_flag(SUBTRACT_FLAG, true);
        self.set_flag(HALF_CARRY_FLAG, (self.b & 0x0F) == 0x0F);
    }

    /// DEC C - Decrement register C
    /// Opcode: 0x0D
    /// Length: 1 byte
    /// Flags: Z 1 H -
    /// Z: Set if result is zero
    /// N: Set
    /// H: Set if no borrow from bit 4
    /// C: Not affected
    /// Cycles: 4
    pub fn dec_c(&mut self) {
        self.c = self.c.wrapping_sub(1);
        self.set_flag(ZERO_FLAG, self.c == 0);
        self.set_flag(SUBTRACT_FLAG, true);
        self.set_flag(HALF_CARRY_FLAG, (self.c & 0x0F) == 0x0F);
    }


    /// DEC D - Decrement register D
    /// Opcode: 0x15
    /// Length: 1 byte
    /// Flags: Z 1 H -
    ///  Z: Set if result is zero
    /// N: Set
    /// H: Set if no borrow from bit 4
    /// C: Not affected
    /// Cycles: 4
    pub fn dec_d(&mut self) {
        self.d = self.d.wrapping_sub(1);
        self.set_flag(ZERO_FLAG, self.d == 0);
        self.set_flag(SUBTRACT_FLAG, true);
        self.set_flag(HALF_CARRY_FLAG, (self.d & 0x0F) == 0x0F);
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

    /// ADC A,C - Add C and Carry flag to A
    /// Opcode: 0x89
    /// Length: 1 byte
    /// Flags: Z 0 H C
    ///   Z: Set if result is zero
    ///   N: Reset
    ///   H: Set if carry from bit 3
    ///   C: Set if carry from bit 7
    pub fn adc_a_c(&mut self) {
        let a = self.a as u16;
        let c = self.c as u16;
        let carry = if self.get_flag(CARRY_FLAG) { 1u16 } else { 0u16 };
        
        let result = a + c + carry;
        let half_carry = ((a & 0x0F) + (c & 0x0F) + carry) > 0x0F;
        
        self.a = result as u8;
        
        self.set_flag(ZERO_FLAG, self.a == 0);
        self.set_flag(SUBTRACT_FLAG, false);
        self.set_flag(HALF_CARRY_FLAG, half_carry);
        self.set_flag(CARRY_FLAG, result > 0xFF);
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
            self.pc = self.pc.wrapping_add(n as i8 as i16 as u16);
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

    /// DI - Disable interrupts
    /// Opcode: 0xF3
    /// Length: 1 byte
    /// Flags: None affected
    /// Cycles: 4
    pub fn di(&mut self) {
        self.interrupt_enabled = false;
    }

    /// EI - Enable interrupts
    /// Opcode: 0xFB
    /// Length: 1 byte
    /// Flags: None affected
    /// Cycles: 4
    pub fn ei(&mut self) {
        self.interrupt_enabled = true;
    }
    // endregion

    // region: Stack Operations
    /// RST 18h - Push current PC on stack and jump to 0x0018
    /// Opcode: 0xDF
    /// Length: 1 byte
    /// Flags: None affected
    /// Cycles: 16
    pub fn rst_18(&mut self) -> Result<(), CPUError> {
        // Decrement SP and write high byte
        self.sp = self.sp.wrapping_sub(1);
        if let Some(mmu) = &mut self.mmu {
            mmu.write_byte(self.sp, (self.pc >> 8) as u8);
            
            // Decrement SP and write low byte
            self.sp = self.sp.wrapping_sub(1);
            mmu.write_byte(self.sp, self.pc as u8);
            
            // Jump to 0x0018
            self.pc = 0x0018;
            Ok(())
        } else {
            Err(CPUError::NoMMU)
        }
    }

    /// RST 38h - Push current PC on stack and jump to 0x0038
    /// Opcode: 0xFF
    /// Length: 1 byte
    /// Flags: None affected
    /// Cycles: 16
    pub fn rst_38(&mut self) -> Result<(), CPUError> {
        // Decrement SP and write high byte
        self.sp = self.sp.wrapping_sub(1);
        if let Some(mmu) = &mut self.mmu {
            mmu.write_byte(self.sp, (self.pc >> 8) as u8);
            
            // Decrement SP and write low byte
            self.sp = self.sp.wrapping_sub(1);
            mmu.write_byte(self.sp, self.pc as u8);
            
            // Jump to 0x0038
            self.pc = 0x0038;
            Ok(())
        } else {
            Err(CPUError::NoMMU)
        }
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
                if self.debug_mode {
                    println!("NOP - No operation");
                }
                self.nop();
                Ok(())
            },
            0x06 => {
                let n = self.fetch_byte()?;
                if self.debug_mode {
                    println!("LD B,n - Load immediate value into B (n={:02X})", n);
                }
                self.ld_b_n(n);
                Ok(())
            },
            0x04 => {
                if self.debug_mode {
                    println!("INC B - Increment register B");
                }
                self.inc_b();
                Ok(())
            },
            0x05 => {
                if self.debug_mode {
                    println!("DEC B - Decrement register B");
                }
                self.dec_b();
                Ok(())
            },
            0x0C => {
                if self.debug_mode {
                    println!("INC C - Increment register C");
                }
                self.inc_c();
                Ok(())
            },
            0x0D => {
                if self.debug_mode {
                    println!("DEC C - Decrement register C");
                }
                self.dec_c();
                Ok(())
            },
            0x0E => {
                let n = self.fetch_byte()?;
                if self.debug_mode {
                    println!("LD C,n - Load immediate value into C (n={:02X})", n);
                }
                self.ld_c_n(n);
                Ok(())
            },
            0x14 => {
                if self.debug_mode {
                    println!("INC D - Increment register D");
                }
                self.inc_d();
                Ok(())
            },
            0x15 => {
                if self.debug_mode {
                    println!("DEC D - Decrement register D");
                }
                self.dec_d();
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
            },
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
            0x3E => {
                let n = self.fetch_byte()?;
                if self.debug_mode {
                    println!("LD A,${:02X}", n);
                }
                self.ld_a_n(n);
                Ok(())
            },
            0x89 => {
                if self.debug_mode {
                    println!("ADC A,C");
                }
                self.adc_a_c();
                Ok(())
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
            0xDF => {
                if self.debug_mode {
                    println!("RST 18H");
                }
                self.rst_18()
            },
            0xFF => {
                if self.debug_mode {
                    println!("RST 38H");
                }
                self.rst_38()
            },
            0x7A => {
                if self.debug_mode {
                    println!("LD A,D");
                }
                self.ld_a_d();
                Ok(())
            },
            0xE0 => {
                let n = self.fetch_byte()?;
                if self.debug_mode {
                    println!("LDH (${:02X}),A [A=${:02X}]", n, self.a);
                }
                self.ldh_n_a(n)
            }
            0xF3 => {
                if self.debug_mode {
                    println!("DI - Disable interrupts");
                }
                self.di();
                Ok(())
            },
            0xFB => {
                if self.debug_mode {
                    println!("EI - Enable interrupts");
                }
                self.ei();
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
        println!("Flags: Z:{} N:{} H:{} C:{}",
            self.get_flag(ZERO_FLAG) as u8,
            self.get_flag(SUBTRACT_FLAG) as u8,
            self.get_flag(HALF_CARRY_FLAG) as u8,
            self.get_flag(CARRY_FLAG) as u8);
        println!("Interrupts: {}\n", if self.interrupt_enabled { "Enabled" } else { "Disabled" });
    }
    // endregion
}
