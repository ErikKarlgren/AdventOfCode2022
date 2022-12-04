use std::{
    io::{self, BufRead, ErrorKind},
    str::FromStr,
};

enum Play {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Play {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Play::Rock),
            "B" => Ok(Play::Paper),
            "C" => Ok(Play::Scissors),
            _ => Err(io::Error::new(ErrorKind::Other, "Wrong play format")),
        }
    }
}

enum State {
    Lose,
    Draw,
    Win,
}

impl FromStr for State {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(State::Lose),
            "Y" => Ok(State::Draw),
            "Z" => Ok(State::Win),
            _ => Err(io::Error::new(ErrorKind::Other, "Wrong state")),
        }
    }
}
pub fn solve() -> io::Result<()> {
    let mut score = 0;

    for line in io::stdin().lock().lines().map(|l| l.unwrap()) {
        let elems: Vec<_> = line.split_ascii_whitespace().take(2).collect();
        let (enemy_play, state) = (elems[0], elems[1]);
        let (enemy_play, state) = (Play::from_str(enemy_play)?, State::from_str(state)?);

        use Play::*;
        use State::*;

        let my_play = match (&enemy_play, &state) {
            (_, State::Draw) => enemy_play,
            (Rock, Win) | (Scissors, Lose) => Paper,
            (Paper, Win) | (Rock, Lose) => Scissors,
            _ => Rock,
        };

        score += match &my_play {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };

        score += match state {
            Lose => 0,
            Draw => 3,
            Win => 6,
        }
    }

    println!("Score: {}", score);
    Ok(())
}
