mod ast;
mod err;
mod prog;
mod runner;
mod token;

use ast::{Parse, AST};
use err::{CompileError, RuntimeError};
use prog::{BinProgram, Program};
use runner::Runner;
use token::Code;

pub fn compile(code: &str) -> Result<BinProgram, CompileError> {
    let code = Code::new(code);
    let seq = code.lex()?;
    let ast = AST::parse(&seq)?;
    let prog = Program::new(&ast)?;
    Ok(prog.to_bin())
}

pub fn run(prog: &BinProgram) -> Result<(), RuntimeError> {
    let mut runner = Runner::new();
    runner.load(&prog)?;
    runner.run()
}

/// return None when load error
pub fn load_prog(path: &str) -> Option<BinProgram> {
    BinProgram::load(path)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
