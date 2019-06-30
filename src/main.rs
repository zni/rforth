use std::io;
use std::process;

enum ErrorType {
    WordNotFound,
    StackUnderflow
}

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

    fn execute(&mut self, word: &String) -> Result<(), ErrorType> {
        match word.as_ref() {
            "+" => {
                let a = match self.pop() {
                    Some(n) => n,
                    None => return Err(ErrorType::StackUnderflow),
                };
                let b = match self.pop() {
                    Some(n) => n,
                    None => return Err(ErrorType::StackUnderflow)
                };
                self.push(a + b);
                Ok(())
            },
            "-" => {
                let a = match self.pop() {
                    Some(n) => n,
                    None => return Err(ErrorType::StackUnderflow)
                };
                let b = match self.pop() {
                    Some(n) => n,
                    None => return Err(ErrorType::StackUnderflow)
                };
                self.push(a - b);
                Ok(())
            },
            "*" => {
                let a = match self.pop() {
                    Some(n) => n,
                    None => return Err(ErrorType::StackUnderflow)
                };
                let b = match self.pop() {
                    Some(n) => n,
                    None => return Err(ErrorType::StackUnderflow)
                };
                self.push(a * b);
                Ok(())
            },
            "/" => {
                let a = match self.pop() {
                    Some(n) => n,
                    None => return Err(ErrorType::StackUnderflow)
                };
                let b = match self.pop() {
                    Some(n) => n,
                    None => return Err(ErrorType::StackUnderflow)
                };
                self.push(a / b);
                Ok(())
            },
            "dup" => {
                let a = match self.pop() {
                    Some(n) => n,
                    None => return Err(ErrorType::StackUnderflow)
                };
                self.push(a);
                self.push(a);
                Ok(())
            },
            "drop" => {
                let _a = match self.pop() {
                    Some(n) => n,
                    None => return Err(ErrorType::StackUnderflow)
                };
                Ok(())
            },
            "swap" => {
                let a = match self.pop() {
                    Some(n) => n,
                    None => return Err(ErrorType::StackUnderflow)
                };
                let b = match self.pop() {
                    Some(n) => n,
                    None => return Err(ErrorType::StackUnderflow)
                };
                self.push(a);
                self.push(b);
                Ok(())
            },
            "over" => {
                let a = match self.pop() {
                    Some(n) => n,
                    None => return Err(ErrorType::StackUnderflow)
                };
                let b = match self.pop() {
                    Some(n) => n,
                    None => return Err(ErrorType::StackUnderflow)
                };
                self.push(b);
                self.push(a);
                self.push(b);
                Ok(())
            },
            "rot" => {
                let a = match self.pop() {
                    Some(n) => n,
                    None => return Err(ErrorType::StackUnderflow)
                };
                let b = match self.pop() {
                    Some(n) => n,
                    None => return Err(ErrorType::StackUnderflow)
                };
                let c = match self.pop() {
                    Some(n) => n,
                    None => return Err(ErrorType::StackUnderflow)
                };
                self.push(b);
                self.push(a);
                self.push(c);
                Ok(())
            },
            "." => {
                let a = match self.pop() {
                    Some(n) => n,
                    None => return Err(ErrorType::StackUnderflow)
                };
                println!("{}", a);
                Ok(())
            },
            ".s" => {
                println!("{:?}", self.stack);
                Ok(())
            }
            _ => {
                println!("{}?", word);
                Err(ErrorType::WordNotFound)
            },
        }
    }
}

fn handle_line(machine: &mut Machine, line: &String) {
    let mut had_error: bool = false;
    let words = line.split_whitespace();
    for word in words {
        match word.parse::<i32>() {
            Ok(num) => {
                machine.push(num);
            },
            Err(_)  => {
                match machine.execute(&word.to_string()) {
                    Ok(_) => (),
                    Err(ErrorType::WordNotFound) => {
                        had_error = true;
                    },
                    Err(ErrorType::StackUnderflow) => {
                        println!("stack underflow");
                        had_error = true;
                    },
                }
            },
        }
    }

    if !had_error {
        println!("ok");
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
    }
}
