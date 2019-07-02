use std::collections::HashMap;

use crate::vm::instructions;
use crate::vm::ErrorType;
use crate::vm::Value;

pub enum Function {
    Builtin(fn(&mut Machine) -> Result<(), ErrorType>),
    UserDefined(Vec<Value>)
}

pub struct Machine {
    pub dictionary: HashMap<String, Function>,
    pub stack: Vec<i32>
}

impl Machine {
    pub fn new() -> Machine {
        let mut dictionary = HashMap::new();
        dictionary.insert(String::from("+"), Function::Builtin(instructions::add));
        dictionary.insert(String::from("-"), Function::Builtin(instructions::sub));
        dictionary.insert(String::from("*"), Function::Builtin(instructions::mult));
        dictionary.insert(String::from("/"), Function::Builtin(instructions::div));
        dictionary.insert(String::from("dup"), Function::Builtin(instructions::dup));
        dictionary.insert(String::from("drop"), Function::Builtin(instructions::drop));
        dictionary.insert(String::from("swap"), Function::Builtin(instructions::swap));
        dictionary.insert(String::from("over"), Function::Builtin(instructions::over));
        dictionary.insert(String::from("rot"), Function::Builtin(instructions::rot));
        dictionary.insert(String::from("."), Function::Builtin(instructions::dot));
        dictionary.insert(String::from(".s"), Function::Builtin(instructions::sdot));
        dictionary.insert(String::from("="), Function::Builtin(instructions::eq));
        dictionary.insert(String::from(">"), Function::Builtin(instructions::greater_than));
        dictionary.insert(String::from("<"), Function::Builtin(instructions::less_than));
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

        self.dictionary.insert(name, Function::UserDefined(definition));
        Ok(())
    }


    pub fn execute(&mut self, value: &Value) -> Result<(), ErrorType> {
        let word = match value {
            Value::Number(n) => {
                self.push(*n);
                return Ok(());
            },
            Value::Word(s) => s,
        };

        match self.dictionary.get(word) {
            Some(Function::Builtin(f)) => f(self),
            Some(Function::UserDefined(f)) => {
                let function = f.clone();
                return self.execute_function(&function);
            },
            None => {
                println!("{}?", word);
                Err(ErrorType::WordNotFound)
            }
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
