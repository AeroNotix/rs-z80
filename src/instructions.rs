use std::collections::HashMap;

const REGISTER_TABLE_8_BIT: [Register; 7] = [
    Register::A,
    Register::B,
    Register::C,
    Register::D,
    Register::E,
    Register::H,
    Register::L,
];

const REGISTER_TABLE_SP: [Register; 4] = [
    Register::BC,
    Register::DE,
    Register::HL,
    Register::SP,
];

const REGISTER_TABLE_AF: [Register; 4] = [
    Register::BC,
    Register::DE,
    Register::HL,
    Register::AF,
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

#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operand {
    Immediate16,
    Immediate8,
    IndirectImmediate,
    IndirectRegister(Register),
    IndirectWithOffset(Register, Box<Operand>),
    Register(Register),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    NOP,
    Halt,

    In(Operand, u8),
    Out(u8, Operand),

    Add(Operand, Operand),
    Inc(Operand),
    Dec(Operand),

    Call(Operand),
    ConditionalRet(Condition),

    DJNZ,
    Exchange(Operand, Operand),

    LD(Operand, Operand),

    Pop(Operand),
    Push(Operand),

    UnconditionalRet,
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

    Unknown,
}

pub struct CPU {
    registers: HashMap<Register, Box<u8>>,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: [
                (Register::A, Box::new(0)),
                (Register::B, Box::new(0)),
                (Register::C, Box::new(0)),
                (Register::D, Box::new(0)),
                (Register::E, Box::new(0)),
                (Register::H, Box::new(0)),
                (Register::L, Box::new(0)),
            ].iter().cloned().collect()
        }
    }

    pub fn ld(to: &mut u8, from: u8) {
        *to = from;
    }
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
    pub fn from_u8(raw_opcode: u8) -> Opcode {
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
        use Instruction::*;
        use Register::*;
        match self.z {
            0 => match self.y {
                0 => NOP,
                1 => Exchange(
                    Operand::Register(AF),
                    Operand::Register(AF),
                ),
                2 => DJNZ,
                3 => JR(Operand::Immediate8),
                4..=7 => ConditionalJR(
                    // -4 because that's just how it works.
                    CONDITION_TABLE[self.y as usize - 4], Operand::Immediate8,
                ),
                _ => Unknown,
            },
            1 => match self.q {
                0 => LD(Operand::Register(REGISTER_TABLE_SP[self.p as usize]), Operand::Immediate16),
                1 => Add(Operand::Register(HL), Operand::Register(REGISTER_TABLE_SP[self.p as usize])),
                _ => Unknown,

            },
            2 => match (self.q, self.p) {
                (0, 0) => LD(Operand::IndirectRegister(BC), Operand::Register(A)),
                (0, 1) => LD(Operand::IndirectRegister(DE), Operand::Register(A)),
                (0, 2) => LD(Operand::IndirectImmediate,    Operand::Register(HL)),
                (0, 3) => LD(Operand::IndirectImmediate,    Operand::Register(A)),
                (1, 0) => LD(Operand::Register(A),          Operand::IndirectRegister(BC)),
                (1, 1) => LD(Operand::Register(A),          Operand::IndirectRegister(DE)),
                (1, 2) => LD(Operand::Register(HL),         Operand::IndirectImmediate),
                (1, 3) => LD(Operand::Register(A),          Operand::IndirectImmediate),
                _ => Unknown,
            },
            3 => match self.q {
                0 => Inc(Operand::Register(REGISTER_TABLE_SP[self.p as usize])),
                1 => Dec(Operand::Register(REGISTER_TABLE_SP[self.p as usize])),
                _ => Unknown,
            }
            4 => Inc(Operand::Register(REGISTER_TABLE_8_BIT[self.y as usize])),
            5 => Dec(Operand::Register(REGISTER_TABLE_8_BIT[self.y as usize])),
            6 => LD(Operand::Register(REGISTER_TABLE_8_BIT[self.y as usize]), Operand::Immediate8),
            7 => match self.y {
                0 => RLCA,
                1 => RRCA,
                2 => RLA,
                3 => RRA,
                4 => DAA,
                5 => CPL,
                6 => SCF,
                7 => CCF,
                _ => Unknown,

            }
            _ => Unknown,
        }
    }

    fn decode_x1(self) -> Instruction {
        if self.y == 6 {
            return Instruction::Halt;
        }
        Instruction::LD(
            Operand::Register(REGISTER_TABLE_8_BIT[self.y as usize]),
            Operand::Register(REGISTER_TABLE_8_BIT[self.z as usize]),
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
            .map(|x| Opcode::from_u8(*x).decode())
            .collect();
        assert_eq!(&program[..], &expected_program[..])
    }

    #[test]
    fn ld() {
        let expected_program = vec![
            Instruction::LD(
                Operand::Register(Register::A),
                Operand::Register(Register::B),
            ),
            Instruction::LD(
                Operand::Register(Register::B),
                Operand::Register(Register::C),
            ),
            Instruction::LD(
                Operand::Register(Register::C),
                Operand::Register(Register::D),
            ),
            Instruction::LD(
                Operand::Register(Register::D),
                Operand::Register(Register::E),
            ),
        ];
        run_test("src/roms/ld.rom", expected_program);
    }

    #[test]
    fn inc_dec() {
        let expected_program = vec![
            Instruction::Inc(Operand::Register(Register::A)),
            Instruction::Inc(Operand::Register(Register::B)),
            Instruction::Dec(Operand::Register(Register::A)),
            Instruction::Dec(Operand::Register(Register::B)),
        ];
        run_test("src/roms/inc_dec.rom", expected_program);
    }

    #[test]
    fn x_table_all_valid() {
        for x in 0..0b00111111 {
            let raw_opcode = Opcode::from_u8(x);
            println!("0x{:x} - {:?}", x, raw_opcode.decode());
            if let Instruction::Unknown = Opcode::from_u8(x).decode() {
                panic!("Should not be an Uknown opcode in the z=0 table");
            }
        }
    }
}
