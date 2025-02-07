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
    mmu: Option<crate::mmu::MMU>,
}

// Flag bit positions
const ZERO_FLAG: u8 = 7;
const SUBTRACT_FLAG: u8 = 6;
const HALF_CARRY_FLAG: u8 = 5;
const CARRY_FLAG: u8 = 4;

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

    // Instructions
    pub fn ld_b_n(&mut self, n: u8) {
        self.b = n;
    }

    pub fn inc_b(&mut self) {
        self.b = self.b.wrapping_add(1);
        self.set_flag(ZERO_FLAG, self.b == 0);
        self.set_flag(SUBTRACT_FLAG, false);
        self.set_flag(HALF_CARRY_FLAG, (self.b & 0x0F) == 0);
    }

    pub fn dec_b(&mut self) {
        self.b = self.b.wrapping_sub(1);
        self.set_flag(ZERO_FLAG, self.b == 0);
        self.set_flag(SUBTRACT_FLAG, true);
        self.set_flag(HALF_CARRY_FLAG, (self.b & 0x0F) == 0x0F);
    }

    pub fn set_mmu(&mut self, mmu: crate::mmu::MMU) {
        self.mmu = Some(mmu);
    }
}
