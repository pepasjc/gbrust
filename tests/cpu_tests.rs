use gbrust::cpu::CPU;

// Flag bit positions (copied from cpu.rs since they're private)
const ZERO_FLAG: u8 = 7;
const SUBTRACT_FLAG: u8 = 6;
const HALF_CARRY_FLAG: u8 = 5;
const CARRY_FLAG: u8 = 4;

#[test]
fn test_ld_b_n() {
    // Test loading an immediate value into register B
    // Expected: Register B should contain 0x42 after execution
    let mut cpu = CPU::new();
    cpu.ld_b_n(0x42);
    assert_eq!(cpu.b, 0x42);
}

#[test]
fn test_ld_c_n() {
    // Test loading an immediate value into register C
    // Expected: Register C should contain 0x42 after execution
    let mut cpu = CPU::new();
    cpu.ld_c_n(0x42);
    assert_eq!(cpu.c, 0x42);
}

#[test]
fn test_inc_b() {
    // Test 1: incrementing register B from 0x41 to 0x42
    // Expected: 
    // - B should be 0x42
    // - Zero flag should be false (result is not zero)
    // - Subtract flag should be false (we're adding)
    let mut cpu = CPU::new();
    cpu.b = 0x41;
    cpu.inc_b();
    assert_eq!(cpu.b, 0x42);
    assert_eq!(cpu.get_flag(ZERO_FLAG), false);
    assert_eq!(cpu.get_flag(SUBTRACT_FLAG), false);

    // Test 2: incrementing register B from 0xFF to 0x00 (overflow case)
    // Expected:
    // - B should wrap to 0x00
    // - Zero flag should be true (result is zero)
    cpu.initialize();
    cpu.b = 0xFF;
    cpu.inc_b();
    assert_eq!(cpu.b, 0x00);
    assert_eq!(cpu.get_flag(ZERO_FLAG), true);

    // Test 3: incrementing register B from 0x0F to 0x10 with half carry
    // Expected:
    // - B should be 0x10
    // - Half carry flag should be true (half carry from bit 3)
    cpu.initialize();
    cpu.b = 0x0F;
    cpu.inc_b();
    assert_eq!(cpu.b, 0x10);
    assert_eq!(cpu.get_flag(HALF_CARRY_FLAG), true);

}
#[test]
fn test_inc_c() {
    // Test incrementing register C from 0x41 to 0x42
    // Expected:
    // - C should be 0x42
    // - Zero flag should be false (result is not zero)
    // - Subtract flag should be false (we're adding)
    let mut cpu = CPU::new();
    cpu.c = 0x41;
    cpu.inc_c();
    assert_eq!(cpu.c, 0x42);
    assert_eq!(cpu.get_flag(ZERO_FLAG), false);
    assert_eq!(cpu.get_flag(SUBTRACT_FLAG), false);
}
#[test]
fn test_inc_d() {
    // Test incrementing register D from 0x41 to 0x42
    // Expected:
    // - D should be 0x42
    // - Zero flag should be false (result is not zero)
    // - Subtract flag should be false (we're adding)
    let mut cpu = CPU::new();
    cpu.d = 0x41;
    cpu.inc_d();
    assert_eq!(cpu.d, 0x42);
    assert_eq!(cpu.get_flag(ZERO_FLAG), false);
    assert_eq!(cpu.get_flag(SUBTRACT_FLAG), false);
}

#[test]
fn test_dec_b() {
    // Test 1: decrementing register B from 0x42 to 0x41
    // Expected:
    // - B should be 0x41
    // - Zero flag should be false (result is not zero)
    // - Subtract flag should be true (we're subtracting)
    let mut cpu = CPU::new();
    cpu.b = 0x42;
    cpu.dec_b();
    assert_eq!(cpu.b, 0x41);
    assert_eq!(cpu.get_flag(ZERO_FLAG), false);
    assert_eq!(cpu.get_flag(SUBTRACT_FLAG), true);

    // Test 2: decrementing register B from 0x01 to 0x00
    // Expected:
    // - B should be 0x00
    // - Zero flag should be true (result is zero)
    cpu.initialize();
    cpu.b = 0x01;
    cpu.dec_b();
    assert_eq!(cpu.b, 0x00);
    assert_eq!(cpu.get_flag(ZERO_FLAG), true);

    // Test 3: decrementing register B from 0x00 to 0xFF (overflow case)
    // Expected:
    // - B should wrap to 0xFF
    // - Zero flag should be false (result is not zero)
    cpu.initialize();
    cpu.b = 0x00;
    cpu.dec_b();
    assert_eq!(cpu.b, 0xFF);
    assert_eq!(cpu.get_flag(ZERO_FLAG), false);

    // Test 4: decrementing register B from 0x10 to 0x0F with half carry
    // Expected:
    // - B should be 0x0F
    // - Half carry flag should be true (half carry from bit 4)
    cpu.initialize();
    cpu.b = 0x10;
    cpu.dec_b();
    assert_eq!(cpu.b, 0x0F);
    assert_eq!(cpu.get_flag(HALF_CARRY_FLAG), true);


}

