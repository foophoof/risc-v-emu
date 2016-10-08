use instruction::Instruction;

#[derive(Debug)]
pub enum Opcode {
    Load,
    LoadFP,
    MiscMem,
    OpImm,
    Auipc,
    OpImm32,
    Store,
    StoreFP,
    Amo,
    Op,
    Lui,
    Op32,
    Madd,
    Msub,
    Nmsub,
    Nmadd,
    OpFp,
    Branch,
    Jalr,
    Jal,
    System,
}

impl Opcode {
    pub fn from_u8(opcode: u8) -> Option<Opcode> {
        match opcode {
            0b0000011 => Some(Opcode::Load),
            0b0000111 => Some(Opcode::LoadFP),
            0b0001111 => Some(Opcode::MiscMem),
            0b0010011 => Some(Opcode::OpImm),
            0b0010111 => Some(Opcode::Auipc),
            0b0011011 => Some(Opcode::OpImm32),
            0b0100011 => Some(Opcode::Store),
            0b0100111 => Some(Opcode::StoreFP),
            0b0101111 => Some(Opcode::Amo),
            0b0110011 => Some(Opcode::Op),
            0b0110111 => Some(Opcode::Lui),
            0b0111011 => Some(Opcode::Op32),
            0b1000011 => Some(Opcode::Madd),
            0b1000111 => Some(Opcode::Msub),
            0b1001011 => Some(Opcode::Nmsub),
            0b1001111 => Some(Opcode::Nmadd),
            0b1010011 => Some(Opcode::OpFp),
            0b1100011 => Some(Opcode::Branch),
            0b1100111 => Some(Opcode::Jalr),
            0b1101111 => Some(Opcode::Jal),
            0b1110011 => Some(Opcode::System),
            _ => None,
        }
    }

    pub fn parse_instruction(&self, instruction: u32) -> Instruction {
        use opcode::Opcode::*;
        match *self {
            OpImm | OpImm32 | Jalr | Load | LoadFP | MiscMem | System => {
                Instruction::I(instruction)
            }
            Lui | Auipc => Instruction::U(instruction),
            Op | Op32 | OpFp | Amo | Madd | Msub | Nmadd | Nmsub => Instruction::R(instruction),
            Jal => Instruction::UJ(instruction),
            Branch => Instruction::SB(instruction),
            Store | StoreFP => Instruction::S(instruction),
        }
    }
}
