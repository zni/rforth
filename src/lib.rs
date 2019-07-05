use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub mod vm;

pub fn run_file(machine: &mut vm::machine::Machine, file: &String) -> Result<(), std::io::Error> {
    let path = Path::new(file);
    let mut file = File::open(&path)?;

    let mut source = String::new();
    file.read_to_string(&mut source)?;

    run(machine, &source);
    Ok(())
}

pub fn run_prompt(machine: &mut vm::machine::Machine) -> Result<(), std::io::Error> {

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        run(machine, &line);
    }
}

fn run(machine: &mut vm::machine::Machine, line: &String) {
    let input = tokenize_input(line);

    let mut had_error: bool = false;
    match machine.execute(&input) {
        Ok(_) => (),
        Err(vm::ErrorType::WordNotFound) => {
            had_error = true;
        },
        Err(vm::ErrorType::StackUnderflow) => {
            println!("stack underflow");
            had_error = true;
        },
        Err(vm::ErrorType::CompilationError) => {
            println!("compilation error");
            had_error = true;
        },
        Err(vm::ErrorType::OutsideCompileMode) => {
            println!("compile operator used outside compile mode");
            had_error = true;
        },
        Err(vm::ErrorType::InvalidOffset) => {
            println!("invalid offset");
            had_error = true;
        },
        Err(vm::ErrorType::BranchOutOfBounds) => {
            println!("branch out of bounds");
            had_error = true;
        },
        Err(vm::ErrorType::UnbalancedControl) => {
            println!("unbalanced control structure");
            had_error = true;
        }
    }

    if !had_error {
        println!("ok");
    }
}

fn tokenize_input(line: &String) -> Vec<vm::Value> {
    let words = line.split_whitespace();
    let mut input: Vec<vm::Value> = Vec::new();
    for word in words {
        let token = match word.parse::<i32>() {
            Ok(num) => {
                vm::Value::Number(num)
            },
            Err(_) => {
                vm::Value::Word(word.to_string())
            }
        };

        input.push(token);
    }

    input
}

