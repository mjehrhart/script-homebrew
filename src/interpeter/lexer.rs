//!The lexer procceses the input converts to a vector of tokens
#[allow(
    unused_variables,
    dead_code,
    non_camel_case_types,
    unused_imports,
    clippy::module_inception
)]
pub mod lexer {
    use super::*;
    use crate::enums::{self, Token, TokenKind};
    //use crate::formula::controller;

    use std::any::type_name;
    use std::collections::HashMap;
    use std::fs::OpenOptions;
    use std::iter::Peekable;
    use std::str::Chars;
    use std::vec::IntoIter;

    #[derive(Debug, Clone)]
    pub struct Tokenizer<'a> {
        pub expr: Peekable<Chars<'a>>,
        //expr_iter: Peekable<IntoIter<char>>,
        pub keywords: [&'a str; 39],
    }

    /// Example
    /// ```
    /// let exp = "Water is helpful"
    /// let mut lexy = Tokenizer::new(exp);
    /// ```
    impl<'a> Tokenizer<'a> {
        pub fn new(new_expr: &'a str) -> Self {
            println!("Expression == {:?}", new_expr);
            Tokenizer {
                //expr: new_expr.chars().collect::<Vec<_>>().into_iter().peekable(),
                expr: new_expr.chars().peekable(),
                keywords: [
                    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else",
                    "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop",
                    "match", "mod", "move", "mut", "pub", "ref", "return", "Self", "self",
                    "static", " struct", "super", "trait", "true", "type", "union", "unsafe",
                    "use", "where", "while",
                ],
            }
        }
    }

    impl<'a> Iterator for Tokenizer<'a> {
        type Item = Token;

