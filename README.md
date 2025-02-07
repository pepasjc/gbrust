# GBRust - Game Boy Emulator in Rust

This repository is a Game Boy emulator implementation in Rust, used for learning Rust and AI-assisted coding.

## Documentation References

- [Pan Docs - CPU Instruction Set](https://gbdev.io/pandocs/CPU_Instruction_Set.html)
- [Game Boy CPU Technical Reference](https://gekkio.fi/files/gb-docs/gbctr.pdf)

## Building

```bash
cargo build
```

## Running Tests

```bash
cargo test
```

Or use the provided batch file:
```bash
test.bat
```

## Running the Emulator

1. Using batch file (recommended):
```bash
debug.bat <path-to-rom>
```
Example:
```bash
debug.bat rom.gb
```

2. Using cargo directly:
```bash
cargo run -- <path-to-rom>
```

## Debugger Commands

Once the emulator is running, you can use these commands:
- `s` - Step (execute one instruction)
- `c` - Continue (run 100 instructions)
- `r` - Run until specific PC (enter address in hex, e.g., 0x0150)
- `q` - Quit the emulator
- `h` - Show help message

## Project Structure

- `src/`
  - `main.rs` - Emulator entry point and debugger interface
  - `cpu.rs` - CPU implementation
  - `mmu.rs` - Memory Management Unit
- `tests/`
  - `cpu_tests.rs` - CPU instruction tests
