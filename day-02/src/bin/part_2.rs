/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(|s| s.to_owned()).collect();
    let mut score = 0;
    for line in lines.into_iter() {
        let v = line.split_whitespace().collect::<Vec<&str>>();
        let opp = Rps::from(v[0]);
        let outcome: Outcome = Outcome::from(v[1]);
        let me = opp.calculate(&outcome);

        score += &me.score();
        score += &me.battle(&opp).score();
    }
    score
}

#[derive(Debug, PartialEq)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Rps {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Rps::Rock,
            "B" | "Y" => Rps::Paper,
            "C" | "Z" => Rps::Scissors,
            _ => {
                let msg = format!("Invalid RPS: <{}>", s);
                dbg!(msg);
                panic!("Invalid RPS");
            }
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl From<&str> for Outcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => {
                let msg = format!("Invalid Outcome: <{}>", s);
                dbg!(msg);
                panic!("Invalid Outcome");
            }
        }
    }
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

impl Rps {
    fn battle(&self, other: &Rps) -> Outcome {
        match self {
            Rps::Rock => match other {
                Rps::Rock => Outcome::Draw,
                Rps::Paper => Outcome::Lose,
                Rps::Scissors => Outcome::Win,
            },
            Rps::Paper => match other {
                Rps::Rock => Outcome::Win,
                Rps::Paper => Outcome::Draw,
                Rps::Scissors => Outcome::Lose,
            },
            Rps::Scissors => match other {
                Rps::Rock => Outcome::Lose,
                Rps::Paper => Outcome::Win,
                Rps::Scissors => Outcome::Draw,
            },
        }
    }
}

impl Rps {
    fn calculate(&self, outcome: &Outcome) -> Self {
        match self {
            Rps::Rock => match outcome {
                Outcome::Win => Rps::Paper,
                Outcome::Lose => Rps::Scissors,
                Outcome::Draw => Rps::Rock,
            },
            Rps::Paper => match outcome {
                Outcome::Win => Rps::Scissors,
                Outcome::Lose => Rps::Rock,
                Outcome::Draw => Rps::Paper,
            },
            Rps::Scissors => match outcome {
                Outcome::Win => Rps::Rock,
                Outcome::Lose => Rps::Paper,
                Outcome::Draw => Rps::Scissors,
            },
        }
    }
}

impl Rps {
    fn score(&self) -> i32 {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let answer = solve(&input);
    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_compute() {
        let tests = vec![(
            "A Y
B X
C Z",
            12,
        )];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
