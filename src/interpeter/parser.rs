// //!This is the primary workload for manipulating tokens and nodes

// #[allow(
//     unused_variables,
//     dead_code,
//     non_camel_case_types,
//     unused_imports,
//     clippy::module_inception
// )]
// pub mod parser {
//     use super::*;
//     use crate::enums::{TokenKind, Token};
//     use crate::interpeter;
//     use crate::interpeter::ast::{BNode, Node};
//     use crate::interpeter::lexer::lexer::Tokenizer;
//     use std::error::Error;
//     use std::iter::Peekable;
//     use std::str::Chars;
//     use std::{any::type_name, fmt};

//     #[derive(Debug, Clone, Copy)]
//     pub struct ExpressionSplitter {
//         pub found: bool,
//         pub start: Option<usize>,
//         pub end: Option<usize>,
//     }

//     #[derive(Debug, Clone)]
//     pub struct Parser<'a> {
//         pub current_token: Token,
//         pub tokenizer: Tokenizer<'a>,
//         pub node_list: Vec<Node>,
//         pub token_list: Vec<TokenKind>,
//         pub exempt_key_words: [&'a str; 41],
//     }

//     impl<'a> Parser<'a> {
//         /// Creates a new Parser instance.
//         /// Example
//         /// ```
//         /// let exp = "Water is helpful"
//         /// let mut parsie = formula::parser::parser::Parser::new(&exp);
//         /// ```
//         pub fn new(exp: &'a str) -> Result<Self, String> {
//             let mut lexy = Tokenizer::new(exp);
//             match lexy.next() {
//                 None => Ok(Parser {
//                     tokenizer: lexy,
//                     current_token: Token::Undefined,
//                     node_list: vec![],
//                     token_list: vec![],
//                     exempt_key_words: [
//                         "_ENCODING_",
//                         "_LINE_",
//                         "_FILE_",
//                         "BEGIN",
//                         "END",
//                         "alias",
//                         "and",
//                         "begin",
//                         "break",
//                         "case",
//                         "class",
//                         "def",
//                         "defined?",
//                         "do",
//                         "else",
//                         "elsif",
//                         "end",
//                         "ensure",
//                         "false",
//                         "for",
//                         "if",
//                         "in",
//                         "module",
//                         "next",
//                         "nil",
//                         "not",
//                         "or",
//                         "redo",
//                         "rescue",
//                         "retry",
//                         "return",
//                         "self",
//                         "super",
//                         "then",
//                         "true",
//                         "undef",
//                         "unless",
//                         "until",
//                         "when",
//                         "while",
//                         "yield",
//                     ],
//                 }),
//                 Some(token) => Ok(Parser {
//                     tokenizer: lexy,
//                     current_token: token,
//                     node_list: vec![],
//                     token_list: vec![],
//                     exempt_key_words: [
//                         "_ENCODING_",
//                         "_LINE_",
//                         "_FILE_",
//                         "BEGIN",
//                         "END",
//                         "alias",
//                         "and",
//                         "begin",
//                         "break",
//                         "case",
//                         "class",
//                         "def",
//                         "defined?",
//                         "do",
//                         "else",
//                         "elsif",
//                         "end",
//                         "ensure",
//                         "false",
//                         "for",
//                         "if",
//                         "in",
//                         "module",
//                         "next",
//                         "nil",
//                         "not",
//                         "or",
//                         "redo",
//                         "rescue",
//                         "retry",
//                         "return",
//                         "self",
//                         "super",
//                         "then",
//                         "true",
//                         "undef",
//                         "unless",
//                         "until",
//                         "when",
//                         "while",
//                         "yield",
//                     ],
//                 }),
//             }
//         }

