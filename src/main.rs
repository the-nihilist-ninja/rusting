use clap::Parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

/// Creating a grep tool
#[derive(Parser, Debug)]
struct Cli{
    /// Pattern to look for
    pattern: String,

    /// Path to read file from
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    println!("{:?}",args);

    // let content = std::fs::read_to_string(&args.path).expect("Could not read file!");
    
    let f = File::open(&args.path)?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let l = line?;
        if l.contains(&args.pattern){
            println!("{}", l);
        }
    }


    // for line in content.lines(){
    //    if line.contains(&args.pattern){
    //        println!("{}", line);
    //    }
    // }

    Ok(())
}
