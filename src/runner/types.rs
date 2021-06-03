pub trait Compiler {
    fn compile(&self);
    fn execute(&self);
}

pub trait Interpreter {
    fn execute(&self);
}