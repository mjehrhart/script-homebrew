//!This is the primary workload for manipulating tokens and nodes

#[allow(unused_variables, dead_code, non_camel_case_types, unused_imports)]
pub mod parser {
    use super::*;
    use crate::enums::TokenKind;
    use crate::formula;
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
        pub node_list: Vec<Node>,
        pub token_list: Vec<TokenKind>,
        exempt_key_words: [&'a str; 41],
    }

    impl<'a> Parser<'a> {
        ///Creates a new Parser
        pub fn new(exp: &'a str) -> Result<Self, String> {
            let mut lexy = Tokenizer::new(exp);
            return match lexy.next() {
                None => Ok(Parser {
                    tokenizer: lexy,
                    current_token: TokenKind::Undefined,
                    node_list: vec![],
                    token_list: vec![],
                    exempt_key_words: [
                        "_ENCODING_",
                        "_LINE_",
                        "_FILE_",
                        "BEGIN",
                        "END",
                        "alias",
                        "and",
                        "begin",
                        "break",
                        "case",
                        "class",
                        "def",
                        "defined?",
                        "do",
                        "else",
                        "elsif",
                        "end",
                        "ensure",
                        "false",
                        "for",
                        "if",
                        "in",
                        "module",
                        "next",
                        "nil",
                        "not",
                        "or",
                        "redo",
                        "rescue",
                        "retry",
                        "return",
                        "self",
                        "super",
                        "then",
                        "true",
                        "undef",
                        "unless",
                        "until",
                        "when",
                        "while",
                        "yield",
                    ],
                }),
                Some(token) => Ok(Parser {
                    tokenizer: lexy,
                    current_token: token,
                    node_list: vec![],
                    token_list: vec![],
                    exempt_key_words: [
                        "_ENCODING_",
                        "_LINE_",
                        "_FILE_",
                        "BEGIN",
                        "END",
                        "alias",
                        "and",
                        "begin",
                        "break",
                        "case",
                        "class",
                        "def",
                        "defined?",
                        "do",
                        "else",
                        "elsif",
                        "end",
                        "ensure",
                        "false",
                        "for",
                        "if",
                        "in",
                        "module",
                        "next",
                        "nil",
                        "not",
                        "or",
                        "redo",
                        "rescue",
                        "retry",
                        "return",
                        "self",
                        "super",
                        "then",
                        "true",
                        "undef",
                        "unless",
                        "until",
                        "when",
                        "while",
                        "yield",
                    ],
                }),
            };
        }

        ///Gets the next token from the Parser
        pub fn get_next_token(&mut self) -> TokenKind {
            let next_token = match self.tokenizer.next() {
                Some(token) => token,
                None => TokenKind::Undefined,
            };
            self.current_token = next_token.clone();
            next_token
        }

