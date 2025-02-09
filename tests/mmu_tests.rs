use gbrust::mmu::MMU;

#[test]
fn test_memory_regions() {
    let mut mmu = MMU::new();
    
    // Test ROM bank 0 (read-only)
    mmu.write_byte(0x0000, 0x42);
    assert_eq!(mmu.read_byte(0x0000), 0x00);  // Should not change
    
    // Test VRAM
    mmu.write_byte(0x8000, 0x42);
    assert_eq!(mmu.read_byte(0x8000), 0x42);
    
    // Test WRAM
    mmu.write_byte(0xC000, 0x42);
    assert_eq!(mmu.read_byte(0xC000), 0x42);
    
    // Test HRAM
    mmu.write_byte(0xFF80, 0x42);
    assert_eq!(mmu.read_byte(0xFF80), 0x42);
    
    // Test IE register
    mmu.write_byte(0xFFFF, 0x42);
    assert_eq!(mmu.read_byte(0xFFFF), 0x42);
}
