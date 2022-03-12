pub mod brew_formula {
    use std::{collections::HashMap, fs::{OpenOptions, File}, io::{Read, BufWriter, Write}};

    use crate::formula::{lexer::lexer::TokenKind, parser::parser};

    #[derive(Default, Debug, Clone, Copy)]
    pub struct Formula_Rb<'a> {
        pub class_name: &'a str,
        pub desc: &'a str,
        pub homepage: &'a str,
        pub url: &'a str,
        pub version: &'a str,
        pub sha256: &'a str,
        pub license: &'a str,
    }

    impl<'a> Formula_Rb<'a> {
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
                        let _s = String::from_utf8((&[byte.into()]).to_vec())
                            .map(|c| buf_string.push(c));
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
    }

    pub fn get_tokenkind_map(
        mut mapping: HashMap<String, TokenKind>,
    ) -> HashMap<String, TokenKind> {
        mapping.insert(
            String::from("class"),
            TokenKind::Class {
                raw: String::from(""),
            },
        );
        mapping.insert(
            String::from("homepage"),
            TokenKind::Variable {
                raw: String::from(""),
            },
        );
        mapping.insert(
            String::from("desc"),
            TokenKind::Variable {
                raw: String::from(""),
            },
        );
        mapping.insert(
            String::from("url"),
            TokenKind::Variable {
                raw: String::from(""),
            },
        );
        mapping.insert(
            String::from("version"),
            TokenKind::Variable {
                raw: String::from(""),
            },
        );
        mapping.insert(
            String::from("sha256"),
            TokenKind::Variable {
                raw: String::from(""),
            },
        );
        mapping.insert(
            String::from("license"),
            TokenKind::Variable {
                raw: String::from(""),
            },
        );
        mapping.insert(String::from("end"), TokenKind::End);
        mapping.insert(String::from("def"), TokenKind::Def);

        mapping
    }
}
