#![allow(unused)]

use crate::token::Sequence;

pub trait Parse {
    fn parse(seq: &Sequence) -> Self;
}

pub struct AST;

impl Parse for AST {
    fn parse(seq: &Sequence) -> Self {
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
