/// Module ProcessJ
/// This module contains the functions for Processing a J format instruction.
///
/// AUTHOR: Zach LeBlanc
/// DATE: 2017-6-14

use process;

/// This function processes the J format instructions and prints the assembly
/// code for J instructions.
/// Parameters:
/// *string: the instruction to be translated.
/// Returns: void.
pub fn process_j_format(string: &str, line_number: i32) -> () {
    let op_code = process::get_op_code(string);
    let address = process::bin_to_dec(string, 6, 31);

    if op_code == 2 {
        println!("j\t{}", address * 4);
        return;
    } else if op_code == 3 {
        println!("jal\t{}", address * 4);
        return;
    } else {
        println!("Error line {}: OP code was not valid.", line_number);
        return;
    }
}
