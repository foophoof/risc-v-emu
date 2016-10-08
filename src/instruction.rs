use opcode::Opcode;

#[derive(Debug)]
pub enum Instruction {
    R(u32),
    I(u32),
    S(u32),
    SB(u32),
    U(u32),
    UJ(u32),
}

impl Instruction {
    pub fn opcode(&self) -> Option<Opcode> {
        let opcode_val = (self.data() & 0x7F) as u8;
        Opcode::from_u8(opcode_val)
    }

    pub fn immediate(&self) -> Option<u32> {
        match *self {
            Instruction::R(_) => None,
            Instruction::I(data) => {
                let immediate = data & 0xFFF00000;
                Some(((immediate as i32) >> 20) as u32)
            }
            Instruction::S(data) => {
                let immediate_high = data & 0xFE000000;
                let immediate_low = data & 0xF80;

                let immediate_high_shifted = ((immediate_high as i32) >> 20) as u32;
                let immediate_low_shifted = immediate_low >> 7;

                Some(immediate_high_shifted | immediate_low_shifted)
            }
            Instruction::SB(data) => {
                let sign_extension = (((data & 0x80000000) as i32) >> 19) as u32;
                let high = (data & 0x80) << 4;
                let mid = (data & 0x7E000000) >> 20;
                let low = (data & 0xF00) >> 7;

                Some(sign_extension | high | mid | low)
            }
            Instruction::U(data) => Some(data & 0xFFFFF000),
            Instruction::UJ(data) => {
                let sign_extension = (((data & 0x80000000) as i32) >> 11) as u32;
                let high = data & 0xFF000;
                let mid = (data & 0x100000) >> 9;
                let low = (data & 0x7FE00000) >> 20;

                Some(sign_extension | high | mid | low)
            }
        }
    }

    pub fn rs1(&self) -> Option<u8> {
        use instruction::Instruction::*;
        match *self {
            R(data) | I(data) | S(data) | SB(data) => Some(((data & 0xF8000) >> 15) as u8),
            _ => None,
        }
    }

    pub fn rs2(&self) -> Option<u8> {
        use instruction::Instruction::*;
        match *self {
            R(data) | S(data) | SB(data) => Some(((data & 0x1F00000) >> 20) as u8),
            _ => None,
        }
    }

    pub fn rd(&self) -> Option<u8> {
        use instruction::Instruction::*;
        match *self {
            R(data) | I(data) | U(data) | UJ(data) => Some(((data & 0xF80) >> 7) as u8),
            _ => None,
        }
    }

    pub fn funct3(&self) -> Option<u8> {
        use instruction::Instruction::*;
        match *self {
            R(data) | I(data) | S(data) | SB(data) => Some(((data & 0x7000) >> 12) as u8),
            _ => None,
        }
    }

    pub fn funct7(&self) -> Option<u8> {
        use instruction::Instruction::*;
        match *self {
            R(data) => Some(((data & 0xFE000000) >> 25) as u8),
            _ => None,
        }
    }

    fn data(&self) -> u32 {
        use instruction::Instruction::*;
        match *self {
            R(data) | I(data) | S(data) | SB(data) | U(data) | UJ(data) => data,
        }
    }
}
