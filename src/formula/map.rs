use std::fs::File;
use std::io::{BufWriter, Write};
use std::{fs::OpenOptions, io::Read};
 
use crate::formula::{
    parser::{parser},
    lexer::lexer::{TokenKind},
};

pub fn read_ruby_template(path: &str) -> Result<(), std::io::Error> {
    let mut f = OpenOptions::new()
        .read(true)
        .write(false)
        .open(path)
        .unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    let mut buf_string: Vec<String> = Vec::new();

    let _ensual = match f.read_to_end(&mut buffer) {
        Ok(_bit_count) => {
            for byte in buffer {
                let _s = String::from_utf8((&[byte.into()]).to_vec()).map(|c| buf_string.push(c));
            }
        }
        Err(e) => {
            panic!(
                "let suc = match file_from.read_to_end(&mut buffer)^^^ERROR {:?}",
                e
            )
        }
    };

    let mut page = String::new();
    for b in &buf_string {
        page.push_str(b);
    }

    let mut token_list: Vec<TokenKind> = vec![];
    let mut parsie = parser::Parser::new(&page).unwrap();
 
    for _c in page.chars() {
        let y = parsie.get_next_token();
        if y != TokenKind::Undefined {
            token_list.push(y)
        }
    }

    let ruby_template = parser::Parser::convert_token_to_node(token_list);
    //let mut brew = brew_formula::brew_formula::Formula_Rb::default();

    //"class Temp < Formula\r\n desc \"\r\n homepage \"\r\nend\r\n"
    let mut buffer = BufWriter::new(File::create("pmet.rb")?);
    buffer.write_all(ruby_template.as_bytes())?;
    buffer.flush()?;

    Ok(())
 
}
