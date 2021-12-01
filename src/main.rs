use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let files_contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

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
    println!("The number of increases is {:?}", measurement_inc);
}

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &Vec<String>) -> Config {
        let filename = args[1].clone();

        Config { filename }
    }
}

