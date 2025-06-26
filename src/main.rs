use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let arguments = Cli::parse();

    let content = std::fs::read_to_string(&arguments.path).expect("Could not read file");

    for line in content.lines() {
        if line.contains(&arguments.pattern) {
            println!("{}", line);
        }
    }
}
