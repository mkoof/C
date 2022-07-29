#![allow(unused)]

use crate::err::CompileError;
use crate::token::Sequence;

pub trait Parse: Sized {
    fn parse(seq: &Sequence) -> Result<Self, CompileError>;
}

pub struct AST;

impl Parse for AST {
    fn parse(seq: &Sequence) -> Result<Self, CompileError> {
        todo!()
    }
}

pub enum GlobalItem {
    FuncDec(FuncDec),
    FuncImpl(FuncImpl),
    GlobalStmt(DeclareStmt),
}

pub struct FuncDec;

pub struct FuncImpl;

pub struct IfStmt;

pub struct WhileStmt;

pub struct ForStmt;

pub struct DeclareStmt;

pub struct ExprStmt;

pub enum Stmt {}
