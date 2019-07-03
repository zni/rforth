use std::collections::HashMap;
use std::fmt;

use crate::vm::instructions;
use crate::vm::ErrorType;
use crate::vm::Value;

pub enum Function {
    Builtin(fn(&mut Machine) -> Result<(), ErrorType>),
    UserDefined(Vec<Value>)
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function")
    }
}

#[derive(Debug)]
pub struct Machine {
    pub compile_mode: bool,
    pub compile_buffer: Vec<Value>,
    pub dictionary: HashMap<String, Function>,
    pub stack: Vec<i32>
}

impl Machine {
    pub fn new() -> Machine {
        let mut dictionary = HashMap::new();
        dictionary.insert(String::from(":"), Function::Builtin(compile));
        dictionary.insert(String::from(";"), Function::Builtin(finish_compile));
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
            compile_mode: false,
            compile_buffer: Vec::new(),
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

    pub fn execute(&mut self, value: &Value) -> Result<(), ErrorType> {
        if let Value::Word(s) = value {
            if s == ";" {
                self.compile_mode = false;
            }
        }

        if self.compile_mode {
            self.compile_buffer.push(value.clone());
            return Ok(());
        }

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

fn compile(machine: &mut Machine) -> Result<(), ErrorType> {
    machine.compile_mode = true;
    Ok(())
}

fn finish_compile(machine: &mut Machine) -> Result<(), ErrorType> {
    machine.compile_mode = false;

    let mut word;
    if machine.compile_buffer.len() > 0 {
        word = match machine.compile_buffer.remove(0) {
            Value::Number(n) => n.to_string(),
            Value::Word(w) => w
        };
    } else {
        return Err(ErrorType::CompilationError);
    }

    machine.dictionary.insert(word.clone(), Function::UserDefined(machine.compile_buffer.clone()));

    machine.compile_buffer.clear();
    Ok(())
}
