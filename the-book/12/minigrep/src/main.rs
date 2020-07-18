use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    println!("Searching for {}", config.query);
    println!("in file {}", config.filename);
    // --

    let mut f = File::open(config.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");


    println!("Write text: \n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    let config = Config { query, filename };
    config
}