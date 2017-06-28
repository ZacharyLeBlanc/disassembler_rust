// Module ProcessI
// This module contains the helper functions for the I format instructions.
//
// AUTHOR: Zach LeBlanc
// DATE: 2017-6-2014

use process;

// This function gets the name of the I format instructions from the op code.
// Parameters:
// *op_code: number of the op code (first six bits in instruction).
// Returns: string of the name of the instruction.
pub fn get_i_names(op_code: usize) -> String {
    /* Create a static (persistent) array of the mnemonic names,
     **    each of which is a string (char *).
     */
    let mut i_names: [&str; 44] = [""; 44];
    i_names[4] = "beq";
    i_names[5] = "bne";
    i_names[8] = "addi";
    i_names[9] = "addiu";
    i_names[10] = "slti";
    i_names[11] = "sltiu";
    i_names[12] = "andi";
    i_names[13] = "ori";
    i_names[15] = "lui";
    i_names[35] = "lw";
    i_names[43] = "sw";
    // returns the mnemonic name for the function if it is not null
    if op_code > 44 {
        return String::from("ERR");
    }
    if i_names[op_code] != "" {
        return String::from(i_names[op_code]);
    } else {
        return String::from("ERR");
    }
}

pub fn process_i_format(string: &str, line_number: i32) -> () {
    let op_code = process::get_op_code(string);
    let rs = process::get_reg_name(process::get_reg_number(string, 1) as usize);
    let rt = process::get_reg_name(process::get_reg_number(string, 2) as usize);
    let constant = process::bin_to_dec(string, 16, 31);
    let instruction_name = get_i_names(op_code as usize);

    if instruction_name == "ERR" {
        println!("Error line {}: OP code was not a vaild I format instruction.",
                 line_number);
        return;
    }

    if rs == "ERR" || rt == "ERR" {
        println!("Error line {}: something went wrong with the registers.",
                 line_number);
        return;
    }

    if op_code == 4 || op_code == 5 {
        println!("{}\t{}, {}, {}", instruction_name, rs, rt, constant * 4);
        return;
    } else if op_code <= 13 && op_code > 5 {
        println!("{}\t{}, {}, {}", instruction_name, rt, rs, constant);
        return;
    } else if op_code >= 35 {
        println!("{}\t{}, {}({})", instruction_name, rt, constant, rs);
        return;
    } else if op_code == 15 {
        println!("{}\t{}, {}", instruction_name, rt, constant);
        return;
    } else {
        println!("Error line {}: op code was not valid.", line_number);
        return;
    }
}
