use std::collections::HashMap;

use parse_display::{Display, FromStr};

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

#[derive(Clone, Debug)]
struct Monkey {
    index: usize,
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    throw_true: usize,
    throw_false: usize,
    times_inspected: u64,
}

impl std::str::FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkey = s.lines().nth(0).unwrap().parse::<MonkeyIndex>().unwrap();
        let items = s.lines().nth(1).unwrap().parse::<Items>().unwrap();
        let operation = s.lines().nth(2).unwrap().parse::<Operation>().unwrap();
        let test = s.lines().nth(3).unwrap().trim().parse::<Test>().unwrap();
        let throw_true = s
            .lines()
            .nth(4)
            .unwrap()
            .trim()
            .parse::<ThrowTrue>()
            .unwrap();
        let throw_false = s
            .lines()
            .nth(5)
            .unwrap()
            .trim()
            .parse::<ThrowFalse>()
            .unwrap();

        Ok(Monkey {
            index: monkey.0,
            items: items.0,
            operation,
            test,
            throw_true: throw_true.monkey,
            throw_false: throw_false.monkey,
            times_inspected: 0,
        })
    }
}

#[derive(Display, FromStr, Debug)]
#[display("Monkey {0}:")]
struct MonkeyIndex(usize);

#[derive(Clone, Debug)]
struct Operation {
    left: Operand,
    right: Operand,
    operator: Operator,
}

#[derive(Display, FromStr, Debug, Clone)]
enum Operator {
    #[display("+")]
    Add,
    #[display("*")]
    Multiply,
}

#[derive(Display, FromStr, Debug, Clone)]
enum Operand {
    #[display("old")]
    Old,
    #[display("{0}")]
    Number(u64),
}

impl std::str::FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s
            .split('=')
            .map(|s| s.trim())
            .last()
            .unwrap()
            .split_whitespace();
        let left = parts.next().unwrap().parse().unwrap();
        let operator = parts.next().unwrap().parse().unwrap();
        let right = parts.next().unwrap().parse().unwrap();
        Ok(Operation {
            left,
            right,
            operator,
        })
    }
}

impl Operation {
    fn apply(&self, old: &u64) -> u64 {
        match self.operator {
            Operator::Add => match self.left {
                Operand::Old => {
                    old + match self.right {
                        Operand::Old => *old,
                        Operand::Number(n) => n,
                    }
                }
                Operand::Number(n) => {
                    n + match self.right {
                        Operand::Old => *old,
                        Operand::Number(n) => n,
                    }
                }
            },
            Operator::Multiply => match self.left {
                Operand::Old => {
                    old * match self.right {
                        Operand::Old => *old,
                        Operand::Number(n) => n,
                    }
                }
                Operand::Number(n) => {
                    n * match self.right {
                        Operand::Old => *old,
                        Operand::Number(n) => n,
                    }
                }
            },
        }
    }
}

#[derive(Debug)]
struct Items(Vec<u64>);

#[derive(Display, FromStr, Debug, Clone)]
struct Test {
    #[display("Test: divisible by {}")]
    divisible_by: usize,
}

#[derive(Display, FromStr, Debug)]
struct ThrowTrue {
    #[display("If true: throw to monkey {}")]
    monkey: usize,
}

#[derive(Display, FromStr, Debug)]
struct ThrowFalse {
    #[display("If false: throw to monkey {}")]
    monkey: usize,
}

impl std::str::FromStr for Items {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s
            .split(':')
            .last()
            .unwrap()
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        Ok(Self(items))
    }
}

/// Solve the Puzzle
fn solve(input: &str) -> u64 {
    // Config
    let number_of_rounds = 10_000;

    let mut monkey_map: HashMap<usize, Monkey> = HashMap::new();
    let monkey_strs = input
        .split(LINE_ENDING.repeat(2).as_str())
        .collect::<Vec<_>>();
    for monkey_str in monkey_strs {
        let monkey = monkey_str.parse::<Monkey>().unwrap();
        monkey_map.insert(monkey.index, monkey);
    }

    // Chinese Remainder Theorem
    //
    // Credit: https://www.youtube.com/watch?v=0RkTrYDyzmE
    let magic_trick = monkey_map
        .values()
        .map(|m| m.test.divisible_by as u64)
        .product::<u64>();

    for _ in 0..number_of_rounds {
        for index in 0..monkey_map.len() {
            let monkey = monkey_map.get_mut(&index).unwrap();
            monkey.times_inspected += monkey.items.len() as u64;

            // Get a copy of monky to avoid borrowing issues
            let map = monkey_map.clone();
            let monkey = map.get(&index).unwrap();

            // go through each item and apply the operation
            for item in &monkey.items.clone() {
                // Apply operation
                let mut new_value = monkey.operation.apply(item);
                // Manage Worry Level
                new_value %= magic_trick;
                // Test worry level
                if new_value.clone() % (monkey.test.divisible_by as u64) == 0 {
                    // If true, throw to monkey
                    monkey_map
                        .get_mut(&monkey.throw_true)
                        .unwrap()
                        .items
                        .push(new_value);
                } else {
                    // If false, throw to monkey
                    monkey_map
                        .get_mut(&monkey.throw_false)
                        .unwrap()
                        .items
                        .push(new_value);
                }
            }
            // Remove items from this monkey
            monkey_map.get_mut(&index).unwrap().items.clear();
        }
    }

    // Get 2 highest times inspected
    let mut times_inspected: Vec<_> = monkey_map.values().collect();
    times_inspected.sort_by(|a, b| b.times_inspected.cmp(&a.times_inspected));
    let highest = times_inspected[0].times_inspected.clone();
    let second_highest = times_inspected[1].times_inspected.clone();

    let monkey_business = highest * second_highest;

    monkey_business
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
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
            2713310158,
        )];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