#[test]
fn test_dec_c() {
    // Test decrementing register C from 0x42 to 0x41
    // Expected:
    // - C should be 0x41
    // - Zero flag should be false (result is not zero)
    // - Subtract flag should be true (we're subtracting)
    let mut cpu = CPU::new();
    cpu.c = 0x42;
    cpu.dec_c();
    assert_eq!(cpu.c, 0x41);
    assert_eq!(cpu.get_flag(ZERO_FLAG), false);
    assert_eq!(cpu.get_flag(SUBTRACT_FLAG), true);
}
#[test]
fn test_dec_d() {
    // Test decrementing register D from 0x42 to 0x41
    // Expected:
    // - D should be 0x41
    // - Zero flag should be false (result is not zero)
    // - Subtract flag should be true (we're subtracting)
    let mut cpu = CPU::new();
    cpu.d = 0x42;
    cpu.dec_d();
    assert_eq!(cpu.d, 0x41);
    assert_eq!(cpu.get_flag(ZERO_FLAG), false);
    assert_eq!(cpu.get_flag(SUBTRACT_FLAG), true);
}

#[test]
fn test_jp() {
    // Test jumping to a specific address
    // Expected: PC should be set to 0x1234 after execution
    let mut cpu = CPU::new();
    cpu.jp(0x1234);
    assert_eq!(cpu.pc, 0x1234);
}

#[test]
fn test_jr_nz_n() {
    // Test 1 jumping to a relative address if Z flag is reset
    // Expected: PC should be set to 0x1234 after execution
    let mut cpu = CPU::new();
    cpu.set_flag(ZERO_FLAG, false);
    cpu.pc = 0x1230;
    cpu.jr_nz_n(4);
    assert_eq!(cpu.pc, 0x1234);

    // Test 2 jumping to a relativer address if Z flag is reset (negative offset)
    // Expected: PC should be set to 0x122C after execution
    cpu.initialize();
    cpu.set_flag(ZERO_FLAG, false);
    cpu.pc = 0x1230;
    cpu.jr_nz_n(0xFC);
    assert_eq!(cpu.pc, 0x122C);

    // Test 3 not jumping to a relative address if Z flag is set
    cpu.initialize();
    cpu.set_flag(ZERO_FLAG, true);
    cpu.pc = 0x1230;
    cpu.jr_nz_n(4);
    assert_eq!(cpu.pc, 0x1230);
}

#[test]
fn test_xor_a() {
    // Test XORing A with itself
    // Expected:
    // - A should be 0
    // - Zero flag should be set
    // - All other flags should be reset
    let mut cpu = CPU::new();
    cpu.a = 0xFF; // Set A to non-zero value
    cpu.xor_a();
    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.get_flag(ZERO_FLAG), true);
    assert_eq!(cpu.get_flag(SUBTRACT_FLAG), false);
    assert_eq!(cpu.get_flag(HALF_CARRY_FLAG), false);
    assert_eq!(cpu.get_flag(CARRY_FLAG), false);
}

#[test]
fn test_ld_hl_nn() {
    // Test loading 16-bit immediate value into HL
    // Expected:
    // - H should contain high byte (0x12)
    // - L should contain low byte (0x34)
    let mut cpu = CPU::new();
    cpu.ld_hl_nn(0x1234);
    assert_eq!(cpu.h, 0x12);
    assert_eq!(cpu.l, 0x34);
}

#[test]
fn test_ld_sp_nn() {
    // Test loading 16-bit immediate value into SP
    // Expected: SP should be set to 0x1234 after execution
    let mut cpu = CPU::new();
    cpu.ld_sp_nn(0x1234);
    assert_eq!(cpu.sp, 0x1234);
}

