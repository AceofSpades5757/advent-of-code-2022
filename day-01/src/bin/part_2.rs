/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    let mut sums: Vec<i32> = input
        .split("\n\n")
        .map(|workload| workload.lines().map(|s| s.parse::<i32>().unwrap()).sum())
        .collect();
    sums.sort();
    sums.reverse();

    sums.iter().take(3).sum()
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
            45000,
        )];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
