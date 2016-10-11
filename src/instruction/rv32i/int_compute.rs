// Copyright 2016 risc-v-emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use instruction::{encoding, Instruction};
use cpu::CPU;

#[derive(Debug)]
pub struct OpImm {
    typ: ImmediateOperationType,
    dest: u8,
    src: u8,
    immediate: i32,
}

#[derive(Debug, PartialEq)]
pub enum ImmediateOperationType {
    Add,
    SetLessThan,
    SetLessThanUnsigned,
    And,
    Or,
    Xor,
    ShiftLeftLogical,
    ShiftRightLogical,
    ShiftRightArithmetic,
}

impl OpImm {
    pub fn parse(instruction: u32) -> Option<OpImm> {
        let decoded = encoding::I::parse(instruction);

        if decoded.opcode != 0x13 {
            // Not a OP-IMM opcode
            return None;
        }

        let typ = match decoded.funct3 {
            0b000 => ImmediateOperationType::Add,
            0b001 => ImmediateOperationType::ShiftLeftLogical,
            0b010 => ImmediateOperationType::SetLessThan,
            0b011 => ImmediateOperationType::SetLessThanUnsigned,
            0b100 => ImmediateOperationType::Xor,
            0b101 => {
                if decoded.immediate & (1 << 10) == 0 {
                    ImmediateOperationType::ShiftRightLogical
                } else {
                    ImmediateOperationType::ShiftRightArithmetic
                }
            }
            0b110 => ImmediateOperationType::Or,
            0b111 => ImmediateOperationType::And,
            _ => return None,
        };

        Some(OpImm {
            typ: typ,
            dest: decoded.rd,
            src: decoded.rs1,
            immediate: decoded.immediate,
        })
    }
}

impl Instruction for OpImm {
    fn execute(&self, cpu: &mut CPU) {
        if self.typ == ImmediateOperationType::Add && self.src == 0 && self.dest == 0 &&
           self.immediate == 0 {
            // NOP
            return;
        }

        let src = cpu.get_register(self.src);

        cpu.set_register(self.dest,
                         match self.typ {
                             ImmediateOperationType::Add => src.wrapping_add(self.immediate as u32),
                             ImmediateOperationType::SetLessThan => {
                                 if (src as i32) < self.immediate { 1 } else { 0 }
                             }
                             ImmediateOperationType::SetLessThanUnsigned => {
                                 if src < (self.immediate as u32) { 1 } else { 0 }
                             }
                             ImmediateOperationType::And => src & (self.immediate as u32),
                             ImmediateOperationType::Or => src | (self.immediate as u32),
                             ImmediateOperationType::Xor => src ^ (self.immediate as u32),
                             ImmediateOperationType::ShiftLeftLogical => {
                                 src << (self.immediate & 0x1F)
                             }
                             ImmediateOperationType::ShiftRightLogical => {
                                 src >> (self.immediate & 0x1F)
                             }
                             ImmediateOperationType::ShiftRightArithmetic => {
                                 ((src as i32) >> (self.immediate & 0x1F)) as u32
                             }
                         });
    }
}

#[derive(Debug)]
pub struct Op {
    typ: OperationType,
    dest: u8,
    src1: u8,
    src2: u8,
}

#[derive(Debug)]
pub enum OperationType {
    Add,
    Sub,
    SetLessThan,
    SetLessThanUnsigned,
    And,
    Or,
    Xor,
    ShiftLeftLogical,
    ShiftRightLogical,
    ShiftRightArithmetic,
}

impl Op {
    pub fn parse(instruction: u32) -> Option<Op> {
        let decoded = encoding::R::parse(instruction);

        if decoded.opcode != 0x33 {
            // Not a OP-IMM opcode
            return None;
        }

        let typ = match (decoded.funct7, decoded.funct3) {
            (0, 0b000) => OperationType::Add,
            (0x20, 0b000) => OperationType::Sub,
            (0, 0b001) => OperationType::ShiftLeftLogical,
            (0, 0b010) => OperationType::SetLessThan,
            (0, 0b011) => OperationType::SetLessThanUnsigned,
            (0, 0b100) => OperationType::Xor,
            (0, 0b101) => OperationType::ShiftRightLogical,
            (0x20, 0b101) => OperationType::ShiftRightArithmetic,
            (0, 0b110) => OperationType::Or,
            (0, 0b111) => OperationType::And,
            _ => return None,
        };

        Some(Op {
            typ: typ,
            dest: decoded.rd,
            src1: decoded.rs1,
            src2: decoded.rs2,
        })
    }
}

