use std::{fs::OpenOptions, io::Read};

use crate::formula::parser;
use crate::formula::parser::parser::Parser;
use crate::formula::{
    self,
    brew_formula::{self, brew_formula::Formula_Rb},
    lexer::lexer::{TokenKind, Tokenizer},
};

pub fn read_pdf(path: &str) {
    let mut f = OpenOptions::new()
        .read(true)
        .write(false)
        .open(path)
        .unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    let mut buf_string: Vec<String> = Vec::new();
  
    let _ensual = match f.read_to_end(&mut buffer) {
        Ok(_bit_count) => {
            //clone_buffer = buffer.clone();
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
    let mut parsie = formula::parser::parser::Parser::new(&page).unwrap();

    //call Parse here (aFTER PARSIE)
    for _c in page.chars() {
        let y = parsie.get_next_token();
        if y != TokenKind::Undefined {
            token_list.push(y)
        }
    }

    let mut brew = brew_formula::brew_formula::Formula_Rb::default();
 
    for i in 0..token_list.len() {
        println!("..{:?}", token_list[i]);

        // match &token_list[i] {
        //     TokenKind::Latin { raw, kind } => {}
        //     TokenKind::Number(_) => {}
        //     TokenKind::Numbers { raw, kind } => {}
        //     TokenKind::Punctuation(_) => {}
        //     TokenKind::Whitespace { raw, kind } => {}
        //     TokenKind::EOF => {}
        //     TokenKind::Undefined => {}
        //     TokenKind::ISSUE => {}
        //     TokenKind::Object(val) => {
        //         if val == "class" {
        //             println!("^^^^^^^^^ {:?}", val);
        //             let this = &token_list[i + 1];

        //             match this {
        //                 TokenKind::Object(val) => {
        //                     brew.class_name = val; 
        //                 }
        //                 _ => {}
        //             } 
        //         }
        //     }
        //     TokenKind::Uri(_) => {}
        //     TokenKind::CRLF { raw, kind } => {}
        //     TokenKind::Comment => todo!(),
        //     TokenKind::Signature { raw, kind } => {}
        // }
    }

    //println!("Brew HA HA::{:?}", brew);
     
    // //PDF_VERSION - keep 12/15
    // let delimiter = maps::Maps::get_matching_delimiter(&page, "%", "%");
    // let pdf_version = String::from_utf8(
    //     clone_buffer[delimiter.start.unwrap()..=delimiter.end.unwrap()].to_vec());
}
