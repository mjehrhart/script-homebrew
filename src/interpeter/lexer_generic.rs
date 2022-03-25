#[allow(dead_code, unused_imports)]
pub mod generic {
    use super::*;
    use crate::enums::{self, Token,};
    use crate::interpeter::lexer::lexer::Tokenizer;
 
    use std::any::type_name;
    use std::collections::HashMap;
    use std::fs::OpenOptions;
    use std::iter::Peekable;
    use std::str::Chars;
    use std::vec::IntoIter;

       /// Example
    /// ```
    /// let exp = "Water is helpful"
    /// let mut lexy = Tokenizer::new(exp);
    /// ```
    impl<'a> Tokenizer<'a> {
        pub fn new(new_expr: &'a str) -> Self {
            println!("Expression == {:?}", new_expr);
            Tokenizer { 
                expr: new_expr.chars().peekable(), 
                keywords: Self::load_keywords(),
            }
        }

        pub fn load_keywords() -> HashMap<&'a str, Token> {
            let mut map: HashMap<&'a str, Token> = HashMap::new();
 
            map.insert("as", Token::KW_As);
            map.insert("async", Token::KW_Async);
            map.insert("await", Token::KW_Await);
            map.insert("break", Token::KW_Break);
            map.insert("const", Token::KW_Const);
            map.insert("continue", Token::KW_Contine);
            map.insert("crate", Token::KW_Crate);
            map.insert("dyn", Token::KW_Dyn);
            map.insert("else", Token::KW_Else); 
            map.insert("enum", Token::KW_Enum);
            map.insert("extern", Token::KW_Extern);
            map.insert("false", Token::KW_False);
            map.insert("fn", Token::KW_Fn);
            map.insert("for", Token::KW_For);
            map.insert("if", Token::KW_If);
            map.insert("impl", Token::KW_Impl);
            map.insert("in", Token::KW_In);
            map.insert("let", Token::KW_Let);
            map.insert("loop", Token::KW_Loop); 
            map.insert("match", Token::KW_Match);
            map.insert("mod", Token::KW_Mod);
            map.insert("move", Token::KW_Move);
            map.insert("mut", Token::KW_Mut);
            map.insert("pub", Token::KW_Pub);
            map.insert("ref", Token::KW_Ref);
            map.insert("return", Token::KW_Return);
            map.insert("Self", Token::KW_SELF);
            map.insert("self", Token::KW_Self); 
            map.insert("static", Token::KW_Static);
            map.insert("struct", Token::KW_Struct); 
            map.insert("super", Token::KW_Super);
            map.insert("trait", Token::KW_Trait);
            map.insert("true", Token::KW_True);
            map.insert("type", Token::KW_Type);
            map.insert("union", Token::KW_Union);
            map.insert("unsafe", Token::KW_Unsafe);
            map.insert("use", Token::KW_Use);
            map.insert("where", Token::KW_Where);
            map.insert("while", Token::KW_While);

            //self.keyword = map;
            map
        }

        pub fn translate_token_to_keyword_token( token: &Token, value: String) ->  Option<Token>{
 
            match token { 
                Token::KW_As => Some(Token::KW_As),
                Token::KW_Async => Some(Token::KW_Async),
                Token::KW_Await => Some(Token::KW_Await),
                Token::KW_Break => Some(Token::KW_Break),
                Token::KW_Const => Some(Token::KW_Const),
                Token::KW_Contine => Some(Token::KW_Contine),
                Token::KW_Crate => Some(Token::KW_Crate),
                Token::KW_Dyn => Some(Token::KW_Dyn),
                Token::KW_Else => Some(Token::KW_Else),  
                Token::KW_Enum => Some(Token::KW_Enum),
                Token::KW_Extern => Some(Token::KW_Extern),
                Token::KW_False => Some(Token::KW_False),
                Token::KW_Fn => Some(Token::KW_Fn),
                Token::KW_For => Some(Token::KW_For), 
                Token::KW_If => Some(Token::KW_If),
                Token::KW_Impl => Some(Token::KW_Impl),
                Token::KW_In => Some(Token::KW_In),
                Token::KW_Let => Some(Token::KW_Let),
                Token::KW_Loop => Some(Token::KW_Loop),
                Token::KW_Match =>Some(Token::KW_Match),
                Token::KW_Mod => Some(Token::KW_Mod),
                Token::KW_Move => Some(Token::KW_Move),
                Token::KW_Mut => Some(Token::KW_Mut), 
                Token::KW_Pub => Some(Token::KW_Pub),
                Token::KW_Ref => Some(Token::KW_Ref),
                Token::KW_Return => Some(Token::KW_Return),
                Token::KW_SELF => Some(Token::KW_SELF),
                Token::KW_Self => Some(Token::KW_Self),
                Token::KW_Static => Some(Token::KW_Static),
                Token::KW_Struct => Some(Token::KW_Struct),
                Token::KW_Super => Some(Token::KW_Super),
                Token::KW_Trait => Some(Token::KW_Trait),
                Token::KW_True => Some(Token::KW_True),
                Token::KW_Type => Some(Token::KW_Type),
                Token::KW_Union => Some(Token::KW_Union),
                Token::KW_Unsafe => Some(Token::KW_Unsafe),
                Token::KW_Use => Some(Token::KW_Use),
                Token::KW_Where => Some(Token::KW_Where),
                Token::KW_While => Some(Token::KW_While),
                _ => Some(Token::Word(value))
            }
        }
        
