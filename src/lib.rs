mod ast;
mod prog;
mod runner;
mod token;

use ast::{Parse, AST};
use prog::{BinProgram, Program};
use runner::Runner;
use token::Code;

pub fn compile(code: &str) -> BinProgram {
    let code = Code::new(code);
    let seq = code.lex();
    let ast = AST::parse(&seq);
    let prog = Program::new(&ast);
    prog.to_bin()
}

pub fn run(prog: &BinProgram) {
    let mut runner = Runner::new();
    runner.load(&prog);
    runner.run();
}

pub fn load_prog(path: &str) -> BinProgram {
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