//         ///Gets the next token from the Parser
//         /// Example
//         /// ```
//         /// for _c in catcher.chars() {
//         ///   let y = parsie.get_next_token();
//         ///   if y != TokenKind::Undefined {
//         ///     token_list.push(y)
//         ///   }
//         /// }
//         /// ```
//         pub fn get_next_token(&mut self) -> Token {
//             let next_token = match self.tokenizer.next() {
//                 Some(token) => token,
//                 None => Token::Undefined,
//             };
//             self.current_token = next_token.clone();
//             next_token
//         }

//         ///Converts tokens to meaningful types and values
//         /// Example
//         /// ```
//         /// let parsie = parsie
//         ///     .parse_tokens()
//         /// ```
//         pub fn parse_tokens(mut self) -> Self {
//             //Step One::Searches for pair of " ", and remove those nodes after copying value;
//             let mut exs = Parser::tmp_get_matching_delim(&self.token_list);
//             while exs.found {
//                 exs = Parser::tmp_get_matching_delim(&self.token_list);

//                 if exs.found {
//                     let mut catcher = String::from("");
//                     for x in (exs.start.unwrap()..=exs.end.unwrap()).rev() {
//                         match Some(&self.token_list[x].clone()) {
//                             Some(TokenKind::Latin(val)) => catcher.insert(0, *val),
//                             Some(TokenKind::Punctuation(val)) => catcher.insert(0, *val),
//                             Some(TokenKind::Digit(number)) => {
//                                 for num in number.to_string().chars() {
//                                     catcher.insert(0, num);
//                                 }
//                             }
//                             Some(tk) => {
//                                 //println!("____________________________tk::{:?}", tk);
//                             }
//                             None => {}
//                         }
//                         self.token_list.remove(x);
//                     }
//                     //println!("catcher:: {:?}", &catcher);
//                     let new_token = TokenKind::Temp(catcher);
//                     self.token_list.insert(exs.start.unwrap(), new_token);
//                 }
//             }

//             //Step Two:: Searches for Words and Letters
//             let mut exs = Parser::tmp_get_latin_delim(&self.token_list);
//             while exs.found {
//                 exs = Parser::tmp_get_latin_delim(&self.token_list);

//                 if exs.found {
//                     let mut token_removal_indexer: Vec<usize> = vec![];
//                     let mut catcher = "".to_string();
//                     for i in exs.start.unwrap()..self.token_list.len() {
//                         match Some(&self.token_list[i]) {
//                             Some(TokenKind::Latin(char)) => {
//                                 catcher.push(*char);
//                                 token_removal_indexer.push(i);
//                             }
//                             Some(_) => break,
//                             None => break,
//                         }
//                     }

//                     for i in (0..token_removal_indexer.len()).rev() {
//                         self.token_list.remove(exs.start.unwrap());
//                     }

//                     //token_list.remove(exs.start.unwrap());
//                     if catcher.len() == 1 {
//                         let new_token = TokenKind::Letter(catcher.clone());
//                         self.token_list.insert(exs.start.unwrap(), new_token);
//                     } else {
//                         let new_token = TokenKind::Word(catcher.clone());
//                         self.token_list.insert(exs.start.unwrap(), new_token);
//                     }
//                 }
//             }

//             //Step 3:: Searches for numbers
//             let mut exs = Parser::tmp_get_number_delim(&self.token_list);
//             while exs.found {
//                 exs = Parser::tmp_get_number_delim(&self.token_list);
//                 let mut token_removal_indexer: Vec<usize> = vec![];
//                 let mut catcher = String::from("");

//                 if exs.start == None {
//                     break;
//                 }

//                 let mut flag_back_one = false;
//                 for i in (exs.start.unwrap() - 1)..self.token_list.len() - 1 {
//                     match Some(&self.token_list[i]) {
//                         Some(TokenKind::Digit(num)) => {
//                             catcher.push_str(&num.to_string());
//                             token_removal_indexer.push(i);
//                             //println!("-- -- Digit:{}, {}", i, &num.to_string());
//                         }
//                         Some(TokenKind::Punctuation('.')) => {
//                             catcher.push('.');
//                             token_removal_indexer.push(i);
//                             //println!("-- -- Punctuation:");
//                         }
//                         Some(TokenKind::Word(word)) => {
//                             catcher.push_str(&*word);
//                             token_removal_indexer.push(i);
//                             //self.token_list.remove(i);
//                             flag_back_one = true;
//                             //println!("-- -- Word:{}, {}", i, word);
//                         }
//                         Some(_) => {
//                             break;
//                         }
//                         None => {
//                             break;
//                         }
//                     }
//                 }

