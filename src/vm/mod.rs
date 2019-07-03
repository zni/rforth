pub mod instructions;
pub mod machine;

#[derive(Debug, Clone)]
pub enum Value {
    Word(String),
    Number(i32)
}

pub enum ErrorType {
    BranchOutOfBounds,
    CompilationError,
    InvalidOffset,
    OutsideCompileMode,
    StackUnderflow,
    WordNotFound,
}
