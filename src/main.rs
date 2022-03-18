//! This is the main comment from main.rs 1
#[allow(unused_imports)]
mod enums;
mod formula;
// use formula::controller;
// use std::fs::File;
// use std::io::prelude::*;
// use std::io::BufReader;
// use std::io::BufWriter;
use std::process::Command;

/// Example
/// ```rust
/// # main() -> Result<(), std::io::Error> {
/// let catcher = "!a \'crazy\' dog is 2.5 \"delete\" a #20 cat 65 is not".to_string();
/// formula::brew::ruby::Formula_Rb::read_string(catcher);
/// ```
/// or
/// ```
/// let path = "/Users/matthew/dev/projects/script-homebrew/temp.rb";
/// let this = formula::brew::ruby::Formula_Rb::read_ruby_template(path);
/// match this {
///     Ok(_) => {}
///     Err(_) => {}
/// }
/// ```
///
#[allow(unused)]
fn main() -> Result<(), std::io::Error> {
    /* let formula_rb = controller::ruby::Formula_Rb::new();

       let path = "/Users/matthew/dev/projects/script-homebrew/temp.rb";
       let this = formula_rb.read_ruby_template(path);
       match this {
           Ok(page) => {
               let mut parsie = formula::parser::parser::Parser::new(&page).unwrap();

               //loop through char vecotr to populate the token_list
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

               //Write out the node_list instructions ( creates a text page)
               let ruby_string = formula::ast::eval_instruction_set(parsie.node_list);
               match ruby_string {
                   Ok(rs) => {
                       println!("\nruby_string:: {:?}", &rs);
                       println!("\nruby_string:: {:?}", &rs.len());

                       let mut buffer = BufWriter::new(File::create("pmet.rb")?);
                       buffer.write_all(rs.as_bytes())?;
                       buffer.flush()?;
                   }
                   Err(_) => {}
               }

           }
           Err(_) => todo!(),
       }
    */
    //*********************************************************************************************************************************/
    // sandbox

    //1 checked
    let cargo = {
        Command::new("cargo")
            .arg("build")
            .arg("--release")
            .arg("--manifest-path")
            .arg("/Users/matthew/dev/projects/script-homebrew/Cargo.toml")
            .output()
            .expect("failed to execute process")
    };
    let hash = cargo.stdout;
    let value = std::str::from_utf8(&hash).unwrap();
    let hash_value = value.to_owned();
    println!("cargo::{:?}", hash_value);

    //2 checked
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::fs::File;

    let tar_gz = File::create("script-homebrew.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_file(
        "script-homebrew",
        &mut File::open(
            "/Users/matthew/dev/projects/script-homebrew/target/release/script-homebrew",
        )
        .unwrap(),
    );
  
    //3
    let shasum = {
        Command::new("shasum")
            .arg("-a")
            .arg("256")
            .arg(
                "/Users/matthew/dev/projects/script-homebrew/target/release/script-homebrew.tar.gz",
            )
            .output()
            .expect("failed to execute process")
    };

    let hash = shasum.stdout;
    let value = std::str::from_utf8(&hash).unwrap();
    let hash_value = value.to_owned();
    let x = hash_value.split_once(' ').unwrap();

    println!("CHECKED shasum::{:?}", x.0);

    //*********************************************************************************************************************************/
    Ok(())
}

// let catcher =
//     "!a \'crazy\' dog is 2.5 \"delete\" happy \"than\" a #20 cat 65 is not".to_string();
// let catcher = "! dog link is http://www.google.com";
// formula::brew::ruby::Formula_Rb::read_string(catcher.to_string());

/* 
   //brew create https://example.com/foo-0.1.tar.gz
   //3
   let brew =
       {
           Command::new("brew")
       .arg("create")
       .arg("/Users/matthew/dev/projects/script-homebrew/target/release/script-homebrew.tar.gz")
       .output()
           .expect("failed to execute process")
       };

   let hello = brew.stdout;
   println!("brew:: {:?}", std::str::from_utf8(&hello).unwrap());
   //println!("brew:: {:?}", hello);
*/
/* Command::new("brew")
       .arg("create")
       .arg("/Users/matthew/dev/projects/script-homebrew/target/release/script-homebrew/script-homebrew.tar.gz")
       .spawn()
       .expect("failed to execute process");
*/

////Users/matthew/dev/projects/script-homebrew/target/release/script-homebrew
//let directory = "/Users/matthew/dev/projects/script-homebrew/target/release/script-homebrew";
//let cargo_dir = "/Users/matthew/dev/projects/script-homebrew";
