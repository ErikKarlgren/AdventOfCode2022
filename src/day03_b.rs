use itertools::{izip, Itertools};
use std::io::{self, BufRead};

pub fn solve() -> io::Result<()> {
    let mut sum = 0;

    for three_lines in &io::stdin()
        .lock()
        .lines()
        .into_iter()
        .map(|l| l.unwrap())
        .chunks(3)
    {
        let (r1, r2, r3) = three_lines
            .map(|l| l.trim().to_string())
            .collect_tuple()
            .expect("Could not find 3 elves");

        let badge = find_badge(&r1, &r2, &r3);
        let priority = badge.map_or(0, priority);
        sum += priority;
    }

    println!("{}", sum);

    Ok(())
}

fn find_badge(rucksack1: &str, rucksack2: &str, rucksack3: &str) -> Option<char> {
    // Really ugly code tbh, but ig it's fast af since it uses tuples instead of sets

    let to_present_items = |rucksack: &str| {
        // If i = priority(c) => items[i] is (e, d), where:
        // - e is whether the item c is present in he rucksack
        // - if e, then d=c, otherwise d=\0
        let mut items = [(false, '\0'); 53];
        rucksack
            .chars()
            .for_each(|c| items[priority(c) as usize] = (true, c));
        items
    };

    izip!(
        to_present_items(rucksack1),
        to_present_items(rucksack2),
        to_present_items(rucksack3)
    )
    .find(|((a, _), (b, _), (c, _))| *a && *b && *c)
    .map(|((_, c), _, _)| c)
}

fn priority(item: char) -> u32 {
    let num: u32 = item.into();
    if !item.is_ascii_alphabetic() {
        panic!("Unexpected character: {}", item);
    } else if item.is_ascii_lowercase() {
        let min_value: u32 = 'a'.into();
        num - min_value + 1
    } else {
        let min_value: u32 = 'A'.into();
        num - min_value + 27
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('b'), 2);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }
}
