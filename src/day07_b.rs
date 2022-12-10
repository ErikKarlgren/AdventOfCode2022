use std::{
    collections::HashSet,
    io::{self},
};

use anyhow::{bail, Result};

pub fn solve() -> Result<()> {
    let disk_space: usize = 70000000;
    let update_size: usize = 30000000;

    _ = read_line()?; //.expect("Expected '$ cd /' line");
    let dir_data = solve_rec()?;

    let available_space = disk_space - dir_data.consumed_space;

    let needed_space_to_delete = match update_size.checked_sub(available_space) {
        Some(space) => space,
        None => bail!("There's no need to delete any space for the update"),
    };

    let smallest_big_dir_size = dir_data
        .dir_sizes
        .into_iter()
        .filter(|&size| size >= needed_space_to_delete)
        .min()
        .expect("Found no directories");

    println!("{}", smallest_big_dir_size);
    Ok(())
}

struct DirData {
    dir_sizes: HashSet<usize>,
    consumed_space: usize,
}

impl DirData {
    fn new() -> Self {
        Self {
            dir_sizes: HashSet::new(),
            consumed_space: 0,
        }
    }

    // fn merge(&mut self, other: DirData) {
    //     self.consumed_space += other.consumed_space;
    //     self.dir_sizes.extend(other.dir_sizes);
    // }
}

fn solve_rec() -> Result<DirData> {
    // Assume the "cd /" or "cd \w+" has been read already
    _ = read_line().expect("Expected '$ ls' line");

    let mut dir_data = DirData::new();
    let mut dir_size = 0;

    while let Some(line) = read_line()? {
        if let Some(size) = parse_file_size(&line) {
            dir_size += size;
        } else if let Some(subdir) = parse_cd_ins(&line) {
            match subdir.as_str() {
                ".." => break,
                _ => {
                    let subdir_data = solve_rec()?;
                    dir_size += subdir_data.consumed_space;
                    dir_data.dir_sizes.extend(subdir_data.dir_sizes);
                }
            }
        } // else: line ~= dir \w+
    }

    dir_data.consumed_space += dir_size;
    _ = dir_data.dir_sizes.insert(dir_size);

    Ok(dir_data)
}

fn read_line() -> Result<Option<String>> {
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf)? {
        0 => Ok(None),
        _ => Ok(Some(buf.trim().to_string())),
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
