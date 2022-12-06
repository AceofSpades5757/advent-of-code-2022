use std::collections::HashSet;

/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    let MARKER_SIZE = 14;
    let chars = input.chars();
    let mut stack = Vec::with_capacity(MARKER_SIZE);
    for (index, chr) in chars.enumerate() {
        if stack.len() == MARKER_SIZE {
            stack.remove(0);
        }
        stack.push(chr);
        let set: HashSet<char> = HashSet::from_iter(stack.iter().cloned());
        if set.len() == MARKER_SIZE {
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
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
