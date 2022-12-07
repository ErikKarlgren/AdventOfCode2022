mod day07_a;
mod day07_b;
mod day06_a;
mod day06_b;
mod day05_a;
mod day05_b;
mod day04_a;
mod day04_b;
mod day03_a;
mod day03_b;
mod day01_b;
mod day02_b;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    problem: String,
}

fn main() {
    let args = Args::parse();

    match args.problem.as_str() {
        "day07_a" => day07_a::solve().unwrap(),
        "day07_b" => day07_b::solve().unwrap(),
        "day06_a" => day06_a::solve().unwrap(),
        "day06_b" => day06_b::solve().unwrap(),
        "day05_a" => day05_a::solve().unwrap(),
        "day05_b" => day05_b::solve().unwrap(),
        "day04_a" => day04_a::solve().unwrap(),
        "day04_b" => day04_b::solve().unwrap(),
        "day03_a" => day03_a::solve().unwrap(),
        "day03_b" => day03_b::solve().unwrap(),
        "day02_b" => day02_b::solve().unwrap(),
        "day01_b" => day01_b::solve().unwrap(),
        _ => unimplemented!("Unimplemented or not a problem: {}", args.problem),
    }
}