#[test]
fn test_ld_hl_dec_a() {
    // Test storing A into (HL) and decrementing HL
    let mut cpu = CPU::new();
    let mut mmu = gbrust::mmu::MMU::new();
    
    // Initialize CPU and MMU state
    cpu.set_mmu(mmu);
    cpu.a = 0x42;
    cpu.h = 0x80;  // Changed from 0x20 to 0x80 to write to VRAM instead of ROM
    cpu.l = 0x00;  // HL = 0x8000 (start of VRAM)
    
    cpu.ld_hl_dec_a().unwrap();
    
    // Check if value was written to memory
    if let Some(ref mmu) = cpu.mmu {
        assert_eq!(mmu.read_byte(0x8000), 0x42);
    }
    
    // Check if HL was decremented
    assert_eq!(cpu.h, 0x7F);
    assert_eq!(cpu.l, 0xFF);
}

#[test]
fn test_rra() {
    // Test rotating A right through carry
    // Expected:
    // - Bit 0 moves to carry
    // - Carry moves to bit 7
    // - Zero flag is reset
    let mut cpu = CPU::new();
    
    // Test case 1: with carry flag reset
    cpu.a = 0x85;  // 1000 0101
    cpu.set_flag(CARRY_FLAG, false);
    cpu.rra();
    assert_eq!(cpu.a, 0x42);  // 0100 0010
    assert_eq!(cpu.get_flag(CARRY_FLAG), true);  // last bit was 1
    assert_eq!(cpu.get_flag(ZERO_FLAG), false);
    
    // Test case 2: with carry flag set
    cpu.a = 0x04;  // 0000 0100
    cpu.set_flag(CARRY_FLAG, true);
    cpu.rra();
    assert_eq!(cpu.a, 0x82);  // 1000 0010
    assert_eq!(cpu.get_flag(CARRY_FLAG), false);  // last bit was 0
}

#[test]
fn test_ld_a_d() {
    // Test loading A with D
    // Expected: A should be set to 0x42 after execution
    let mut cpu = CPU::new();
    assert_eq!(cpu.a, 0);
    cpu.d = 0x42;
    cpu.ld_a_d();
    assert_eq!(cpu.a, 0x42);
}

#[test]
fn test_adc_a_c() {
    let mut cpu = CPU::new();

    // Test case 1: Simple addition with no carry
    cpu.a = 0x11;
    cpu.c = 0x22;
    cpu.set_flag(CARRY_FLAG, false);
    cpu.adc_a_c();
    assert_eq!(cpu.a, 0x33);
    assert_eq!(cpu.get_flag(CARRY_FLAG), false);
    assert_eq!(cpu.get_flag(ZERO_FLAG), false);

    // Test case 2: Addition with initial carry flag set
    cpu.a = 0x11;
    cpu.c = 0x22;
    cpu.set_flag(CARRY_FLAG, true);
    cpu.adc_a_c();
    assert_eq!(cpu.a, 0x34);  // 0x11 + 0x22 + 1
    assert_eq!(cpu.get_flag(CARRY_FLAG), false);

    // Test case 3: Addition causing carry
    cpu.a = 0xFF;
    cpu.c = 0x02;
    cpu.set_flag(CARRY_FLAG, false);
    cpu.adc_a_c();
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.get_flag(CARRY_FLAG), true);

    // Test case 4: Addition causing half carry
    cpu.a = 0x0F;
    cpu.c = 0x01;
    cpu.set_flag(CARRY_FLAG, false);
    cpu.adc_a_c();
    assert_eq!(cpu.a, 0x10);
    assert_eq!(cpu.get_flag(HALF_CARRY_FLAG), true);
}

#[test]
fn test_rst_18() {
    // Test RST 18H instruction
    // Expected:
    // - PC should be pushed onto stack
    // - SP should be decremented by 2
    // - PC should jump to 0x0018
    let mut cpu = CPU::new();
    let mmu = gbrust::mmu::MMU::new();
    
    cpu.set_mmu(mmu);
    cpu.sp = 0xFFFE;
    cpu.pc = 0x1234;
    
    cpu.rst_18().unwrap();
    
    // Check if PC was correctly pushed to stack
    if let Some(ref mmu) = cpu.mmu {
        assert_eq!(mmu.read_byte(0xFFFD), 0x12);  // High byte
        assert_eq!(mmu.read_byte(0xFFFC), 0x34);  // Low byte
    }
    
    // Check if SP was decremented
    assert_eq!(cpu.sp, 0xFFFC);
    
    // Check if PC jumped to correct address
    assert_eq!(cpu.pc, 0x0018);
}

