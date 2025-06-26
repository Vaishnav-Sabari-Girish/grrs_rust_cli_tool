struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("No Patterns given");
    let path = std::env::args().nth(2).expect("No Path given");

    let arguments = Cli {
        pattern, 
        path: std::path::PathBuf::from(path),
    };

    print!("pattern: {:?}, path: {:?}", arguments.pattern, arguments.path);
}
