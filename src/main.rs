mod instructions;

fn main() {
    for raw_opcode in [0b1111000, 0b1000001, 0b1001010, 0b1010011] {
        let opcode = instructions::Opcode::new(raw_opcode).decode();
        if let instructions::Instruction::Unknown =  opcode {
            println!("{:?}", instructions::Opcode::new(raw_opcode));
        }
        println!("{:?}", opcode);
    }
}
