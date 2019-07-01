use std::collections::HashMap;
use std::io;
use std::process;

enum ErrorType {
    WordNotFound,
    StackUnderflow
}

#[derive(Debug)]
struct Machine {
    dictionary: HashMap<String, Vec<String>>,
    stack: Vec<i32>
}

impl Machine {
    fn new() -> Machine {
        Machine {
            dictionary: HashMap::new(),
            stack: Vec::new()
        }
    }

    fn push(&mut self, input: i32) {
        self.stack.push(input);
    }

    fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }

    fn compile(&mut self, line: &String) {
        let mut words = line.split_whitespace();

        // Skip ':'.
        words.next();

        // Get definition name.
        let name = match words.next() {
            Some(n) => n.to_string(),
            None => panic!("No word found.")
        };

        let mut definition: Vec<String> = Vec::new();
        for word in words {
            if word == ";" {
                break;
            } else {
                definition.push(word.to_string());
            }
        }

        self.dictionary.insert(name, definition);
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
                match self.dictionary.get(word) {
                    Some(f) => {
                        let function = f.clone();
                        return self.execute_function(&function);
                    },
                    None => {
                        println!("{}?", word);
                        Err(ErrorType::WordNotFound)
                    }
                }
            },
        }
    }

    fn execute_function(&mut self, function: &Vec<String>) -> Result<(), ErrorType>  {
        for word in function {
            match self.execute(word) {
                Ok(_) => continue,
                Err(e) => return Err(e)
            }
        }

        return Ok(());
    }
}

fn handle_line(machine: &mut Machine, line: &String) {
    let mut had_error: bool = false;
    let words = line.split_whitespace();
    for word in words {
        if word == ":" {
            machine.compile(line);
            break;
        }

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

        println!("{:?}", machine);
    }
}
