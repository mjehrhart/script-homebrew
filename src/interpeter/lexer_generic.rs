#[allow(dead_code, unused_imports)]
pub mod generic {
    use super::*;
    use crate::enums::{self,};
    use crate::interpeter::lexer::lexer::Tokenizer;
 
    use std::any::type_name;
    use std::collections::HashMap;
    use std::fs::OpenOptions;
    use std::iter::Peekable;
    use std::str::Chars;
    use std::vec::IntoIter;

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
