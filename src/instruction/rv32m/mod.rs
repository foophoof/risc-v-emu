// Copyright 2016 risc-v-emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use instruction::{encoding, Instruction};
use cpu::CPU;

#[derive(Debug)]
pub struct Op {
    typ: OperationType,
    dest: u8,
    operand1: u8, // multiplicand/dividend
    operand2: u8, // multiplier/divisor
}

#[derive(Debug)]
pub enum OperationType {
    Mul,
    MulHighSigned,
    MulHighUnsigned,
    MulHighSignedUnsigned,
    Div,
    DivUnsigned,
    Remainder,
    RemainderUnsigned,
}

impl Op {
    pub fn parse(instruction: u32) -> Option<Op> {
        let decoded = encoding::R::parse(instruction);

        if decoded.opcode != 0x33 {
            // Not a OP opcode
            return None;
        }

        if decoded.funct7 != 1 {
            return None;
        }

        let typ = match decoded.funct3 {
            0b000 => OperationType::Mul,
            0b001 => OperationType::MulHighSigned,
            0b010 => OperationType::MulHighSignedUnsigned,
            0b011 => OperationType::MulHighUnsigned,
            0b100 => OperationType::Div,
            0b101 => OperationType::DivUnsigned,
            0b110 => OperationType::Remainder,
            0b111 => OperationType::RemainderUnsigned,
            _ => unreachable!(),
        };

        Some(Op {
            typ: typ,
            dest: decoded.rd,
            operand1: decoded.rs1,
            operand2: decoded.rs2,
        })
    }
}

impl Instruction for Op {
    fn execute(&self, cpu: &mut CPU) {
        let operand1 = cpu.get_register(self.operand1);
        let operand2 = cpu.get_register(self.operand2);

        let result = match self.typ {
            OperationType::Mul => operand1.wrapping_mul(operand2),
            OperationType::MulHighSigned => {
                (((operand1 as i32 as i64) * (operand2 as i32 as i64)) >> 32) as u32
            }
            OperationType::MulHighUnsigned => {
                (((operand1 as u64) * (operand2 as u64)) >> 32) as u32
            }
            OperationType::MulHighSignedUnsigned => {
                (((operand1 as i32 as i64) * (operand2 as i64)) >> 32) as u32
            }
            OperationType::Div => {
                if operand2 == 0 {
                    (-1i32) as u32
                } else {
                    (operand1 as i32).wrapping_div(operand2 as i32) as u32
                }
            }
            OperationType::DivUnsigned => {
                if operand2 == 0 {
                    (-1i32) as u32
                } else {
                    operand1 / operand2
                }
            }
            OperationType::Remainder => {
                if operand2 == 0 {
                    operand1
                } else {
                    (operand1 as i32).wrapping_rem(operand2 as i32) as u32
                }
            }
            OperationType::RemainderUnsigned => {
                if operand2 == 0 {
                    operand1
                } else {
                    operand1 % operand2
                }
            }
        };

        cpu.set_register(self.dest, result);
    }
}

#[cfg(test)]
mod tests {
    use std::i32;
    use super::*;
    use ram::RAM;
    use cpu::CPU;
    use instruction::Instruction;

    #[test]
    fn test_mul() {
        let mut cpu = CPU::new(RAM::new(1024));

        let instr = Op::parse(0b0000001_00011_00010_000_00001_0110011)
            .expect("couldn't parse MUL x1, x2, x3");

        macro_rules! test_mul {
            ($result:expr, $val1:expr, $val2:expr) => {
                cpu.set_register(2, $val1);
                cpu.set_register(3, $val2);
                instr.execute(&mut cpu);
                assert_eq!(cpu.get_register(1), $result);
            }
        }

        test_mul!(0x00001200, 0x00007e00, 0xb6db6db7);
        test_mul!(0x00001240, 0x00007fc0, 0xb6db6db7);

        test_mul!(0x00000000, 0x00000000, 0x00000000);
        test_mul!(0x00000001, 0x00000001, 0x00000001);
        test_mul!(0x00000015, 0x00000003, 0x00000007);

        test_mul!(0x00000000, 0x00000000, 0xffff8000);
        test_mul!(0x00000000, 0x80000000, 0x00000000);
        test_mul!(0x00000000, 0x80000000, 0xffff8000);

        test_mul!(0x0000ff7f, 0xaaaaaaab, 0x0002fe7d);
        test_mul!(0x0000ff7f, 0x0002fe7d, 0xaaaaaaab);

        test_mul!(0x00000000, 0xff000000, 0xff000000);

        test_mul!(0x00000001, 0xffffffff, 0xffffffff);
        test_mul!(0xffffffff, 0xffffffff, 0x00000001);
        test_mul!(0xffffffff, 0x00000001, 0xffffffff);
    }

