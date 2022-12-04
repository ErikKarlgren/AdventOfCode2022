mod day02_b;
mod day01_b;

fn main() {
    let problem = "";
    match problem {
        "day02_b" => day02_b::solve().unwrap(),
        "day01_b" => day01_b::solve().unwrap(),
        _ => unimplemented!("Unimplemented or not a problem"),
    }
}
