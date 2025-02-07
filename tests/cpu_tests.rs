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