    #[test]
    fn test_mulh() {
        let mut cpu = CPU::new(RAM::new(1024));

        let instr = Op::parse(0b0000001_00011_00010_001_00001_0110011)
            .expect("couldn't parse MULH x1, x2, x3");

        macro_rules! test_mulh {
            ($result:expr, $val1:expr, $val2:expr) => {
                cpu.set_register(2, $val1);
                cpu.set_register(3, $val2);
                instr.execute(&mut cpu);
                assert_eq!(cpu.get_register(1), $result);
            }
        }

        test_mulh!(0x00000000, 0x00000000, 0x00000000);
        test_mulh!(0x00000000, 0x00000001, 0x00000001);
        test_mulh!(0x00000000, 0x00000003, 0x00000007);

        test_mulh!(0x00000000, 0x00000000, 0xffff8000);
        test_mulh!(0x00000000, 0x80000000, 0x00000000);
        test_mulh!(0x00000000, 0x80000000, 0x00000000);

        test_mulh!(0xffff0081, 0xaaaaaaab, 0x0002fe7d);
        test_mulh!(0xffff0081, 0x0002fe7d, 0xaaaaaaab);

        test_mulh!(0x00010000, 0xff000000, 0xff000000);

        test_mulh!(0x00000000, 0xffffffff, 0xffffffff);
        test_mulh!(0xffffffff, 0xffffffff, 0x00000001);
        test_mulh!(0xffffffff, 0x00000001, 0xffffffff);
    }

    #[test]
    fn test_mulhsu() {
        let mut cpu = CPU::new(RAM::new(1024));

        let instr = Op::parse(0b0000001_00011_00010_010_00001_0110011)
            .expect("couldn't parse MULHSU x1, x2, x3");

        macro_rules! test_mulhsu {
            ($result:expr, $val1:expr, $val2:expr) => {
                cpu.set_register(2, $val1);
                cpu.set_register(3, $val2);
                instr.execute(&mut cpu);
                assert_eq!(cpu.get_register(1), $result);
            }
        }

        test_mulhsu!(0x00000000, 0x00000000, 0x00000000);
        test_mulhsu!(0x00000000, 0x00000001, 0x00000001);
        test_mulhsu!(0x00000000, 0x00000003, 0x00000007);

        test_mulhsu!(0x00000000, 0x00000000, 0xffff8000);
        test_mulhsu!(0x00000000, 0x80000000, 0x00000000);
        test_mulhsu!(0x80004000, 0x80000000, 0xffff8000);

        test_mulhsu!(0xffff0081, 0xaaaaaaab, 0x0002fe7d);
        test_mulhsu!(0x0001fefe, 0x0002fe7d, 0xaaaaaaab);

        test_mulhsu!(0xff010000, 0xff000000, 0xff000000);

        test_mulhsu!(0xffffffff, 0xffffffff, 0xffffffff);
        test_mulhsu!(0xffffffff, 0xffffffff, 0x00000001);
        test_mulhsu!(0x00000000, 0x00000001, 0xffffffff);
    }

    #[test]
    fn test_mulhu() {
        let mut cpu = CPU::new(RAM::new(1024));

        let instr = Op::parse(0b0000001_00011_00010_011_00001_0110011)
            .expect("couldn't parse MULHU x1, x2, x3");

        macro_rules! test_mulhu {
            ($result:expr, $val1:expr, $val2:expr) => {
                cpu.set_register(2, $val1);
                cpu.set_register(3, $val2);
                instr.execute(&mut cpu);
                assert_eq!(cpu.get_register(1), $result);
            }
        }

        test_mulhu!(0x00000000, 0x00000000, 0x00000000);
        test_mulhu!(0x00000000, 0x00000001, 0x00000001);
        test_mulhu!(0x00000000, 0x00000003, 0x00000007);

        test_mulhu!(0x00000000, 0x00000000, 0xffff8000);
        test_mulhu!(0x00000000, 0x80000000, 0x00000000);
        test_mulhu!(0x7fffc000, 0x80000000, 0xffff8000);

        test_mulhu!(0x0001fefe, 0xaaaaaaab, 0x0002fe7d);
        test_mulhu!(0x0001fefe, 0x0002fe7d, 0xaaaaaaab);

        test_mulhu!(0xfe010000, 0xff000000, 0xff000000);

        test_mulhu!(0xfffffffe, 0xffffffff, 0xffffffff);
        test_mulhu!(0x00000000, 0xffffffff, 0x00000001);
        test_mulhu!(0x00000000, 0x00000001, 0xffffffff);
    }

