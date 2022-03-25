// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let mut tokenizer = Tokenizer::new("Water");
        //assert_eq!(tokenizer.next().unwrap(), TokenKind::Latin('W'))
    }
}
