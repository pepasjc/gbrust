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
    // Test incrementing register B from 0x41 to 0x42
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
}

#[test]
fn test_inc_b_zero() {
    // Test incrementing register B from 0xFF to 0x00 (overflow case)
    // Expected:
    // - B should wrap to 0x00
    // - Zero flag should be true (result is zero)
    let mut cpu = CPU::new();
    cpu.b = 0xFF;
    cpu.inc_b();
    assert_eq!(cpu.b, 0x00);
    assert_eq!(cpu.get_flag(ZERO_FLAG), true);
}

#[test]
fn test_dec_b() {
    // Test decrementing register B from 0x42 to 0x41
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
    // Test jumping to a relative address if Z flag is reset
    // Expected: PC should be set to 0x1234 after execution
    let mut cpu = CPU::new();
    cpu.set_flag(ZERO_FLAG, false);
    cpu.pc = 0x1230;
    cpu.jr_nz_n(4);
    assert_eq!(cpu.pc, 0x1234);
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
    // Expected:
    // - Memory at initial HL should contain value of A
    // - HL should be decremented after operation
    let mut cpu = CPU::new();
    let mut mmu = gbrust::mmu::MMU::new();
    
    cpu.set_mmu(mmu);
    cpu.a = 0x42;
    cpu.h = 0x20;
    cpu.l = 0x00;  // HL = 0x2000
    
    cpu.ld_hl_dec_a().unwrap();
    
    // Check if value was written to memory
    if let Some(ref mmu) = cpu.mmu {
        assert_eq!(mmu.read_byte(0x2000), 0x42);
    }
    
    // Check if HL was decremented
    assert_eq!(cpu.h, 0x1F);
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