        ///Converts tokens to meaningful types and values
        pub fn parse_tokens(mut self) -> Self {
            //Step One::Searches for pair of " ", and remove those nodes after copying value;
            let mut exs = Parser::tmp_get_matching_delim(&self.token_list);
            while exs.found != false {
                exs = Parser::tmp_get_matching_delim(&self.token_list);

                if exs.found {
                    let mut catcher = String::from("");
                    for x in (exs.start.unwrap()..=exs.end.unwrap()).rev() {
                        match Some(&self.token_list[x].clone()) {
                            Some(TokenKind::Latin(val)) => catcher.insert(0, *val),
                            Some(TokenKind::Punctuation(val)) => catcher.insert(0, *val),
                            Some(TokenKind::Digit(number)) => {
                                for num in number.to_string().chars() {
                                    catcher.insert(0, num);
                                }
                            }
                            Some(tk) => {
                                println!("____________________________tk::{:?}", tk);
                            }
                            None => {}
                        }
                        self.token_list.remove(x);
                    }
                    //println!("catcher:: {:?}", &catcher);
                    let new_token = TokenKind::Temp(catcher);
                    self.token_list.insert(exs.start.unwrap(), new_token);
                }
            }

            //Step Two:: Searches for Words and Letters
            let mut exs = Parser::tmp_get_latin_delim(&self.token_list);
            while exs.found != false {
                exs = Parser::tmp_get_latin_delim(&self.token_list);

                if exs.found {
                    let mut token_removal_indexer: Vec<usize> = vec![];
                    let mut catcher = "".to_string();
                    for i in exs.start.unwrap()..self.token_list.len() {
                        match Some(&self.token_list[i]) {
                            Some(TokenKind::Latin(char)) => {
                                catcher.push(*char);
                                token_removal_indexer.push(i);
                            }
                            Some(_) => break,
                            None => break,
                        }
                    }

                    for i in (0..token_removal_indexer.len()).rev() {
                        self.token_list.remove(exs.start.unwrap());
                    }

                    //token_list.remove(exs.start.unwrap());
                    if catcher.len() == 1 {
                        let new_token = TokenKind::Letter(catcher.clone());
                        self.token_list.insert(exs.start.unwrap(), new_token);
                    } else {
                        let new_token = TokenKind::Word(catcher.clone());
                        self.token_list.insert(exs.start.unwrap(), new_token);
                    }
                }
            }

            //Step 3:: Searches for numbers
            let mut exs = Parser::tmp_get_number_delim(&self.token_list);
            while exs.found != false {
                exs = Parser::tmp_get_number_delim(&self.token_list);
                let mut token_removal_indexer: Vec<usize> = vec![];
                let mut catcher = String::from("");

                if exs.start == None {
                    break;
                }

                let mut flag_back_one = false;
                for i in (exs.start.unwrap() - 1)..self.token_list.len() - 1 {
                    match Some(&self.token_list[i]) {
                        Some(TokenKind::Digit(num)) => {
                            catcher.push_str(&num.to_string());
                            token_removal_indexer.push(i);
                            println!("-- -- Digit:{}, {}", i, &num.to_string());
                        }
                        Some(TokenKind::Punctuation('.')) => {
                            catcher.push('.');
                            token_removal_indexer.push(i);
                            println!("-- -- Punctuation:");
                        }
                        Some(TokenKind::Word(word)) => {
                            catcher.push_str(&*word);
                            token_removal_indexer.push(i);
                            //self.token_list.remove(i);
                            flag_back_one = true;
                            println!("-- -- Word:{}, {}", i, word);
                        }
                        Some(_) => {
                            break;
                        }
                        None => {
                            break;
                        }
                    }
                }

                //Remove tokens
                //let mut insert: Option<usize> = None;
                for i in (0..token_removal_indexer.len()).rev() {
                    //insert = Some(token_removal_indexer[i]);
                    self.token_list.remove(token_removal_indexer[i]);
                }

                //Insert newly minted token

                if flag_back_one {
                    let new_token = TokenKind::Word(catcher);
                    self.token_list.insert(exs.start.unwrap() - 1, new_token);
                } else {
                    let new_token = TokenKind::Number(catcher);
                    self.token_list.insert(exs.start.unwrap(), new_token);
                }
            }

            //Step 0:: Match key words
            for i in 0..self.token_list.len() {
                match &self.token_list[i] {
                    TokenKind::Variable(val) => {
                        let value = val;
                        if self.exempt_key_words.contains(&value.as_str()) {
                            println!("@...{}", &value.as_str())
                        }
                    }
                    TokenKind::Word(val) => {
                        let value1 = val;
                        if self.exempt_key_words.contains(&value1.as_str()) {
                            println!("#...{}, {}", i, &value1.as_str());

                            let token = TokenKind::KeyWord((&value1.as_str()).to_string());
                            self.token_list.remove(i);
                            self.token_list.insert(i, token);
                            //break; //TODO comment this out for looping
                        }
                    }
                    TokenKind::Temp(val) => {
                        let value = val;
                        if self.exempt_key_words.contains(&value.as_str()) {
                            println!("$...{}", &value.as_str())
                        }
                    }
                    _ => {}
                }
            }

            Self {
                current_token: self.current_token,
                tokenizer: self.tokenizer,
                node_list: self.node_list,
                token_list: self.token_list,
                exempt_key_words: self.exempt_key_words,
            }
        }