impl Instruction for Op {
    fn execute(&self, cpu: &mut CPU) {
        let src1 = cpu.get_register(self.src1);
        let src2 = cpu.get_register(self.src2);

        cpu.set_register(self.dest,
                         match self.typ {
                             OperationType::Add => src1.wrapping_add(src2),
                             OperationType::Sub => src1.wrapping_sub(src2),
                             OperationType::SetLessThan => {
                                 if (src1 as i32) < (src2 as i32) { 1 } else { 0 }
                             }
                             OperationType::SetLessThanUnsigned => if src1 < src2 { 1 } else { 0 },
                             OperationType::And => src1 & src2,
                             OperationType::Or => src1 | src2,
                             OperationType::Xor => src1 ^ src2,
                             OperationType::ShiftLeftLogical => src1 << src2,
                             OperationType::ShiftRightLogical => src1 >> src2,
                             OperationType::ShiftRightArithmetic => ((src1 as i32) >> src2) as u32,
                         });
    }
}

#[derive(Debug)]
pub struct Lui {
    dest: u8,
    immediate: u32,
}

impl Lui {
    pub fn parse(instruction: u32) -> Option<Lui> {
        let decoded = encoding::U::parse(instruction);

        if decoded.opcode != 0x37 {
            // Not a LUI opcode
            return None;
        }

        Some(Lui {
            dest: decoded.rd,
            immediate: decoded.immediate as u32,
        })
    }
}

impl Instruction for Lui {
    fn execute(&self, cpu: &mut CPU) {
        cpu.set_register(self.dest, self.immediate);
    }
}

#[derive(Debug)]
pub struct Auipc {
    dest: u8,
    immediate: u32,
}

impl Auipc {
    pub fn parse(instruction: u32) -> Option<Auipc> {
        let decoded = encoding::U::parse(instruction);

        if decoded.opcode != 0x17 {
            // Not a AUIPC opcode
            return None;
        }

        Some(Auipc {
            dest: decoded.rd,
            immediate: decoded.immediate as u32,
        })
    }
}

