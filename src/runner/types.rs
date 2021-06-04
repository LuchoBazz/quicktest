use std::time::Duration;

pub trait Compiler {
    fn compile(&self);
    fn execute(&self, timeout: u64) -> Duration;
}

pub trait Interpreter {
    fn execute(&self, timeout: u64) -> Duration;
}