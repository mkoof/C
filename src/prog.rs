#![allow(unused)]

use crate::ast::*;
use crate::err::CompileError;
pub struct Program;

impl Program {
    pub fn new(ast: &AST) -> Result<Program, CompileError> {
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

    /// return None when path not exist
    pub fn load(path: &str) -> Option<BinProgram> {
        unimplemented!()
    }
}