        /// Transforms the Parser's tokens into AST nodes,
        pub fn convert_to_ast_nodes(mut self) -> Self {
            //
            for token in &self.token_list {
                match token {
                    TokenKind::Comment => {
                        let this = Node::Comment(Box::new(BNode {
                            value: '#'.to_string(),
                        }));
                        self.node_list.push(this);
                    }
                    TokenKind::Letter(letter) => {
                        let this = Node::Letter(Box::new(BNode {
                            value: letter.clone(),
                        }));
                        self.node_list.push(this);
                    }
                    TokenKind::Punctuation(punctuation) => {
                        let this = Node::Punctuation(Box::new(BNode {
                            value: punctuation.to_string(),
                        }));
                        self.node_list.push(this);
                    }
                    TokenKind::Number(number) => {
                        let this = Node::Number(Box::new(BNode {
                            value: number.to_string(),
                        }));
                        self.node_list.push(this);
                    }
                    TokenKind::Variable(value) => {
                        let this = Node::Variable(Box::new(BNode {
                            value: value.to_string(),
                        }));
                        self.node_list.push(this);
                    }
                    TokenKind::WhiteSpace => {
                        let this = Node::WhiteSpace;
                        self.node_list.push(this);
                    }
                    TokenKind::Word(word) => {
                        let this = Node::Word(Box::new(BNode {
                            value: word.to_string(),
                        }));
                        self.node_list.push(this);
                    }
                    TokenKind::Temp(value) => {
                        let this = Node::Variable(Box::new(BNode {
                            value: value.to_string(),
                        }));
                        self.node_list.push(this);
                    }
                    TokenKind::Undefined => {}
                    TokenKind::Latin(_) => {}
                    TokenKind::Digit(_) => {}
                    TokenKind::KeyWord(word) => {
                        let this = Node::KeyWord(Box::new(BNode {
                            value: word.to_string(),
                        }));
                        self.node_list.push(this);
                    }
                    TokenKind::CRLF => {}
                    TokenKind::EOF => {}
                }
            }

            Self {
                current_token: self.current_token,
                tokenizer: self.tokenizer,
                node_list: self.node_list,
                token_list: self.token_list,
                exempt_key_words: self.exempt_key_words,
            }
        }

        /// Takes a Vec<Node> and searches for all Node::Variable. Then transform those nodes into Node::Assignment(_,_)
        pub fn transform_nodes_to_assignment_nodes(mut self) -> Self {
            let mut exs = Parser::tmp_get_node_variable_delim(&self.node_list);
            //println!("exs::{:?}", exs);

            while exs.found != false {
                exs = Parser::tmp_get_node_variable_delim(&self.node_list);
                //println!("exs::{:?}", exs);

                //// Find node

                println!();
                //Step One, find Node::Variable and work backwards
                let mut variable_index = 0;
                let mut variable_value = "";
                if exs.start != None {
                    for i in exs.start.unwrap()..self.node_list.len() - 1 {
                        match &self.node_list[i] {
                            Node::Variable(val) => {
                                let b_node = val;
                                variable_value = &b_node.value;
                                println!("-Node.value..{:?}", b_node);
                                variable_index = i;
                                break;
                            }
                            _ => {}
                        }
                    }
                    println!("index at {}", variable_index);
                }

                //Step Two, find corresponding Word or Letter
                let mut node_index = 0;
                let mut node_value = "";
                for i in (0..=variable_index).rev() {
                    match &self.node_list[i] {
                        Node::Letter(val) => {
                            let b_node = val;
                            node_value = &b_node.value;
                            node_index = i;
                            break;
                        }
                        Node::Word(val) => {
                            let b_node = val;
                            node_value = &b_node.value;
                            node_index = i;
                            break;
                        }
                        _ => {}
                    }
                }

                if variable_value != "" {
                    println!(
                        "matching node is {}, {:?}",
                        node_index, &self.node_list[node_index]
                    );

                    //Print node
                    let bn1 = BNode {
                        value: node_value.to_string(),
                    };
                    let bn2 = BNode {
                        value: variable_value.to_string(),
                    };
                    let box1 = Box::new(bn1);
                    let box2 = Box::new(bn2);

                    let b_node = Node::Assignment(box1, box2);

                    self.node_list.remove(variable_index); //TODO, remove the exta Whitespace but not urgent
                    self.node_list.remove(node_index);
                    self.node_list.insert(node_index, b_node);
                }
            }

            Self {
                current_token: self.current_token,
                tokenizer: self.tokenizer,
                node_list: self.node_list,
                token_list: self.token_list,
                exempt_key_words: self.exempt_key_words,
            }
        }