#[test]
fn test_rst_38() {
    // Test RST 38H instruction
    // Expected:
    // - PC should be pushed onto stack
    // - SP should be decremented by 2
    // - PC should jump to 0x0038
    let mut cpu = CPU::new();
    let mmu = gbrust::mmu::MMU::new();
    
    cpu.set_mmu(mmu);
    cpu.sp = 0xFFFE;
    cpu.pc = 0x1234;
    
    cpu.rst_38().unwrap();
    
    // Check if PC was correctly pushed to stack
    if let Some(ref mmu) = cpu.mmu {
        assert_eq!(mmu.read_byte(0xFFFD), 0x12);  // High byte
        assert_eq!(mmu.read_byte(0xFFFC), 0x34);  // Low byte
    }
    
    // Check if SP was decremented
    assert_eq!(cpu.sp, 0xFFFC);
    
    // Check if PC jumped to correct address
    assert_eq!(cpu.pc, 0x0038);
}

#[test]
fn test_ld_a_n() {
    // Test loading immediate value into A
    // Expected: A should contain 0x42 after execution
    let mut cpu = CPU::new();
    cpu.ld_a_n(0x42);
    assert_eq!(cpu.a, 0x42);
}

#[test]
fn test_di() {
    // Test disabling interrupts
    // Expected: interrupt_enabled flag should be false after execution
    let mut cpu = CPU::new();
    assert_eq!(cpu.interrupt_enabled, true);  // Should be enabled by default
    cpu.di();
    assert_eq!(cpu.interrupt_enabled, false);
}

#[test]
fn test_ei() {
    // Test enabling interrupts
    // Expected: interrupt_enabled flag should be true after execution
    let mut cpu = CPU::new();
    cpu.interrupt_enabled = false;
    cpu.ei();
    assert_eq!(cpu.interrupt_enabled, true);
}

#[test]
fn test_ldh_n_a() {
    // Test storing A in high RAM (FF00+n)
    // Expected:
    // - Memory at FF00+n should contain value of A
    let mut cpu = CPU::new();
    let mmu = gbrust::mmu::MMU::new();
    
    cpu.set_mmu(mmu);
    cpu.a = 0x42;

    let n: u8 = 0x80;
    
    // Store A at FF80
    cpu.pc = 0;
    if let Some(ref mut mmu) = cpu.mmu {
        mmu.write_byte(cpu.pc, 0x80); // Write offset 0x80 as immediate value
    }
    
    cpu.ldh_n_a(n).unwrap();
    
    // Check if value was written to high RAM
    if let Some(ref mmu) = cpu.mmu {
        assert_eq!(mmu.read_byte(0xFF80), 0x42);
    }
}

#[test]
fn test_ldh_a_n() {
    // Test loading A from high RAM (FF00+n)
    // Expected:
    // - A should contain value from memory at FF00+n
    let mut cpu = CPU::new();
    let mmu = gbrust::mmu::MMU::new();
    
    cpu.set_mmu(mmu);
    cpu.a = 0;  // Clear A
    
    let n: u8 = 0x80;
    
    // Store value at FF80
    if let Some(ref mut mmu) = cpu.mmu {
        mmu.write_byte(0xFF80, 0x42);
    }
    
    // Load value from FF80 into A
    cpu.pc = 0;
    if let Some(ref mut mmu) = cpu.mmu {
        mmu.write_byte(cpu.pc, 0x80);  // Write offset 0x80 as immediate value
    }
    
    cpu.ldh_a_n(n).unwrap();
    
    // Check if A was loaded correctly
    assert_eq!(cpu.a, 0x42);
}

#[test]
fn test_cp_n() {
    let mut cpu = CPU::new();

    // Test case 1: A == n (sets zero flag)
    cpu.a = 0x42;
    cpu.cp_n(0x42);
    assert_eq!(cpu.get_flag(ZERO_FLAG), true);
    assert_eq!(cpu.get_flag(SUBTRACT_FLAG), true);
    assert_eq!(cpu.get_flag(CARRY_FLAG), false);
    assert_eq!(cpu.get_flag(HALF_CARRY_FLAG), false);
    assert_eq!(cpu.a, 0x42);  // A should not be modified

    // Test case 2: A > n
    cpu.a = 0x42;
    cpu.cp_n(0x21);
    assert_eq!(cpu.get_flag(ZERO_FLAG), false);
    assert_eq!(cpu.get_flag(CARRY_FLAG), false);

    // Test case 3: A < n (sets carry flag)
    cpu.a = 0x21;
    cpu.cp_n(0x42);
    assert_eq!(cpu.get_flag(ZERO_FLAG), false);
    assert_eq!(cpu.get_flag(CARRY_FLAG), true);

    // Test case 4: Half carry check
    cpu.a = 0x10;
    cpu.cp_n(0x01);
    assert_eq!(cpu.get_flag(HALF_CARRY_FLAG), true);
}