//                 //Remove tokens
//                 //let mut insert: Option<usize> = None;
//                 for i in (0..token_removal_indexer.len()).rev() {
//                     //insert = Some(token_removal_indexer[i]);
//                     self.token_list.remove(token_removal_indexer[i]);
//                 }

//                 //Insert newly minted token

//                 if flag_back_one {
//                     let new_token = TokenKind::Word(catcher);
//                     self.token_list.insert(exs.start.unwrap() - 1, new_token);
//                 } else {
//                     let new_token = TokenKind::Number(catcher);
//                     self.token_list.insert(exs.start.unwrap(), new_token);
//                 }
//             }

//             //Step 0:: Match key words
//             for i in 0..self.token_list.len() {
//                 match &self.token_list[i] {
//                     TokenKind::Variable(val) => {
//                         let value = val;
//                         if self.exempt_key_words.contains(&value.as_str()) {
//                             //println!("@...{}", &value.as_str())
//                         }
//                     }
//                     TokenKind::Word(val) => {
//                         let value1 = val;
//                         if self.exempt_key_words.contains(&value1.as_str()) {
//                             //println!("#...{}, {}", i, &value1.as_str());

//                             let token = TokenKind::KeyWord((&value1.as_str()).to_string());
//                             self.token_list.remove(i);
//                             self.token_list.insert(i, token);
//                             //break; //TODO comment this out for looping
//                         }
//                     }
//                     TokenKind::Temp(val) => {
//                         let value = val;
//                         if self.exempt_key_words.contains(&value.as_str()) {
//                             //println!("$...{}", &value.as_str())
//                         }
//                     }
//                     _ => {}
//                 }
//             }

//             Self {
//                 current_token: self.current_token,
//                 tokenizer: self.tokenizer,
//                 node_list: self.node_list,
//                 token_list: self.token_list,
//                 exempt_key_words: self.exempt_key_words,
//             }
//         }

//         /// Loop through char vector to populate the token_list
//         pub fn intialize_tokens(mut self, chars: String) -> Self {
//             for _c in chars.chars() {
//                 let y = self.get_next_token();
//                 if y != crate::enums::Token::Undefined {
//                     println!(".....{:?}", &y);
//                     self.token_list.push(y);
//                 }
//             }

