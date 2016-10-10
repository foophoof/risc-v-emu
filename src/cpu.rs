use std::fmt;

use instruction;
use ram::RAM;

pub struct CPU {
    regs: [u32; 32],
    pub pc: u32,
    pub ram: RAM,
}

impl CPU {
    pub fn new(ram: RAM) -> CPU {
        let mut regs = [0; 32];
        regs[2] = 1020 * 1024; // Stack Pointer

        CPU {
            regs: regs,
            pc: 0,
            ram: ram,
        }
    }

    pub fn run(&mut self, entry_point: u32) {
        self.pc = entry_point;

        while let Some(instr) = self.get_instruction() {
            instr.execute(self);
            self.pc = self.pc.wrapping_add(4);
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
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CPU").field("regs", &self.regs).field("pc", &self.pc).finish()
    }
}
