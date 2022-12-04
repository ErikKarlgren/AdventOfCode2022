use std::io::{self, BufRead};

pub fn solve() -> io::Result<()> {
    let mut three_best_elfs: Vec<i32> = vec![0, 0, 0];
    let mut last_elf_calories = 0;

    for line in io::stdin().lock().lines() {
        let line = line?;

        if line.is_empty() {
            three_best_elfs = best_new_three(three_best_elfs, last_elf_calories);
            last_elf_calories = 0;
        } else {
            let calories: i32 = line.parse().unwrap();
            last_elf_calories += calories;
        }
    }
    // There could be an error in the calculation if the last line isn't empty

    let sum_best_three: i32 = three_best_elfs.iter().sum();
    dbg!(three_best_elfs);
    println!("Sum: {}", sum_best_three);

    Ok(())
}

fn best_new_three(three_best_elfs: Vec<i32>, last_elf_calories: i32) -> Vec<i32> {
    let mut best_three_and_last = Vec::with_capacity(4);
    best_three_and_last.extend(three_best_elfs);
    best_three_and_last.push(last_elf_calories);
    // sorts from lowest to highest, so we'll ignore the first value when returning
    best_three_and_last.sort();
    best_three_and_last.into_iter().skip(1).collect()
}
