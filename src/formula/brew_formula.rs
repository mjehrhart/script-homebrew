pub mod brew_formula {
    use std::{
        collections::HashMap,
        fs::{File, OpenOptions},
        io::{BufWriter, Read, Write},
    };

    use crate::formula::parser::parser;
    use crate::{enums::TokenKind, formula};

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

            //Part ONE
            let mut token_list: Vec<TokenKind> = vec![];
            let mut parsie = parser::Parser::new(&page).unwrap();

            for _c in page.chars() {
                let y = parsie.get_next_token();
                if y != TokenKind::Undefined {
                    token_list.push(y)
                }
            }

            //"class Temp < Formula\r\n desc \"\r\n homepage \"\r\nend\r\n"
            let ruby_template = parser::Parser::convert_to_ast_form(token_list.clone());
            println!("\ruby_template length:: {}", &ruby_template.len());
            for some in &ruby_template {
                println!("..{:?}", some);
            }

            /* let mut buffer = BufWriter::new(File::create("pmet.rb")?);
            buffer.write_all(ruby_template.as_bytes())?;
            buffer.flush()?; */

            // let catcher = parser::Parser::find_token(String::from("url"), token_list);
            // println!("\n\ncatcher:: {:#?}", catcher);

            Ok(())
        }

        pub fn read_string(expr: String) {
            //Test Two
            let catcher = expr;
            println!("\n({})String:: {:#?}\n", catcher.len(), catcher);
            let mut token_list: Vec<TokenKind> = vec![];
            let mut parsie = parser::Parser::new(&catcher).unwrap();

            for _c in catcher.chars() {
                let y = parsie.get_next_token();
                if y != TokenKind::Undefined {
                    token_list.push(y)
                }
            }

            println!("length:: {}", &token_list.len());

            let new_token_list = parser::Parser::parse_tokens(token_list.clone());

            let something = parser::Parser::convert_to_ast_form(new_token_list);
            println!("\nsomething length:: {}", &something.len());
            for some in &something {
                println!("..{:?}", some);
            }

            let ruby_string = formula::ast::eval(something);
            match ruby_string {
                Ok(_) => {
                    println!("\nruby_string:: {:?}", &ruby_string);
                    println!("\nruby_string:: {:?}", &ruby_string.unwrap().len());
                }
                Err(_) => {}
            }

            ////////////
           
             
        }
    }

    pub fn get_tokenkind_map(
        mut mapping: HashMap<String, TokenKind>,
    ) -> HashMap<String, TokenKind> {
        // mapping.insert(
        //     String::from("class"),
        //     TokenKind::Class {
        //         raw: String::from(""),
        //     },
        // );
        // mapping.insert(
        //     String::from("homepage"),
        //     TokenKind::Variable {
        //         raw: String::from(""),
        //     },
        // );
        // mapping.insert(
        //     String::from("desc"),
        //     TokenKind::Variable {
        //         raw: String::from(""),
        //     },
        // );
        // mapping.insert(
        //     String::from("url"),
        //     TokenKind::Variable {
        //         raw: String::from(""),
        //     },
        // );
        // mapping.insert(
        //     String::from("version"),
        //     TokenKind::Variable {
        //         raw: String::from(""),
        //     },
        // );
        // mapping.insert(
        //     String::from("sha256"),
        //     TokenKind::Variable {
        //         raw: String::from(""),
        //     },
        // );
        // mapping.insert(
        //     String::from("license"),
        //     TokenKind::Variable {
        //         raw: String::from(""),
        //     },
        // );
        // mapping.insert(String::from("end"), TokenKind::End);
        // mapping.insert(String::from("def"), TokenKind::Def);

        mapping
    }

    // Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_current_token() { 
            let catcher = "henry is a dog".to_string();
            let parsie = parser::Parser::new(&catcher).unwrap();
            assert_eq!(parsie.current_token, TokenKind::Latin('h'));
        }

        #[test]
        fn test_token_length() {
            let catcher = "henry is a dog".to_string(); 
            let mut token_list: Vec<TokenKind> = vec![];
            let mut parsie = parser::Parser::new(&catcher).unwrap();

            for _c in catcher.chars() {
                let y = parsie.get_next_token();
                if y != TokenKind::Undefined {
                    token_list.push(y)
                }
            } 
            assert_eq!(13, token_list.len());
        }

        #[test]
        fn test_ast_node() { 
            let catcher = "!henry".to_string();
            let mut token_list: Vec<TokenKind> = vec![];
            let mut parsie = parser::Parser::new(&catcher).unwrap();

            for _c in catcher.chars() {
                let y = parsie.get_next_token();
                if y != TokenKind::Undefined {
                    token_list.push(y)
                }
            }
            let new_token_list = parser::Parser::parse_tokens(token_list.clone());
            let node_list = parser::Parser::convert_to_ast_form(new_token_list);

            for node in node_list {
                match node {
                    formula::ast::Node::Assignment(_, _) => {}
                    formula::ast::Node::Comment(_) => {}
                    formula::ast::Node::Letter(_) => {}
                    formula::ast::Node::Number(_) => {}
                    formula::ast::Node::Punctuation(_) => {}
                    formula::ast::Node::Word(word) => {
                        let node = *word;
                        let value = node.value;
                        assert_eq!(value , "henry".to_string());
                    }
                    formula::ast::Node::Variable(_) => {}
                }
            }
        }

        #[test]
        fn test_ast_node_length() {
            let catcher = "henry is a dog".to_string();
            let mut token_list: Vec<TokenKind> = vec![];
            let mut parsie = parser::Parser::new(&catcher).unwrap();

            for _c in catcher.chars() {
                let y = parsie.get_next_token();
                if y != TokenKind::Undefined {
                    token_list.push(y)
                }
            }
            let new_token_list = parser::Parser::parse_tokens(token_list.clone());
            let something = parser::Parser::convert_to_ast_form(new_token_list);
            assert_eq!(7, something.len());
        }
    }
}
