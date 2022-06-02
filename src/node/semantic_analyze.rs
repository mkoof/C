use crate::span::Span;

pub trait SemanticAnalyze {
    fn semantic_analyze(&self) -> Result<(), Span>;
}
