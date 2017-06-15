extern crate disassembler;

fn main() {
    disassembler::process::processj::process_j_format("00001100000000000000100111000100");
    disassembler::process::processr::process_r_format("00000010010010000100000000100000");
    disassembler::disassemble::disassemble_mips("00001100000000000000100111000100", 1);
    disassembler::disassemble::disassemble_mips("00000010010010000100000000100000", 2);
}
