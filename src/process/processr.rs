use process;

pub fn get_shitft_amount(string: &str) -> i32 {
    return process::bin_to_dec(string, 21, 25);
}

pub fn get_function_code(string: &str) -> i32 {
    return process::bin_to_dec(string, 26, 31);
}

pub fn get_r_function_name(function_code: usize) -> String {
    /* Create a static (persistent) array of the mnemonic names,
     **    each of which is a string (char *).
     */
    let mut rfunc_names: [&str; 44] = [""; 44];
    rfunc_names[0] = "sll";
    rfunc_names[2] = "srl";
    rfunc_names[8] = "jr";
    rfunc_names[32] = "add";
    rfunc_names[33] = "addu";
    rfunc_names[34] = "sub";
    rfunc_names[35] = "subu";
    rfunc_names[36] = "and";
    rfunc_names[37] = "or";
    rfunc_names[39] = "nor";
    rfunc_names[42] = "slt";
    rfunc_names[43] = "sltu";
    // returns the mnemonic name for the function if it is not null
    // TODO Do something about null index in array
    if rfunc_names[function_code] != "" {
        return String::from(rfunc_names[function_code]);
    } else {
        return String::from("ERR");
    }
}