//             Self {
//                 current_token: self.current_token,
//                 tokenizer: self.tokenizer,
//                 node_list: self.node_list,
//                 token_list: self.token_list,
//                 exempt_key_words: self.exempt_key_words,
//             }
//         }
//         /// Transforms the Parser's tokens into AST nodes,
//         /// Example
//         /// ```
//         /// let parsie = parsie
//         ///     .parse_tokens()
//         ///     .convert_to_ast_nodes()
//         /// ```
//         pub fn convert_to_ast_nodes(mut self) -> Self {
//             //
//             for token in &self.token_list {
//                 match token {
//                     TokenKind::Comment => {
//                         let this = Node::Comment(Box::new(BNode {
//                             value: '#'.to_string(),
//                         }));
//                         self.node_list.push(this);
//                     }
//                     TokenKind::Letter(letter) => {
//                         let this = Node::Letter(Box::new(BNode {
//                             value: letter.clone(),
//                         }));
//                         self.node_list.push(this);
//                     }
//                     TokenKind::Punctuation(punctuation) => {
//                         let this = Node::Punctuation(Box::new(BNode {
//                             value: punctuation.to_string(),
//                         }));
//                         self.node_list.push(this);
//                     }
//                     TokenKind::Number(number) => {
//                         let this = Node::Number(Box::new(BNode {
//                             value: number.to_string(),
//                         }));
//                         self.node_list.push(this);
//                     }
//                     TokenKind::Variable(value) => {
//                         let this = Node::Variable(Box::new(BNode {
//                             value: value.to_string(),
//                         }));
//                         self.node_list.push(this);
//                     }
//                     TokenKind::WhiteSpace => {
//                         let this = Node::WhiteSpace;
//                         self.node_list.push(this);
//                     }
//                     TokenKind::Word(word) => {
//                         let this = Node::Word(Box::new(BNode {
//                             value: word.to_string(),
//                         }));
//                         self.node_list.push(this);
//                     }
//                     TokenKind::Temp(value) => {
//                         let this = Node::Variable(Box::new(BNode {
//                             value: value.to_string(),
//                         }));
//                         self.node_list.push(this);
//                     }
//                     TokenKind::Undefined => {}
//                     TokenKind::Latin(_) => {}
//                     TokenKind::Digit(_) => {}
//                     TokenKind::KeyWord(word) => {
//                         let this = Node::KeyWord(Box::new(BNode {
//                             value: word.to_string(),
//                         }));
//                         self.node_list.push(this);
//                     }
//                     TokenKind::CRLF => {}
//                     TokenKind::EOF => {}
//                     _ => {}
//                 }
//             }

//             Self {
//                 current_token: self.current_token,
//                 tokenizer: self.tokenizer,
//                 node_list: self.node_list,
//                 token_list: self.token_list,
//                 exempt_key_words: self.exempt_key_words,
//             }
//         }

//         /// Takes a Vec<Node> and searches for all Node::Variable. Then transform those nodes into Node::Assignment(_,_)
//         /// Example
//         /// ```
//         /// let parsie = parsie
//         ///     .parse_tokens()
//         ///     .convert_to_ast_nodes()
//         ///     .transform_nodes_to_assignment_nodes();
//         /// ```
//         pub fn construct_expression_nodes(mut self) -> Self {
//             let mut exs = Parser::tmp_get_node_variable_delim(&self.node_list);
//             //println!("exs::{:?}", exs);

//             while exs.found {
//                 exs = Parser::tmp_get_node_variable_delim(&self.node_list); 
//                 println!();
//                 //Step One, find Node::Variable and work backwards
//                 let mut variable_index = 0;
//                 let mut variable_value = "";
//                 if exs.start != None {
//                     for i in exs.start.unwrap()..self.node_list.len() - 1 { 
//                         if let Node::Variable(val) = &self.node_list[i] {
//                             let b_node = val;
//                             variable_value = &b_node.value; 
//                             variable_index = i;
//                             break;
//                         }
//                     } 
//                 }

//                 //Step Two, find corresponding Word or Letter
//                 let mut node_index = 0;
//                 let mut node_value = "";
//                 for i in (0..=variable_index).rev() {
//                     match &self.node_list[i] {
//                         Node::Letter(val) => {
//                             let b_node = val;
//                             node_value = &b_node.value;
//                             node_index = i;
//                             break;
//                         }
//                         Node::Word(val) => {
//                             let b_node = val;
//                             node_value = &b_node.value;
//                             node_index = i;
//                             break;
//                         }
//                         _ => {}
//                     }
//                 }

//                 if !variable_value.is_empty() {
//                     /* println!(
//                         "matching node is {}, {:?}",
//                         node_index, &self.node_list[node_index]
//                     ); */

//                     //Print node
//                     let bn1 = BNode {
//                         value: node_value.to_string(),
//                     };
//                     let bn2 = BNode {
//                         value: variable_value.to_string(),
//                     };
//                     let box1 = Box::new(bn1);
//                     let box2 = Box::new(bn2);

//                     let b_node = Node::Expression(box1, box2);

