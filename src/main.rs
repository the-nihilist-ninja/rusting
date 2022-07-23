mod loaders;

use loaders::record::*;

fn main(){


    if let Err(err) = read_record("data/movies_metadata.csv".to_string()) {
        println!("{}", err);
        std::process::exit(1);
    }

}
