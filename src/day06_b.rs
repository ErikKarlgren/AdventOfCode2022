use std::{
    collections::HashSet,
    io::{self},
};

pub fn solve() -> io::Result<()> {
    let mut line = "".to_string();
    let _ = io::stdin().read_line(&mut line)?;
    let msg = line.trim().to_string();

    let num_chars = 14;

    for (i, _) in msg.char_indices() {
        if i + num_chars > msg.len() {
            break;
        }
        let mark = &msg[i..i + num_chars];
        let chars_set: HashSet<char> = HashSet::from_iter(mark.chars());
        if chars_set.len() == num_chars {
            println!("{}", i + num_chars);
            return Ok(());
        }
    }
    println!("Marker not found!");

    Ok(())
}
