use std::fs;
mod instructions;

fn run_prog(path: &str) {
    let mut cpu = instructions::CPU::new(fs::read(path).expect("Unable to read file"));
    for _ in 1..cpu.program.len() {
        cpu.decode(cpu.fetch()).execute(&mut cpu);
    }
}

fn main() {
    run_prog("src/roms/ld.rom");
    run_prog("src/roms/inc_dec.rom");
}
