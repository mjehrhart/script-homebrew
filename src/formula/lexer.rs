//!The lexer procceses the input converts to a vector of tokens
#[allow(unused_variables, dead_code, non_camel_case_types, unused_imports)]
pub mod lexer {
    use super::*;
    use crate::enums::TokenKind;
    use crate::formula::controller;
    use crate::formula::controller::ruby::get_tokenkind_map;

    use std::any::type_name;
    use std::collections::HashMap;
    use std::fs::OpenOptions;
    use std::iter::Peekable;
    use std::str::Chars;

    #[derive(Debug, Clone)]
    pub struct Tokenizer<'a> {
        expr: Peekable<Chars<'a>>,
    }

    impl<'a> Tokenizer<'a> {
        pub fn new(new_expr: &'a str) -> Self {
            Tokenizer {
                expr: new_expr.chars().peekable(),
            }
        }
    }

    impl<'a> Iterator for Tokenizer<'a> {
        type Item = TokenKind;

        fn next(&mut self) -> Option<TokenKind> {
            let next_char = self.expr.next();

            match next_char {
                Some('#') => Some(TokenKind::Comment),
                Some('0'..='9') => {
                    let number = next_char?.to_string();
                    let number: u8 = number.parse().unwrap();
                    Some(TokenKind::Digit(number))
                }
                Some(' ') => {
                    let var = next_char?.to_string();
                    Some(TokenKind::WhiteSpace)
                }
                Some('A'..='z') => Some(TokenKind::Latin(next_char.unwrap())),

                Some(_) => {
                    let character = next_char?.to_string();
                    let character = character.chars().next();
                    Some(TokenKind::Punctuation(character.unwrap()))
                }
                None => Some(TokenKind::Undefined),
            }
        }
    }

    // Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_tokenizer() {
            let mut tokenizer = Tokenizer::new("Water");
            assert_eq!(tokenizer.next().unwrap(), TokenKind::Latin('W'))
        }
    }
}
