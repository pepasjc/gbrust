mod cpu;
mod mmu;

fn main() {
    println!("GBRust - Game Boy Emulator");
    let mut cpu = cpu::CPU::new();
    let mmu = mmu::MMU::new();
    
    cpu.set_mmu(mmu);
    cpu.initialize();
    
    // Test some instructions
    cpu.ld_b_n(0x42);
    println!("Loaded B with 0x42");
    cpu.inc_b();
    println!("Incremented B to 0x{:02X}", cpu.b);
    cpu.dec_b();
    println!("Decremented B to 0x{:02X}", cpu.b);
}