//                     self.node_list.remove(variable_index); //TODO, remove the exta Whitespace but not urgent
//                     self.node_list.remove(node_index);
//                     self.node_list.insert(node_index, b_node);
//                 }
//             }

//             Self {
//                 current_token: self.current_token,
//                 tokenizer: self.tokenizer,
//                 node_list: self.node_list,
//                 token_list: self.token_list,
//                 exempt_key_words: self.exempt_key_words,
//             }
//         }

//         /// Parses self.node_list for keywords and creates new nodes depending on value
//         /// Example
//         /// ```
//         /// let parsie = parsie
//         ///     .parse_tokens()
//         ///     .convert_to_ast_nodes()
//         ///     .transform_nodes_to_assignment_nodes()
//         ///     .transform_nodes_to_keyword_nodes();
//         /// ```
//         pub fn construct_keyword_nodes(mut self) -> Self {
//             fn tmp_get_node_keyword_delim(node_list: &[Node]) -> ExpressionSplitter {
//                 let mut flag = false;
//                 let mut start = None;

//                 //for i in 0..node_list.len() {
//                 for (i, node) in node_list.iter().enumerate() {
//                     match Some(&node_list[i]) {
//                         Some(Node::KeyWord(_)) => {
//                             start = Some(i);
//                             flag = true;
//                             break;
//                         }
//                         Some(_) => {}
//                         None => {}
//                     }
//                 }

//                 ExpressionSplitter {
//                     found: flag,
//                     start,
//                     end: None,
//                 }
//             }

//             //println!("__ pub fn transform_nodes_to_keyword_nodes");

//             let mut exs = tmp_get_node_keyword_delim(&self.node_list);
//             while exs.found {
//                 exs = tmp_get_node_keyword_delim(&self.node_list);

//                 //// Find node
//                 println!();
//                 //Step One, find Node::Variable and work backwards
//                 let mut variable_index = 0;
//                 let mut variable_value = "";
//                 if exs.start == None {
//                     break;
//                 }

//                 for i in exs.start.unwrap()..self.node_list.len() {
//                     // match &self.node_list[i] {
//                     //     //
//                     //     Node::KeyWord(val) => {
//                     //         let b_node = val;
//                     //         variable_value = &b_node.value;
//                     //         println!("#### Keyword::Node.value..{:?}", b_node);
//                     //         variable_index = i;
//                     //         break;
//                     //     }
//                     //     _ => {}
//                     // }
//                     if let Node::KeyWord(val) = &self.node_list[i] {
//                         let b_node = val;
//                         variable_value = &b_node.value;
//                         println!("#### Keyword::Node.value..{:?}", b_node);
//                         variable_index = i;
//                         break;
//                     }
//                 }
//                 //println!("index at {}", variable_index);

//                 //Step Two, find corresponding Word or Letter
//                 let mut node_index = 0;
//                 let mut node_value = "";
//                 for i in variable_index..self.node_list.len() {
//                     //println!("i={:?}", i);
//                     match &self.node_list[i] {
//                         Node::Letter(val) => {
//                             let b_node = val;
//                             node_value = &b_node.value;
//                             node_index = i;
//                             //println!("______Letter::Node.value..{:?}", b_node);
//                             //println!("*i=={:?}", i);
//                             break;
//                         }
//                         Node::Word(val) => {
//                             let b_node = val;
//                             node_value = &b_node.value;
//                             //println!("_______Word::Node.value..{:?}", b_node);
//                             //println!("^i=={:?}", i);
//                             node_index = i;
//                             break;
//                         }
//                         node => {
//                             //println!("_______others {:?}", node);
//                         }
//                     }
//                 }

//                 if !variable_value.is_empty() {
//                     /* println!(
//                         "matching Keyword variable is {}, {:?}",
//                         variable_index, &variable_value
//                     );

//                     println!(
//                         "matching letter/word node is {}, {:?}",
//                         node_index, &node_value
//                     ); */