        pub fn check_if_keyword(&mut self, potential_keyword: &str) -> Option<&Token> {
            let token = self.keywords.get(potential_keyword);
            token
        }

    }

    pub mod numeric {
        use std::{iter::Peekable, str::Chars};

        use crate::{enums::Token, interpeter::lexer::lexer::Tokenizer};

        impl<'a> Tokenizer<'a> {
            pub fn is_numeric_with_period(c: char) -> bool {
                c.is_ascii_digit() || c == '.' || c == '_'
            }
        }
    }
    pub mod comment {
        use crate::interpeter::lexer::lexer::Tokenizer;

        impl<'a> Tokenizer<'a> {
            pub fn is_comment(c: char) -> bool {
                c == '/' || c == '*'
            }
        }
    }

    pub mod escapes {
        use crate::interpeter::lexer::lexer::Tokenizer;

        impl<'a> Tokenizer<'a> {
            pub fn is_escaped(c: char) -> bool {
                c == '\x41' || c == '\n' || c == '\r' || c == '\t' || c == '\\' || c == '\0' ||
                c == '\x7F' || c == '\'' || c == '\"'
            }
        }
    }

    pub mod boolean {
        use crate::interpeter::lexer::lexer::Tokenizer;

        impl<'a> Tokenizer<'a> {
            pub fn is_boolean(c: char) -> bool {
                c == 't' || c == 'r' || c == 'u' || c == 'e' || c == 'f' || c == 'a' ||
                c == 'l' || c == 's' || c == 'e'
            }
        }
    }

    pub mod generic {
        use crate::interpeter::lexer::lexer::Tokenizer;

        impl<'a> Tokenizer<'a> {
            pub fn is_new_line(c: char) -> bool {
                c == '\r' || c == '\n'
            }

            pub fn is_statement_delim(c: char) -> bool {
                c == ';'
            }

            pub fn is_whitespace(c: char) -> bool {
                c == ' '
            }

            pub fn is_string_value(c: char) -> bool {
                c == '"'
            }

            pub fn is_less_than_greater_than_string_value(c: char) -> bool {
                c == '<'
            }

            pub fn is_double_colon(c: char) -> bool {
                c.is_alphabetic() || c == ':' || c == '_'
            }

            pub fn is_word(c: char) -> bool {
                c.is_alphanumeric() || c == '_'
            }

            pub fn is_reference(c: char) -> bool {
                c == '&'
            }

            pub fn bracket_delimiters(c: char) -> bool {
                c == '{' || c == '[' || c == '(' || c == ')' || c == ']' || c == '}'
            }

            pub fn is_fat_arrow(c: char) -> bool {
                c == '='
            }

            pub fn is_math_operator(c: char) -> bool {
                c == '+'
                    || c == '-'
                    || c == '*'
                    || c == '/'
                    || c == '%'
                    || c == '^'
                    || c == '!'
                    || c == '&'
                    || c == '|'
                    || c == '='
                    || c == ' '
                    || c == '>'
            }

            pub fn starts_with_equal_sign(c: char) -> bool {
                c == '='
            }

            pub fn is_gt_lt_fat_arrow(c: char) -> bool {
                c == '>' || c == '<' || c == '='
            }

            pub fn is_lesser_punctutation(c: char) -> bool {
                c == '@'
                    || c == '_'
                    || c == '.'
                    || c == ','
                    || c == ';'
                    || c == ':'
                    || c == '-'
                    || c == '#'
                    || c == '$'
                    || c == '?'
                //|| c == '>' //needed for RArrow ->
            }

            pub fn is_dot_or_eq(c: char) -> bool {
                c == '.' || c == '='
            }
        }
    }
}