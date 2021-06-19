use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

const REGISTER_TABLE_8_BIT: [Register; 8] = [
    Register::B,
    Register::C,
    Register::D,
    Register::E,
    Register::H,
    Register::L,
    Register::HL,
    Register::A,
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
    SP,
    PC,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operand {
    Immediate16,
    Immediate8,
    IndirectImmediate,
    IndirectRegister(Rc<RefCell<u8>>),
    IndirectWithOffset(Rc<RefCell<u8>>, Rc<RefCell<u8>>),
    CPURegister(Rc<RefCell<u8>>),
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

impl Instruction {
    pub fn execute(&self, cpu: &mut CPU) {
        use Instruction::*;
        use Operand::*;
        match self {
            LD(CPURegister(lhs), CPURegister(rhs)) => {
                cpu.pc += 1;
                *lhs.borrow_mut() = *rhs.borrow();
            },
            LD(CPURegister(lhs), Immediate8) => {
                cpu.pc += 1;
                *lhs.borrow_mut() = cpu.fetch_u8();
            }
            el => {
                println!("{:?}", el);
                panic!("Not implemented")
            }
        }
    }
}

#[derive(Debug)]
pub struct CPU {
    pub program: Vec<u8>,
    pc: u16,
    registers: HashMap<Register, Rc<RefCell<u8>>>,
}

impl CPU {
    pub fn new(program: Vec<u8>) -> CPU {
        CPU {
            program,
            pc: 0,
            registers: [
                (Register::A, Rc::new(RefCell::new(0))),
                (Register::B, Rc::new(RefCell::new(0))),
                (Register::C, Rc::new(RefCell::new(0))),
                (Register::D, Rc::new(RefCell::new(0))),
                (Register::E, Rc::new(RefCell::new(0))),
                (Register::H, Rc::new(RefCell::new(0))),
                (Register::L, Rc::new(RefCell::new(0))),
                (Register::AF, Rc::new(RefCell::new(0))),
                (Register::BC, Rc::new(RefCell::new(0))),
                (Register::DE, Rc::new(RefCell::new(0))),
                (Register::HL, Rc::new(RefCell::new(0))),
                (Register::SP, Rc::new(RefCell::new(0))),
            ].iter().cloned().collect()
        }
    }

    pub fn fetch_u8(&mut self) -> u8 {
        let n = self.program[self.pc as usize];
        self.pc += 1;
        n
    }

    pub fn fetch(&self) -> u8 {
        self.program[self.pc as usize]
    }

    pub fn decode(&self, opcode: u8) -> Instruction {
        Opcode::from_u8(opcode).decode(&self)
    }

    fn get_register(&self, reg: Register) -> Rc<RefCell<u8>> {
        self.registers.get(&reg)
            .expect("Requested a register, which wasn't in the CPU. Shouldn't get here")
            .clone()
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

    fn decode_x0(self, cpu: &CPU) -> Instruction {
        use Instruction::*;
        use Operand::*;
        use Register::*;
        match self.z {
            0 => match self.y {
                0 => NOP,
                1 => Exchange(
                    CPURegister(cpu.get_register(AF)),
                    CPURegister(cpu.get_register(AF)),
                ),
                2 => DJNZ,
                3 => JR(Immediate8),
                4..=7 => ConditionalJR(
                    // -4 because that's just how it works.
                    CONDITION_TABLE[self.y as usize - 4], Immediate8,
                ),
                _ => Unknown,
            },
            1 => match self.q {
                0 => LD(CPURegister(cpu.get_register(REGISTER_TABLE_SP[self.p as usize])), Immediate16),
                1 => Add(CPURegister(cpu.get_register(HL)), CPURegister(cpu.get_register(REGISTER_TABLE_SP[self.p as usize]))),
                _ => Unknown,

            },
            2 => match (self.q, self.p) {
                (0, 0) => LD(IndirectRegister(cpu.get_register(BC)), CPURegister(cpu.get_register(A))),
                (0, 1) => LD(IndirectRegister(cpu.get_register(DE)), CPURegister(cpu.get_register(A))),
                (0, 2) => LD(IndirectImmediate, CPURegister(cpu.get_register(HL))),
                (0, 3) => LD(IndirectImmediate, CPURegister(cpu.get_register(A))),
                (1, 0) => LD(CPURegister(cpu.get_register(A)), IndirectRegister(cpu.get_register(BC))),
                (1, 1) => LD(CPURegister(cpu.get_register(A)), IndirectRegister(cpu.get_register(DE))),
                (1, 2) => LD(CPURegister(cpu.get_register(HL)), IndirectImmediate),
                (1, 3) => LD(CPURegister(cpu.get_register(A)), IndirectImmediate),
                _ => Unknown,
            },
            3 => match self.q {
                0 => Inc(CPURegister(cpu.get_register(REGISTER_TABLE_SP[self.p as usize]))),
                1 => Dec(CPURegister(cpu.get_register(REGISTER_TABLE_SP[self.p as usize]))),
                _ => Unknown,
            }
            4 => Inc(CPURegister(cpu.get_register(REGISTER_TABLE_8_BIT[self.y as usize]))),
            5 => Dec(CPURegister(cpu.get_register(REGISTER_TABLE_8_BIT[self.y as usize]))),
            6 => LD(CPURegister(cpu.get_register(REGISTER_TABLE_8_BIT[self.y as usize])), Immediate8),
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

    fn decode_x1(self, cpu: &CPU) -> Instruction {
        if self.y == 6 {
            return Instruction::Halt;
        }
        Instruction::LD(
            Operand::CPURegister(cpu.get_register(REGISTER_TABLE_8_BIT[self.y as usize])),
            Operand::CPURegister(cpu.get_register(REGISTER_TABLE_8_BIT[self.z as usize])),
        )
    }

    pub fn decode(self, cpu: &CPU) -> Instruction {
        match self.x {
            0 => self.decode_x0(cpu),
            1 => self.decode_x1(cpu),
            _ => Instruction::Unknown,
        }
    }
}


#[cfg(test)]
mod cpu_tests {
    use super::*;

    #[test]
    fn get_register_for_cpu() {
        let cpu = CPU::new();
        let reg0 = cpu.get_register(Register::A);
        *reg0.borrow_mut() = 123;
        let reg1 = cpu.get_register(Register::A);
        assert_eq!(123, *reg1.borrow());
    }
}


#[cfg(test)]
mod opcode_tests {
    use super::*;
    use std::fs;
    use Register::*;
    use Instruction::*;
    use Operand::*;

    fn run_test(cpu: &CPU, rom: &str, expected_program: &Vec<Instruction>) {
        let program: Vec<Instruction> = fs::read(rom).expect("Unable to read file")
            .iter()
            .map(|x| Opcode::from_u8(*x).decode(cpu))
            .collect();
        assert_eq!(&program[..], &expected_program[..])
    }

    #[test]
    fn ld() {
        let cpu = CPU::new();
        let expected_program = vec![
            LD(CPURegister(cpu.get_register(A)), Immediate8),
            LD(CPURegister(cpu.get_register(A)), CPURegister(cpu.get_register(B))),
            LD(CPURegister(cpu.get_register(B)), CPURegister(cpu.get_register(C))),
            LD(CPURegister(cpu.get_register(C)), CPURegister(cpu.get_register(D))),
            LD(CPURegister(cpu.get_register(D)), CPURegister(cpu.get_register(E))),
        ];
        run_test(&cpu, "src/roms/ld.rom", &expected_program);

        for instruction in expected_program {
            instruction.execute();
        }
    }

    #[test]
    fn inc_dec() {
        let cpu = CPU::new();
        let expected_program = vec![
            Inc(CPURegister(cpu.get_register(A))),
            Inc(CPURegister(cpu.get_register(B))),
            Dec(CPURegister(cpu.get_register(A))),
            Dec(CPURegister(cpu.get_register(B))),
        ];
        run_test(&cpu, "src/roms/inc_dec.rom", &expected_program);
    }

    #[test]
    fn x_table_all_valid() {
        let cpu = CPU::new();
        for x in 0..0b00111111 {
            let raw_opcode = Opcode::from_u8(x);
            println!("0x{:x} - {:?}", x, raw_opcode.decode(&cpu));
            if let Unknown = Opcode::from_u8(x).decode(&cpu) {
                panic!("Should not be an Uknown opcode in the z=0 table");
            }
        }
    }
}
