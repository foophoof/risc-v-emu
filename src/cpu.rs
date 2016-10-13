// Copyright 2016 risc-v-emulator Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::fmt;

use instruction;
use ram::RAM;

pub struct CPU {
    regs: [u32; 32],
    csr: CSRs,
    pub pc: u32,
    pub ram: RAM,
}

struct CSRs {
    cycles: u64,
    mepc: u32,
}

impl CPU {
    pub fn new(ram: RAM) -> CPU {
        let mut regs = [0; 32];
        regs[2] = 1020 * 1024; // Stack Pointer

        CPU {
            regs: regs,
            csr: CSRs {
                cycles: 0,
                mepc: 0,
            },
            pc: 0,
            ram: ram,
        }
    }

    pub fn run(&mut self, entry_point: u32) {
        self.set_register(1, 0); // Return address
        self.pc = entry_point;

        while let Some(instr) = self.get_instruction() {
            instr.execute(self);
            println!("{:05X} {:?} {:08x}", self.pc, self, instr.to_raw());
            self.pc = self.pc.wrapping_add(4);
            self.csr.cycles = self.csr.cycles.wrapping_add(1);
            if self.pc == 0 {
                break;
            }
        }
    }

    fn get_instruction(&self) -> Option<Box<instruction::Instruction>> {
        instruction::parse(self.ram.get_u32(self.pc))
    }

    pub fn get_register(&self, reg: u8) -> u32 {
        self.regs[reg as usize]
    }

    pub fn set_register(&mut self, reg: u8, value: u32) {
        if reg == 0 {
            return;
        }

        self.regs[reg as usize] = value
    }

    pub fn get_csr(&self, csr: u16) -> u32 {
        match csr {
            0x341 => self.csr.mepc,
            0x700...0x702 => 0,
            0x704...0x706 => 0,
            0x708...0x70A => 0,
            0xC00 => self.csr.cycles as u32,
            0xC80 => (self.csr.cycles >> 32) as u32,
            0xF10 => (1 << 30) | (1 << 8) | (1 << 12), // misa RV32IM
            _ => panic!("read csr 0x{:03X} not implemented", csr),
        }
    }

    pub fn set_csr(&mut self, csr: u16, value: u32) {
        match csr {
            0x780 => {}
            _ => panic!("write csr 0x{:03X} not implemented", csr),
        }
    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CPU").field("regs", &self.regs).field("pc", &self.pc).finish()
    }
}
