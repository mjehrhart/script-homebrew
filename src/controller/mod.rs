//! Contains functions to read in ruby.rb file or string
#[allow(dead_code)]
pub mod ruby {
    use std::{fs::OpenOptions, io::Read};

    // use crate::enums::TokenKind;
    // use crate::formula::parser::parser::{self, Parser};

    //Standard format for hombrew create struct
    #[derive(Debug, Clone)]
    pub struct FormulaRb<'a> {
        pub class_name: &'a str,
        pub desc: &'a str,
        pub homepage: &'a str,
        pub url: &'a str,
        pub version: &'a str,
        pub sha256: &'a str,
        pub license: &'a str,
    }

    impl<'a> FormulaRb<'a> {
        pub fn new() -> FormulaRb<'a> {
            Self {
                class_name: "",
                desc: "",
                homepage: "",
                url: "",
                version: "",
                sha256: "",
                license: "",
            }
        }

        pub fn read_template(self, path: &str) -> Result<String, std::io::Error> {
            let mut f = OpenOptions::new()
                .read(true)
                .write(false)
                .open(path)
                .unwrap();

            let mut buffer: Vec<u8> = Vec::new();
            let _ensual = match f.read_to_end(&mut buffer) {
                Ok(_bit_count) => {
                    // for byte in buffer {
                    //     let _s = String::from_utf8((&[byte]).to_vec())
                    //         .map(|c| buf_string.push(c));
                    // }
                }
                Err(e) => {
                    panic!(
                        "let suc = match file_from.read_to_end(&mut buffer)^^^ERROR {:?}",
                        e
                    )
                }
            };
            let page = std::str::from_utf8(&buffer).unwrap();

            Ok(page.to_string())
        }

    
    }
}
