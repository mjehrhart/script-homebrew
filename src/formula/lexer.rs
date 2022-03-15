#[allow(unused_variables, dead_code, non_camel_case_types, unused_imports)]
pub mod lexer {
    use super::*;
    use crate::formula::brew_formula;
    use crate::formula::brew_formula::brew_formula::get_tokenkind_map;
    use std::any::type_name;
    use std::collections::HashMap;
    use std::fs::OpenOptions;
    use std::iter::Peekable;
    use std::str::Chars;

    #[derive(Debug, PartialEq, Clone, Eq, Hash)]
    pub enum Kind {
        CRLF,
        Integer,
        Latin,
        Object,
        Comment,
        Whitespace,
    }

    #[derive(Debug, PartialEq, Clone, Eq, Hash)]
    pub enum TokenKind {
        ALPHABETICAL(char),
        WHITESPACE,

        NUMBER(u8),
        Number(String),

        Letter(String),
        Word(String),

        Def,
        End,
        Class { raw: String },
        Variable { raw: String },
        // Latin { raw: char, kind: Kind },
        // Number(u8),
        // Numbers { raw: String, kind: Kind },
        Punctuation(char),
        Value(String),
        Whitespace { raw: char, kind: Kind },
        // EOF,
        Undefined,
        Object(String),
        CRLF { raw: String, kind: Kind },
        Comment,
        Signature { raw: String, kind: Kind },
    }

    // Tokenizer struct contains a Peekable iterator on the expression
    #[derive(Debug, Clone)]
    pub struct Tokenizer<'a> {
        expr: Peekable<Chars<'a>>,
    }

    // Constructs a new instance of Tokenizer
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
                Some('0'..='9') => {
                    let number = next_char?.to_string();
                    let number: u8 = number.parse().unwrap();
                    Some(TokenKind::NUMBER(number))
                }
                Some(' ') => {
                    let var = next_char?.to_string();
                    Some(TokenKind::WHITESPACE)
                }
                Some('A'..='z') => Some(TokenKind::ALPHABETICAL(next_char.unwrap())),

                Some(_) => {
                    let character = next_char?.to_string();
                    let character = character.chars().next();
                    Some(TokenKind::Punctuation(character.unwrap()))
                }
                None => Some(TokenKind::Undefined),
            }
        }
    }

    //Helpers
    #[allow(unused_imports, unused_variables, dead_code)]
    pub fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    // Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_tokenizer() {
            let mut tokenizer = Tokenizer::new("300");
            assert_eq!(tokenizer.next().unwrap(), TokenKind::Punctuation('3'))
        }
    }
}

/*



match next_char {
                // Some('#') => Some(TokenKind::Comment),
                /* Some(' ') => {
                    let whitespace = next_char?.to_string();

                    //let val = whitespace.chars().next();
                    //let val = self.expr.peek();

                    return Some(TokenKind::Whitespace {
                        raw: ' ',
                        kind: Kind::Whitespace,
                    });

                    //Some(TokenKind::Undefined)
                } */
                // Some('\r' | '\n') => {
                //     let character = next_char?.to_string();
                //     let character = character.chars().next();
                //     if let Some(next_char) = self.expr.peek() {
                //         if character == Some('\r') && next_char == &'\n' {
                //             self.expr.next(); //move forward 1 position  TODO review
                //             return Some(TokenKind::CRLF {
                //                 raw: "\r\n".to_string(),
                //                 kind: Kind::CRLF,
                //             });
                //         }
                //     }
                //     Some(TokenKind::CRLF {
                //         raw: "\r\n".to_string(),
                //         kind: Kind::CRLF,
                //     })
                // }
                // Some('<') => Some(TokenKind::Signature {
                //     raw: "<".to_string(),
                //     kind: Kind::Object,
                // }),
                // Some('"') => {
                //     let character = next_char?.to_string();
                //     let character = character.chars().next();
                //     if let Some(next_char) = self.expr.peek() {  //TODO this doesnt make sense
                //         if character.unwrap() == '"' && next_char == &'"' {
                //             return Some(TokenKind::Value);
                //         } else {
                //             return Some(TokenKind::Punctuation(character.unwrap()));
                //         }
                //     }
                //     return Some(TokenKind::Punctuation('-'));
                // }
                Some('A'..='z') => {
                    let alphabetic = next_char?.to_string();
                    let next_alphabetic = alphabetic.chars().peekable().next();

                    let mut catcher = next_alphabetic.unwrap().to_string();
                    while let Some(next) = self.expr.next() {
                        //
                        // if self.expr.peek() == Some(&' ')
                        //     || self.expr.peek() == Some(&'\n')
                        //     || self.expr.peek() == Some(&'\'')
                        // {
                        //     catcher.push(next);
                        //     return Some(TokenKind::Object(catcher));
                        // }

                        match Some(next) {
                            // Some('\'') => {
                            //     //catcher.push(' ');

                            //     return Some(TokenKind::Uri(catcher));
                            // }
                            // Some(' ') => {
                            //     return Some(TokenKind::Whitespace {
                            //         raw: ' ',
                            //         kind: Kind::Whitespace,
                            //     });
                            // }
                            Some('A'..='z') => {
                                //catcher.push(next);
                                return Some(TokenKind::Object(next.to_string()));
                            }
                            Some(_) => {
                                println!("___next {:?} ", next);
                            }
                            None => {
                                println!("NONE {:?} ", next);
                            }
                        }
                    }
                    return Some(TokenKind::Temp(catcher));
                }
                Some(_) => {
                    let character = next_char?.to_string();
                    let character = character.chars().next();
                    //println!("77 ::{:?}", character);
                    Some(TokenKind::Punctuation(character.unwrap()))
                }
                None => Some(TokenKind::Undefined),
            }



*/
