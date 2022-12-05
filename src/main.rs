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
        "day03_a" => day03_a::solve().unwrap(),
        "day03_b" => day03_b::solve().unwrap(),
        "day02_b" => day02_b::solve().unwrap(),
        "day01_b" => day01_b::solve().unwrap(),
        _ => unimplemented!("Unimplemented or not a problem: {}", args.problem),
    }
}
