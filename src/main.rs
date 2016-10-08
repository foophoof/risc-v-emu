extern crate elf;

mod cpu;
mod opcode;
mod instruction;

use cpu::CPU;

// use std::fs::File;
use std::env;

fn load_elf_to_ram(path: &str, ram: &mut Vec<u8>) {
    let elf_file = match elf::File::open_path(path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    let text_section = elf_file.get_section(".text").unwrap();
    for (i, &data) in text_section.data.as_slice().iter().enumerate() {
        ram[(text_section.shdr.addr as usize) + i] = data;
    }
}

fn main() {
    let mut ram: Vec<u8> = vec![0; 4 * 1024];

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} program-name", args[0]);
        return;
    }
    load_elf_to_ram(args[1].as_str(), &mut ram);

    let mut cpu = CPU::new(ram);
    println!("cpu before: {:?}", cpu);

    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();
    cpu.step();

    println!("cpu after: {:?}", cpu);
}
