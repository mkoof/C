#![allow(unused)]

use crate::ast::*;

pub struct Program;

impl Program {
    pub fn new(ast: &AST) -> Program {
        todo!();
    }

    pub fn to_bin(&self) -> BinProgram {
        unimplemented!()
    }

    fn analyze_ast(&self, ast: &AST) {
        unimplemented!()
    }

    fn analyze_func_dec(&self, f: &FuncDec) {
        todo!()
    }

    fn analyze_func_impl(&self, f: &FuncImpl) {
        todo!()
    }
}

pub struct BinProgram;

impl BinProgram {
    pub fn save(&self, path: &str) {
        unimplemented!()
    }

    pub fn load(path: &str) -> BinProgram {
        unimplemented!()
    }
}
