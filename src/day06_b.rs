use std::{
    collections::HashSet,
    io::{self},
};

use itertools::Itertools;

pub fn solve() -> io::Result<()> {
    let mut line = "".to_string();
    let _ = io::stdin().read_line(&mut line)?;
    let msg = line.trim().to_string();

    if let Some(pos) = find_marker_pos(&msg) {
        println!("{}", pos);
    } else {
        println!("Marker not found!");
    }

    Ok(())
}

fn find_marker_pos(msg: &str) -> Option<usize> {
    let mut buffer = Buffer::new();

    msg.chars().take(BUFFER_SIZE).for_each(|c| buffer.append(c));
    let mut other_chars = msg.chars().skip(BUFFER_SIZE);

    while !all_chars_are_unique(&buffer) {
        if let Some(c) = other_chars.next() {
            buffer.append(c);
        } else {
            return None;
        }
    }
    Some(buffer.num_chars())
}

fn all_chars_are_unique(buf: &Buffer) -> bool {
    let mut chars_set = HashSet::with_capacity(BUFFER_SIZE);
    for c in buf.read_last_chars() {
        if !chars_set.insert(c) {
            return false;
        }
    }
    true
}

const BUFFER_SIZE: usize = 14;
struct Buffer {
    chars: [char; BUFFER_SIZE],
    num_chars: usize,
}

impl Buffer {
    fn new() -> Buffer {
        Buffer {
            chars: ['\0'; BUFFER_SIZE],
            num_chars: 0,
        }
    }

    fn num_chars(&self) -> usize {
        self.num_chars
    }

    fn append(&mut self, c: char) {
        self.chars[self.num_chars % BUFFER_SIZE] = c;
        self.num_chars += 1;
    }

    fn read_last_chars(&self) -> Vec<char> {
        // let mut last_chars = Vec::with_capacity(BUFFER_SIZE);
        let actual_pos = |n_char: usize| {
            if n_char == 0 {
                None
            } else {
                Some((n_char - 1) % BUFFER_SIZE)
            }
        };

        let pos_of_4th_from_last_char = self.num_chars.saturating_sub(BUFFER_SIZE) + 1;
        (pos_of_4th_from_last_char..=self.num_chars)
            .map(actual_pos)
            .filter(|i| i.is_some())
            .map(|opt| self.chars[opt.unwrap()])
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer() {
        let mut buffer = Buffer::new();

        let first_chars = ['a', 'b', 'c'];
        first_chars.iter().for_each(|&c| buffer.append(c));
        assert_eq!(buffer.read_last_chars(), ['a', 'b', 'c']);

        let second_chars = ['d', 'e', 'f'];
        second_chars.iter().for_each(|&c| buffer.append(c));
        assert_eq!(buffer.read_last_chars(), ['c', 'd', 'e', 'f']);
    }
}
