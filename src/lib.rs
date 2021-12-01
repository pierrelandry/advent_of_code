use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<i32, Box<dyn Error>> {
    let files_contents = fs::read_to_string(config.filename)?;

    let mut str_vec: Vec<&str> = files_contents.split("\n").collect();
    // remove the last line
    str_vec.pop().unwrap();

    // convert Vec<&str> to Vec<&i32>
    let mut int_vecs = Vec::new();
    for v in str_vec {
        int_vecs.push(v.parse::<i32>().unwrap());
    }

    // Count the number of increases
    let mut measurement_inc = 0;
    for i in 0..int_vecs.len() - 1 {
        if (int_vecs[i + 1] - int_vecs[i]) > 0 {
            measurement_inc = measurement_inc + 1;
        }
    }

    Ok(measurement_inc)
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