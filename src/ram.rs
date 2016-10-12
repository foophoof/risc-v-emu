// Copyright 2016 risc-v-emulator Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::ops;

pub struct RAM {
    data: Vec<u8>,
}

impl RAM {
    pub fn new(capacity: usize) -> RAM {
        RAM { data: vec![0; capacity] }
    }

    pub fn get_u16(&self, index: u32) -> u16 {
        (self[index] as u16) | (self[index + 1] as u16) << 8
    }

    pub fn get_u32(&self, index: u32) -> u32 {
        (self[index] as u32) | (self[index + 1] as u32) << 8 | (self[index + 2] as u32) << 16 |
        (self[index + 3] as u32) << 24
    }

    pub fn set_u16(&mut self, index: u32, data: u16) {
        self[index] = (data & 0x00FF) as u8;
        self[index + 1] = ((data & 0xFF00) >> 8) as u8;
    }

    pub fn set_u32(&mut self, index: u32, data: u32) {
        self[index] = (data & 0x000000FF) as u8;
        self[index + 1] = ((data & 0x0000FF00) >> 8) as u8;
        self[index + 2] = ((data & 0x00FF0000) >> 16) as u8;
        self[index + 3] = ((data & 0xFF000000) >> 24) as u8;
    }
}

impl ops::Index<u32> for RAM {
    type Output = u8;

    fn index(&self, index: u32) -> &u8 {
        self.data.get(index as usize).unwrap()
    }
}

impl ops::IndexMut<u32> for RAM {
    fn index_mut(&mut self, index: u32) -> &mut u8 {
        self.data.get_mut(index as usize).unwrap()
    }
}
