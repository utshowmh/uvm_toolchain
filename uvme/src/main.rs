use std::{env::args, process::exit};

use uvm::core::UVM;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 2 {
        let mut vm = UVM::new();
        vm.run(&args[1]);
    } else if args.len() == 3 {
        let limit = match args[2].parse() {
            Ok(limit) => limit,
            Err(_) => {
                print_help();
                exit(1);
            }
        };
        let mut vm = UVM::new();
        vm.emulate(&args[1], limit);
    } else {
        print_help();
        exit(1);
    }
}

fn print_help() {
    eprintln!(
        "
Program: UVM

Usage:
<source_path>           : executes the (given) file.
<source_path> <limit>   : executes the (given) file with specified execution limit.
    "
    );
}
