//! This is the main comment from main.rs 1
mod controller;

#[allow(unused_imports)]
#[warn(missing_docs)]
mod enums;
mod interpeter;
// use formula::controller;
// use std::fs::File;
// use std::io::prelude::*;
// use std::io::BufReader;
// use std::io::BufWriter;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::enums::Token;

//use interpeter::ast::{BNode, Node};

///Example
/// ```
///    let formula_rb = controller::ruby::FormulaRb::new();
///
///    let path = "/Users/matthew/dev/projects/script-homebrew/temp.rb";
///    let this = formula_rb.read_ruby_template(path);
///
///    if let Ok(page) = this {
///        
///        let mut parsie = formula::parser::parser::Parser::new(&page).unwrap();
///
///        //loop through char vector to populate the token_list
///        for _c in page.chars() {
///            let y = parsie.get_next_token();
///            if y != enums::TokenKind::Undefined {
///                parsie.token_list.push(y);
///            }
///        }
///
///        let parsie = parsie
///            .parse_tokens()
///            .convert_to_ast_nodes()
///            .transform_nodes_to_assignment_nodes()
///            .transform_nodes_to_keyword_nodes()
///            .update_node_assignment(String::from("desc"), String::from("my cool description!"))
///            .print_tokens()
///            .print_nodes();
///
///        //Write out the node_list instructions ( creates a text page)
///        let ruby_string = formula::ast::eval_instruction_set(parsie.node_list);
///
///        if let Ok(rs) = ruby_string {
///            println!("\nruby_string:: {:?}", &rs);
///            println!("\nruby_string:: {:?}", &rs.len());
///
///            let mut buffer = BufWriter::new(File::create("pmet.rb")?);
///            buffer.write_all(rs.as_bytes())?;
///        }
///    }
///
/// ```

fn main() -> Result<(), std::io::Error> {
    // #![recursion_limit = "256"]

    //*************************************************************************************************** */
    //*************************************************************************************************** */
    //Test 1
    let frb = controller::ruby::FormulaRb::new();

    let path = "/Users/matthew/dev/projects/script-homebrew/temp.txt";
    let this = frb.read_template(path);

    if let Ok(page) = this {
        let tmp = page.clone();

        let mut token_container = vec![];
        let tokenizer = interpeter::lexer::lexer::Tokenizer::new(&tmp);

        let mut i = 0;
        //while let Some(token) = tokenizer.next() {
        for token in tokenizer {
            match Some(token) {
                Some(t) => match t {
                    enums::Token::Undefined => break,
                    enums::Token::WhiteSpace => {}
                    _ => {
                        println!("{}. {:?}", i, t);
                        token_container.push(t);
                    }
                },
                None => break,
            }
            i += 1;

            if i > 1000 {
                break;
            }
        }

        //Transforms tokens into Token::RawString, Token::RawByteString if found
        let mut found = token_container.iter().position(|r| r == &Token::Pound);
        while found != None {
            let index = found.unwrap();
            if token_container[index - 1] == Token::Word("r".to_string())
                || token_container[index - 1] == Token::Word("br".to_string())
            {
                let token_plus_one = &token_container[index + 1];
                match Some(token_plus_one) {
                    Some(Token::String(str)) => {
                        println!("Found R Word: {:?}, {}", token_container[index - 1], str);

                        let mut raw_token = Token::RawString(str.to_string());
                        if token_container[index - 1] == Token::Word("br".to_string()) {
                            raw_token = Token::RawByteString(str.to_string());
                        }

                        token_container.remove(index + 2); // '#'
                        token_container.remove(index + 1); // 'str'
                        token_container.remove(index); // '#'
                        token_container.remove(index - 1); // 'r'

                        token_container.insert(index - 1, raw_token);
                    }
                    Some(_) => {}
                    None => {}
                }
            }

            found = token_container.iter().position(|r| r == &Token::Pound);
        }

 
       /*  //Transforms tokens into Token::Byte, Token::ByteString
        let mut found = token_container.iter().position(|r| r == &Token::Word("b".to_string()));
        while found != None {
            let index = found.unwrap();

            if token_container[index + 1] == Token::SingleQuote
                && token_container[index + 3] == Token::SingleQuote
            {
                let k = &token_container[index + 2];
                match k {
                    Token::Word(c) => {
                        //println!("Found b'H' Word: {:?}, {}", token_container[index + 2], c);

                        let raw_token = Token::Byte(c.to_string());

                        token_container.remove(index + 3); // '''
                        token_container.remove(index + 2); // Token::Word('H')
                        token_container.remove(index + 1); // '''
                        token_container.remove(index); // 'b'

                        token_container.insert(index, raw_token);
                    }
                    _ => {}
                }
            } else {
                let token_plus_one = &token_container[index + 1];
 
                match Some(token_plus_one) {
                    Some(Token::String(str)) => { 

                        let raw_token = Token::ByteString(str.to_string());
                        token_container.remove(index + 1); // Token::String()
                        token_container.remove(index); // 'b'

                        token_container.insert(index, raw_token);
                    }
                    Some(_) => {}
                    None => {}
                }
            }

            found = token_container
                .iter()
                .position(|r| r == &Token::Word("b".to_string()));
        }
 */
        //Print Finalized list of tokens
        println!("________________________");
        let mut i = 0;
        for token in token_container {
            println!("{}. {:?}", i, &token);
            i += 1;
        }

        //let parsie = interpeter::parser::parser::Parser::new(&tmp).unwrap();

        //let parsie = parsie
        //.intialize_tokens(page)
        //.parse_tokens()
        //.convert_to_ast_nodes()
        //.construct_expression_nodes()
        //.construct_keyword_nodes()
        //.update_node_assignment(String::from("desc"), String::from("my cool description!"))
        //.print_tokens();
        //.print_nodes();

        //let instruction_set_text = interpeter::ast::eval_instruction_set(parsie.node_list);

        //Write out the node_list instructions ( creates a text page)
        /* if let Ok(instruction_set) = instruction_set_text {
            // println!("\ninstruction_set:: {:?}", &instruction_set);
            // println!("\ninstruction_set:: {:?}", &instruction_set.len());

            let mut buffer = BufWriter::new(File::create("pmet.rb")?);
            buffer.write_all(instruction_set.as_bytes())?;
        } */
        //*************************************************************************************************** */
        //*************************************************************************************************** */
        //Test 2
        /* let expr = "!class MyClass < time def install animal catdog\n # something\n end?";
        let parsie = interpeter::parser::parser::Parser::new(&expr).unwrap();

        let parsie = parsie
            .intialize_tokens(expr.to_string())
            .parse_tokens()
            .convert_to_ast_nodes()
            .construct_expression_nodes()
            .construct_keyword_nodes()
            .print_tokens()
            .print_nodes();

        let instruction_set_text = interpeter::ast::eval_instruction_set(parsie.node_list);

        //Write out the node_list instructions ( creates a text page)
        if let Ok(instr) = instruction_set_text {
            println!("\nruby_string:: {:?}", &instr);
            println!("\nruby_string:: {:?}", &instr.len());
        } */

        //*************************************************************************************************** */
        //*************************************************************************************************** */
    }

    Ok(())
}
