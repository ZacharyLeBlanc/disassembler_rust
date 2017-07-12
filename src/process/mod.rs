/// Module Process
/// This module contains the helper functions for the disassembler program.
/// Right now all of the functions are public but may change in the future to
/// change accessibility within the crate.
///
/// AUTHOR: Zach LeBlanc
/// DATE: 2017-6-15

/// This function takes a string of 0 and 1 and returns the decimal value for
/// the given numbers.
///
/// Parameters:
/// *String: string of 0 and 1.
/// *begin: the index where it will begin to read the numbers.
/// *end: the index where the reading of numbers will end.
/// Returns: integer of the decimal value of the binary input.
pub fn bin_to_dec(string: &str, begin: i32, end: i32) -> i32 {
    let mut decimal = 0;
    let mut pow = 1;

    for i in (begin..end + 1).rev() {
        if string.chars().nth(i as usize).unwrap() == '1' {
            decimal += pow;
        }
        pow *= 2;
    }
    return decimal;
}

/// This function checks to make sure the instruction is 32 characters long and
/// that it only contains 0s and 1s.
///
/// Parameters:
/// *instruction: the string to be verified.
/// *linenum: the number of the line that is being read.
/// Returns: boolean true if MIPS instruction is valid, false if MIPS instruction is invalid.
pub fn verify_mips_instruction(instruction: &str, linenum: i32) -> bool {
    const INSTRUCTION_LENGTH: usize = 32;
    let length = instruction.len();

    if length != INSTRUCTION_LENGTH {
        println!("ERROR - Line {} does not have 32 characters.", linenum);
        return false;
    }

    for i in 0..length {
        if instruction.chars().nth(i as usize).unwrap() != '1' &&
           instruction.chars().nth(i as usize).unwrap() != '0' {
            println!("ERROR - Line {} has a character that is not 0 or 1",
                     linenum);
            return false;
        }
    }
    return true;
}

/// This function gets the name of the register.
/// Parameters:
/// *reg_number: number of the register you wish to return.
/// Returns: string of the name of the register.
pub fn get_reg_name(reg_number: usize) -> String {
    let reg_array = ["$zero", "$at", "$v0", "$v1", "$a0", "$a1", "$a2", "$a3", "$t0", "$t1",
                     "$t2", "$t3", "$t4", "$t5", "$t6", "$t7", "$s0", "$s1", "$s2", "$s3", "$s4",
                     "$s5", "$s6", "$s7", "$t8", "$t9", "$k0", "$k1", "$gp", "$sp", "$fp", "$ra"];
    if reg_number <= 32 {
        return String::from(reg_array[reg_number]);
    } else {
        return String::from("ERR");
    }
}

/// This function returns the decimal value of the operation code.
/// Parameters:
/// *string: the instruction in MIPS format.
/// Returns: integer of the op code.
pub fn get_op_code(string: &str) -> i32 {
    return bin_to_dec(string, 0, 5);
}

/// This function returns the number of the register.
/// reg 1 = rs, reg 2 = rt, reg 3 = rd
// Parameters:
/// *string: the instruction.
/// *reg the register you want to receive the value of.
/// Returns: integer of the register number.
pub fn get_reg_number(string: &str, reg: i32) -> i32 {
    return bin_to_dec(string, ((5 * reg) + 1), (5 * (reg + 1)));
}

/// This function gets the type of instruction.
/// Usually need get_op_code to pass as parameter.
/// Parameters:
/// *op_code: the operation code from the instruction (first six bits of the instruction).
/// Returns: string of type of instruction.
pub fn get_instruction_type(op_code: i32) -> String {
    if op_code == 0 {
        return String::from("R");
    } else if op_code == 2 || op_code == 3 {
        return String::from("J");
    } else {
        return String::from("I");
    }
}

// To make submodules.
pub mod processj;
pub mod processr;
pub mod processi;