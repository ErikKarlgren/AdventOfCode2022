use std::io::{self, BufRead};

use lazy_static::lazy_static;
use regex::Regex;

type Stacks = Vec<Vec<char>>;

struct Instruction {
    num_crates: u32,
    from: usize,
    to: usize,
}

pub fn solve() -> io::Result<()> {
    let mut stacks = vec![
        vec!['S', 'Z', 'P', 'D', 'L', 'B', 'F', 'C'],
        vec!['N', 'V', 'G', 'P', 'H', 'W', 'B'],
        vec!['F', 'W', 'B', 'J', 'G'],
        vec!['G', 'J', 'N', 'F', 'L', 'W', 'C', 'S'],
        vec!['W', 'J', 'L', 'T', 'P', 'M', 'S', 'H'],
        vec!['B', 'C', 'W', 'G', 'F', 'S'],
        vec!['H', 'T', 'P', 'M', 'Q', 'B', 'W'],
        vec!['F', 'S', 'W', 'T'],
        vec!['N', 'C', 'R'],
    ];

    // Skip 10 lines since it just shows the crates
    for line in io::stdin().lock().lines().skip(10).map(|l| l.unwrap()) {
        let instr = parse_instruction(line);
        execute_instruction(instr, &mut stacks);
    }

    println!("{}", crates_on_top(&stacks));

    Ok(())
}

fn parse_instruction(line: String) -> Instruction {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^move (\d+) from (\d+) to (\d+)").expect("Could not build regex");
    }

    let cap = RE.captures(&line).expect("Match not found");
    Instruction {
        num_crates: cap[1].parse().unwrap(),
        from: cap[2].parse().unwrap(),
        to: cap[3].parse().unwrap(),
    }
}

fn execute_instruction(instr: Instruction, stacks: &mut Stacks) {
    let mut removed_crates = vec![];

    for _ in 0..instr.num_crates {
        let _crate = stacks[instr.from - 1]
            .pop()
            .expect("Cannot take crate: stack is empty");
        removed_crates.push(_crate);
    }

    for _crate in removed_crates.into_iter().rev() {
        stacks[instr.to - 1].push(_crate);
    }
}

fn crates_on_top(stacks: &Stacks) -> String {
    stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&' '))
        .collect()
}
