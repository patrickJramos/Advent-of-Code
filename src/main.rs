use advent_of_code::run;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args);

    let input = fs::read_to_string(config.path).unwrap();

    let result = run(config.challenge.to_string(), input);
    println!("{}", result.unwrap());
}

fn parse_args(args: &[String]) -> Config {
    let challenge = &args[1];
    let path = &args[2];

    Config { challenge, path }
}

struct Config<'a> {
    challenge: &'a String,
    path: &'a String,
}
