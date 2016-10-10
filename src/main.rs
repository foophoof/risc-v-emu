extern crate elf;

mod cpu;
mod instruction;
mod ram;

use cpu::CPU;
use ram::RAM;

// use std::fs::File;
use std::env;

fn load_elf_to_ram(path: &str, ram: &mut RAM) -> u32 {
    let elf_file = match elf::File::open_path(path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    for section in elf_file.sections {
        for (i, &data) in section.data.as_slice().iter().enumerate() {
            let addr = (section.shdr.addr as u32) + (i as u32);
            ram[addr] = data;
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
