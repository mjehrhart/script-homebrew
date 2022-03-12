#[allow(unused_variables, dead_code, non_camel_case_types, unused_imports)]
pub mod parser {
    use super::*;
    use std::{any::type_name, fmt};
    use std::iter::Peekable;
    use std::str::Chars;

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
    }

    impl<'a> Parser<'a> {
        pub fn get_next_token(&mut self) -> TokenKind {
            let next_token = match self.tokenizer.next() {
                Some(token) => token,
                None => TokenKind::Undefined,
            };

            self.current_token = next_token.clone();
            next_token
        }
    }

    // Private methods of Parser

    impl<'a> Parser<'a> {
        // Retrieve the next token from arithmetic expression and set it to current_token field in Parser struct
        // pub fn get_next_token(&mut self) -> Result<(), ParseError> {
        //     let next_token = match self.tokenizer.next() {
        //         Some(token) => token,
        //         None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        //     };
        //     self.current_token = next_token;
        //     Ok(())
        // }

        // Main workhorse method that is called recursively

        // fn generate_ast(&mut self, oper_prec: OperPrec) -> Result<Node, ParseError> {
        //     let mut left_expr = self.parse_number()?;

        //     while oper_prec < self.current_token.get_oper_prec() {
        //         if self.current_token == Token::EOF {
        //             break;
        //         }
        //         let right_expr = self.convert_token_to_node(left_expr.clone())?;
        //         left_expr = right_expr;
        //     }
        //     Ok(left_expr)
        // }

        // Construct AST node for numbers, taking into account negative prefixes while handling parenthesis

        // fn parse_number(&mut self) -> Result<Node, ParseError> {
        //     let token = self.current_token.clone();
        //     match token {
        //         Token::Subtract => {
        //             self.get_next_token()?;
        //             let expr = self.generate_ast(OperPrec::Negative)?;
        //             Ok(Node::Negative(Box::new(expr)))
        //         }
        //         Token::Num(i) => {
        //             self.get_next_token()?;
        //             Ok(Node::Number(i))
        //         }
        //         Token::LeftParen => {
        //             self.get_next_token()?;
        //             let expr = self.generate_ast(OperPrec::DefaultZero)?;
        //             self.check_paren(Token::RightParen)?;
        //             if self.current_token == Token::LeftParen {
        //                 let right = self.generate_ast(OperPrec::MulDiv)?;
        //                 return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
        //             }

        //             Ok(expr)
        //         }
        //         _ => Err(ParseError::UnableToParse("Unable to parse".to_string())),
        //     }
        // }

        // Check for balancing parenthesis

        // fn check_paren(&mut self, expected: Token) -> Result<(), ParseError> {
        //     if expected == self.current_token {
        //         self.get_next_token()?;
        //         Ok(())
        //     } else {
        //         Err(ParseError::InvalidOperator(format!(
        //             "Expected {:?}, got {:?}",
        //             expected, self.current_token
        //         )))
        //     }
        // }

        // Construct Operator AST nodes

        // fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
        //     match self.current_token {
        //         Token::Add => {
        //             self.get_next_token()?;
        //             //Get right-side expression
        //             let right_expr = self.generate_ast(OperPrec::AddSub)?;
        //             Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
        //         }
        //         Token::Subtract => {
        //             self.get_next_token()?;
        //             //Get right-side expression
        //             let right_expr = self.generate_ast(OperPrec::AddSub)?;
        //             Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
        //         }
        //         Token::Multiply => {
        //             self.get_next_token()?;
        //             //Get right-side expression
        //             let right_expr = self.generate_ast(OperPrec::MulDiv)?;
        //             Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
        //         }
        //         Token::Divide => {
        //             self.get_next_token()?;
        //             //Get right-side expression
        //             let right_expr = self.generate_ast(OperPrec::MulDiv)?;
        //             Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
        //         }
        //         Token::Caret => {
        //             self.get_next_token()?;
        //             //Get right-side expression
        //             let right_expr = self.generate_ast(OperPrec::Power)?;
        //             Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
        //         }
        //         _ => Err(ParseError::InvalidOperator(format!(
        //             "Please enter valid operator {:?}",
        //             self.current_token
        //         ))),
        //     }
        // }
    
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

    /*impl error::Error for ParseError {
        fn description(&self) -> &str {
            match &self {
                self::ParseError::UnableToParse(e) => &e,
                self::ParseError::InvalidOperator(e) => &e,
            }
        }
    }*/

    // Handle error thrown from AST module

    impl std::convert::From<std::boxed::Box<dyn std::error::Error>> for ParseError {
        fn from(_evalerr: std::boxed::Box<dyn std::error::Error>) -> Self {
            return ParseError::UnableToParse("Unable to parse".into());
        }
    }
}
