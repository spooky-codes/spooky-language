pub fn tokenize() -> bool {
    println!("Fake tokenizing method");
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = tokenize();
        assert_eq!(result, true);
    }
}
