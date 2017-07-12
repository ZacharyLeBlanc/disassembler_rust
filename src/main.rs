/// This program takes a MIPS instruction and reads it from a file. For each line, the program calls disassemble.
///
/// Usage: cargo run [filename]
/// To compile run cargo build, to run the program run cargo run (this will compile and run the program).
/// where filename is the name of the file you wish to translate from MIPS to assembly.
/// this argument is optional.
///
/// Input:
///     The program reads its input from a file passed in as a parameter
///     on the command line, or it reads from the standard input.
///
/// Output:
///     For each valid line, the program prints the mips instruction.
///     For each invalid line, the program prints an error message saying what the error was and
///     on what line it occurred.
///
/// AUTHOR: Zach LeBlanc
/// DATE: 2017-7-12

extern crate disassembler;
use std::env;
use std::fs::File;
use std::io::*;
use std::io;

fn main() {
    //Get arguments from the command line
    let args: Vec<String> = env::args().collect();

    // if there is an argument process it
    // only argument that should be passed is a filename
    if args.len() > 1 {
        let filename = &args[1];
        let f = File::open(filename).expect("Error: file not found.");
        let reader = BufReader::new(f);
        let mut i = 0;

        for line in reader.lines() {
            let input = line.unwrap(); //have to unwrap it
            let input_slice: &str = &input[..]; // disassemble_mips takes an input slice as argument, this creates the slice for the input
            disassembler::disassemble::disassemble_mips(input_slice, i);
            i += 1;
        }
    } else {
        // no arguments passed
        let mut counter = 1;
        println!("Enter a 32 bit MIPS instruction:\nType exit to stop the program (or control-c on Unix operating systems):",);
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let output = input.trim(); // need to trim input or it will not be able to find the file
            if output.to_lowercase() == "exit" {
                break;
            }
            disassembler::disassemble::disassemble_mips(output, counter);
            counter += 1;
        }
    }
}
