use std::io;
use std::process;

mod vm;

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

fn run(machine: &mut vm::machine::Machine, line: &String) {
    let input = tokenize_input(line);

    let mut had_error: bool = false;
    for token in input {
        if let vm::Value::Word(word) = &token {
            if word == ":" {
                match machine.compile(line) {
                    Ok(_) => break,
                    Err(vm::ErrorType::CompilationError) => {
                        println!("compilation error");
                        had_error = true;
                        break;
                    },
                    _ => ()
                }
            }
        }

        match machine.execute(&token) {
            Ok(_) => (),
            Err(vm::ErrorType::WordNotFound) => {
                had_error = true;
            },
            Err(vm::ErrorType::StackUnderflow) => {
                println!("stack underflow");
                had_error = true;
            },
            _ => (),
        }
    }

    if !had_error {
        println!("ok");
    }
}

fn main() {
    let mut machine = vm::machine::Machine::new();

    loop {
        let mut line = String::new();
        if let Err(_) = io::stdin().read_line(&mut line) {
            println!("Failed to read line.");
            process::exit(1);
        }

        run(&mut machine, &line);
    }
}