        pub fn transform_nodes_to_keyword_nodes(mut self) -> Self {
            fn tmp_get_node_keyword_delim(node_list: &Vec<Node>) -> ExpressionSplitter {
                let mut flag = false;
                let mut start = None;

                for i in 0..node_list.len() {
                    match Some(&node_list[i]) {
                        Some(Node::KeyWord(_)) => {
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

            println!("__ pub fn transform_nodes_to_keyword_nodes");

            let mut exs = tmp_get_node_keyword_delim(&self.node_list);
            while exs.found != false {
                exs = tmp_get_node_keyword_delim(&self.node_list);

                //// Find node
                println!();
                //Step One, find Node::Variable and work backwards
                let mut variable_index = 0;
                let mut variable_value = "";
                if exs.start == None {
                    break;
                }

                for i in exs.start.unwrap()..self.node_list.len() {
                    match &self.node_list[i] {
                        //
                        Node::KeyWord(val) => {
                            let b_node = val;
                            variable_value = &b_node.value;
                            println!("#### Keyword::Node.value..{:?}", b_node);
                            variable_index = i;
                            break;
                        }
                        _ => {}
                    }
                }
                println!("index at {}", variable_index);

                //Step Two, find corresponding Word or Letter
                let mut node_index = 0;
                let mut node_value = "";
                for i in variable_index..self.node_list.len() {
                    println!("i={:?}", i);
                    match &self.node_list[i] {
                        Node::Letter(val) => {
                            let b_node = val;
                            node_value = &b_node.value;
                            node_index = i;
                            println!("______Letter::Node.value..{:?}", b_node);
                            println!("*i=={:?}", i);
                            break;
                        }
                        Node::Word(val) => {
                            let b_node = val;
                            node_value = &b_node.value;
                            println!("_______Word::Node.value..{:?}", b_node);
                            println!("^i=={:?}", i);
                            node_index = i;
                            break;
                        }
                        node => {
                            println!("_______others {:?}", node);
                        }
                    }
                }

                if variable_value != "" {
                    println!(
                        "matching Keyword variable is {}, {:?}",
                        variable_index, &variable_value
                    );

                    println!(
                        "matching letter/word node is {}, {:?}",
                        node_index, &node_value
                    );

                    let bn1 = BNode {
                        value: node_value.to_string(),
                    };
                    let bn2 = BNode {
                        value: variable_value.to_string(),
                    };
                    let box1 = Box::new(bn1);
                    let box2 = Box::new(bn2);
                    let b_node = Node::Class(box2, box1);

                    if &node_value != &"" {
                        self.node_list.remove(node_index);
                    }
                    
                    self.node_list.remove(variable_index);
                    self.node_list.insert(variable_index, b_node);

                     
                }
            }

            Self {
                current_token: self.current_token,
                tokenizer: self.tokenizer,
                node_list: self.node_list,
                token_list: self.token_list,
                exempt_key_words: self.exempt_key_words,
            }
        }

        /// Updates the value for an assignment node
        pub fn update_node_assignment(mut self, name: String, new_value: String) -> Self {
            println!("pub update_node_assignment");

            let mut index = 0;
            for i in 0..self.node_list.len() {
                match &self.node_list[i] {
                    Node::Assignment(val, _) => {
                        let b_node = val;
                        if b_node.value == name {
                            println!("..______name::{} {:?}", i, name);
                            index = i;
                        }
                    }
                    _ => {}
                }
            }

            self.node_list.remove(index);

            let string_list = vec!["\"".to_string(), new_value, "\"".to_string()];
            let joined = string_list.join("");

            let node = Node::Assignment(
                Box::new(BNode {
                    value: name.to_string(),
                }),
                Box::new(BNode {
                    value: joined.to_string(),
                }),
            );
            self.node_list.insert(index, node);

            Self {
                current_token: self.current_token,
                tokenizer: self.tokenizer,
                node_list: self.node_list,
                token_list: self.token_list,
                exempt_key_words: self.exempt_key_words,
            }
        }

        pub fn print_tokens(self) -> Self {
            //Display for testing purposes TOKEN_LIST
            println!();
            let mut i = 0;
            for token in &self.token_list {
                println!("{}. {:?}", i, token);
                i += 1;
            }

            Self {
                current_token: self.current_token,
                tokenizer: self.tokenizer,
                node_list: self.node_list,
                token_list: self.token_list,
                exempt_key_words: self.exempt_key_words,
            }
        }

        pub fn print_nodes(self) -> Self {
            //Display for testing purposes NODE_LIST
            println!();
            let mut i = 0;
            for node in &self.node_list {
                println!("{}. {:?}", i, node);
                i += 1;
            }

            Self {
                current_token: self.current_token,
                tokenizer: self.tokenizer,
                node_list: self.node_list,
                token_list: self.token_list,
                exempt_key_words: self.exempt_key_words,
            }
        }
    }

    impl<'a> Parser<'a> {
        /// Looks for Letters and Word tokens
        fn tmp_get_node_variable_delim(node_list: &Vec<Node>) -> ExpressionSplitter {
            let mut flag = false;
            let mut start = None;

            for i in 0..node_list.len() {
                match Some(&node_list[i]) {
                    Some(Node::Variable(_)) => {
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

        /// Searches for mathgin Punctuation("'");
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

        /// Looks for Letters and Word tokens
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

        /// Forms numbers from tokens
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
