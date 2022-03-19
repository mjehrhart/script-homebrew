//! This is the main comment from main.rs 1
#[allow(unused_imports)]
#[warn(missing_docs)]
mod enums;
mod formula;
mod controller;
// use formula::controller;
// use std::fs::File;
// use std::io::prelude::*;
// use std::io::BufReader;
// use std::io::BufWriter;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

use formula::{
    ast::{BNode, Node}, 
};
 
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

//#[allow(unused)]
fn main() -> Result<(), std::io::Error> {
    let formula_rb = controller::ruby::FormulaRb::new();
 
    let path = "/Users/matthew/dev/projects/script-homebrew/temp.rb";
    let this = formula_rb.read_ruby_template(path);

    if let Ok(page) = this {
        let mut parsie = formula::parser::parser::Parser::new(&page).unwrap();

        //loop through char vector to populate the token_list
        for _c in page.chars() {
            let y = parsie.get_next_token();
            if y != enums::TokenKind::Undefined {
                parsie.token_list.push(y);
            }
        }

        let parsie = parsie
            .parse_tokens()
            .convert_to_ast_nodes()
            .transform_nodes_to_assignment_nodes()
            .transform_nodes_to_keyword_nodes()
            .update_node_assignment(String::from("desc"), String::from("my cool description!"))
            .print_tokens()
            .print_nodes();

        let ruby_string = formula::ast::eval_instruction_set(parsie.node_list);

        //Write out the node_list instructions ( creates a text page)
        if let Ok(rs) = ruby_string {
            println!("\nruby_string:: {:?}", &rs);
            println!("\nruby_string:: {:?}", &rs.len());

            let mut buffer = BufWriter::new(File::create("pmet.rb")?);
            buffer.write_all(rs.as_bytes())?;
        }

        //*************************************************************************************************** */
        //*************************************************************************************************** */
         
    }

    Ok(())
}
