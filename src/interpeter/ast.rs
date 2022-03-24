//! This module converts the AST nodes to an instruction set
use std::error;

/// A Node representation which takes a value
#[derive(Debug, Clone)]
pub struct BNode {
    /// A node must have a String value
    pub(crate) value: String,
}

#[allow(dead_code,clippy::upper_case_acronyms,clippy::enum_variant_names)]
///Does this show for enum Node
#[derive(Debug, Clone)]
pub enum Node {
    ///Assignment is same as `let dog = "rescue"`
    Expression(Box<BNode>, Box<BNode>),
    Class(Box<BNode>, Box<BNode>),
    Comment(Box<BNode>),
    KeyWord(Box<BNode>),
    Letter(Box<BNode>),
    Number(Box<BNode>),
    Punctuation(Box<BNode>),
    WhiteSpace,
    Word(Box<BNode>),
    Variable(Box<BNode>),
}

/// Given an AST list, calculate the correct instruction value.
/// Example
/// ```
/// #use extern crate formula::ast;
/// #use formula::{ ast::{BNode, Node},
/// #    controller,
/// #    };
/// 
/// let test_node_list:[Node;3] = [
///     Node::Word(Box::new(BNode { value: "Test".to_string() })), 
///     Node::WhiteSpace, 
///     Node::Word(Box::new(BNode { value: "Me".to_string() }))
/// ];
/// let instruction_set = formula::ast::eval_instruction_set(test_node_list.to_vec());
/// #assert_eq!(instruction_set.unwrap(), "Test Me")
///```
#[allow(unused)]
pub fn eval_instruction_set(expr: Vec<Node>) -> Result<String, Box<dyn error::Error>> {
    use self::Node::*;

    let mut full_text = String::from("");
    for node in expr {
        match node {
            Expression(box1, box2) => {
                let b_node_1 = box1;
                let b_node_2 = box2;
                full_text.push_str(&b_node_1.value);
                full_text.push(' ');
                full_text.push_str(&b_node_2.value);
            }
            Comment(_) => {
                full_text.push('#');
            }
            KeyWord(keyword) => {
                let node = *keyword;
                full_text.push_str(&node.value);
            }
            Letter(letter) => {
                let node = *letter;
                full_text.push_str(&node.value);
            }
            Number(number) => {
                let node = *number;
                full_text.push_str(&node.value);
            }
            Punctuation(punctuation) => {
                let node = *punctuation;
                full_text.push_str(&node.value);
            }
            Word(word) => {
                let node = *word;
                full_text.push_str(&node.value);
            }
            Variable(value) => {
                let node = *value;
                full_text.push_str(&node.value);
            }
            WhiteSpace => full_text.push(' '),
            Class(key, value) => {
                let b_node_key = key;
                let b_node_value = value;

                let x = b_node_key.value;
                match x.as_ref() {
                    "class" => {
                        full_text.push_str("class ");
                        full_text.push_str(&b_node_value.value);
                    }
                    "def" => {
                        full_text.push_str("def ");
                        full_text.push_str(&b_node_value.value);
                    }
                    "end" => {
                        full_text.push_str("end ");
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(full_text)
}

// Unit tests
#[cfg(test)]
mod tests {
    use crate::interpeter;

    use super::*;

    #[test]
    fn test_eval_instruction_set() {
        let test_node_list: [Node; 3] = [
            Node::Word(Box::new(BNode {
                value: "Test".to_string(),
            })),
            Node::WhiteSpace,
            Node::Word(Box::new(BNode {
                value: "Me".to_string(),
            })),
        ];
        let instruction_set = interpeter::ast::eval_instruction_set(test_node_list.to_vec());
        assert_eq!(instruction_set.unwrap(), "Test Me")
    }
}
