use parse_display::{Display, FromStr};

#[derive(Debug)]
struct Environment {
    x: i32,
    cycle: usize,
}

impl Default for Environment {
    fn default() -> Self {
        Self { x: 1, cycle: 1 }
    }
}

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
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....",
        )];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}

/// Solve the Puzzle
fn solve(input: &str) -> String {
    let mut environ: Environment = Default::default();
    let mut level: usize = 0;
    let mut canvas: [[char; 40]; 6] = [[' '; 40]; 6];
    println!("{} {}", environ.cycle, level);
    for line in input.lines() {
        let command = line.parse::<Command>().unwrap();
        match command {
            Command::Noop => {
                let x_range = (environ.x - 1)..=(environ.x + 1);
                level = (environ.cycle - 1) / 40;
                let col = environ.cycle % 40;
                //canvas[level][col] = '#';
                if x_range.contains(&(col as i32)) {
                    canvas[level][col] = '#';
                } else {
                    canvas[level][col] = '.';
                }
                environ.cycle += 1;
                //println!("{} {}", environ.cycle, level);
            }
            Command::Add(value) => {
                let x_range = (environ.x - 1)..=(environ.x + 1);
                level = (environ.cycle - 1) / 40;
                let col = environ.cycle % 40;
                //canvas[level][col] = '#';
                if x_range.contains(&(col as i32)) {
                    canvas[level][col] = '#';
                } else {
                    canvas[level][col] = '.';
                }
                environ.cycle += 1;
                //println!("{} {}", environ.cycle, level);

                environ.x += value;
                let x_range = (environ.x - 1)..=(environ.x + 1);
                level = (environ.cycle - 1) / 40;
                let col = environ.cycle % 40;
                //canvas[level][col] = '#';
                if x_range.contains(&(col as i32)) {
                    canvas[level][col] = '#';
                } else {
                    canvas[level][col] = '.';
                }
                environ.cycle += 1;
                //println!("{} {}", environ.cycle, level);
            }
        }
    }
    let mut output = String::new();
    for row in canvas.iter() {
        for col in row.iter() {
            output.push(*col);
        }
        output.push('\n');
    }
    output
}
