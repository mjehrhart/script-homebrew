pub mod brew_formula {
    use std::collections::HashMap;

    use crate::formula::lexer::lexer::TokenKind;

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
