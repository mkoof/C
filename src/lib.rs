mod span;
mod token;
mod node;

pub fn lex(code: String) -> token::Tokens {
    let cs = token::CharSeq::new(code);
    token::lex(&cs)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
