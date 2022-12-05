use std::io::{self, BufRead};

pub fn solve() -> io::Result<()> {
    let mut sum = 0;

    for line in io::stdin().lock().lines().map(|l| l.unwrap()) {
        let rucksack = line.trim().to_string();
        let (left, right) = split_rucksack(rucksack);
        let item = find_item_in_common(&left, &right);
        let priority = item.map_or(0, priority);
        sum += dbg!(priority);
    }

    println!("{}", sum);

    Ok(())
}

fn split_rucksack(rucksack: String) -> (String, String) {
    let half_len = rucksack.len() / 2;
    let (left, right) = (&rucksack[..half_len], &rucksack[half_len..]);
    (left.to_string(), right.to_string())
}

fn find_item_in_common(left: &str, right: &str) -> Option<char> {
    // Assume there's only one item in common
    let idx = |c: char| priority(c) as usize;

    let mut left_items = [false; 53];
    left.chars().for_each(|c| left_items[idx(c)] = true);
    let left_items = left_items;

    right.chars().find(|&c| left_items[idx(c)])
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
    fn test_split_rucksack() {
        assert_eq!(
            split_rucksack("aaaaBBBB".to_string()),
            ("aaaa".to_string(), "BBBB".to_string())
        );
    }

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('b'), 2);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }
}
