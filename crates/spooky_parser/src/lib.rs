pub fn parse() -> bool {
    if spooky_lexer::tokenize() {
        println!("Fake parsing method");
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = parse();
        assert_eq!(result, true);
    }
}
