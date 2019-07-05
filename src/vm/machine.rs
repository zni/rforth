use std::collections::HashMap;
use std::fmt;

use crate::vm::instructions;
use crate::vm::ErrorType;
use crate::vm::Value;

pub enum Function {
    Builtin(fn(&mut Machine) -> Result<(), ErrorType>),
    UserDefined(Vec<Value>),
    Action,
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
    pub stack: Vec<i32>,
    pub return_stack: Vec<usize>,
    pub control_flow_stack: Vec<i32>,
    pub pc: usize,
    pub data: Vec<Value>,
    pub context: Vec<Box<Vec<Value>>>,
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
        dictionary.insert(String::from("mod"), Function::Builtin(instructions::mod_));
        dictionary.insert(String::from("dup"), Function::Builtin(instructions::dup));
        dictionary.insert(String::from("drop"), Function::Builtin(instructions::drop));
        dictionary.insert(String::from("swap"), Function::Builtin(instructions::swap));
        dictionary.insert(String::from("over"), Function::Builtin(instructions::over));
        dictionary.insert(String::from("rot"), Function::Builtin(instructions::rot));
        dictionary.insert(String::from("."), Function::Builtin(instructions::dot));
        dictionary.insert(String::from(".s"), Function::Builtin(instructions::sdot));
        dictionary.insert(String::from(".\""), Function::Builtin(instructions::dot_quote));
        dictionary.insert(String::from("="), Function::Builtin(instructions::eq));
        dictionary.insert(String::from(">"), Function::Builtin(instructions::greater_than));
        dictionary.insert(String::from("<"), Function::Builtin(instructions::less_than));
        dictionary.insert(String::from("and"), Function::Builtin(instructions::and));
        dictionary.insert(String::from("or"), Function::Builtin(instructions::or));
        dictionary.insert(String::from("invert"), Function::Builtin(instructions::invert));
        dictionary.insert(String::from("clearstack"), Function::Builtin(instructions::clearstack));
        dictionary.insert(String::from("0branch"), Function::Builtin(instructions::branch0));
        dictionary.insert(String::from("branch"), Function::Builtin(instructions::branch));
        dictionary.insert(String::from(">r"), Function::Builtin(instructions::to_r));
        dictionary.insert(String::from("r>"), Function::Builtin(instructions::from_r));

        dictionary.insert(String::from("if"), Function::Builtin(instructions::if_));
        dictionary.insert(String::from("then"), Function::Builtin(instructions::then));
        dictionary.insert(String::from("else"), Function::Builtin(instructions::else_));
        dictionary.insert(String::from("do"), Function::Action);
        dictionary.insert(String::from("loop"), Function::Action);

        Machine {
            compile_mode: false,
            compile_buffer: Vec::new(),
            dictionary,
            stack: Vec::new(),
            return_stack: Vec::new(),
            control_flow_stack: Vec::new(),
            pc: 0,
            data: Vec::new(),
            context: Vec::new(),
        }
    }

    pub fn push(&mut self, input: i32) {
        self.stack.push(input);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }

    pub fn execute(&mut self, input: &Vec<Value>) -> Result<(), ErrorType> {
        self.pc = 0;
        self.data = input.to_vec();
        while self.pc < input.len() {
            // Get value.
            let value = &input[self.pc];
            self.pc += 1;

            // Convert the current cell to string.
            let current_word = match value {
                Value::Number(n) => n.to_string(),
                Value::Word(w) => w.clone(),
            };

            // If we're in compile mode, keep compiling.
            if self.compile_mode && current_word != ";" {
                if let Err(e) = self.compile_word(&value) {
                    return Err(e);
                }

                continue;
            }

            // Get the current word.
            let word = match value {
                Value::Number(n) => {
                    self.push(*n);
                    continue;
                },
                Value::Word(s) => s,
            };

            match self.dictionary.get(word) {
                Some(Function::Builtin(f)) => {
                    if let Err(e) = f(self) {
                        return Err(e);
                    }
                },
                Some(Function::UserDefined(f)) => {
                    let function = f.clone();
                    self.return_stack.push(self.pc);
                    self.context.push(Box::new(self.data.clone()));

                    if let Err(e) = self.execute(&function) {
                        return Err(e);
                    };

                    self.data = *self.context.pop().unwrap();
                    self.pc = match self.return_stack.pop() {
                        Some(n) => n,
                        None => return Err(ErrorType::StackUnderflow),
                    };
                },
                Some(Function::Action) => {
                    return Err(ErrorType::OutsideCompileMode);
                },
                None => {
                    println!("{}?", word);
                    return Err(ErrorType::WordNotFound);
                }
            };
        }

        return Ok(())
    }

    fn compile_word(&mut self, value: &Value) -> Result<(), ErrorType> {
        self.compile_buffer.push(value.clone());
        return Ok(());
    }
}

fn compile(machine: &mut Machine) -> Result<(), ErrorType> {
    machine.compile_mode = true;
    Ok(())
}

fn finish_compile(machine: &mut Machine) -> Result<(), ErrorType> {
    if !machine.compile_mode {
        return Err(ErrorType::OutsideCompileMode);
    }

    machine.compile_mode = false;

    // Get the compiled definition name.
    let word;
    if machine.compile_buffer.len() > 0 {
        word = match machine.compile_buffer.remove(0) {
            Value::Number(n) => n.to_string(),
            Value::Word(w) => w
        };
    } else {
        return Err(ErrorType::CompilationError);
    }

    // Check if words in definition are valid.
    let mut string_literal = false;
    for word in machine.compile_buffer.clone() {
        if let Value::Word(w) = word {
            if w.contains("\"") && string_literal {
                string_literal = false;
                continue;
            }

            if string_literal {
                continue;
            }

            if w == ".\"" {
                string_literal = true;
            }

            if let None = machine.dictionary.get(&w) {
                println!("undefined word: {}", w);
                machine.compile_buffer.clear();
                return Err(ErrorType::CompilationError);
            }
        }
    }

    machine.dictionary.insert(word.clone(), Function::UserDefined(machine.compile_buffer.clone()));

    machine.compile_buffer.clear();
    Ok(())
}

fn translate_if(values: &mut Vec<Value>) -> Result<(), ErrorType> {
    let mut i = 0;
    while i < values.len() {
        if let Value::Word(w) = &values[i] {
            if w == "if" {
                values[i] = Value::Word("0branch".to_string());
                let offset = match calculate_offset(i, values) {
                    Ok(n) => n,
                    Err(e) => return Err(e),
                };
                values.insert(i + 1, Value::Number(offset - 1));
                values.remove(i + offset as usize);
            }
        }
        i += 1;
    }

    return Ok(());
}

fn calculate_offset(i: usize, values: &mut Vec<Value>) -> Result<i32, ErrorType> {
    let mut offset: i32 = 1;
    let mut ifs = 0;
    let iter = values.iter().skip(i);
    for v in iter {
        if let Value::Word(w) = &v {
            if w == "then" && ifs == 0 {
                return Ok(offset);
            } else if w == "if" {
                ifs += 1;
                offset += 1;
            } else if w == "then" && ifs != 0 {
                ifs -= 1;
                offset -= 1;
            }
            offset += 1;
        } else {
            offset += 1;
        }
    }

    return Err(ErrorType::UnbalancedControl);
}