impl Instruction for Auipc {
    fn execute(&self, cpu: &mut CPU) {
        let result = cpu.pc.wrapping_add(self.immediate);
        cpu.set_register(self.dest, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpu::CPU;
    use ram::RAM;
    use instruction::Instruction;

    macro_rules! test_imm_op {
        ($cpu:expr, $op:expr, $result:expr, $val1:expr, $imm:expr) => {
            $cpu.set_register(1, $val1);
            let raw_instruction = (($imm & 0xFFF) << 20) | (1 << 15) | (3 << 7) | $op << 12 | 0x13;
            let instr = OpImm::parse(raw_instruction).expect("couldn't parse instruction");
            instr.execute(&mut $cpu);
            assert_eq!($cpu.get_register(3), $result);
        }
    }

    macro_rules! test_imm_src1_eq_dest {
        ($cpu:expr, $op:expr, $result:expr, $val1:expr, $imm:expr) => {
            $cpu.set_register(1, $val1);
            let raw_instruction = (($imm & 0xFFF) << 20) | (1 << 15) | (0 << 12) | (1 << 7) | $op << 12 | 0x13;
            let instr = OpImm::parse(raw_instruction).expect("couldn't parse instruction");
            instr.execute(&mut $cpu);
            assert_eq!($cpu.get_register(1), $result);
        }
    }

    #[test]
    fn test_addi() {
        let mut cpu = CPU::new(RAM::new(1024));

        test_imm_op!(cpu, 0b000, 0x00000000, 0x00000000, 0x000);
        test_imm_op!(cpu, 0b000, 0x00000002, 0x00000001, 0x001);
        test_imm_op!(cpu, 0b000, 0x0000000a, 0x00000003, 0x007);

        test_imm_op!(cpu, 0b000, 0xfffff800, 0x00000000, 0x800);
        test_imm_op!(cpu, 0b000, 0x80000000, 0x80000000, 0x000);
        test_imm_op!(cpu, 0b000, 0x7ffff800, 0x80000000, 0x800);

        test_imm_op!(cpu, 0b000, 0x000007ff, 0x00000000, 0x7ff);
        test_imm_op!(cpu, 0b000, 0x7fffffff, 0x7fffffff, 0x000);
        test_imm_op!(cpu, 0b000, 0x800007fe, 0x7fffffff, 0x7ff);

        test_imm_op!(cpu, 0b000, 0x800007ff, 0x80000000, 0x7ff);
        test_imm_op!(cpu, 0b000, 0x7ffff7ff, 0x7fffffff, 0x800);

        test_imm_op!(cpu, 0b000, 0xffffffff, 0x00000000, 0xfff);
        test_imm_op!(cpu, 0b000, 0x00000000, 0xffffffff, 0x001);
        test_imm_op!(cpu, 0b000, 0xfffffffe, 0xffffffff, 0xfff);

        test_imm_op!(cpu, 0b000, 0x80000000, 0x7fffffff, 0x001);

        test_imm_src1_eq_dest!(cpu, 0b000, 24, 13, 11);
    }

    #[test]
    fn test_slli() {
        let mut cpu = CPU::new(RAM::new(1024));

        test_imm_op!(cpu, 0b001, 0x00000001, 0x00000001, 0);
        test_imm_op!(cpu, 0b001, 0x00000002, 0x00000001, 1);
        test_imm_op!(cpu, 0b001, 0x00000080, 0x00000001, 7);
        test_imm_op!(cpu, 0b001, 0x00004000, 0x00000001, 14);
        test_imm_op!(cpu, 0b001, 0x80000000, 0x00000001, 31);

        test_imm_op!(cpu, 0b001, 0xffffffff, 0xffffffff, 0);
        test_imm_op!(cpu, 0b001, 0xfffffffe, 0xffffffff, 1);
        test_imm_op!(cpu, 0b001, 0xffffff80, 0xffffffff, 7);
        test_imm_op!(cpu, 0b001, 0xffffc000, 0xffffffff, 14);
        test_imm_op!(cpu, 0b001, 0x80000000, 0xffffffff, 31);

        test_imm_op!(cpu, 0b001, 0x21212121, 0x21212121, 0);
        test_imm_op!(cpu, 0b001, 0x42424242, 0x21212121, 1);
        test_imm_op!(cpu, 0b001, 0x90909080, 0x21212121, 7);
        test_imm_op!(cpu, 0b001, 0x48484000, 0x21212121, 14);
        test_imm_op!(cpu, 0b001, 0x80000000, 0x21212121, 31);

        test_imm_src1_eq_dest!(cpu, 0b001, 0x00000080, 0x00000001, 7);
    }

    #[test]
    fn test_slti() {
        let mut cpu = CPU::new(RAM::new(1024));
        
        test_imm_op!(cpu, 0b010, 0, 0x00000000, 0x000);
        test_imm_op!(cpu, 0b010, 0, 0x00000001, 0x001);
        test_imm_op!(cpu, 0b010, 1, 0x00000003, 0x007);
        test_imm_op!(cpu, 0b010, 0, 0x00000007, 0x003);

        test_imm_op!(cpu, 0b010, 0, 0x00000000, 0x800);
        test_imm_op!(cpu, 0b010, 1, 0x80000000, 0x000);
        test_imm_op!(cpu, 0b010, 1, 0x80000000, 0x800);

        test_imm_op!(cpu, 0b010, 1, 0x00000000, 0x7ff);
        test_imm_op!(cpu, 0b010, 0, 0x7fffffff, 0x000);
        test_imm_op!(cpu, 0b010, 0, 0x7fffffff, 0x7ff);

        test_imm_op!(cpu, 0b010, 1, 0x80000000, 0x7ff);
        test_imm_op!(cpu, 0b010, 0, 0x7fffffff, 0x800);

        test_imm_op!(cpu, 0b010, 0, 0x00000000, 0xfff);
        test_imm_op!(cpu, 0b010, 1, 0xffffffff, 0x001);
        test_imm_op!(cpu, 0b010, 0, 0xffffffff, 0xfff);

        test_imm_src1_eq_dest!(cpu, 0b010, 1, 11, 13);
    }

    #[test]
    fn test_sltiu() {
        let mut cpu = CPU::new(RAM::new(1024));

        test_imm_op!(cpu, 0b011, 0, 0x00000000, 0x000);
        test_imm_op!(cpu, 0b011, 0, 0x00000001, 0x001);
        test_imm_op!(cpu, 0b011, 1, 0x00000003, 0x007);
        test_imm_op!(cpu, 0b011, 0, 0x00000007, 0x003);

        test_imm_op!(cpu, 0b011, 1, 0x00000000, 0x800);
        test_imm_op!(cpu, 0b011, 0, 0x80000000, 0x000);
        test_imm_op!(cpu, 0b011, 1, 0x80000000, 0x800);

        test_imm_op!(cpu, 0b011, 1, 0x00000000, 0x7ff);
        test_imm_op!(cpu, 0b011, 0, 0x7fffffff, 0x000);
        test_imm_op!(cpu, 0b011, 0, 0x7fffffff, 0x7ff);

        test_imm_op!(cpu, 0b011, 0, 0x80000000, 0x7ff);
        test_imm_op!(cpu, 0b011, 1, 0x7fffffff, 0x800);

        test_imm_op!(cpu, 0b011, 1, 0x00000000, 0xfff);
        test_imm_op!(cpu, 0b011, 0, 0xffffffff, 0x001);
        test_imm_op!(cpu, 0b011, 0, 0xffffffff, 0xfff);

        test_imm_src1_eq_dest!(cpu, 0b011, 1, 11, 13);
    }

    #[test]
    fn test_xori() {
        let mut cpu = CPU::new(RAM::new(1024));

        test_imm_op!(cpu, 0b100, 0xff00f00f, 0x00ff0f00, 0xf0f);
        test_imm_op!(cpu, 0b100, 0x0ff00f00, 0x0ff00ff0, 0x0f0);
        test_imm_op!(cpu, 0b100, 0x00ff0ff0, 0x00ff08ff, 0x70f);
        test_imm_op!(cpu, 0b100, 0xf00ff0ff, 0xf00ff00f, 0x0f0);

        test_imm_src1_eq_dest!(cpu, 0b100, 0xff00f00f, 0xff00f700, 0x70f);
    }

    #[test]
    fn test_srli() {
        let mut cpu = CPU::new(RAM::new(1024));

        test_imm_op!(cpu, 0b101, 0x80000000 >>  0, 0x80000000, 0);
        test_imm_op!(cpu, 0b101, 0x80000000 >>  1, 0x80000000, 1);
        test_imm_op!(cpu, 0b101, 0x80000000 >>  7, 0x80000000, 7);
        test_imm_op!(cpu, 0b101, 0x80000000 >> 14, 0x80000000, 14);
        test_imm_op!(cpu, 0b101, 0x80000001 >> 31, 0x80000001, 31);
        test_imm_op!(cpu, 0b101, 0xffffffff >>  0, 0xffffffff, 0);
        test_imm_op!(cpu, 0b101, 0xffffffff >>  1, 0xffffffff, 1);
        test_imm_op!(cpu, 0b101, 0xffffffff >>  7, 0xffffffff, 7);
        test_imm_op!(cpu, 0b101, 0xffffffff >> 14, 0xffffffff, 14);
        test_imm_op!(cpu, 0b101, 0xffffffff >> 31, 0xffffffff, 31);
        test_imm_op!(cpu, 0b101, 0x21212121 >>  0, 0x21212121, 0);
        test_imm_op!(cpu, 0b101, 0x21212121 >>  1, 0x21212121, 1);
        test_imm_op!(cpu, 0b101, 0x21212121 >>  7, 0x21212121, 7);
        test_imm_op!(cpu, 0b101, 0x21212121 >> 14, 0x21212121, 14);
        test_imm_op!(cpu, 0b101, 0x21212121 >> 31, 0x21212121, 31);

        test_imm_src1_eq_dest!(cpu, 0b101, 0x01000000, 0x80000000, 7);
    }

    #[test]
    fn test_srai() {
        let mut cpu = CPU::new(RAM::new(1024));

        test_imm_op!(cpu, 0b101, 0x00000000, 0x00000000, 0  | 1 << 10);
        test_imm_op!(cpu, 0b101, 0xc0000000, 0x80000000, 1  | 1 << 10);
        test_imm_op!(cpu, 0b101, 0xff000000, 0x80000000, 7  | 1 << 10);
        test_imm_op!(cpu, 0b101, 0xfffe0000, 0x80000000, 14 | 1 << 10);
        test_imm_op!(cpu, 0b101, 0xffffffff, 0x80000001, 31 | 1 << 10);

        test_imm_op!(cpu, 0b101, 0x7fffffff, 0x7fffffff, 0  | 1 << 10);
        test_imm_op!(cpu, 0b101, 0x3fffffff, 0x7fffffff, 1  | 1 << 10);
        test_imm_op!(cpu, 0b101, 0x00ffffff, 0x7fffffff, 7  | 1 << 10);
        test_imm_op!(cpu, 0b101, 0x0001ffff, 0x7fffffff, 14 | 1 << 10);
        test_imm_op!(cpu, 0b101, 0x00000000, 0x7fffffff, 31 | 1 << 10);

        test_imm_op!(cpu, 0b101, 0x81818181, 0x81818181, 0  | 1 << 10);
        test_imm_op!(cpu, 0b101, 0xc0c0c0c0, 0x81818181, 1  | 1 << 10);
        test_imm_op!(cpu, 0b101, 0xff030303, 0x81818181, 7  | 1 << 10);
        test_imm_op!(cpu, 0b101, 0xfffe0606, 0x81818181, 14 | 1 << 10);
        test_imm_op!(cpu, 0b101, 0xffffffff, 0x81818181, 31 | 1 << 10);

        test_imm_src1_eq_dest!(cpu, 0b101, 0xff000000, 0x80000000, 7 | 1 << 10);
    }

    #[test]
    fn test_ori() {
        let mut cpu = CPU::new(RAM::new(1024));

        test_imm_op!(cpu, 0b110, 0xffffff0f, 0xff00ff00, 0xf0f);
        test_imm_op!(cpu, 0b110, 0x0ff00ff0, 0x0ff00ff0, 0x0f0);
        test_imm_op!(cpu, 0b110, 0x00ff07ff, 0x00ff00ff, 0x70f);
        test_imm_op!(cpu, 0b110, 0xf00ff0ff, 0xf00ff00f, 0x0f0);

        test_imm_src1_eq_dest!(cpu, 0b110, 0xff00fff0, 0xff00ff00, 0x0f0);
    }

    #[test]
    fn test_andi() {
        let mut cpu = CPU::new(RAM::new(1024));

        test_imm_op!(cpu, 0b111, 0xff00ff00, 0xff00ff00, 0xf0f );
        test_imm_op!(cpu, 0b111, 0x000000f0, 0x0ff00ff0, 0x0f0 );
        test_imm_op!(cpu, 0b111, 0x0000000f, 0x00ff00ff, 0x70f );
        test_imm_op!(cpu, 0b111, 0x00000000, 0xf00ff00f, 0x0f0 );

        test_imm_src1_eq_dest!(cpu, 0b111, 0x00000000, 0xff00ff00, 0x0f0);
    }
}
