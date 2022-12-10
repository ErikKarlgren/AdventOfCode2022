use std::io::{self};

use anyhow::Result;

pub fn solve() -> Result<()> {
    let limit = 100000;
    _ = read_line(); //.expect("Expected '$ cd /' line");
    let PartialSol {
        sum_dirs_under_limit,
        ..
    } = solve_rec(limit)?;
    println!("{}", sum_dirs_under_limit);
    Ok(())
}

struct PartialSol {
    root_dir_size: usize,
    sum_dirs_under_limit: usize,
}
impl PartialSol {
    fn new(root_dir_size: usize, sum_dirs_under_limit: usize) -> Self {
        Self {
            root_dir_size,
            sum_dirs_under_limit,
        }
    }
}

fn solve_rec(limit: usize) -> Result<PartialSol> {
    // Assume the "cd /" or "cd \w+" has been read already
    _ = read_line().expect("Expected '$ ls' line");

    let mut sol = 0;
    let mut total_subdirs_size = 0;
    let mut total_files_size = 0;

    while let Some(line) = read_line() {
        if let Some(size) = parse_file_size(&line) {
            total_files_size += size;
        } else if let Some(subdir) = parse_cd_ins(&line) {
            match subdir.as_str() {
                ".." => break,
                _ => {
                    let PartialSol {
                        root_dir_size: dir_size,
                        sum_dirs_under_limit,
                    } = solve_rec(limit)?;
                    total_subdirs_size += dir_size;
                    sol += sum_dirs_under_limit;
                }
            }
        } // else: line ~= dir \w+
    }

    let total_dir_size = total_subdirs_size + total_files_size;
    if total_dir_size <= limit {
        Ok(PartialSol::new(total_dir_size, sol + total_dir_size))
    } else {
        Ok(PartialSol::new(total_dir_size, sol))
    }
}

fn read_line() -> Option<String> {
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(buf.trim().to_string()),
    }
}

fn parse_file_size(line: &str) -> Option<usize> {
    let word = line.split(' ').next()?;
    match word.parse() {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

fn parse_cd_ins(line: &str) -> Option<String> {
    line.strip_prefix("$ cd ").map(|s| s.trim().to_string())
}
