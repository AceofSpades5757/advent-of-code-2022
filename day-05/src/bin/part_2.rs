use regex::Regex;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

// use pest to read the rest
#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct GrammarParser;

#[derive(Debug)]
struct Move {
    count: i32,
    from: usize,
    to: usize,
}

/// Solve the Puzzle
fn solve(input: &str) -> String {
    let lines: Vec<String> = input.lines().map(|s| s.to_owned()).collect();
    let mut reader = lines.iter();

    let number_of_levels = lines.first().unwrap().len() / 4 + 1;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..number_of_levels {
        stacks.push(Vec::new());
    }

    // boxes
    loop {
        let line = reader.next().unwrap();
        let re = Regex::new(r"(\s*(\d)\s*)+").unwrap();
        if re.is_match(line) {
            break;
        }

        let mut level = 1;
        line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .for_each(|chunk| {
                let string = chunk.iter().take(3).collect::<String>();

                if string == "   " {
                    // ...
                } else {
                    let char = chunk[1];
                    stacks[level - 1].push(char);
                }
                level += 1;
            });
    }

    // reverse stacks
    stacks.iter_mut().for_each(|stack| stack.reverse());

    // empty line
    reader.next();

    // movements
    let string = reader
        .collect::<Vec<&String>>()
        .into_iter()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
        .join("\n");
    let movements = GrammarParser::parse(Rule::movements, &string)
        .expect("successful parse")
        .next()
        .unwrap();

    let mut moves: Vec<Move> = Vec::new();
    for movement in movements.into_inner() {
        match movement.as_rule() {
            Rule::movement => {
                let mut move_ = Move {
                    count: 0,
                    from: 0,
                    to: 0,
                };
                for i in movement.into_inner() {
                    match i.as_rule() {
                        Rule::move_ => {
                            move_.count = i
                                .into_inner()
                                .next()
                                .unwrap()
                                .as_str()
                                .parse::<i32>()
                                .unwrap();
                        }
                        Rule::from => {
                            move_.from = i
                                .into_inner()
                                .next()
                                .unwrap()
                                .as_str()
                                .parse::<usize>()
                                .unwrap();
                        }
                        Rule::to => {
                            move_.to = i
                                .into_inner()
                                .next()
                                .unwrap()
                                .as_str()
                                .parse::<usize>()
                                .unwrap();
                        }
                        _ => unreachable!(),
                    }
                }
                moves.push(move_);
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    // Apply Moves
    for move_ in moves {
        let from: usize = move_.from - 1;
        let to: usize = move_.to - 1;
        let count = move_.count;

        let mut boxes = Vec::new();
        for _ in 0..count {
            let char = stacks[from].pop().unwrap();
            boxes.push(char);
        }
        boxes.reverse();
        for c in boxes {
            stacks[to].push(c);
        }
    }

    // get top of stacks
    let mut result = String::new();
    for stack in stacks {
        result.push(stack.last().unwrap().clone());
    }

    result
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
            "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
            "MCD",
        )];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
