use clap::Parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use anyhow::{Context,  Result};

/// Creating a grep tool
#[derive(Parser)]
struct Cli{
    /// Pattern to look for
    pattern: String,

    /// Path to read file from
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // let content = std::fs::read_to_string(&args.path).expect("Could not read file!");
    
    let f = File::open(&args.path).with_context(|| format!("Unable to locate file: {}", &args.path.into_os_string().into_string().unwrap()))?;
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
