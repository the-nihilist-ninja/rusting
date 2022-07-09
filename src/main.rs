use clap::Parser;

/// Creating a grep tool
#[derive(Parser, Debug)]
struct Cli{
    /// Pattern to look for
    pattern: String,

    /// Path to read file from
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}",args);

    let content = std::fs::read_to_string(&args.path).expect("Could not read file!");

    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{}", line);
        }
    }


}
