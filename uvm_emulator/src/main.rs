use std::{env::args, process::exit};

use uvm::core::UVM;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 2 {
        let mut vm = UVM::new();
        vm.run(&args[1]);
    } else {
        eprintln!(
            "
Program: UVM

Usage:
    <source_path>: executes the (given) file.
        "
        );
        exit(1);
    }
}
