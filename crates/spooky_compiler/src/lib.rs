pub fn compile() {
    if spooky_parser::parse() {
        println!("Fake compilation :)");
    } else {
        println!("Cannot compile...");
    }
}
