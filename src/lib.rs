use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<i32, Box<dyn Error>> {
    let files_contents = fs::read_to_string(config.filename)?;

    let mut str_vec: Vec<&str> = files_contents.split("\n").collect();

    str_vec.pop().unwrap();

    let mut tuple_vecs: Vec<(&str, i32)> = Vec::new();

    for v in str_vec {
        let mut split = v.split(" ");
        let my_tuple = (split.next().unwrap(), split.next().unwrap().parse::<i32>().unwrap());
        tuple_vecs.push(my_tuple);
    }

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for tuple in tuple_vecs {
        if tuple.0.eq("forward") {
            x = x + tuple.1;
        }

        if tuple.0.eq("down") {
            y = y + tuple.1;
        }

        if tuple.0.eq("up") {
            y = y - tuple.1;
        }
    }

    let product = x * y;
    Ok(product)
}

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}