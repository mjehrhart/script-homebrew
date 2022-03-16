//! This is the main comment from main.rs 1
mod enums;
mod formula;

use formula::controller;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
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
fn main() -> Result<(), std::io::Error> {
    let options: formula::controller::ruby::Formula_Rb = Default::default();
    println!("options:: {:?}", options);

    let path = "/Users/matthew/dev/projects/script-homebrew/temp.rb";
    let this = formula::controller::ruby::Formula_Rb::read_ruby_template(path);
    match this {
        Ok(_) => {}
        Err(_) => {}
    }

    // let catcher =
    //     "!a \'crazy\' dog is 2.5 \"delete\" happy \"than\" a #20 cat 65 is not".to_string();
    // let catcher = "! dog link is http://www.google.com";
    // formula::brew::ruby::Formula_Rb::read_string(catcher.to_string());




    /*  //1
       Command::new("cargo")
           .arg("build")
           .arg("--release")
           .arg("--manifest-path")
           .arg("/Users/matthew/dev/projects/script-homebrew/Cargo.toml")
           .spawn()
           .expect("failed to execute process");

       //3
       Command::new("tar")
           .arg("-c")
           .arg("-z")
           .arg("-f")
           .arg("./target/release/script-homebrew.tar.gz")
           .arg("/Users/matthew/dev/projects/script-homebrew/target/release/script-homebrew")
           .spawn()
           .expect("failed to execute process");
       //4
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

       let hello = shasum.stdout;
       println!("shasum:: {:?}", std::str::from_utf8(&hello).unwrap());

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
    Ok(())
}

////Users/matthew/dev/projects/script-homebrew/target/release/script-homebrew
//let directory = "/Users/matthew/dev/projects/script-homebrew/target/release/script-homebrew";
//let cargo_dir = "/Users/matthew/dev/projects/script-homebrew";