//                     let bn1 = BNode {
//                         value: node_value.to_string(),
//                     };
//                     let bn2 = BNode {
//                         value: variable_value.to_string(),
//                     };
//                     let box1 = Box::new(bn1);
//                     let box2 = Box::new(bn2);
//                     let b_node = Node::Class(box2, box1);

//                     if !node_value.is_empty() {
//                         self.node_list.remove(node_index);
//                     }

//                     self.node_list.remove(variable_index);
//                     self.node_list.insert(variable_index, b_node);
//                 }
//             }

//             Self {
//                 current_token: self.current_token,
//                 tokenizer: self.tokenizer,
//                 node_list: self.node_list,
//                 token_list: self.token_list,
//                 exempt_key_words: self.exempt_key_words,
//             }
//         }

//         /// Updates the value for an assignment node
//         /// Example
//         /// ```
//         /// let parsie = parsie
//         ///     .parse_tokens()
//         ///     .convert_to_ast_nodes()
//         ///     .transform_nodes_to_assignment_nodes()
//         ///     .transform_nodes_to_keyword_nodes()
//         ///     .update_node_assignment(String::from("desc"), String::from("my cool description!"));
//         /// ```
//         pub fn update_node_assignment(mut self, name: String, new_value: String) -> Self {
//             //println!("pub update_node_assignment");

//             let mut index = 0;
//             for i in 0..self.node_list.len() {
//                 // match &self.node_list[i] {
//                 //     Node::Assignment(val, _) => {
//                 //         let b_node = val;
//                 //         if b_node.value == name {
//                 //             println!("..______name::{} {:?}", i, name);
//                 //             index = i;
//                 //         }
//                 //     }
//                 //     _ => {}
//                 // }
//                 if let Node::Expression(val, _) = &self.node_list[i] {
//                     let b_node = val;
//                     if b_node.value == name {
//                         //println!("..______name::{} {:?}", i, name);
//                         index = i;
//                     }
//                 }
//             }

//             self.node_list.remove(index);

//             let string_list = vec!["\"".to_string(), new_value, "\"".to_string()];
//             let joined = string_list.join("");

//             let node = Node::Expression(
//                 Box::new(BNode { value: name }),
//                 Box::new(BNode { value: joined }),
//             );
//             self.node_list.insert(index, node);

//             Self {
//                 current_token: self.current_token,
//                 tokenizer: self.tokenizer,
//                 node_list: self.node_list,
//                 token_list: self.token_list,
//                 exempt_key_words: self.exempt_key_words,
//             }
//         }

//         /// Prints the current Parser::token_list.
//         /// Usually, this is used in debugging and testing.
//         /// Example
//         /// ```
//         /// let parsie = parsie
//         ///     .parse_tokens()
//         ///     .convert_to_ast_nodes()
//         ///     .print_tokens();
//         /// ```
//         pub fn print_tokens(self) -> Self {
//             println!();
//             for (i, token) in self.token_list.iter().enumerate() {
//                 println!("{}. {:?}", i, token);
//             }

//             Self {
//                 current_token: self.current_token,
//                 tokenizer: self.tokenizer,
//                 node_list: self.node_list,
//                 token_list: self.token_list,
//                 exempt_key_words: self.exempt_key_words,
//             }
//         }

//         /// Prints the current Parser::node_list.
//         /// Usually, this is used in debugging and testing.
//         /// Example
//         /// ```
//         /// let parsie = parsie
//         ///     .parse_tokens()
//         ///     .convert_to_ast_nodes()
//         ///     .print_tokens();
//         /// ```
//         pub fn print_nodes(self) -> Self {
//             println!();
//             for (i, node) in self.node_list.iter().enumerate() {
//                 println!("{}. {:?}", i, node);
//             }

//             Self {
//                 current_token: self.current_token,
//                 tokenizer: self.tokenizer,
//                 node_list: self.node_list,
//                 token_list: self.token_list,
//                 exempt_key_words: self.exempt_key_words,
//             }
//         }
//     }

