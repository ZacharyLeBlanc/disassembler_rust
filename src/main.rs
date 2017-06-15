extern crate disassembler;

fn main() {
    println!("hello");
    disassembler::process::processj::process_j_format("00001100000000000000100111000100");
}
