use process;

pub fn process_j_format(string: &str) -> () {
    let op_code = process::get_op_code(string);
    let address = process::bin_to_dec(string, 6, 31);

    if op_code == 2 {
        println!("j\t{}", address * 4);
        return;
    } else if op_code == 3 {
        println!("jal\t{}", address * 4);
        return;
    } else {
        println!("Error: OP code was not valid.");
        return;
    }
}
