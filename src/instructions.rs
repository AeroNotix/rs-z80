const REGISTER_TABLE_8_BIT: [Register8Bit; 8] = [
    Register8Bit::B,
    Register8Bit::C,
    Register8Bit::D,
    Register8Bit::E,
    Register8Bit::H,
    Register8Bit::L,
    Register8Bit::HL,
    Register8Bit::A,
];

const REGISTER_TABLE_SP: [Register16Bit; 4] = [
    Register16Bit::BC,
    Register16Bit::DE,
    Register16Bit::HL,
    Register16Bit::SP,
];

const REGISTER_TABLE_AF: [Register16Bit; 4] = [
    Register16Bit::BC,
    Register16Bit::DE,
    Register16Bit::HL,
    Register16Bit::AF,
];

#[derive(Debug)]
pub enum Condition {
    NotZero,
    Zero,
    NoCarry,
    Carry,
    ParityOverflow,

}

#[derive(Debug)]
pub enum Flag {
    Carry,
    AddSubtract,
    ParityOverflow,
    HalfCarry,
    Zero,
    Sign,
    X,
}

#[derive(Debug, Clone, Copy)]
pub enum Register8Bit {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    HL,
    I,
    R,
    IXH,
    IXL,
    IYH,
    IYL,
}

#[derive(Debug, Clone, Copy)]
pub enum Register16Bit {
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
    IX,
    IY,
}

#[derive(Debug)]
pub enum Operand {
    Address(u16),
    Immediate(u8),
    Indirect16Bit(Register16Bit),
    Indirect16BitWithOffset(Register16Bit, u8),
    Register8Bit(Register8Bit),
}

#[derive(Debug)]
pub enum Instruction {
    LD(Operand, Operand),
    Halt,
    Unknown,
}

#[derive(Debug)]
pub struct Opcode {
    x: u8,
    y: u8,
    z: u8,
    p: u8,
    q: u8,
}

impl Opcode {
    pub fn new(raw_opcode: u8) -> Opcode {
        Opcode {
            x: (raw_opcode >> 6) & 0x03,
            y: (raw_opcode >> 3) & 0x07,
            z: (raw_opcode >> 0) & 0x07,
            p: (raw_opcode >> 4) & 0x03,
            q: (raw_opcode >> 3) & 0x03,
        }
    }

    pub fn decode(self) -> Instruction {
        println!("{:?}", self);
        match self.x {
            1 if self.y == 6 => Instruction::Halt,
            1 => Instruction::LD(
                Operand::Register8Bit(REGISTER_TABLE_8_BIT[self.y as usize]),
                Operand::Register8Bit(REGISTER_TABLE_8_BIT[self.z as usize]),
            ),
            _ => Instruction::Unknown,
        }
    }
}

