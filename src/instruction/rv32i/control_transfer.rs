// Copyright 2016 risc-v-emulator Developers
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

        if decoded.opcode != 0x67 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use cpu::CPU;
    use ram::RAM;
    use instruction::Instruction;

    macro_rules! test_br2_op_taken {
        ($cpu:expr, $op:expr, $val1:expr, $val2:expr) => {
            $cpu.set_register(1, $val1 as u32);
            $cpu.set_register(2, $val2 as u32);
            let raw_instruction = (4 << 8) | (2 << 20) | (1 << 15) | $op << 12 | 0x63;
            let instr = Branch::parse(raw_instruction).expect("couldn't parse instruction");
            $cpu.pc = 100;
            instr.execute(&mut $cpu);
            $cpu.pc = $cpu.pc.wrapping_add(4);
            assert_eq!($cpu.pc, 108);
        }
    }

    macro_rules! test_br2_op_not_taken {
        ($cpu:expr, $op:expr, $val1:expr, $val2:expr) => {
            $cpu.set_register(1, $val1 as u32);
            $cpu.set_register(2, $val2 as u32);
            let raw_instruction = (4 << 8) | (2 << 20) | (1 << 15) | $op << 12 | 0x63;
            let instr = Branch::parse(raw_instruction).expect("couldn't parse instruction");
            $cpu.pc = 100;
            instr.execute(&mut $cpu);
            $cpu.pc = $cpu.pc.wrapping_add(4);
            assert_eq!($cpu.pc, 104);
        }
    }

    #[test]
    fn test_beq() {
        let mut cpu = CPU::new(RAM::new(1024));

        test_br2_op_taken!(cpu, 0b000, 0, 0);
        test_br2_op_taken!(cpu, 0b000, 1, 1);
        test_br2_op_taken!(cpu, 0b000, -1i32, -1i32);

        test_br2_op_not_taken!(cpu, 0b000, 0, 1);
        test_br2_op_not_taken!(cpu, 0b000, 1, 0);
        test_br2_op_not_taken!(cpu, 0b000, -1i32, 1);
        test_br2_op_not_taken!(cpu, 0b000, 1, -1i32);
    }

    #[test]
    fn test_bne() {
        let mut cpu = CPU::new(RAM::new(1024));

        test_br2_op_taken!(cpu, 0b001, 0, 1);
        test_br2_op_taken!(cpu, 0b001, 1, 0);
        test_br2_op_taken!(cpu, 0b001, -1i32, 1);
        test_br2_op_taken!(cpu, 0b001, 1, -1i32);

        test_br2_op_not_taken!(cpu, 0b001, 0, 0);
        test_br2_op_not_taken!(cpu, 0b001, 1, 1);
        test_br2_op_not_taken!(cpu, 0b001, -1i32, -1i32);
    }

    #[test]
    fn test_blt() {
        let mut cpu = CPU::new(RAM::new(1024));

        test_br2_op_taken!(cpu, 0b100, 0, 1);
        test_br2_op_taken!(cpu, 0b100, -1i32, 1);
        test_br2_op_taken!(cpu, 0b100, -2i32, -1i32);

        test_br2_op_not_taken!(cpu, 0b100, 1, 0);
        test_br2_op_not_taken!(cpu, 0b100, 1, -1i32);
        test_br2_op_not_taken!(cpu, 0b100, -1i32, -2i32);
        test_br2_op_not_taken!(cpu, 0b100, 1, -2i32);
    }

    #[test]
    fn test_bge() {
        let mut cpu = CPU::new(RAM::new(1024));

        test_br2_op_taken!(cpu, 0b101, 0, 0);
        test_br2_op_taken!(cpu, 0b101, 1, 1);
        test_br2_op_taken!(cpu, 0b101, -1i32, -1i32);
        test_br2_op_taken!(cpu, 0b101, 1, 0);
        test_br2_op_taken!(cpu, 0b101, 1, -1i32);
        test_br2_op_taken!(cpu, 0b101, -1i32, -2i32);

        test_br2_op_not_taken!(cpu, 0b101, 0, 1);
        test_br2_op_not_taken!(cpu, 0b101, -1i32, 1);
        test_br2_op_not_taken!(cpu, 0b101, -2i32, -1i32);
        test_br2_op_not_taken!(cpu, 0b101, -2i32, 1);
    }

    #[test]
    fn test_bltu() {
        let mut cpu = CPU::new(RAM::new(1024));

        test_br2_op_taken!(cpu, 0b110, 0x00000000, 0x00000001);
        test_br2_op_taken!(cpu, 0b110, 0xfffffffe, 0xffffffff);
        test_br2_op_taken!(cpu, 0b110, 0x00000000, 0xffffffff);

        test_br2_op_not_taken!(cpu, 0b110, 0x00000001, 0x00000000 );
        test_br2_op_not_taken!(cpu, 0b110, 0xffffffff, 0xfffffffe );
        test_br2_op_not_taken!(cpu, 0b110, 0xffffffff, 0x00000000 );
        test_br2_op_not_taken!(cpu, 0b110, 0x80000000, 0x7fffffff );
    }

    #[test]
    fn test_bgeu() {
        let mut cpu = CPU::new(RAM::new(1024));

        test_br2_op_taken!(cpu, 0b111, 0x00000000, 0x00000000);
        test_br2_op_taken!(cpu, 0b111, 0x00000001, 0x00000001);
        test_br2_op_taken!(cpu, 0b111, 0xffffffff, 0xffffffff);
        test_br2_op_taken!(cpu, 0b111, 0x00000001, 0x00000000);
        test_br2_op_taken!(cpu, 0b111, 0xffffffff, 0xfffffffe);
        test_br2_op_taken!(cpu, 0b111, 0xffffffff, 0x00000000);

        test_br2_op_not_taken!(cpu, 0b111, 0x00000000, 0x00000001);
        test_br2_op_not_taken!(cpu, 0b111, 0xfffffffe, 0xffffffff);
        test_br2_op_not_taken!(cpu, 0b111, 0x00000000, 0xffffffff);
        test_br2_op_not_taken!(cpu, 0b111, 0x7fffffff, 0x80000000);
    }
}
