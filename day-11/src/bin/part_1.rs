use std::collections::HashMap;

use parse_display::{Display, FromStr};

#[derive(Clone, Debug)]
struct Monkey {
    index: usize,
    items: Vec<usize>,
    operation: Operation,
    test: Test,
    throw_true: usize,
    throw_false: usize,
    times_inspected: usize,
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
            operation: operation,
            test: test,
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
    Number(usize),
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
    fn apply(&self, old: usize) -> usize {
        match self.operator {
            Operator::Add => match self.left {
                Operand::Old => {
                    old + match self.right {
                        Operand::Old => old,
                        Operand::Number(n) => n,
                    }
                }
                Operand::Number(n) => {
                    n + match self.right {
                        Operand::Old => old,
                        Operand::Number(n) => n,
                    }
                }
            },
            Operator::Multiply => match self.left {
                Operand::Old => {
                    old * match self.right {
                        Operand::Old => old,
                        Operand::Number(n) => n,
                    }
                }
                Operand::Number(n) => {
                    n * match self.right {
                        Operand::Old => old,
                        Operand::Number(n) => n,
                    }
                }
            },
        }
    }
}

#[derive(Debug)]
struct Items(Vec<usize>);

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
fn solve(input: &str) -> i32 {
    // Config
    let number_of_rounds = 20;

    let mut monkey_map: HashMap<usize, Monkey> = HashMap::new();
    let monkey_strs = input.split("\r\n\r\n").collect::<Vec<_>>();
    dbg!(&monkey_strs);
    for monkey_str in monkey_strs {
        let monkey = monkey_str.parse::<Monkey>().unwrap();
        monkey_map.insert(monkey.index, monkey);
    }
    dbg!(&monkey_map);

    for _ in 0..number_of_rounds {
        for index in 0..monkey_map.len() {
            let mut monkey = monkey_map.get_mut(&index).unwrap();
            monkey.times_inspected += monkey.items.len();

            // Get a copy of monky to avoid borrowing issues
            let map = monkey_map.clone();
            let monkey = map.get(&index).unwrap();

            // go through each item and apply the operation
            for item in &monkey.items.clone() {
                // Apply operation
                let mut new_value = monkey.operation.apply(*item);
                // Worry level is reducded, divided by 3
                new_value = new_value / 3;
                // Test worry level
                if new_value % monkey.test.divisible_by == 0 {
                    // If true, throw to monkey
                    //monkey_map.get_mut(&monkey.throw_true).unwrap().items.push(new_value);
                    // Get result, print error if not found
                    let result = monkey_map.get_mut(&monkey.throw_true);
                    match result {
                        Some(m) => m.items.push(new_value),
                        None => {
                            println!("Error: Monkey {} not found", monkey.throw_true);
                            dbg!(monkey_map);
                            panic!();
                        }
                    }
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
            monkey_map.get_mut(&index).unwrap().items = vec![];
        }
    }

    // Get 2 highest times inspected
    let mut times_inspected: Vec<_> = monkey_map.values().collect();
    times_inspected.sort_by(|a, b| b.times_inspected.cmp(&a.times_inspected));
    let highest = times_inspected[0].times_inspected;
    let second_highest = times_inspected[1].times_inspected;

    let monkey_business = highest * second_highest;

    monkey_business as i32
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
            10605,
        )];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
