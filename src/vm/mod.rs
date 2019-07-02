pub mod instructions;
pub mod machine;

#[derive(Debug, Clone)]
pub enum Value {
    Word(String),
    Number(i32)
}

pub enum ErrorType {
    CompilationError,
    WordNotFound,
    StackUnderflow
}