        fn next(&mut self) -> Option<Token> {
            // let mut previous_token: Option<Token> = None;
            // let mut return_token: Option<Token> = None;

            let next_char = self.expr.next();
            let x = self.expr.clone().count().to_string();

            //println!("..{:?}, {}", next_char, x);

            match next_char {
                // raw String
                Some('r') => {
                    //
                    let mut value = 'r'.to_string();
                    while let Some(peeking) = self.expr.peek() {
                        match Some(peeking) {
                            // RawString
                            Some('#') => {
                                value.push('#');
                                self.expr.next();
                                while let Some(peek_again) = self.expr.peek() {
                                    match Some(peek_again) {
                                        Some('#') => {
                                            value.push('#');
                                            self.expr.next();
                                            return Some(Token::RawString(value));
                                        }
                                        Some(cc) => {
                                            value.push(*cc);
                                            self.expr.next();
                                        }
                                        None => break,
                                    }
                                }
                                break;
                            }
                            // Word - Catch All
                            Some(cc) if Self::is_word(*cc) => {
                                value.push(*cc);
                                self.expr.next();
                            }
                            Some(_) => {
                                break;
                            }
                            None => break,
                        }
                    }

                    Some(Token::Word(value))
                }
                // Raw Binary String
                Some('b') => {
                     
                    let mut value = 'b'.to_string();
                    while let Some(peeking) = self.expr.peek() {
                        match Some(peeking) {
                            // RawBinaryString(br#"hello"#)
                            Some('r') => {
                                value.push('r');
                                self.expr.next();
                                while let Some(peek_again) = self.expr.peek() {
                                    match Some(peek_again) {
                                        Some('#') => {
                                            value.push('#');
                                            self.expr.next();
                                            while let Some(peek_again) = self.expr.peek() {
                                                match Some(peek_again) {
                                                    Some('#') => {
                                                        value.push('#');
                                                        self.expr.next();
                                                        return Some(Token::RawByteString(value));
                                                    }
                                                    Some(cc) => {
                                                        value.push(*cc);
                                                        self.expr.next();
                                                    }
                                                    None => break,
                                                }
                                            }
                                        }
                                        Some(_) => break,
                                        None => break,
                                    }
                                }
                                break;
                            }
                            // Byte(b'H')
                            Some('\'') => {
                                value.push('\'');
                                self.expr.next();
                                while let Some(peek_again) = self.expr.peek() {
                                    match Some(peek_again) {
                                        Some('\'') => {
                                            value.push('\'');
                                            self.expr.next();
                                            return Some(Token::Byte(value));
                                        }
                                        Some(cc) => {
                                            value.push(*cc);
                                            self.expr.next();
                                        }
                                        None => break,
                                    }
                                }
                            }
                            // ByteString(b"Hello")
                            Some('\"') => {
                                value.push('\"');
                                self.expr.next();
                                while let Some(peek_again) = self.expr.peek() {
                                    match Some(peek_again) {
                                        Some('\"') => {
                                            value.push('\'');
                                            self.expr.next();
                                            return Some(Token::ByteString(value));
                                        }
                                        Some(cc) => {
                                            value.push(*cc);
                                            self.expr.next();
                                        }
                                        None => break,
                                    }
                                }
                            }
                            
                            // Word - Catch All
                            Some(cc) if Self::is_word(*cc) => {
                                value.push(*cc);
                                self.expr.next();
                            }
                            Some(_) => {
                                break;
                            }
                            None => break,
                        }
                    } 

                    Some(Token::Word(value))
                }
 
                // String Value
                Some(c) if Self::is_string_value(c) => {
                    //println!("Inside Some(c) if Self::is_string_value(c) => ");
                    let mut value = c.to_string();
                    while let Some(peeking) = self.expr.peek() {
                        match Some(peeking) {
                            Some('"') => {
                                value.push('"');
                                self.expr.next();
                                break;
                            }
                            Some(c) => {
                                value.push(*c);
                                self.expr.next();
                            }
                            None => break,
                        }
                    }

                    Some(Token::String(value))
                }
                // (3) /* */ //
                Some(c) if Self::is_comment(c) => {
                    //
                    let mut value = c.to_string();
                    let c = self.expr.peek();
                    if c.unwrap() == &'/' && value == "/" {
                        value.push(*c.unwrap());
                        self.expr.next();
                        return Some(Token::LineComment(value));
                    } else if c.unwrap() == &'*' && value == "/" {
                        value.push(*c.unwrap());
                        self.expr.next();
                        return Some(Token::BlockCommentStart(value));
                    } else if c.unwrap() == &'/' && value == "*" {
                        value.push(*c.unwrap());
                        self.expr.next();
                        return Some(Token::BlockCommentStop(value));
                    } else {
                        match Some(value.as_str()) {
                            Some("/") => return Some(Token::Slash),
                            Some("*") => return Some(Token::Star),
                            Some(_) => return Some(Token::Temp((*c.unwrap()).to_string())),
                            None => return Some(Token::Temp((*c.unwrap()).to_string())),
                        }
                    }
                }
                // (9) \x41 \x7F \n \r \t \\ \0 \' \"
                Some(c) if Self::is_escaped(c) => match Some(c) {
                    Some('\x41') => return Some(Token::BitCharacterCode7(c.to_string())),
                    Some('\x7F') => return Some(Token::BitCharacterCode8(c.to_string())),
                    Some('\n') => return Some(Token::Newline),
                    Some('\r') => return Some(Token::CarriageReturn),
                    Some('\t') => return Some(Token::Tab),
                    Some('\\') => return Some(Token::Backslash),
                    Some('\0') => return Some(Token::Null),
                    Some('\'') => return Some(Token::SingleQuote),
                    Some('\"') => return Some(Token::DoubleQuote),
                    Some(_) => return Some(Token::Undefined),
                    None => return Some(Token::Undefined),
                },
                // (1) ' '
                Some(c) if Self::is_whitespace(c) => Some(Token::WhiteSpace),
                // (20) .. ... ..= :: -> @ _ . , ; : # $ ?  ... ..= .. :: ->  -
                Some(c) if Self::is_lesser_punctutation(c) => {
                    //... ..= .. :: ->
                    while let Some(peeking) = self.expr.peek() {
                        match Some(c) {
                            Some('.') => match Some(peeking) {
                                Some('.') => {
                                    self.expr.next();
                                    let x = self.expr.peek();
                                    let x = x.unwrap();
                                    match Some(x) {
                                        Some('.') => {
                                            self.expr.next();
                                            return Some(Token::DotDotDot);
                                        }
                                        Some('=') => {
                                            self.expr.next();
                                            return Some(Token::DotDotEq);
                                        }
                                        Some(cc) if Self::is_dot_or_eq(*cc) => {
                                            return Some(Token::DotDot); //Todo review this
                                        }
                                        Some(_) => return Some(Token::DotDot), //Todo
                                        None => break,
                                    }
                                }

                                Some(_) => break,
                                None => break,
                            },
                            Some(':') => match Some(peeking) {
                                Some(':') => {
                                    self.expr.next();
                                    return Some(Token::PathSep);
                                }
                                Some(_) => break,
                                None => break,
                            },
                            Some('-') => match Some(peeking) {
                                Some('>') => {
                                    self.expr.next();
                                    return Some(Token::RArrow);
                                }
                                Some(_) => break,
                                None => break,
                            },
                            Some(_) => break,
                            None => break,
                        }
                    }

                    match Some(c) {
                        Some('@') => return Some(Token::At),
                        Some('_') => return Some(Token::Underscore),
                        Some('.') => return Some(Token::Dot),
                        Some(',') => return Some(Token::Comma),
                        Some(';') => return Some(Token::Semi),
                        Some(':') => return Some(Token::Colon),
                        Some('#') => return Some(Token::Pound),
                        Some('$') => return Some(Token::Dollar),
                        Some('?') => return Some(Token::Question),
                        Some('-') => return Some(Token::Minus),
                        Some(_) => return Some(Token::Undefined),
                        None => return Some(Token::Undefined),
                    }
                }
                // (8) => << <= >> >= > < =
                Some(c) if Self::is_gt_lt_fat_arrow(c) => {
                    while let Some(peeking) = self.expr.peek() {
                        match Some(c) {
                            Some('=') => match Some(peeking) {
                                Some('>') => {
                                    self.expr.next();
                                    return Some(Token::FatArrow);
                                }
                                Some(_) => break,
                                None => break,
                            },
                            Some('<') => match Some(peeking) {
                                Some('<') => {
                                    self.expr.next();

                                    let x = self.expr.peek();
                                    if x.unwrap() == &'=' {
                                        self.expr.next();
                                        return Some(Token::ShlEq);
                                    }
                                    return Some(Token::Shl);
                                }
                                Some('=') => {
                                    self.expr.next();
                                    return Some(Token::Le);
                                }
                                Some(_) => break,
                                None => break,
                            },
                            Some('>') => match Some(peeking) {
                                Some('>') => {
                                    self.expr.next();

                                    let x = self.expr.peek();
                                    if x.unwrap() == &'=' {
                                        self.expr.next();
                                        return Some(Token::ShrEq);
                                    }
                                    return Some(Token::Shr);
                                }
                                Some('=') => {
                                    self.expr.next();
                                    return Some(Token::Ge);
                                }
                                Some(_) => break,
                                None => break,
                            },
                            Some(_) => break,
                            None => break,
                        }
                    }

                    match Some(c) {
                        Some('>') => return Some(Token::Gt),
                        Some('<') => return Some(Token::Lt),
                        Some('=') => return Some(Token::Eq),
                        Some(_) => return Some(Token::Undefined),
                        None => return Some(Token::Undefined),
                    }
                }
                // (11) + * / % ^ & | = ! && ||
                Some(c) if Self::is_math_operator(c) => {
                    let value = c.to_string();
                    while let Some(peeking) = self.expr.peek() {
                        match Some(peeking) {
                            Some('=') => {
                                self.expr.next();
                                match Some(value.as_str()) {
                                    Some("+") => return Some(Token::PlusEq),
                                    Some("-") => return Some(Token::MinusEq),
                                    Some("*") => return Some(Token::StarEq),
                                    Some("/") => return Some(Token::SlashEq),
                                    Some("%") => return Some(Token::PercentEq),
                                    Some("^") => return Some(Token::CaretEq),
                                    Some("&") => return Some(Token::AndEq),
                                    Some("|") => return Some(Token::OrEq),
                                    Some("=") => return Some(Token::EqEq),
                                    Some("!") => return Some(Token::NotEq),
                                    //Some(">") => return  Some(Token::FatArrow),
                                    Some(_) => {}
                                    None => {}
                                }
                            }
                            Some('&') => {
                                self.expr.next();
                                return Some(Token::AndAnd);
                            }
                            Some('|') => {
                                self.expr.next();
                                return Some(Token::OrOr);
                            }
                            Some(c) => {
                                break;
                            }
                            None => break,
                        }
                    }

                    match Some(value.as_str()) {
                        Some("+") => return Some(Token::Plus),
                        Some("-") => return Some(Token::Minus),
                        Some("*") => return Some(Token::Star),
                        Some("/") => return Some(Token::Slash),
                        Some("%") => return Some(Token::Percent),
                        Some("^") => return Some(Token::Caret),
                        Some("!") => return Some(Token::Not),
                        Some("&") => return Some(Token::And),
                        Some("|") => return Some(Token::Or),
                        Some("=") => return Some(Token::Eq),
                        Some(_) => return Some(Token::Undefined),
                        None => return Some(Token::Undefined),
                    }
                }
                // (6) {}[]()
                Some(c) if Self::bracket_delimiters(c) => match Some(c) {
                    Some('{') => return Some(Token::CurlyBraceLeft),
                    Some('}') => return Some(Token::CurlyBraceRight),
                    Some('[') => return Some(Token::BracketLeft),
                    Some(']') => return Some(Token::BracketRight),
                    Some('(') => return Some(Token::ParenLeft),
                    Some(')') => return Some(Token::ParenRight),
                    Some(_) => return Some(Token::Undefined),
                    None => return Some(Token::Undefined),
                },
                // Numeric, Floating
                Some(c) if Self::is_numeric_with_period(c) => {
                    let mut value = c.to_string();
                    while let Some(peeking) = self.expr.peek() {
                        match Some(peeking) {
                            Some(cc) if Self::is_numeric_with_period(*cc) => {
                                value.push(*cc);
                                self.expr.next();
                            }
                            Some(_) => {
                                break;
                            }
                            None => break,
                        }
                    }

                    if value.contains('.') {
                        return Some(Token::Floating(value));
                    }

                    Some(Token::Numeric(value.parse().unwrap()))
                }
                // true, false
                Some(c) if Self::is_boolean(c) => {
                    //
                    let mut value = c.to_string();
                    while let Some(peeking) = self.expr.peek() {
                        if c == 't' {
                            match Some(peeking) {
                                Some('r') => {
                                    value.push('r');
                                    self.expr.next();
                                    while let Some(peek_again) = self.expr.peek() {
                                        match Some(peek_again) {
                                            Some('u') => {
                                                value.push('u');
                                                self.expr.next();
                                                while let Some(peek_again_again) = self.expr.peek()
                                                {
                                                    match Some(peek_again_again) {
                                                        Some('e') => {
                                                            value.push('e');
                                                            self.expr.next();
                                                            return Some(Token::BoolTrue);
                                                        }
                                                        Some(_) => break,
                                                        None => break,
                                                    }
                                                }
                                                break;
                                            }
                                            Some(_) => break,
                                            None => break,
                                        }
                                    }
                                    break;
                                }
                                Some(_) => break,
                                None => break,
                            }
                        } else if c == 'f' {
                            match Some(peeking) {
                                Some('a') => {
                                    value.push('a');
                                    self.expr.next();
                                    while let Some(peek_again) = self.expr.peek() {
                                        match Some(peek_again) {
                                            Some('l') => {
                                                value.push('l');
                                                self.expr.next();
                                                while let Some(peek_again_again) = self.expr.peek()
                                                {
                                                    match Some(peek_again_again) {
                                                        Some('s') => {
                                                            value.push('s');
                                                            self.expr.next();
                                                            while let Some(peek_again_again) =
                                                                self.expr.peek()
                                                            {
                                                                match Some(peek_again_again) {
                                                                    Some('e') => {
                                                                        value.push('e');
                                                                        self.expr.next();
                                                                        return Some(
                                                                            Token::BoolFalse,
                                                                        );
                                                                    }
                                                                    Some(_) => break,
                                                                    None => break,
                                                                }
                                                            }
                                                            break;
                                                        }
                                                        Some(_) => break,
                                                        None => break,
                                                    }
                                                }
                                                break;
                                            }
                                            Some(_) => break,
                                            None => break,
                                        }
                                    }
                                    break;
                                }
                                Some(_) => break,
                                None => break,
                            }
                        } else {
                            match Some(peeking) {
                                Some(_) => break,
                                None => break,
                            }
                        }
                    }

                    Some(Token::Temp2(value))
                }
                // Word()
                Some(c) if Self::is_word(c) => {
                    let mut value = c.to_string();
                    //println!("Inside ::'{}'", value);
                    while let Some(peeking) = self.expr.peek() {
                        match Some(peeking) {
                            Some(cc) if Self::is_word(*cc) => {
                                value.push(*cc);
                                self.expr.next();
                            }
                            Some(_) => {
                                break;
                            }
                            None => break,
                        }
                    }

                    Some(Token::Word(value))
                }
                // dCharacter()
                Some(c) => Some(Token::Character(c.to_string())),
                None => return Some(Token::Undefined),
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
            //assert_eq!(tokenizer.next().unwrap(), TokenKind::Latin('W'))
        }
    }
}
