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

    let mut sums: Vec<i32> = elves.into_iter()
        .map(|e|
            e.into_iter()
            .map(|s| s.parse::<i32>().unwrap())
            .sum::<i32>()
        )
        .collect();
    sums.sort();
    sums.reverse();

    let top_3: i32 = sums.iter().take(3).sum();

    top_3
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect(
        "Input file exists and is readable in the root member's directory: day-XX/input.txt",
    );
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
        let tests = vec![
            ("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000", 45000)
        ];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