//     impl<'a> Parser<'a> {
//         /// Looks for Letters and Word nodes
//         /// Example
//         /// ```
//         /// let mut exs = Parser::tmp_get_node_variable_delim(&self.node_list);
//         /// while !exs.found {}
//         /// ```
//         fn tmp_get_node_variable_delim(node_list: &[Node]) -> ExpressionSplitter {
//             let mut flag = false;
//             let mut start = None;

//             //for i in 0..node_list.len() {
//             for (i, val) in node_list.iter().enumerate() {
//                 match Some(&node_list[i]) {
//                     Some(Node::Variable(_)) => {
//                         start = Some(i);
//                         flag = true;
//                         break;
//                     }
//                     Some(_) => {}
//                     None => {}
//                 }
//             }

//             ExpressionSplitter {
//                 found: flag,
//                 start,
//                 end: None,
//             }
//         }

//         /// Searches for mathgin Punctuation("'");
//         /// Example
//         /// ```
//         /// let mut exs = Parser::tmp_get_matching_delim(&self.token_list);
//         /// while !exs.found {}
//         /// ```
//         fn tmp_get_matching_delim(token_list: &[TokenKind]) -> ExpressionSplitter {
//             let symbol1 = TokenKind::Punctuation('"');
//             let symbol2 = TokenKind::Punctuation('"');

//             let flag = token_list.contains(&symbol1) && token_list.contains(&symbol2);

//             let mut start = 0;
//             //for i in 0..token_list.len() {
//             for (i, val) in token_list.iter().enumerate() {
//                 if symbol1 == token_list[i] {
//                     start = i;
//                     break;
//                 }
//             }

//             let mut end = 0;
//             //for i in start + 1..token_list.len() {
//             for (i, val) in token_list.iter().enumerate().skip(start + 1) {
//                 if symbol2 == token_list[i] {
//                     end = i;
//                     break;
//                 }
//             }
//             ExpressionSplitter {
//                 found: flag,
//                 start: Some(start),
//                 end: Some(end),
//             }
//         }

//         /// Looks for Letters and Word tokens
//         /// Example
//         /// ```
//         /// let mut exs = Parser::tmp_get_latin_delim(&self.token_list);
//         /// while !exs.found {}
//         /// ```
//         fn tmp_get_latin_delim(token_list: &[TokenKind]) -> ExpressionSplitter {
//             let mut flag = false;
//             let mut start = None;
//             //for i in 0..token_list.len() {
//             for (i, val) in token_list.iter().enumerate() {
//                 match Some(&token_list[i]) {
//                     Some(TokenKind::Latin(_)) => {
//                         start = Some(i);
//                         flag = true;
//                         break;
//                     }
//                     Some(_) => {}
//                     None => {}
//                 }
//             }

//             ExpressionSplitter {
//                 found: flag,
//                 start,
//                 end: None,
//             }
//         }

//         /// Forms numbers from tokens
//         /// Example
//         /// ```
//         /// let mut exs = Parser::tmp_get_number_delim(&self.token_list);
//         /// while !exs.found {}
//         /// ```
//         fn tmp_get_number_delim(token_list: &[TokenKind]) -> ExpressionSplitter {
//             let mut flag = false;
//             let mut start = None;

//             for (i, val) in token_list.iter().enumerate() {
//                 //println!{"121 - {}, {:?}", i , val}
//                 match Some(&token_list[i]) {
//                     Some(TokenKind::Digit(_)) => {
//                         start = Some(i);
//                         flag = true;
//                         break;
//                     }
//                     Some(_) => {}
//                     None => {}
//                 }
//             }

//             ExpressionSplitter {
//                 found: flag,
//                 start,
//                 end: None,
//             }
//         }
//     }

//     // Unit tests
//     #[cfg(test)]
//     mod tests {
//         use super::*;

