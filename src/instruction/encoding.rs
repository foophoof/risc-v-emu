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

pub fn get_rd(instruction: u32) -> u8 {
    ((instruction & 0xF80) >> 7) as u8
}

pub fn get_rs1(instruction: u32) -> u8 {
    ((instruction & 0xF8000) >> 15) as u8
}

pub fn get_rs2(instruction: u32) -> u8 {
    ((instruction & 0x1F00000) >> 20) as u8
}

pub fn get_funct3(instruction: u32) -> u8 {
    ((instruction & 0x7000) >> 12) as u8
}

pub fn get_funct7(instruction: u32) -> u8 {
    ((instruction & 0xFE000000) >> 25) as u8
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
}

impl U {
    pub fn parse(instruction: u32) -> U {
        U {
            opcode: get_opcode(instruction),
            rd: get_rd(instruction),
            immediate: (instruction & 0xFFFFF000) as i32,
        }
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
}
