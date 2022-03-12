#[allow(unused_variables, dead_code, non_camel_case_types, unused_imports)]
pub mod lexer {
    use super::*;
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
        Def,
        End,
        Class { raw: String },
        Variable { raw: String },
        Latin { raw: char, kind: Kind },
        Number(u8),
        Numbers { raw: String, kind: Kind },
        Punctuation(char),
        Value,
        Whitespace { raw: char, kind: Kind },
        EOF,
        Undefined,
        ISSUE,
        Object(String),
        Uri(String),
        CRLF { raw: String, kind: Kind },
        Comment,
        Signature { raw: String, kind: Kind },
    }

    // Tokenizer struct contains a Peekable iterator on the arithmetic expression
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
            let mut mapping = HashMap::new();
            mapping.insert(
                String::from("class"),
                TokenKind::Class {
                    raw: String::from(""),
                },
            );
            mapping.insert(
                String::from("homepage"),
                TokenKind::Variable {
                    raw: String::from(""),
                },
            );
            mapping.insert(
                String::from("desc"),
                TokenKind::Variable {
                    raw: String::from(""),
                },
            );
            mapping.insert(
                String::from("url"),
                TokenKind::Variable {
                    raw: String::from(""),
                },
            );
            mapping.insert(
                String::from("version"),
                TokenKind::Variable {
                    raw: String::from(""),
                },
            );
            mapping.insert(
                String::from("sha256"),
                TokenKind::Variable {
                    raw: String::from(""),
                },
            );
            mapping.insert(
                String::from("license"),
                TokenKind::Variable {
                    raw: String::from(""),
                },
            );
            mapping.insert(
                String::from("end"),
                TokenKind::End,
            );
            mapping.insert(
                String::from("def"),
                TokenKind::Def,
            );

            let now = &self.expr;
            //println!("now::{:?}", now);
            let next_char = self.expr.next();
            //println!(".next_char::{:?}", next_char);

            match next_char {
                Some(' ') => {
                    let whitespace = next_char?.to_string();
                    let val = whitespace.chars().next();
                    if val.unwrap().is_whitespace() {
                        return Some(TokenKind::Whitespace {
                            raw: ' ',
                            kind: Kind::Whitespace,
                        });
                    }
                    Some(TokenKind::Undefined)
                }
                Some('\n' | '\r') => {
                    let character = next_char?.to_string();
                    let character = character.chars().next();
                    if let Some(next_char) = self.expr.peek() {
                        if character == Some('\r') && next_char == &'\n' {
                            self.expr.next(); //move forward 1 position
                            return Some(TokenKind::CRLF {
                                raw: "\r\n".to_string(),
                                kind: Kind::CRLF,
                            });
                        }
                    }
                    Some(TokenKind::CRLF {
                        raw: "\r\n".to_string(),
                        kind: Kind::CRLF,
                    })
                }
                Some('#') => Some(TokenKind::Comment),
                Some('<') => Some(TokenKind::Signature {
                    raw: "<".to_string(),
                    kind: Kind::Object,
                }),
                Some('"') => {
                    let character = next_char?.to_string();
                    let character = character.chars().next();

                    if let Some(next_char) = self.expr.next() {
                        if character.unwrap() == '"' && next_char == '"' {
                            return Some(TokenKind::Value);
                        } else {
                            //return Some(TokenKind::Punctuation(character.unwrap()));
                        }
                    }
                    return Some(TokenKind::Punctuation('-'));
                }
                Some('A'..='z') => {
                    let alphabetic = next_char?.to_string();
                    let next_alphabetic = alphabetic.chars().next();

                    let mut catcher = next_alphabetic.unwrap().to_string();
                    while let Some(next) = self.expr.next() {
                        match Some(next) {
                            Some(_) => {
                                catcher.push(next);

                                //println!("22 {:?}, {:?}", next, self.expr.peek());

                                if mapping.contains_key(&catcher) {
                                    let x = mapping.get(&catcher).unwrap();
                                    match x {
                                        TokenKind::Class { raw: _ } => {
                                            return Some(TokenKind::Class { raw: catcher });
                                        }
                                        TokenKind::Variable { raw: _ } => {
                                            return Some(TokenKind::Variable { raw: catcher });
                                        }
                                        TokenKind::End => {
                                            return Some(TokenKind::End);
                                        }
                                        TokenKind::Def => {
                                            return Some(TokenKind::Def);
                                        }
                                        _ => {
                                            return Some(TokenKind::Class { raw: catcher });
                                        }
                                    }
                                }

                                if self.expr.peek() == Some(&' ') || self.expr.peek() == Some(&'\n')
                                {
                                    return Some(TokenKind::Object(catcher));
                                }
                            }
                            None => {}
                        }
                    }
                    //catcher.push('Z');
                    return Some(TokenKind::Object(catcher));
                }
                Some(_) => {
                    let character = next_char?.to_string();
                    let character = character.chars().next();
                    println!("77 ::{:?}", character);
                    Some(TokenKind::Punctuation(character.unwrap())) //character.unwrap()
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
}

/* if let Some(next_char) = self.expr.peek() {
    let mut catcher = alphabetic.unwrap().to_string();
    while let Some(next) = self.expr.next() {
        match Some(next) {
            Some(_) => {}
            None => {}
        }
        /**** */

        if next == '\n' {
            return Some(TokenKind::CRLF {
                raw: "\r\n".to_string(),
                kind: Kind::CRLF,
            });
        } else if (next != '\n') && (next != ' ') {
            //println!("next:0: {:?}", next);
            catcher.push(next)
        } else {
            if mapping.contains_key(&catcher) {
                let x = mapping.get(&catcher).unwrap();
                match x {
                    TokenKind::Class { raw: _ } => {
                        return Some(TokenKind::Class { raw: catcher });
                    }
                    TokenKind::Variable { raw: _ } => {
                        return Some(TokenKind::Variable { raw: catcher });
                    }
                    _ => {}
                }
            }
            return Some(TokenKind::Object(catcher.to_string()));
        }
        /**** */
    }
} */

// Some('\n' | '\r') => {
//     let character = next_char?.to_string();
//     let character = character.chars().next();
//     if let Some(next_char) = self.expr.peek() {
//         if character == Some('\r') && next_char == &'\n' {
//             self.expr.next(); //move forward 1 position
//             return Some(TokenKind::CRLF {
//                 raw: "\r\n".to_string(),
//                 kind: Kind::CRLF,
//             });
//         }
//     }
//     Some(TokenKind::Punctuation(character.unwrap()))
// }
// Some(' ') => {
//     let whitespace = next_char?.to_string();
//     let val = whitespace.chars().next();
//     if val.unwrap().is_whitespace() {
//         return Some(TokenKind::Whitespace {
//             raw: val.unwrap(),
//             kind: Kind::Whitespace,
//         });
//     }

//     Some(TokenKind::Undefined)
// }
