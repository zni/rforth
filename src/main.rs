use std::io;
use std::process;

#[derive(Debug)]
struct Machine {
    stack: Vec<i32>
}

impl Machine {
    fn new() -> Machine {
        Machine {
            stack: Vec::new()
        }
    }

    fn push(&mut self, input: i32) {
        self.stack.push(input);
    }

    fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }

    fn execute(&mut self, word: &String) {
        match word.as_ref() {
            "+" => {
                let a = match self.pop() {
                    Some(n) => n,
                    None => panic!("stack underflow"),
                };
                let b = match self.pop() {
                    Some(n) => n,
                    None => panic!("stack underflow"),
                };
                self.push(a + b);
            },
            "-" => {
                let a = match self.pop() {
                    Some(n) => n,
                    None => panic!("stack underflow"),
                };
                let b = match self.pop() {
                    Some(n) => n,
                    None => panic!("stack underflow"),
                };
                self.push(a - b);
            },
            "*" => {
                let a = match self.pop() {
                    Some(n) => n,
                    None => panic!("stack underflow"),
                };
                let b = match self.pop() {
                    Some(n) => n,
                    None => panic!("stack underflow"),
                };
                self.push(a * b);
            },
            "/" => {
                let a = match self.pop() {
                    Some(n) => n,
                    None => panic!("stack underflow"),
                };
                let b = match self.pop() {
                    Some(n) => n,
                    None => panic!("stack underflow"),
                };
                self.push(a / b);
            },
            "." => {
                let a = match self.pop() {
                    Some(n) => n,
                    None => panic!("stack underflow"),
                };
                println!("{}", a);
            },
            ".s" => {
                println!("{:?}", self.stack);
            }
            _ => println!("{}?", word),
        }
    }
}

fn handle_line(machine: &mut Machine, line: &String) {
    let words = line.split_whitespace();
    for word in words {
        match word.parse::<i32>() {
            Ok(num) => {
                machine.push(num);
            },
            Err(_)  => {
                machine.execute(&word.to_string());
            },
        }
    }
}

fn main() {
    let mut machine = Machine::new();

    loop {
        let mut line = String::new();
        if let Err(_) = io::stdin().read_line(&mut line) {
            println!("Failed to read line.");
            process::exit(1);
        }

        handle_line(&mut machine, &line);
        // println!("{:?}", machine);
    }
}
