// Copyright 2016 risc-v-emulator Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub struct R {
    pub opcode: u8,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct7: u8,
}

pub struct I {
    pub opcode: u8,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub immediate: i32,
}

pub struct S {
    pub opcode: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub immediate: i32,
}

pub struct SB {
    pub opcode: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub immediate: i32,
}

pub struct U {
    pub opcode: u8,
    pub rd: u8,
    pub immediate: i32,
}

pub struct UJ {
    pub opcode: u8,
    pub rd: u8,
    pub immediate: i32,
}

pub fn get_opcode(instruction: u32) -> u8 {
    (instruction & 0x7F) as u8
}

pub fn set_opcode(opcode: u8) -> u32 {
    (opcode as u32) & 0x7F
}

pub fn get_rd(instruction: u32) -> u8 {
    ((instruction & 0xF80) >> 7) as u8
}

pub fn set_rd(rd: u8) -> u32 {
    ((rd as u32) << 7) & 0xF80
}

pub fn get_rs1(instruction: u32) -> u8 {
    ((instruction & 0xF8000) >> 15) as u8
}

pub fn set_rs1(rs1: u8) -> u32 {
    ((rs1 as u32) << 15) & 0xF8000
}

pub fn get_rs2(instruction: u32) -> u8 {
    ((instruction & 0x1F00000) >> 20) as u8
}

pub fn set_rs2(rs2: u8) -> u32 {
    ((rs2 as u32) << 20) & 0x1F00000
}

pub fn get_funct3(instruction: u32) -> u8 {
    ((instruction & 0x7000) >> 12) as u8
}

pub fn set_funct3(funct3: u8) -> u32 {
    ((funct3 as u32) << 12) & 0x7000
}

pub fn get_funct7(instruction: u32) -> u8 {
    ((instruction & 0xFE000000) >> 25) as u8
}

pub fn set_funct7(funct7: u8) -> u32 {
    ((funct7 as u32) << 25) & 0xFE000000
}

impl R {
    pub fn parse(instruction: u32) -> R {
        R {
            opcode: get_opcode(instruction),
            rd: get_rd(instruction),
            funct3: get_funct3(instruction),
            rs1: get_rs1(instruction),
            rs2: get_rs2(instruction),
            funct7: get_funct7(instruction),
        }
    }

    pub fn to_raw(&self) -> u32 {
        set_opcode(self.opcode) | set_rd(self.rd) | set_funct3(self.funct3) | set_rs1(self.rs1) | set_rs2(self.rs2) | set_funct7(self.funct7)
    }
}

impl I {
    pub fn parse(instruction: u32) -> I {
        let immediate = ((instruction & 0xFFF00000) as i32) >> 20;

        I {
            opcode: get_opcode(instruction),
            rd: get_rd(instruction),
            funct3: get_funct3(instruction),
            rs1: get_rs1(instruction),
            immediate: immediate,
        }
    }

    pub fn to_raw(&self) -> u32 {
        ((self.immediate as u32) << 20) & 0xFFF00000 | set_opcode(self.opcode) | set_rd(self.rd) | set_funct3(self.funct3) | set_rs1(self.rs1)
    }
}

impl S {
    pub fn parse(instruction: u32) -> S {
        let immediate_high = (instruction & 0xFE000000) as i32;
        let immediate_low = (instruction & 0xF80) as i32;

        let immediate_high_shifted = immediate_high >> 20;
        let immediate_low_shifted = immediate_low >> 7;

        let immediate = immediate_high_shifted | immediate_low_shifted;

        S {
            opcode: get_opcode(instruction),
            funct3: get_funct3(instruction),
            rs1: get_rs1(instruction),
            rs2: get_rs2(instruction),
            immediate: immediate,
        }
    }

    pub fn to_raw(&self) -> u32 {
        let immediate_low = (self.immediate as u32) & 0x1F;
        let immediate_high = ((self.immediate as u32) & 0xFE0) >> 5;

        immediate_low << 7 | immediate_high << 25 | set_opcode(self.opcode) | set_funct3(self.funct3) | set_rs1(self.rs1) | set_rs2(self.rs2)
    }
}

impl SB {
    pub fn parse(instruction: u32) -> SB {
        let sign_extension = ((instruction & 0x80000000) as i32) >> 19;
        let high = (instruction & 0x80) << 4;
        let mid = (instruction & 0x7E000000) >> 20;
        let low = (instruction & 0xF00) >> 7;

        let immediate = sign_extension | high as i32 | mid as i32 | low as i32;

        SB {
            opcode: get_opcode(instruction),
            funct3: get_funct3(instruction),
            rs1: get_rs1(instruction),
            rs2: get_rs2(instruction),
            immediate: immediate,
        }
    }

    pub fn to_raw(&self) -> u32 {
        let sign = ((self.immediate as u32) & (1 << 12)) >> 12;
        let high = ((self.immediate as u32) & (1 << 11)) >> 11;
        let mid = ((self.immediate as u32) & 0x7E0) >> 5;
        let low = ((self.immediate as u32) & 0x1E) >> 1;

        sign << 31 | high << 7 | mid << 25 | low << 8 | set_opcode(self.opcode) | set_funct3(self.funct3) | set_rs1(self.rs1) | set_rs2(self.rs2)
    }
}

impl U {
    pub fn parse(instruction: u32) -> U {
        U {
            opcode: get_opcode(instruction),
            rd: get_rd(instruction),
            immediate: (instruction & 0xFFFFF000) as i32,
        }
    }

    pub fn to_raw(&self) -> u32 {
        (self.immediate as u32) & 0xFFFFF000 | set_opcode(self.opcode) | set_rd(self.rd)
    }
}

impl UJ {
    pub fn parse(instruction: u32) -> UJ {
        let sign_extension = ((instruction & 0x80000000) as i32) >> 11;
        let high = instruction & 0xFF000;
        let mid = (instruction & 0x100000) >> 9;
        let low = (instruction & 0x7FE00000) >> 20;

        UJ {
            opcode: get_opcode(instruction),
            rd: get_rd(instruction),
            immediate: sign_extension | high as i32 | mid as i32 | low as i32,
        }
    }

    pub fn to_raw(&self) -> u32 {
        let sign = ((self.immediate as u32) & (1 << 20)) >> 20;
        let high = ((self.immediate as u32) & 0xFF000) >> 12;
        let mid = ((self.immediate as u32) & (1 << 11)) >> 11;
        let low = ((self.immediate as u32) & 0x7FE) >> 1;

        sign << 31 | high << 12 | mid << 20 | low << 21 | set_opcode(self.opcode) | set_rd(self.rd)
    }
}
