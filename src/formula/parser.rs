#[allow(unused_variables, dead_code, non_camel_case_types, unused_imports)]
pub mod parser {
    use super::*;
    use crate::enums::TokenKind;
    use crate::formula::ast::{BNode, Node};
    use crate::formula::lexer::lexer::Tokenizer;
    use std::error::Error;
    use std::iter::Peekable;
    use std::str::Chars;
    use std::{any::type_name, fmt};

    #[derive(Debug, Clone, Copy)]
    pub struct ExpressionSplitter {
        pub found: bool,
        pub start: Option<usize>,
        pub end: Option<usize>,
    }

    #[derive(Debug, Clone)]
    pub struct Parser<'a> {
        pub current_token: TokenKind,
        pub tokenizer: Tokenizer<'a>,
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

        pub fn get_next_token(&mut self) -> TokenKind {
            let next_token = match self.tokenizer.next() {
                Some(token) => token,
                None => TokenKind::Undefined,
            };
            self.current_token = next_token.clone();
            next_token
        }

        pub fn parse_tokens(mut token_list: Vec<TokenKind>) -> Vec<TokenKind> {
            //Step One::Searches for pair of " ", and remove those nodes after copying value;
            let mut exs = Parser::tmp_get_matching_delim(&token_list);
            while exs.found != false {
                exs = Parser::tmp_get_matching_delim(&token_list);

                if exs.found {
                    let mut catcher = String::from("");
                    for x in (exs.start.unwrap()..=exs.end.unwrap()).rev() {
                        match Some(&token_list[x]) {
                            Some(TokenKind::Latin(val)) => catcher.insert(0, *val),
                            Some(_) => {}
                            None => {}
                        }
                        token_list.remove(x);
                    }
                    //println!("catcher:: {:?}", &catcher);
                    let new_token = TokenKind::Temp(catcher);
                    token_list.insert(exs.start.unwrap(), new_token);
                }
            }

            ///Step Two:: Searches for Words and Letters
            let mut exs = Parser::tmp_get_latin_delim(&token_list);
            while exs.found != false {
                exs = Parser::tmp_get_latin_delim(&token_list);

                if exs.found {
                    let mut token_removal_indexer: Vec<usize> = vec![];
                    let mut catcher = "".to_string();
                    for i in exs.start.unwrap()..token_list.len() {
                        match Some(&token_list[i]) {
                            Some(TokenKind::Latin(char)) => {
                                catcher.push(*char);
                                token_removal_indexer.push(i);
                            }
                            Some(_) => break,
                            None => break,
                        }
                    }

                    for i in (0..token_removal_indexer.len()).rev() {
                        token_list.remove(exs.start.unwrap());
                    }

                    //token_list.remove(exs.start.unwrap());
                    if catcher.len() == 1 {
                        let new_token = TokenKind::Letter(catcher.clone());
                        token_list.insert(exs.start.unwrap(), new_token);
                    } else {
                        let new_token = TokenKind::Word(catcher.clone());
                        token_list.insert(exs.start.unwrap(), new_token);
                    }
                }
            }

            ///Step 3:: Searches for numbers
            let mut exs = Parser::tmp_get_number_delim(&token_list);
            while exs.found != false {
                exs = Parser::tmp_get_number_delim(&token_list);
                let mut token_removal_indexer: Vec<usize> = vec![];
                let mut catcher = String::from("");
                // //
                if exs.start == None {
                    break;
                }

                for i in (exs.start.unwrap()..token_list.len() - 1) {
                    match Some(&token_list[i]) {
                        Some(TokenKind::Digit(num)) => {
                            catcher.push_str(&num.to_string());
                            token_removal_indexer.push(i);
                        }
                        Some(TokenKind::Punctuation('.')) => {
                            catcher.push('.');
                            token_removal_indexer.push(i);
                        }
                        Some(_) => {
                            break;
                        }
                        None => {
                            break;
                        }
                    }
                }

                for i in (0..token_removal_indexer.len()).rev() {
                    token_list.remove(exs.start.unwrap());
                }

                //println!("catcher:: {:?}", &catcher);
                let new_token = TokenKind::Number(catcher);
                token_list.insert(exs.start.unwrap(), new_token);
            }

            println!();
            for token in &token_list {
                println!("{:?}", token);
            }

            token_list
        }
  