    #[test]
    fn test_div() {
        let mut cpu = CPU::new(RAM::new(1024));

        let instr = Op::parse(0b0000001_00011_00010_100_00001_0110011)
            .expect("couldn't parse DIV x1, x2, x3");

        macro_rules! test_div {
            ($result:expr, $val1:expr, $val2:expr) => {
                cpu.set_register(2, $val1 as u32);
                cpu.set_register(3, $val2 as u32);
                instr.execute(&mut cpu);
                assert_eq!(cpu.get_register(1), $result as u32);
            }
        }

        test_div!(3, 20, 6);
        test_div!(-3i32, -20i32, 6);
        test_div!(-3i32, 20, -6i32);
        test_div!(3, -20i32, -6i32);

        test_div!(i32::MIN, i32::MIN, 1);
        test_div!(i32::MIN, i32::MIN, -1i32);

        test_div!(-1i32, i32::MIN, 0);
        test_div!(-1i32, 1, 0);
        test_div!(-1i32, 0, 0);
    }

    #[test]
    fn test_divu() {
        let mut cpu = CPU::new(RAM::new(1024));

        let instr = Op::parse(0b0000001_00011_00010_101_00001_0110011)
            .expect("couldn't parse DIVU x1, x2, x3");

        macro_rules! test_divu {
            ($result:expr, $val1:expr, $val2:expr) => {
                cpu.set_register(2, $val1 as u32);
                cpu.set_register(3, $val2 as u32);
                instr.execute(&mut cpu);
                assert_eq!(cpu.get_register(1), $result as u32);
            }
        }

        test_divu!(3, 20, 6);
        test_divu!(715827879, -20i32, 6);
        test_divu!(0, 20, -6i32);
        test_divu!(0, -20i32, -6i32);

        test_divu!(i32::MIN, i32::MIN, 1);
        test_divu!(0, i32::MIN, -1i32);

        test_divu!(-1i32, i32::MIN, 0);
        test_divu!(-1i32, 1, 0);
        test_divu!(-1i32, 0, 0);
    }

    #[test]
    fn test_rem() {
        let mut cpu = CPU::new(RAM::new(1024));

        let instr = Op::parse(0b0000001_00011_00010_110_00001_0110011)
            .expect("couldn't parse REM x1, x2, x3");

        macro_rules! test_rem {
            ($result:expr, $val1:expr, $val2:expr) => {
                cpu.set_register(2, $val1 as u32);
                cpu.set_register(3, $val2 as u32);
                instr.execute(&mut cpu);
                assert_eq!(cpu.get_register(1), $result as u32);
            }
        }

        test_rem!(2, 20, 6);
        test_rem!(-2i32, -20i32, 6);
        test_rem!(2, 20, -6i32);
        test_rem!(-2i32, -20i32, -6i32);

        test_rem!(0, i32::MIN, 1);
        test_rem!(0, i32::MIN, -1i32);

        test_rem!(i32::MIN, i32::MIN, 0);
        test_rem!(1, 1, 0);
        test_rem!(0, 0, 0);
    }

    #[test]
    fn test_remu() {
        let mut cpu = CPU::new(RAM::new(1024));

        let instr = Op::parse(0b0000001_00011_00010_111_00001_0110011)
            .expect("couldn't parse REMU x1, x2, x3");

        macro_rules! test_remu {
            ($result:expr, $val1:expr, $val2:expr) => {
                cpu.set_register(2, $val1 as u32);
                cpu.set_register(3, $val2 as u32);
                instr.execute(&mut cpu);
                assert_eq!(cpu.get_register(1), $result as u32);
            }
        }

        test_remu!(2, 20, 6);
        test_remu!(2, -20i32, 6);
        test_remu!(20, 20, -6i32);
        test_remu!(-20i32, -20i32, -6i32);

        test_remu!(0, i32::MIN, 1);
        test_remu!(i32::MIN, i32::MIN, -1i32);

        test_remu!(i32::MIN, i32::MIN, 0);
        test_remu!(1, 1, 0);
        test_remu!(0, 0, 0);
    }
}
