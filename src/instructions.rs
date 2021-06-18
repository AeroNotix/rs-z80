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

const CONDITION_TABLE: [Condition; 5] = [
    Condition::NotZero,
    Condition::Zero,
    Condition::NoCarry,
    Condition::Carry,
    Condition::ParityOverflow,
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
    Carry(bool),
    AddSubtract(bool),
    ParityOverflow(bool),
    HalfCarry(bool),
    Zero(bool),
    Sign(bool),
    X(bool),
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
    Immediate16(u16),
    Immediate(u8),
    Indirect16Bit(Register16Bit),
    Indirect16BitWithOffset(Register16Bit, u8),
    Register16Bit(Register16Bit),
    Register8Bit(Register8Bit),
}

#[derive(Debug)]
pub enum Instruction {
    Add(Operand, Operand),
    DJNZ,
    Call(Operand),
    ConditionalRet(Condition),
    Dec(Operand),
    Exchange(Operand, Operand),
    Halt,
    In(Operand, u8),
    Inc(Operand),
    LD(Operand, Operand),
    NOP,
    Out(u8, Operand),
    Pop(Operand),
    Push(Operand),
    UnconditionalRet,
    Unknown,
    EXX,
    RST,
    RES(u8, Operand),
    BIT(u8, Operand),
    SET(u8, Operand),
    JR(u8),
    ConditionalJR(Condition, u8),
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

    fn decode_x0(self) -> Instruction {
        match self.z {
            0 => match self.y {
                0 => Instruction::NOP,
                1 => Instruction::Exchange(
                    Operand::Register16Bit(Register16Bit::AF),
                    Operand::Register16Bit(Register16Bit::AF),
                ),
                2 => Instruction::DJNZ,
                // TODO: Some instructions are multi-byte.
                //
                // Opcode decoder should act on a stream and
                // conditionally read bytes from the stream when
                // needed.
                3 => Instruction::JR(99),
                4..=7 => Instruction::ConditionalJR(
                    Condition::Carry, 123,
                ),
                _ => Instruction::Unknown,
            },
            1 => match self.q {
                // TODO: Take which register from self.p / rp[p]
                0 => Instruction::LD(Operand::Register16Bit(Register16Bit::AF), Operand::Immediate16(0)),
                // TODO: Take which register from self.p / rp[p]
                1 => Instruction::Add(Operand::Register16Bit(Register16Bit::HL), Operand::Register16Bit(Register16Bit::AF)),
                _ => Instruction::Unknown,

            },
            // TODO: Implement this section when z=2
            2 => Instruction::Unknown,
            3 => match self.q {
                0 => Instruction::Inc(Operand::Register16Bit(REGISTER_TABLE_SP[self.p as usize])),
                1 => Instruction::Dec(Operand::Register16Bit(REGISTER_TABLE_SP[self.p as usize])),
                _ => Instruction::Unknown,
            }
            4 => Instruction::Inc(Operand::Register8Bit(REGISTER_TABLE_8_BIT[self.y as usize])),
            5 => Instruction::Dec(Operand::Register8Bit(REGISTER_TABLE_8_BIT[self.y as usize])),
            6 => Instruction::LD(Operand::Register8Bit(REGISTER_TABLE_8_BIT[self.y as usize]), Operand::Immediate(0)),
            // TODO: Implement this section when z=7
            7 => Instruction::Unknown,
            _ => Instruction::Unknown,
        }
    }

    fn decode_x1(self) -> Instruction {
        if self.y == 6 {
            return Instruction::Halt;
        }
        Instruction::LD(
            Operand::Register8Bit(REGISTER_TABLE_8_BIT[self.y as usize]),
            Operand::Register8Bit(REGISTER_TABLE_8_BIT[self.z as usize]),
        )
    }

    pub fn decode(self) -> Instruction {
        match self.x {
            0 => self.decode_x0(),
            1 => self.decode_x1(),
            _ => Instruction::Unknown,
        }
    }
}

