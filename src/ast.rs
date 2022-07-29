#![allow(unused)]

use crate::err::CompileError;
use crate::seq::Cursor;

pub trait Parse: Sized {
    fn parse(cur: &mut Cursor) -> Result<Self, CompileError>;
}

pub struct AST;

impl Parse for AST {
    fn parse(cur: &mut Cursor) -> Result<Self, CompileError> {
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
