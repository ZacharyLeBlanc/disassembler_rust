extern crate disassembler;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename).expect("Error: file not found");
    let reader = BufReader::new(f);
    let mut i = 0;

    for line in reader.lines() {
        let input = line.unwrap();
        let input_slice: &str = &input[..];
        disassembler::disassemble::disassemble_mips(input_slice, i);
        i += 1;
    }
}
