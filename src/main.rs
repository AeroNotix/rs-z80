mod instructions;

fn main() {
    println!("{:?}", instructions::Opcode::new(0b0111_0000).decode());
    println!("{:?}", instructions::Opcode::new(0b0110_1111).decode());
}
