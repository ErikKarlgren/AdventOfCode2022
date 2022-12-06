use std::{
    io::{self, BufRead},
    ops::Range,
};

use itertools::Itertools;

pub fn solve() -> io::Result<()> {
    let mut sum = 0;
    for line in io::stdin().lock().lines().map(|l| l.unwrap()) {
        let (first, second) = parse_line(line);
        if ranges_overlap(first, second) {
            sum += 1;
        }
    }

    print!("{}", sum);

    Ok(())
}

fn parse_line(line: String) -> (Range<i32>, Range<i32>) {
    line.trim()
        .split(',')
        .map(|pair| {
            let (min, max) = pair
                .split('-')
                .map(|s| s.parse().expect("Not a number"))
                .collect_tuple()
                .expect("Range does not have 2 elems (min and max)");
            min..max
        })
        .collect_tuple()
        .expect("Expected to find 2 elfs")
}

fn ranges_overlap(first: Range<i32>, second: Range<i32>) -> bool {
    !(first.end < second.start || second.end < first.start)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("12-34,56-78".to_string()), (12..34, 56..78));
    }

    #[test]
    fn test_overlaps() {
        assert!(ranges_overlap(5..7, 7..9));
        assert!(ranges_overlap(2..8, 3..7));
        assert!(ranges_overlap(6..6, 4..6));
        assert!(ranges_overlap(2..6, 4..8));
        assert!(!ranges_overlap(1..4, 6..9));
    }
}
