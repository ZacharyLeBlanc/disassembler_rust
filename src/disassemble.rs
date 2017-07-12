/// This module is used as the final step to disassemble the MIPS instruction.
/// AUTHOR: Zach LeBlanc
/// DATE: 2017-7-12

use process;

/// Prints out the instruction in assembly code
///
///  # Arguments
///
///  * 'string' - A string slice that holds the instruction in binary format
///  * 'line_number' - A 32 bit int that represents the line number when the code is being executed
///
///  # Example
///
///  '''
///  disassembler::disassemble::disassemble_mips(input, i);
///  '''
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
