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
    //*************************************************************************************************** */
    //*************************************************************************************************** */
    //Test 1
    let frb = controller::ruby::FormulaRb::new();

    let path = "/Users/matthew/dev/projects/script-homebrew/temp.rb";
    let this = frb.read_template(path);

    if let Ok(page) = this {
        let tmp = page.clone();

        let mut tokenizer = interpeter::lexer::lexer::Tokenizer::new(&tmp);

        let mut i = 0;
        while let Some(token) = tokenizer.next() {
            match Some(token) {
                Some(t) => match t {
                    enums::Token::Undefined => break,
                    _ => {
                        println!("{}. {:?}", i, t);
                    }
                },
                None => break,
            }
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
