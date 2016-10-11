// Copyright 2016 risc-v-emu Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate elf;

mod cpu;
mod instruction;
mod ram;

use cpu::CPU;
use ram::RAM;

use std::cmp::max;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;
use std::env;

fn load_elf_to_ram(path: &str, ram: &mut RAM) -> u32 {
    let elf_file = match elf::File::open_path(path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    let mut raw_file = File::open(path).expect("couldn't open file");

    for program_header in elf_file.phdrs {
        if program_header.progtype == elf::types::ProgType(1) {
            let mut buf =
                vec![0; max(program_header.memsz as usize, program_header.filesz as usize)];

            for i in 0..(program_header.memsz) {
                buf[i as usize] = 0;
            }

            raw_file.seek(SeekFrom::Start(program_header.offset)).expect("couldn't seek in file");
            raw_file.read(&mut buf[0..(program_header.filesz as usize)])
                .expect("couldn't read file");

            for (i, &data) in buf.iter().enumerate() {
                let addr = (program_header.vaddr as u32) + (i as u32);
                ram[addr] = data;
            }
        }
    }

    elf_file.ehdr.entry as u32
}

fn main() {
    let mut ram = RAM::new(1024 * 1024);

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} program-name", args[0]);
        return;
    }
    let entry_point = load_elf_to_ram(args[1].as_str(), &mut ram);

    let mut cpu = CPU::new(ram);
    cpu.run(entry_point);
}
