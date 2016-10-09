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
        let mut regs = [0; 32];
        regs[2] = 1020 * 1024; // Stack Pointer

        CPU {
            regs: regs,
            pc: 0,
            ram: ram,
        }
    }

    pub fn run(&mut self, entry_point: u32) {
        // ADDI sp,sp,-16
        self.execute(Instruction::from_u32(0xFF010113).unwrap());
        // SW ra,12(sp)
        self.execute(Instruction::from_u32(0x00112623).unwrap());
        // SW s0,8(sp)
        self.execute(Instruction::from_u32(0x00812423).unwrap());
        // ADDI s0,sp,16
        self.execute(Instruction::from_u32(0x01010413).unwrap());

        self.regs[1] = self.pc;
        self.pc = entry_point;


        let mut instr = self.get_instruction();

        while instr.is_some() {
            self.execute(instr.unwrap());
            instr = self.get_instruction();
        }
    }

    fn execute(&mut self, instr: Instruction) {
        // print!("{:08x}: {:08x} (", self.pc, instr.data());
        // for reg in &self.regs {
        //     print!("{:08x}, ", reg);
        // }
        // println!(")");

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
                self.regs[instr.rd().unwrap() as usize] = self.pc.wrapping_add(4);
                let imm = instr.immediate().unwrap();
                let dest = self.pc.wrapping_add(imm);
                self.pc = dest.wrapping_sub(4);
            }
            Opcode::Jalr => {
                self.regs[instr.rd().unwrap() as usize] = self.pc.wrapping_add(4);
                let imm = instr.immediate().unwrap();
                let rs1 = self.regs[instr.rs1().unwrap() as usize];
                let dest = imm.wrapping_add(rs1) & 0xFFFFFFFE;
                self.pc = dest.wrapping_sub(4);
            }
            Opcode::Branch => {
                match instr.funct3().unwrap() {
                    0b000 => {
                        // BEQ
                        if self.regs[instr.rs1().unwrap() as usize] ==
                           self.regs[instr.rs2().unwrap() as usize] {
                            self.pc = self.pc.wrapping_add(instr.immediate().unwrap()).wrapping_sub(4);
                        }
                    }
                    0b001 => {
                        // BNE
                        if self.regs[instr.rs1().unwrap() as usize] !=
                           self.regs[instr.rs2().unwrap() as usize] {
                            self.pc = self.pc.wrapping_add(instr.immediate().unwrap()).wrapping_sub(4);
                        }
                    }
                    0b100 => {
                        // BLT
                        if (self.regs[instr.rs1().unwrap() as usize] as i32) <
                           (self.regs[instr.rs2().unwrap() as usize] as i32) {
                            self.pc = self.pc.wrapping_add(instr.immediate().unwrap()).wrapping_sub(4);
                        }
                    }
                    0b101 => {
                        // BGE
                        if (self.regs[instr.rs1().unwrap() as usize] as i32) >=
                           (self.regs[instr.rs2().unwrap() as usize] as i32) {
                            self.pc = self.pc.wrapping_add(instr.immediate().unwrap()).wrapping_sub(4);
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
                let rs2 = self.regs[instr.rs2().unwrap() as usize];
                match instr.funct3().unwrap() {
                    0b000 => {
                        // SB
                        self.ram[addr as usize] = (rs2 & 0xFF) as u8;
                        self.ram[(addr as usize) + 1] = 0;
                        self.ram[(addr as usize) + 2] = 0;
                        self.ram[(addr as usize) + 3] = 0;
                    }
                    0b001 => {
                        // SH
                        self.ram[addr as usize] = (rs2 & 0xFF) as u8;
                        self.ram[(addr as usize) + 1] = ((rs2 & 0xFF00) >> 8) as u8;
                        self.ram[(addr as usize) + 2] = 0;
                        self.ram[(addr as usize) + 3] = 0;
                    }
                    0b010 => {
                        // SW
                        self.ram[addr as usize] = (rs2 & 0xFF) as u8;
                        self.ram[(addr as usize) + 1] = ((rs2 & 0xFF00) >> 8) as u8;
                        self.ram[(addr as usize) + 2] = ((rs2 & 0xFF0000) >> 16) as u8;
                        self.ram[(addr as usize) + 3] = ((rs2 & 0xFF000000) >> 24) as u8;
                    }
                    _ => unreachable!(),
                }
            }
            Opcode::OpImm => {
                match instr.funct3().unwrap() {
                    0b000 => {
                        // ADDI
                        self.regs[instr.rd().unwrap() as usize] =
                            self.regs[instr.rs1().unwrap() as usize].wrapping_add(instr.immediate().unwrap());
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
                let rs1 = self.get_register(instr.rs1().unwrap());
                let rs2 = self.get_register(instr.rs2().unwrap());
                match (funct7, funct3) {
                    (0b0000000, 0b000) => {
                        // ADD
                        self.set_register(
                            instr.rd().unwrap(),
                            rs1.wrapping_add(rs2)
                        );
                    }
                    (0b0100000, 0b000) => {
                        // SUB
                        self.set_register(
                            instr.rd().unwrap(),
                            rs1.wrapping_sub(rs2)
                        );
                    }
                    (0b0000000, 0b001) => {
                        // SLL
                        self.set_register(
                            instr.rd().unwrap(),
                            rs1 << rs2
                        );
                    }
                    (0b0000000, 0b010) => {
                        // SLT
                        self.set_register(
                            instr.rd().unwrap(),
                            if (rs1 as i32) < (rs2 as i32) { 1 } else { 0 }
                        );
                    }
                    (0b0000000, 0b011) => {
                        // SLTU
                        self.set_register(
                            instr.rd().unwrap(),
                            if rs1 < rs2 { 1 } else { 0 }
                        );
                    }
                    (0b0000000, 0b100) => {
                        // XOR
                        self.set_register(
                            instr.rd().unwrap(),
                            rs1 ^ rs2
                        );
                    }
                    (0b0000000, 0b101) => {
                        // SRL
                        self.set_register(
                            instr.rd().unwrap(),
                            rs1 >> rs2
                        );
                    }
                    (0b0100000, 0b101) => {
                        // SRA
                        self.set_register(
                            instr.rd().unwrap(),
                            ((rs1 as i32) >> rs2) as u32
                        );
                    }
                    (0b0000000, 0b110) => {
                        // OR
                        self.set_register(
                            instr.rd().unwrap(),
                            rs1 | rs2
                        );
                    }
                    (0b0000000, 0b111) => {
                        // AND
                        self.set_register(
                            instr.rd().unwrap(),
                            rs1 & rs2
                        );
                    }
                    (0b0000001, 0b000) => {
                        // MUL
                        self.set_register(
                            instr.rd().unwrap(),
                            ((rs1 as u64) * (rs2 as u64)) as u32
                        );
                    }
                    (0b0000001, 0b001) => {
                        // MULH
                        self.set_register(
                            instr.rd().unwrap(),
                            (((rs1 as i32 as i64) * (rs2 as i32 as i64)) >> 32) as u32
                        );
                    }
                    (0b0000001, 0b010) => {
                        // MULHSU
                        self.set_register(
                            instr.rd().unwrap(),
                            (((rs1 as i32 as i64) * (rs2 as i64)) >> 32) as u32
                        );
                    }
                    (0b0000001, 0b011) => {
                        // MULHU
                        self.set_register(
                            instr.rd().unwrap(),
                            (((rs1 as u64) * (rs2 as u64)) >> 32) as u32
                        );
                    }
                    _ => panic!("op unimplemented: {:07b} {:03b}", funct7, funct3),
                } 
            }
            Opcode::MiscMem => {
                let funct3 = instr.funct3().unwrap();
                match funct3 {
                    0b000 => {
                        // FENCE
                        unimplemented!();
                    }
                    0b001 => {
                        // FENCE.I
                        unimplemented!();
                    }
                    _ => unreachable!(),
                }
            }
            Opcode::System => {
                let funct12 = instr.immediate().unwrap();
                let funct3 = instr.funct3().unwrap();
                match (funct12, funct3) {
                    (0, _) => {
                        match self.regs[10] {
                            0 => {
                                // write()
                                let output = self.regs[11];
                                assert_eq!(output, 0);
                                let mut ptr = self.regs[12];
                                let mut len = self.regs[13];
                                while len > 0 {
                                    print!("{}", self.ram[ptr as usize] as char);
                                    ptr += 1;
                                    len -= 1;
                                }
                            }
                            _ => panic!("unknown syscall {}", self.regs[10]),
                        }
                    }
                    (1, _) => {
                        panic!("ebreak!");
                    }
                    _ => unreachable!(),
                }
            }
            _ => unimplemented!(),
        }

        self.regs[0] = 0;
        self.pc = self.pc.wrapping_add(4);
    }

    fn get_instruction(&self) -> Option<Instruction> {
        let instr_val = (self.ram[self.pc as usize] as u32) |
                        (self.ram[(self.pc + 1) as usize] as u32) << 8 |
                        (self.ram[(self.pc + 2) as usize] as u32) << 16 |
                        (self.ram[(self.pc + 3) as usize] as u32) << 24;

        Instruction::from_u32(instr_val)
    }

    pub fn get_register(&self, reg: u8) -> u32 {
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
