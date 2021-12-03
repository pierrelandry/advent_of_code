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
    let mut aim = 0;

    for tuple in tuple_vecs {
        match tuple.0 {
            "forward" => {
                x = x + tuple.1;
                y = y + aim * tuple.1;
            },
            "down" => {
                aim = aim + tuple.1;
            },
            "up" => {
                aim = aim - tuple.1;
            },
            _ => {}
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