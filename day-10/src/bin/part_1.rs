use parse_display::{Display, FromStr};

#[derive(Debug)]
struct Environment {
    x: i32,
    cycle: i32,
}

impl Default for Environment {
    fn default() -> Self {
        Self { x: 1, cycle: 1 }
    }
}

impl Environment {
    fn signal_strength(&self) -> i32 {
        self.cycle * self.x
    }
}

/*
impl Environment {
    fn execute(&mut self, command: &Command) {
        match command {
            Command::Noop => {
                self.cycle += 1;
            }
            Command::Add(value) => {
                self.x += value;
                self.cycle += 2;
            }
        }
    }
}
*/

#[derive(Display, FromStr, Debug)]
enum Command {
    #[display("noop")]
    Noop,
    #[display("addx {0}")]
    Add(i32),
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
            "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
            13140,
        )];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}

/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    let important_signals = [20, 60, 100, 140, 180, 220];

    let mut environ: Environment = Default::default();
    let mut signals = vec![];
    for line in input.lines() {
        let command = line.parse::<Command>().unwrap();
        match command {
            Command::Noop => {
                environ.cycle += 1;
                if important_signals.contains(&environ.cycle) {
                    signals.push(environ.signal_strength());
                }
            }
            Command::Add(value) => {
                environ.cycle += 1;
                if important_signals.contains(&environ.cycle) {
                    signals.push(environ.signal_strength());
                }
                environ.cycle += 1;
                environ.x += value;
                if important_signals.contains(&environ.cycle) {
                    signals.push(environ.signal_strength());
                }
            }
        }
    }
    signals.iter().sum()
}
