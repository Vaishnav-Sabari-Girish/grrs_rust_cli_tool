use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {

    let arguments = Cli::parse();
    print!("pattern: {:?}, path: {:?}", arguments.pattern, arguments.path);
}
