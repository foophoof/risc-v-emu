use std::fmt;

use opcode::Opcode;
use instruction::Instruction;

pub struct CPU {
    regs: [u32; 32],
    pc: u32,
    ram: Vec<u8>,
}

impl CPU {
    pub fn new(ram: Vec<u8>) -> CPU {
        CPU {
            regs: [0; 32],
            pc: 0,
            ram: ram,
        }
    }

    pub fn step(&mut self) {
        let instr = self.get_instruction();

        let opcode = instr.opcode();
        if opcode.is_none() {
            panic!("invalid opcode");
        }

        match opcode.unwrap() {
            Opcode::Lui => {
                self.regs[instr.rd().unwrap() as usize] = instr.immediate().unwrap();
            }
            Opcode::Auipc => {
                self.regs[instr.rd().unwrap() as usize] = self.pc + instr.immediate().unwrap();
            }
            Opcode::Jal => {
                println!("jumping {} forward", instr.immediate().unwrap());
                self.pc = self.pc.wrapping_add(instr.immediate().unwrap()).wrapping_sub(4);
                self.regs[instr.rd().unwrap() as usize] = self.pc.wrapping_add(8);
            }
            Opcode::Jalr => {
                self.pc = self.pc.wrapping_add(instr.immediate().unwrap());
                self.pc = self.pc.wrapping_add(self.regs[instr.rs1().unwrap() as usize]);
                self.regs[instr.rd().unwrap() as usize] = self.pc.wrapping_add(4);
            }
            Opcode::Branch => {
                match instr.funct3().unwrap() {
                    0b000 => {
                        // BEQ
                        if self.regs[instr.rs1().unwrap() as usize] ==
                           self.regs[instr.rs2().unwrap() as usize] {
                            self.pc = self.pc.wrapping_add(instr.immediate().unwrap());
                        }
                    }
                    0b001 => {
                        // BNE
                        if self.regs[instr.rs1().unwrap() as usize] !=
                           self.regs[instr.rs2().unwrap() as usize] {
                            self.pc = self.pc.wrapping_add(instr.immediate().unwrap());
                        }
                    }
                    0b100 => {
                        // BLT
                        if (self.regs[instr.rs1().unwrap() as usize] as i32) <
                           (self.regs[instr.rs2().unwrap() as usize] as i32) {
                            self.pc = self.pc.wrapping_add(instr.immediate().unwrap());
                        }
                    }
                    0b101 => {
                        // BGE
                        if (self.regs[instr.rs1().unwrap() as usize] as i32) >=
                           (self.regs[instr.rs2().unwrap() as usize] as i32) {
                            self.pc = self.pc.wrapping_add(instr.immediate().unwrap());
                        }
                    }
                    0b110 => {
                        // BLTU
                        if self.regs[instr.rs1().unwrap() as usize] <
                           self.regs[instr.rs2().unwrap() as usize] {
                            self.pc = self.pc.wrapping_add(instr.immediate().unwrap());
                        }
                    }
                    0b111 => {
                        // BGEU
                        if self.regs[instr.rs1().unwrap() as usize] >=
                           self.regs[instr.rs2().unwrap() as usize] {
                            self.pc = self.pc.wrapping_add(instr.immediate().unwrap());
                        }
                    }
                    _ => unreachable!(),
                }
            }
            Opcode::Load => {
                let addr = self.regs[instr.rs1().unwrap() as usize]
                    .wrapping_add(instr.immediate().unwrap());
                match instr.funct3().unwrap() {
                    0b000 => {
                        // LB
                        self.regs[instr.rd().unwrap() as usize] =
                            self.ram[addr as usize] as i8 as i32 as u32;
                    }
                    0b001 => {
                        // LH
                        self.regs[instr.rd().unwrap() as usize] =
                            ((self.ram[addr as usize] as i16) |
                             (self.ram[(addr as usize) + 1] as i16) <<
                             8) as i32 as u32;
                    }
                    0b010 => {
                        // LW
                        self.regs[instr.rd().unwrap() as usize] =
                            (self.ram[addr as usize] as u32) |
                            (self.ram[(addr as usize) + 1] as u32) << 8 |
                            (self.ram[(addr as usize) + 2] as u32) << 16 |
                            (self.ram[(addr as usize) + 3] as u32) << 24;
                    }
                    0b100 => {
                        // LBU
                        self.regs[instr.rd().unwrap() as usize] = self.ram[addr as usize] as u32;
                    }
                    0b101 => {
                        // LHU
                        self.regs[instr.rd().unwrap() as usize] =
                            (self.ram[addr as usize] as u32) |
                            (self.ram[(addr as usize) + 1] as u32) << 8;
                    }
                    _ => unreachable!(),
                }
            }
            Opcode::Store => {
                let addr = self.regs[instr.rs1().unwrap() as usize]
                    .wrapping_add(instr.immediate().unwrap());
                match instr.funct3().unwrap() {
                    0b000 => {
                        // SB
                        self.ram[addr as usize] =
                            (self.regs[instr.rs2().unwrap() as usize] & 0xFF) as u8;
                        self.ram[(addr as usize) + 1] = 0;
                        self.ram[(addr as usize) + 2] = 0;
                        self.ram[(addr as usize) + 3] = 0;
                    }
                    0b001 => {
                        // SH
                        self.ram[addr as usize] =
                            (self.regs[instr.rs2().unwrap() as usize] & 0xFF) as u8;
                        self.ram[(addr as usize) + 1] =
                            (self.regs[instr.rs2().unwrap() as usize] & 0xFF00) as u8;
                        self.ram[(addr as usize) + 2] = 0;
                        self.ram[(addr as usize) + 3] = 0;
                    }
                    0b010 => {
                        // SW
                        self.ram[addr as usize] =
                            (self.regs[instr.rs2().unwrap() as usize] & 0xFF) as u8;
                        self.ram[(addr as usize) + 1] =
                            (self.regs[instr.rs2().unwrap() as usize] & 0xFF00) as u8;
                        self.ram[(addr as usize) + 2] =
                            (self.regs[instr.rs2().unwrap() as usize] & 0xFF0000) as u8;
                        self.ram[(addr as usize) + 3] =
                            (self.regs[instr.rs2().unwrap() as usize] & 0xFF000000) as u8;
                    }
                    _ => unreachable!(),
                }
            }
            Opcode::OpImm => {
                match instr.funct3().unwrap() {
                    0b000 => {
                        // ADDI
                        self.regs[instr.rd().unwrap() as usize] =
                            self.regs[instr.rs1().unwrap() as usize] + instr.immediate().unwrap();
                    }
                    0b010 => {
                        // SLTI
                        if (self.regs[instr.rs1().unwrap() as usize] as i32) <
                           (instr.immediate().unwrap() as i32) {
                            self.regs[instr.rd().unwrap() as usize] = 1;
                        } else {
                            self.regs[instr.rd().unwrap() as usize] = 0;
                        }
                    }
                    0b011 => {
                        // SLTIU
                        if self.regs[instr.rs1().unwrap() as usize] < instr.immediate().unwrap() {
                            self.regs[instr.rd().unwrap() as usize] = 1;
                        } else {
                            self.regs[instr.rd().unwrap() as usize] = 0;
                        }
                    }
                    0b100 => {
                        // XORI
                        self.regs[instr.rd().unwrap() as usize] =
                            self.regs[instr.rs1().unwrap() as usize] ^ instr.immediate().unwrap();
                    }
                    0b110 => {
                        // ORI
                        self.regs[instr.rd().unwrap() as usize] =
                            self.regs[instr.rs1().unwrap() as usize] | instr.immediate().unwrap();
                    }
                    0b111 => {
                        // ANDI
                        self.regs[instr.rd().unwrap() as usize] =
                            self.regs[instr.rs1().unwrap() as usize] & instr.immediate().unwrap();
                    }
                    0b001 => {
                        // SLLI
                        self.regs[instr.rd().unwrap() as usize] =
                            self.regs[instr.rs1().unwrap() as usize] << (instr.immediate().unwrap() & 0x1F);
                    }
                    0b101 => {
                        if instr.immediate().unwrap() & 0x400 == 0 {
                            // SRLI
                            self.regs[instr.rd().unwrap() as usize] =
                                self.regs[instr.rs1().unwrap() as usize] >> (instr.immediate().unwrap() & 0x1F);
                        } else {
                            // SRAI
                            self.regs[instr.rd().unwrap() as usize] =
                                ((self.regs[instr.rs1().unwrap() as usize] as i32) >> (instr.immediate().unwrap() & 0x1F)) as u32;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            Opcode::Op => {
                let funct3 = instr.funct3().unwrap();
                let funct7 = instr.funct7().unwrap();
                match (funct7, funct3) {
                    (0b0000000, 0b000) => {
                        // ADD
                        let rs1 = self.get_register(instr.rs1().unwrap());
                        let rs2 = self.get_register(instr.rs2().unwrap());
                        self.set_register(
                            instr.rd().unwrap(),
                            rs1.wrapping_add(rs2)
                        );
                    }
                    _ => unreachable!(),
                } 
            }
            _ => unimplemented!(),
        }

        self.regs[0] = 0;
        self.pc = self.pc.wrapping_add(4);
    }

    fn get_instruction(&self) -> Instruction {
        let instr_val = (self.ram[self.pc as usize] as u32) |
                        (self.ram[(self.pc + 1) as usize] as u32) << 8 |
                        (self.ram[(self.pc + 2) as usize] as u32) << 16 |
                        (self.ram[(self.pc + 3) as usize] as u32) << 24;

        let opcode_val = (instr_val & 0x7F) as u8;
        Opcode::from_u8(opcode_val).unwrap().parse_instruction(instr_val)
    }

    fn get_register(&self, reg: u8) -> u32 {
        self.regs[reg as usize]
    }

    fn set_register(&mut self, reg: u8, value: u32) {
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
