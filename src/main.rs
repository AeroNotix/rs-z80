use std::fs;
mod instructions;

fn main() {
    let mut cpu = instructions::CPU::new(fs::read("src/roms/ld.rom").expect("Unable to read file"));
    for _ in 1..cpu.program.len() {
        cpu.decode(cpu.fetch()).execute(&mut cpu);
    }
}
