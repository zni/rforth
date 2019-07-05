use std::env;
use std::process;

use rforth::vm;

fn main() {
    let mut machine = vm::machine::Machine::new();
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("usage: rforth <file>");
        process::exit(1);
    } else if args.len() == 2 {
        rforth::run_file(&mut machine, &args[1]).unwrap_or_else(|_err| {
            println!("Failed to run file.");
            process::exit(1);
        });
    } else {
        rforth::run_prompt(&mut machine).unwrap_or_else(|_err| {
            println!("Failed to run prompt.");
            process::exit(1);
        });
    }
}

