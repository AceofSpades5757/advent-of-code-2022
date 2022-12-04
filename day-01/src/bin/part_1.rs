/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(|s| s.to_owned()).collect();
    let mut elves: Vec<Vec<String>> = vec![vec![]];
    for line in lines.into_iter() {
        if line.is_empty() {
            elves.push(vec![]);
        } else {
            elves.last_mut().unwrap().push(line);
        }
    }

    let largest: i32 = elves
        .into_iter()
        .map(|elf| {
            elf.into_iter()
                .map(|s| s.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap();

    largest
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
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
            24000,
        )];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
