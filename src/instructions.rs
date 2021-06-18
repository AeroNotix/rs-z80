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

const CONDITION_TABLE: [Condition; 4] = [
    Condition::NotZero,
    Condition::Zero,
    Condition::NoCarry,
    Condition::Carry,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    NotZero,
    Zero,
    NoCarry,
    Carry,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Flag {
    Carry(bool),
    AddSubtract(bool),
    ParityOverflow(bool),
    HalfCarry(bool),
    Zero(bool),
    Sign(bool),
    X(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum Operand {
    Immediate16,
    Immediate8,
    IndirectImmediate,
    Indirect16Bit(Register16Bit),
    Indirect16BitWithOffset(Register16Bit, Box<Operand>),
    Register16Bit(Register16Bit),
    Register8Bit(Register8Bit),
}

#[derive(Debug, PartialEq, Eq)]
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
    JR(Operand),
    ConditionalJR(Condition, Operand),

    RLCA,
    RRCA,
    RLA,
    RRA,
    DAA,
    CPL,
    SCF,
    CCF,
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
        // From: http://www.z80.info/decoding.htm

        // Essentially:

        // 7   6   5   4   3   2   1   0
        // |-x-|   |---y---|   |---z---|
        //         |-p-|   q
        Opcode {
            x: (raw_opcode >> 6) & 0x03,
            y: (raw_opcode >> 3) & 0x07,
            z: (raw_opcode >> 0) & 0x07,
            p: (raw_opcode >> 4) & 0x03,
            q: (raw_opcode >> 3) & 0x01,
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
                3 => Instruction::JR(Operand::Immediate8),
                4..=7 => Instruction::ConditionalJR(
                    CONDITION_TABLE[self.y as usize - 4], Operand::Immediate8,
                ),
                _ => Instruction::Unknown,
            },
            1 => match self.q {
                0 => Instruction::LD(Operand::Register16Bit(REGISTER_TABLE_SP[self.p as usize]), Operand::Immediate16),
                1 => Instruction::Add(Operand::Register16Bit(Register16Bit::HL), Operand::Register16Bit(REGISTER_TABLE_SP[self.p as usize])),
                _ => Instruction::Unknown,

            },
            2 => match (self.q, self.p) {
                (0, 0) => Instruction::LD(Operand::Indirect16Bit(Register16Bit::BC), Operand::Register8Bit(Register8Bit::A)),
                (0, 1) => Instruction::LD(Operand::Indirect16Bit(Register16Bit::DE), Operand::Register8Bit(Register8Bit::A)),
                (0, 2) => Instruction::LD(Operand::IndirectImmediate, Operand::Register16Bit(Register16Bit::HL)),
                (0, 3) => Instruction::LD(Operand::IndirectImmediate, Operand::Register8Bit(Register8Bit::A)),
                (1, 0) => Instruction::LD(Operand::Register8Bit(Register8Bit::A), Operand::Indirect16Bit(Register16Bit::BC)),
                (1, 1) => Instruction::LD(Operand::Register8Bit(Register8Bit::A), Operand::Indirect16Bit(Register16Bit::DE)),
                (1, 2) => Instruction::LD(Operand::Register16Bit(Register16Bit::HL), Operand::IndirectImmediate),
                (1, 3) => Instruction::LD(Operand::Register8Bit(Register8Bit::A), Operand::IndirectImmediate),
                _ => Instruction::Unknown,
            },
            3 => match self.q {
                0 => Instruction::Inc(Operand::Register16Bit(REGISTER_TABLE_SP[self.p as usize])),
                1 => Instruction::Dec(Operand::Register16Bit(REGISTER_TABLE_SP[self.p as usize])),
                _ => Instruction::Unknown,
            }
            4 => Instruction::Inc(Operand::Register8Bit(REGISTER_TABLE_8_BIT[self.y as usize])),
            5 => Instruction::Dec(Operand::Register8Bit(REGISTER_TABLE_8_BIT[self.y as usize])),
            6 => Instruction::LD(Operand::Register8Bit(REGISTER_TABLE_8_BIT[self.y as usize]), Operand::Immediate8),
            7 => match self.y {
                0 => Instruction::RLCA,
                1 => Instruction::RRCA,
                2 => Instruction::RLA,
                3 => Instruction::RRA,
                4 => Instruction::DAA,
                5 => Instruction::CPL,
                6 => Instruction::SCF,
                7 => Instruction::CCF,
                _ => Instruction::Unknown,

            }
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn run_test(rom: &str, expected_program: Vec<Instruction>) {
        let program: Vec<Instruction> = fs::read(rom).expect("Unable to read file")
            .iter()
            .map(|x| Opcode::new(*x).decode())
            .collect();
        assert_eq!(&program[..], &expected_program[..])
    }

    #[test]
    fn ld() {
        let expected_program = vec![
            Instruction::LD(
                Operand::Register8Bit(Register8Bit::A),
                Operand::Register8Bit(Register8Bit::B),
            ),
            Instruction::LD(
                Operand::Register8Bit(Register8Bit::B),
                Operand::Register8Bit(Register8Bit::C),
            ),
            Instruction::LD(
                Operand::Register8Bit(Register8Bit::C),
                Operand::Register8Bit(Register8Bit::D),
            ),
            Instruction::LD(
                Operand::Register8Bit(Register8Bit::D),
                Operand::Register8Bit(Register8Bit::E),
            ),
        ];
        run_test("src/roms/ld.rom", expected_program);
    }

    #[test]
    fn inc_dec() {
        let expected_program = vec![
            Instruction::Inc(Operand::Register8Bit(Register8Bit::A)),
            Instruction::Inc(Operand::Register8Bit(Register8Bit::B)),
            Instruction::Dec(Operand::Register8Bit(Register8Bit::A)),
            Instruction::Dec(Operand::Register8Bit(Register8Bit::B)),
        ];
        run_test("src/roms/inc_dec.rom", expected_program);
    }

    #[test]
    fn decode_x_table_no_panic() {
        for x in 0..0b00111111 {
            let raw_opcode = Opcode::new(x);
            println!("0x{:x} - {:?}", x, raw_opcode.decode());
            if let Instruction::Unknown = Opcode::new(x).decode() {
                panic!("Should not be an Uknown opcode in the z=0 table");
            }
        }
    }
}