        pub fn convert_to_ast_form(token_list: Vec<TokenKind>) -> Vec<crate::formula::ast::Node> {
            //
            let mut node_list: Vec<crate::formula::ast::Node> = vec![];
            for token in &token_list {
                match token {
                    TokenKind::Comment => {
                        let this = Node::Comment(Box::new(BNode {
                            value: '#'.to_string(),
                        }));
                        node_list.push(this);
                    }
                    TokenKind::CRLF => {}
                    TokenKind::EOF => {}
                    TokenKind::Latin(_) => {}
                    TokenKind::Letter(letter) => {
                        let this = Node::Letter(Box::new(BNode {
                            value: letter.clone(),
                        }));
                        node_list.push(this);
                    }
                    TokenKind::Punctuation(punctuation) => {
                        let this = Node::Punctuation(Box::new(BNode {
                            value: punctuation.to_string(),
                        }));
                        node_list.push(this);
                    }
                    TokenKind::Digit(_) => {}
                    TokenKind::Number(number) => {
                        let this = Node::Number(Box::new(BNode {
                            value: number.to_string(),
                        }));
                        node_list.push(this);
                    }
                    TokenKind::Variable(value) => {
                        let this = Node::Variable(Box::new(BNode {
                            value: value.to_string(),
                        }));
                        node_list.push(this);
                    }
                    TokenKind::WhiteSpace => {
                        let this = Node::Punctuation(Box::new(BNode {
                            value: ' '.to_string(),
                        }));
                        node_list.push(this);
                    }
                    TokenKind::Word(word) => {
                        let this = Node::Word(Box::new(BNode {
                            value: word.to_string(),
                        }));
                        node_list.push(this);
                    }
                    TokenKind::Temp(value) => {
                        let this = Node::Variable(Box::new(BNode {
                            value: value.to_string(),
                        }));
                        node_list.push(this);
                    }
                    TokenKind::Undefined => {}
                }
            }

            node_list
        }
    }

    impl<'a> Parser<'a> {
        fn tmp_get_matching_delim(token_list: &Vec<TokenKind>) -> ExpressionSplitter {
            let symbol1 = TokenKind::Punctuation('"');
            let symbol2 = TokenKind::Punctuation('"');

            let flag = token_list.contains(&symbol1) && token_list.contains(&symbol2);

            let mut start = 0;
            for i in 0..token_list.len() {
                if symbol1 == token_list[i] {
                    start = i;
                    break;
                }
            }

            let mut end = 0;
            for i in start + 1..token_list.len() {
                if symbol2 == token_list[i] {
                    end = i;
                    break;
                }
            }
            ExpressionSplitter {
                found: flag,
                start: Some(start),
                end: Some(end),
            }
        }

        fn tmp_get_latin_delim(token_list: &Vec<TokenKind>) -> ExpressionSplitter {
            let mut flag = false;
            let mut start = None;
            for i in 0..token_list.len() {
                match Some(&token_list[i]) {
                    Some(TokenKind::Latin(_)) => {
                        start = Some(i);
                        flag = true;
                        break;
                    }
                    Some(_) => {}
                    None => {}
                }
            }

            ExpressionSplitter {
                found: flag,
                start: start,
                end: None,
            }
        }

        fn tmp_get_number_delim(token_list: &Vec<TokenKind>) -> ExpressionSplitter {
            let mut flag = false;
            let mut start = None;

            for i in 0..token_list.len() {
                match Some(&token_list[i]) {
                    Some(TokenKind::Digit(_)) => {
                        start = Some(i);
                        flag = true;
                        break;
                    }
                    Some(_) => {}
                    None => {}
                }
            }

            ExpressionSplitter {
                found: flag,
                start: start,
                end: None,
            }
        }
    }

    // Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_tokenizer_next() {
            let mut tokenizer = Tokenizer::new("catdog eats food");
            let token = Some(TokenKind::Latin('c'));
            assert_eq!(tokenizer.next(), token)
        }
    }
}

