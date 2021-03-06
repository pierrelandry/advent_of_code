use std::env;
use std::process;
use advent_of_code::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let product = run(config).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1);
    });

    println!("The product is {:?}", product);
}

