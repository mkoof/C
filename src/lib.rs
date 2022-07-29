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

const DEBUG: bool = true;

pub fn compile(code: &str) -> Result<BinProgram, CompileError> {
    fn _compile(code: &str) -> Result<BinProgram, CompileError> {
        let code = Code::new(code);
        let seq = code.lex()?;
        let ast = AST::parse(&seq)?;
        let prog = Program::new(&ast)?;
        Ok(prog.to_bin())
    }
    let res = _compile(code);
    if DEBUG {
        if let Err(e) = res.as_ref() {
            println!("{:?}", e);
        }
    }
    res
}

pub fn run(prog: &BinProgram) -> Result<(), RuntimeError> {
    fn _run(prog: &BinProgram) -> Result<(), RuntimeError> {
        let mut runner = Runner::new();
        runner.load(&prog)?;
        runner.run()
    }
    let res = _run(prog);
    if DEBUG {
        if let Err(e) = res.as_ref() {
            println!("{:?}", e);
        }
    }
    res
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
