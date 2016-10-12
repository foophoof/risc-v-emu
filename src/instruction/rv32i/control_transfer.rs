// Copyright 2016 risc-v-emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use instruction::{encoding, Instruction};
use cpu::CPU;

#[derive(Debug)]
pub struct Jal {
    dest: u8,
    offset: i32,
}

impl Jal {
    pub fn parse(instruction: u32) -> Option<Jal> {
        let decoded = encoding::UJ::parse(instruction);

        if decoded.opcode != 0x6F {
            // Not a JAL opcode
            return None;
        }

        Some(Jal {
            dest: decoded.rd,
            offset: decoded.immediate,
        })
    }
}

impl Instruction for Jal {
    fn execute(&self, cpu: &mut CPU) {
        let jump_back_target = cpu.pc.wrapping_add(4);
        cpu.set_register(self.dest, jump_back_target);

        let target = cpu.pc.wrapping_add(self.offset as u32);
        // Subtract 4 since the CPU will increment by 4 after the instruction is
        // run.
        cpu.pc = target.wrapping_sub(4);
    }
}

#[derive(Debug)]
pub struct Jalr {
    dest: u8,
    base: u8,
    offset: i32,
}

impl Jalr {
    pub fn parse(instruction: u32) -> Option<Jalr> {
        let decoded = encoding::I::parse(instruction);

        if decoded.opcode != 0x6F {
            // Not a JALR opcode
            return None;
        }

        if decoded.funct3 != 0 {
            return None;
        }

        Some(Jalr {
            dest: decoded.rd,
            base: decoded.rs1,
            offset: decoded.immediate,
        })
    }
}

impl Instruction for Jalr {
    fn execute(&self, cpu: &mut CPU) {
        let jump_back_target = cpu.pc.wrapping_add(4);
        cpu.set_register(self.dest, jump_back_target);

        let base = cpu.get_register(self.base);
        let target = base.wrapping_add(self.offset as u32) & 0xFFFFFFFE;
        // Subtract 4 since the CPU will increment by 4 after the instruction is
        // run.
        cpu.pc = target.wrapping_sub(4);
    }
}

#[derive(Debug)]
pub enum BranchType {
    Equals,
    NotEquals,
    LessThan,
    LessThanUnsigned,
    GreaterOrEqual,
    GreaterOrEqualUnsigned,
}

#[derive(Debug)]
pub struct Branch {
    typ: BranchType,
    src1: u8,
    src2: u8,
    offset: i32,
}

impl Branch {
    pub fn parse(instruction: u32) -> Option<Branch> {
        let decoded = encoding::SB::parse(instruction);

        if decoded.opcode != 0x63 {
            // Not a BRANCH opcode
            return None;
        }

        let typ = match decoded.funct3 {
            0b000 => BranchType::Equals,
            0b001 => BranchType::NotEquals,
            0b100 => BranchType::LessThan,
            0b101 => BranchType::GreaterOrEqual,
            0b110 => BranchType::LessThanUnsigned,
            0b111 => BranchType::GreaterOrEqualUnsigned,
            _ => return None,
        };

        Some(Branch {
            typ: typ,
            src1: decoded.rs1,
            src2: decoded.rs2,
            offset: decoded.immediate,
        })
    }
}

impl Instruction for Branch {
    fn execute(&self, cpu: &mut CPU) {
        let src1 = cpu.get_register(self.src1);
        let src2 = cpu.get_register(self.src2);

        let result = match self.typ {
            BranchType::Equals => src1 == src2,
            BranchType::NotEquals => src1 != src2,
            BranchType::LessThan => (src1 as i32) < (src2 as i32),
            BranchType::GreaterOrEqual => (src1 as i32) >= (src2 as i32),
            BranchType::LessThanUnsigned => src1 < src2,
            BranchType::GreaterOrEqualUnsigned => src1 >= src2,
        };

        if result {
            cpu.pc = cpu.pc.wrapping_add(self.offset as u32).wrapping_sub(4);
        }
    }
}
