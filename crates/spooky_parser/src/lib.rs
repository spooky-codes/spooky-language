pub fn parse() -> bool {
    if spooky_lexer::tokenize() {
        println!("Fake parsing method");
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = parse();
        assert!(result);
    }
}
