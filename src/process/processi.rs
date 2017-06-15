use process;

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
    if i_names[op_code] != "" {
        return String::from(i_names[op_code]);
    } else {
        return String::from("ERR");
    }
}
