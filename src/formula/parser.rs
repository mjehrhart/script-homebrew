#[allow(unused_variables, dead_code, non_camel_case_types, unused_imports)]
pub mod parser {
    use super::*;
    use std::iter::Peekable;
    use std::str::Chars;
    use std::{any::type_name, fmt};

    use crate::formula::lexer::lexer::{TokenKind, Tokenizer};

    #[derive(Debug, Clone)]
    pub struct Parser<'a> {
        current_token: TokenKind,
        tokenizer: Tokenizer<'a>,
    }

    impl<'a> Parser<'a> {
        pub fn new(exp: &'a str) -> Result<Self, String> {
            let mut lexy = Tokenizer::new(exp);
            return match lexy.next() {
                None => Ok(Parser {
                    tokenizer: lexy,
                    current_token: TokenKind::Undefined,
                }),
                Some(token) => Ok(Parser {
                    tokenizer: lexy,
                    current_token: token,
                }),
            };
        }

        pub fn find_token(token_list: Vec<TokenKind>){

            let needle = TokenKind::Variable { raw: "desc".to_string() };
            for i in 0..token_list.len() {
                println!("..{:?}", token_list[i]);

                if token_list[i] == needle{
                    println!("Found it at position {}", i);
                    println!("Found {:?}", token_list[i]);
                }

            }
        }

        pub fn get_next_token(&mut self) -> TokenKind {
            let next_token = match self.tokenizer.next() {
                Some(token) => token,
                None => TokenKind::Undefined,
            };

            self.current_token = next_token.clone();
            next_token
        }

        pub fn convert_token_to_node(token_list: Vec<TokenKind>) -> String {
            let mut rb = String::from("");
            let mut prev: TokenKind;
            for i in 0..token_list.len() {
                println!("..{:?}", token_list[i]);

                match &token_list[i] {
                    TokenKind::Def => {
                        rb.push_str("def");
                    }
                    TokenKind::End => {
                        rb.push_str("end");
                    }
                    TokenKind::Class { raw: val } => {
                        rb.push_str(val);
                    }
                    TokenKind::Variable { raw: val } => {
                        rb.push_str(val);
                    }
                    TokenKind::Punctuation(char) => {
                        rb.push(*char);
                    }
                    TokenKind::Value => {
                        rb.push('"');
                    }
                    TokenKind::Whitespace { raw: char, kind: _ } => {
                        rb.push(*char);
                    }
                    TokenKind::CRLF { raw: val, kind: _ } => {
                        rb.push_str(val);
                    }
                    TokenKind::Comment => {
                        rb.push('#');
                    }
                    TokenKind::Signature { raw: val, kind: _ } => {
                        rb.push_str(val);
                    }
                    TokenKind::Object(val) => {
                        rb.push_str(val);
                    }
                    _ => {}
                }

                //prev = token_list[i].clone(); //todo perhaps a peek forward instead
            }
        
            rb
        }
    }
   
    // Custom error handler for Parser
    #[derive(Debug)]
    pub enum ParseError {
        UnableToParse(String),
        InvalidOperator(String),
    }

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match &self {
                self::ParseError::UnableToParse(e) => write!(f, "Error in evaluating {}", e),
                self::ParseError::InvalidOperator(e) => write!(f, "Error in evaluating {}", e),
            }
        }
    }
 
    // Handle error thrown from AST module

    impl std::convert::From<std::boxed::Box<dyn std::error::Error>> for ParseError {
        fn from(_evalerr: std::boxed::Box<dyn std::error::Error>) -> Self {
            return ParseError::UnableToParse("Unable to parse".into());
        }
    }
}
