use std::collections::HashMap;

use crate::vm::instructions;
use crate::vm::ErrorType;
use crate::vm::Value;

#[derive(Debug)]
pub struct Machine {
    pub dictionary: HashMap<String, Vec<Value>>,
    pub stack: Vec<i32>
}

enum Function {
    Builtin(fn(&mut Machine) -> Result<(), ErrorType>),
    UserDefined(Vec<Value>)
}

impl Machine {
    pub fn new() -> Machine {
        let dictionary = HashMap::new();
        Machine {
            dictionary,
            stack: Vec::new()
        }
    }

    pub fn push(&mut self, input: i32) {
        self.stack.push(input);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }

    pub fn compile(&mut self, line: &String) -> Result<(), ErrorType> {
        let mut words = line.split_whitespace();

        // Skip ':'.
        words.next();

        // Get definition name.
        let name = match words.next() {
            Some(n) => n.to_string(),
            None => return Err(ErrorType::CompilationError)
        };

        let mut definition: Vec<Value> = Vec::new();
        for word in words {
            if word == ";" {
                break;
            }

            let token = match word.parse::<i32>() {
                Ok(num) => {
                    Value::Number(num)
                },
                Err(_) => {
                    Value::Word(word.to_string())
                }
            };
            definition.push(token);
        }

        self.dictionary.insert(name, definition);
        Ok(())
    }


    pub fn execute(&mut self, value: &Value) -> Result<(), ErrorType> {
        let word = match value {
            Value::Number(n) => {
                self.push(*n);
                return Ok(());
            },
            Value::Word(s) => s.as_str(),
        };

        match word {
            "+" => instructions::add(self),
            "-" => instructions::sub(self),
            "*" => instructions::mult(self),
            "/" => instructions::div(self),
            "dup" => instructions::dup(self),
            "drop" => instructions::drop(self),
            "swap" => instructions::swap(self),
            "over" => instructions::over(self),
            "rot" => instructions::rot(self),
            "." => instructions::dot(self),
            ".s" => instructions::sdot(self),
            "=" => instructions::eq(self),
            ">" => instructions::greater_than(self),
            "<" => instructions::less_than(self),
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

    fn execute_function(&mut self, function: &Vec<Value>) -> Result<(), ErrorType>  {
        for word in function {
            match self.execute(word) {
                Ok(_) => continue,
                Err(e) => return Err(e)
            }
        }

        return Ok(());
    }
}
