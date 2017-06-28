// Module Disassemble
// This module is the main function of this program.
// This module contains the function which will be called to run the program.
//
// AUTHOR: Zach LeBLanc
// DATE: 2017-6-15

use process;

pub fn disassemble_mips(string: &str, line_number: i32) -> () {
    if process::verify_mips_instruction(string, line_number) == true {
        let instruction_type = process::get_instruction_type(process::get_op_code(string));

        if instruction_type == "R" {
            process::processr::process_r_format(string, line_number);
            return;
        } else if instruction_type == "I" {
            process::processi::process_i_format(string, line_number);
            return;
        } else if instruction_type == "J" {
            process::processj::process_j_format(string, line_number);
            return;
        } else {
            println!("Error line {}: instruction type was not valid.",
                     line_number);
            return;
        }
    }
}