//         #[test]
//         fn test_current_token() {
//             let catcher = "Henry is a dog.".to_string();
//             let parsie = parser::Parser::new(&catcher).unwrap();
//             assert_eq!(parsie.current_token, TokenKind::Latin('H'));
//         }

//         #[test]
//         fn test_tokenizer_next() {
//             let mut tokenizer = Tokenizer::new("Henry is a dog.");
//             let token = Some(TokenKind::Latin('H'));
//             assert_eq!(tokenizer.next(), token)
//         }

//         #[test]
//         fn test_token_length() {
//             let catcher = "Henry is a dog.".to_string();
//             let mut token_list: Vec<TokenKind> = vec![];
//             let mut parsie = parser::Parser::new(&catcher).unwrap();

//             for _c in catcher.chars() {
//                 let y = parsie.get_next_token();
//                 if y != TokenKind::Undefined {
//                     token_list.push(y)
//                 }
//             }

//             assert_eq!(14, token_list.len());
//         }

//         #[test]
//         fn test_parser_token_list() {
//             let catcher = "!class Temp henry is a catdog".to_string();
//             let mut parsie = interpeter::parser::parser::Parser::new(&catcher).unwrap();

//             for _c in catcher.chars() {
//                 let y = parsie.get_next_token();
//                 if y != crate::enums::TokenKind::Undefined {
//                     parsie.token_list.push(y)
//                 }
//             }

//             let parsie = parsie
//                 .parse_tokens()
//                 .convert_to_ast_nodes()
//                 .construct_expression_nodes()
//                 .construct_keyword_nodes()
//                 //.update_node_assignment(String::from("desc"), String::from("my cool description!"))
//                 .print_tokens()
//                 .print_nodes();

//             let token = &parsie.token_list[0];
//             assert_eq!(token, &TokenKind::KeyWord("class".to_string()));

//             let token = &parsie.token_list[10];
//             assert_eq!(token, &TokenKind::Word("catdog".to_string()));
//         }

//         #[test]
//         fn test_parser_ast_node_list() {
//             let catcher = "!class Temp henry is a catdog".to_string();
//             let mut parsie = interpeter::parser::parser::Parser::new(&catcher).unwrap();

//             for _c in catcher.chars() {
//                 let y = parsie.get_next_token();
//                 if y != crate::enums::TokenKind::Undefined {
//                     parsie.token_list.push(y)
//                 }
//             }

//             let parsie = parsie
//                 .parse_tokens()
//                 .convert_to_ast_nodes()
//                 .construct_expression_nodes()
//                 .construct_keyword_nodes()
//                 //.update_node_assignment(String::from("desc"), String::from("my cool description!"))
//                 .print_tokens()
//                 .print_nodes();

//             let node = &parsie.node_list[9];
//             //let node2 = Node::Word(Box::new(BNode {value: "catdog".to_string()}));

//             let mut temp: String = String::new();
//             match node {
//                 Node::Word(word) => {
//                     let x = &*word;
//                     temp = x.value.clone();
//                 }
//                 _ => {}
//             }

//             assert_eq!(temp, "catdog".to_string());
//         }

//         #[test]
//         fn test_instruction_set_text() {
//             let catcher = "!class Temp henry is a catdog".to_string();
//             let mut parsie = interpeter::parser::parser::Parser::new(&catcher).unwrap();

//             for _c in catcher.chars() {
//                 let y = parsie.get_next_token();
//                 if y != crate::enums::TokenKind::Undefined {
//                     parsie.token_list.push(y)
//                 }
//             }

//             let parsie = parsie
//                 .parse_tokens()
//                 .convert_to_ast_nodes()
//                 .construct_expression_nodes()
//                 .construct_keyword_nodes();

//             let instruction_set = interpeter::ast::eval_instruction_set(parsie.node_list).unwrap();
//             assert_eq!(&instruction_set, "class Temp  henry is a catdog");
//         }
//     }
// }