/*

        pub fn find_token_value_by_name(needle: TokenKind, token_list: Vec<TokenKind>) -> String {
            let mut index = 0;
            //let needle = TokenKind::Variable { raw: needle };

            for i in 0..token_list.len() {
                //println!("..{:?}", token_list[i]);

                if token_list[i] == needle {
                    println!("Found it at index {}", i);
                    println!("Found {:?}", token_list[i]);
                    index = i;
                    break;
                }
            }

            let stop1: TokenKind = TokenKind::CRLF;

            let stop: TokenKind = TokenKind::Punctuation('"');

            let mut flag_stop_1 = false; //change to true after stop 1, stop 2 is the second Punctuation('"')
            let mut i = index; //index + next one //TODO index+1 or just index
            let mut catcher: String = String::from("");
            while let Some(token) = Some(&token_list[i]) {
                match token {
                    // TokenKind::Class { raw: val } => {
                    //     catcher.push_str(val);
                    // }
                    // TokenKind::Variable { raw: val } => {
                    //     catcher.push_str(val);
                    // }
                    // TokenKind::Punctuation(char) => {
                    //     catcher.push(*char);
                    // }
                    // TokenKind::Value => {
                    //     catcher.push('"');
                    // }
                    // TokenKind::Whitespace { raw: char, kind: _ } => {
                    //     catcher.push(*char);
                    // }
                    // TokenKind::CRLF { raw: val, kind: _ } => {
                    //     catcher.push_str(val);
                    // }
                    // TokenKind::Comment => {
                    //     catcher.push('#');
                    // }
                    // TokenKind::Object(val) => {
                    //     catcher.push_str(val);
                    // }
                    _ => {}
                }
                if flag_stop_1 && token == &stop {
                    //println!("token at stop2::{:?}", token);
                    //println!("..catcher::{}", catcher);

                    break;
                } else if token == &stop {
                    //println!("token at stop1::{:?}", token);
                    flag_stop_1 = true;
                }
                i += 1;
            }

            catcher
        }

        pub fn find_token(needle: String, token_list: Vec<TokenKind>) -> String {
            let mut index = 0;
            let needle = TokenKind::Variable { raw: needle };
            for i in 0..token_list.len() {
                //println!("..{:?}", token_list[i]);

                if token_list[i] == needle {
                    println!("Found it at index {}", i);
                    println!("Found {:?}", token_list[i]);
                    index = i;
                    break;
                }
            }

            let stop1: TokenKind = TokenKind::CRLF;

            let stop: TokenKind = TokenKind::Punctuation('"');

            let mut flag_stop_1 = false; //change to true after stop 1, stop 2 is the second Punctuation('"')
            let mut i = index; //index + next one
            let mut catcher: String = String::from("");
            while let Some(token) = Some(&token_list[i]) {
                match token {
                    // TokenKind::Class { raw: val } => {
                    //     catcher.push_str(val);
                    // }
                    // TokenKind::Variable { raw: val } => {
                    //     catcher.push_str(val);
                    // }
                    // TokenKind::Punctuation(char) => {
                    //     catcher.push(*char);
                    // }
                    // TokenKind::Value => {
                    //     catcher.push('"');
                    // }
                    // TokenKind::Whitespace { raw: char, kind: _ } => {
                    //     catcher.push(*char);
                    // }
                    // TokenKind::CRLF { raw: val, kind: _ } => {
                    //     catcher.push_str(val);
                    // }
                    // TokenKind::Comment => {
                    //     catcher.push('#');
                    // }

                    // TokenKind::Object(val) => {
                    //     catcher.push_str(val);
                    // }
                    _ => {}
                }
                if flag_stop_1 && token == &stop {
                    //println!("token at stop2::{:?}", token);
                    //println!("..catcher::{}", catcher);
                    break;
                } else if token == &stop {
                    //println!("token at stop1::{:?}", token);
                    flag_stop_1 = true;
                }
                i += 1;
            }

            catcher
        }


*/
