use std::collections::HashSet;

/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    let chars = input.chars();
    let mut stack = Vec::with_capacity(4);
    for (index, chr) in chars.enumerate() {
        if stack.len() == 4 {
            stack.remove(0);
        }
        stack.push(chr);
        let set: HashSet<char> = HashSet::from_iter(stack.iter().cloned());
        if set.len() == 4 {
            return index as i32 + 1;
        }
    }
    unreachable!();
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
        let tests = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
