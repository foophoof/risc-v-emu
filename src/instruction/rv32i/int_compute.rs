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
                if decoded.immediate & (1 << 31) == 0 {
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
