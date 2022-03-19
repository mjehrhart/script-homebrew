//! Contains functions to read in ruby.rb file or string
#[allow(dead_code)]
pub mod ruby {
    use std::{
        fs::{OpenOptions},
        io::{ Read},
    };

    use crate::formula::{ 
        parser::parser,
    };
    use crate::{enums::TokenKind};

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
        /// Example
        /// ```
        /// let path = "/Users/matthew/dev/projects/script-homebrew/temp.rb";
        /// let this = formula::brew::ruby::FormulaRb::read_ruby_template(path);
        /// match this {
        ///     Ok(_) => {}
        ///     Err(_) => {}
        /// }
        /// ```
        //-> Result<(), std::io::Error>
        pub fn read_ruby_template(self, path: &str) -> Result<String, std::io::Error> {
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

        /// Example
        /// ```rust
        /// let catcher = "!a \'crazy\' dog is 2.5 \"delete\" a #20 cat 65 is not".to_string();
        /// formula::brew::ruby::FormulaRb::read_string(catcher);
        ///
        /// ```
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

            // let new_token_list = parser::Parser::parse_tokens(token_list.clone());

            // let something = parser::Parser::convert_to_ast_nodes(new_token_list);
            // println!("\nsomething length:: {}", &something.len());
            // for some in &something {
            //     println!("..{:?}", some);
            // }

            // let ruby_string = formula::ast::eval(something);
            // match ruby_string {
            //     Ok(_) => {
            //         println!("\nruby_string:: {:?}", &ruby_string);
            //         println!("\nruby_string:: {:?}", &ruby_string.unwrap().len());
            //     }
            //     Err(_) => {}
            // }

            ////////////
        }
    }
 
}
