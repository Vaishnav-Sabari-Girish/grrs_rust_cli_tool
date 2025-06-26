fn main() {
    let pattern = std::env::args().nth(1).expect("No Patterns given");
    let path = std::env::args().nth(2).expect("No Path given");

    print!("pattern: {:?}, path: {:?}", pattern, path);
}
