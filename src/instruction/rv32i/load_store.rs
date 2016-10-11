// Copyright 2016 risc-v-emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use instruction::{encoding, Instruction};
use cpu::CPU;

#[derive(Debug)]
pub struct Load {
    typ: LoadType,
    dest: u8,
    offset: i32,
    base: u8,
}

#[derive(Debug)]
pub enum LoadType {
    Byte,
    ByteUnsigned,
    HalfWord,
    HalfWordUnsigned,
    Word,
}

impl Load {
    pub fn parse(instruction: u32) -> Option<Load> {
        let decoded = encoding::I::parse(instruction);

        if decoded.opcode != 0x03 {
            // Not a LOAD opcode
            return None;
        }

        let typ = match decoded.funct3 {
            0b000 => LoadType::Byte,
            0b001 => LoadType::HalfWord,
            0b010 => LoadType::Word,
            0b100 => LoadType::ByteUnsigned,
            0b101 => LoadType::HalfWordUnsigned,
            _ => return None,
        };

        Some(Load {
            typ: typ,
            dest: decoded.rd,
            offset: decoded.immediate,
            base: decoded.rs1,
        })
    }
}

impl Instruction for Load {
    fn execute(&self, cpu: &mut CPU) {
        let addr = cpu.get_register(self.base).wrapping_add(self.offset as u32);

        let value = match self.typ {
            LoadType::Byte => cpu.ram[addr] as i8 as i32 as u32,
            LoadType::HalfWord => cpu.ram.get_u16(addr) as i16 as i32 as u32,
            LoadType::Word => cpu.ram.get_u32(addr),
            LoadType::ByteUnsigned => cpu.ram[addr] as u32,
            LoadType::HalfWordUnsigned => cpu.ram.get_u16(addr) as u32,
        };

        cpu.set_register(self.dest, value);
    }
}

#[derive(Debug)]
pub struct Store {
    typ: StoreType,
    offset: i32,
    base: u8,
    src: u8,
}

#[derive(Debug)]
pub enum StoreType {
    Byte,
    HalfWord,
    Word,
}

impl Store {
    pub fn parse(instruction: u32) -> Option<Store> {
        let decoded = encoding::S::parse(instruction);

        if decoded.opcode != 0x23 {
            // Not a STORE opcode
            return None;
        }

        let typ = match decoded.funct3 {
            0b000 => StoreType::Byte,
            0b001 => StoreType::HalfWord,
            0b010 => StoreType::Word,
            _ => return None,
        };

        Some(Store {
            typ: typ,
            offset: decoded.immediate,
            base: decoded.rs1,
            src: decoded.rs2,
        })
    }
}

impl Instruction for Store {
    fn execute(&self, cpu: &mut CPU) {
        let addr = cpu.get_register(self.base).wrapping_add(self.offset as u32);
        let value = cpu.get_register(self.src);

        match self.typ {
            StoreType::Byte => cpu.ram[addr] = value as u8,
            StoreType::HalfWord => cpu.ram.set_u16(addr, value as u16),
            StoreType::Word => cpu.ram.set_u32(addr, value),
        };
    }
}
