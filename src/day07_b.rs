use std::io::{self, BufRead};

pub fn solve() -> io::Result<()> {
    for line in io::stdin().lock().lines().map(|l| l.unwrap()){
        // Here's your code
    }

    Ok(())
}