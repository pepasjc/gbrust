use std::env;
use std::io::{self, Write};
use std::fs;

mod cpu;
mod mmu;

fn debug_prompt() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn parse_hex_address(input: &str) -> Option<u16> {
    let cleaned = input.trim();
    if cleaned.len() < 1 {
        return None;
    }
    
    let without_prefix = if cleaned.to_lowercase().starts_with("0x") {
        &cleaned[2..]
    } else {
        cleaned
    };
    
    u16::from_str_radix(without_prefix, 16).ok()
}

fn save_crash_pc(pc: u16) -> io::Result<()> {
    fs::write("last_crash.txt", format!("{:04X}", pc - 1))
}

fn load_crash_pc() -> Option<u16> {
    if let Ok(content) = fs::read_to_string("last_crash.txt") {
        parse_hex_address(&content)
    } else {
        None
    }
}

fn main() {
    println!("GBRust - Game Boy Emulator");
    let mut cpu = cpu::CPU::new();
    let mut mmu = mmu::MMU::new();
    let mut last_crash_pc = load_crash_pc();  // Load from file at startup
    
    // Get ROM file from command line argument
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <rom_file>", args[0]);
        return;
    }

    // Load ROM
    match mmu.load_rom(&args[1]) {
        Ok(_) => println!("ROM loaded successfully"),
        Err(e) => {
            println!("Failed to load ROM: {}", e);
            return;
        }
    }

    cpu.set_mmu(mmu);
    cpu.initialize();
    cpu.debug_mode = true;

    println!("\nDebugger commands:");
    println!("  s - Step (execute one instruction)");
    println!("  c - Continue (run normally)");
    println!("  r - Run until PC reaches specified address");
    println!("  t - Run until last crash PC (loaded from file)");
    println!("  q - Quit");
    println!("  h - Show this help");

    let mut running = true;
    while running {
        match debug_prompt().as_str() {
            "s" => {
                match cpu.step() {
                    Ok(_) => (),
                    Err(e) => {
                        println!("CPU Error: {}", e);
                        last_crash_pc = Some(cpu.pc);
                        if let Err(e) = save_crash_pc(cpu.pc) {
                            println!("Failed to save crash PC: {}", e);
                        }
                        running = false;
                    }
                }
            },
            "c" => {
                cpu.debug_mode = false;
                for _ in 0..100 {
                    match cpu.step() {
                        Ok(_) => (),
                        Err(e) => {
                            println!("CPU Error: {}", e);
                            last_crash_pc = Some(cpu.pc);
                            if let Err(e) = save_crash_pc(cpu.pc) {
                                println!("Failed to save crash PC: {}", e);
                            }
                            running = false;
                            break;
                        }
                    }
                }
                cpu.debug_mode = true;
            },
            "r" => {
                print!("Enter target PC (hex, e.g. 0x0393): ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                
                match parse_hex_address(input.trim()) {
                    Some(target_pc) => {
                        println!("Running until PC = 0x{:04X}", target_pc);
                        let mut reached = false;
                        cpu.debug_mode = false;
                        
                        while !reached {
                            match cpu.step() {
                                Ok(_) => {
                                    if cpu.pc == target_pc {
                                        reached = true;
                                    }
                                },
                                Err(e) => {
                                    println!("CPU Error: {}", e);
                                    last_crash_pc = Some(cpu.pc);
                                    if let Err(e) = save_crash_pc(cpu.pc) {
                                        println!("Failed to save crash PC: {}", e);
                                    }
                                    break;
                                }
                            }
                        }
                        
                        cpu.debug_mode = true;
                        println!("Reached target PC = 0x{:04X}", cpu.pc);
                    },
                    None => println!("Invalid hexadecimal address"),
                }
            },
            "t" => {
                match last_crash_pc {
                    Some(target_pc) => {
                        println!("Running until last crash PC = 0x{:04X}", target_pc);
                        let mut reached = false;
                        cpu.debug_mode = false;
                        
                        while !reached {
                            match cpu.step() {
                                Ok(_) => {
                                    if cpu.pc == target_pc {
                                        reached = true;
                                    }
                                },
                                Err(e) => {
                                    println!("CPU Error: {}", e);
                                    last_crash_pc = Some(cpu.pc);
                                    if let Err(e) = save_crash_pc(cpu.pc) {
                                        println!("Failed to save crash PC: {}", e);
                                    }
                                    break;
                                }
                            }
                        }
                        
                        cpu.debug_mode = true;
                        if reached {
                            println!("Reached crash PC = 0x{:04X}", cpu.pc);
                        }
                    },
                    None => println!("No previous crash PC found in last_crash.txt"),
                }
            },
            "q" => running = false,
            "h" => {
                println!("Commands:");
                println!("  s - Step (execute one instruction)");
                println!("  c - Continue (run normally)");
                println!("  r - Run until PC reaches specified address");
                println!("  t - Run until last crash PC");
                println!("  q - Quit");
                println!("  h - Show this help");
            },
            "" => (),
            cmd => println!("Unknown command: {}", cmd),
        }
    }
}