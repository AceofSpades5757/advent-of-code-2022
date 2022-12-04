/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(|s| s.to_owned()).collect();

    let mut total = 0;
    for pair in lines.into_iter() {
        let first_elf = pair.split(",").next().unwrap();
        let second_elf = pair.split(",").last().unwrap();

        let first_lower = first_elf.split("-").next().unwrap().parse::<i32>().unwrap();
        let first_upper = first_elf.split("-").last().unwrap().parse::<i32>().unwrap();
        let second_lower = second_elf.split("-").next().unwrap().parse::<i32>().unwrap();
        let second_upper = second_elf.split("-").last().unwrap().parse::<i32>().unwrap();
        let first_range = first_lower..=first_upper;
        let second_range = second_lower..=second_upper;

        if first_range.contains(&second_lower) || first_range.contains(&second_upper) {
            total += 1;
        } else if second_range.contains(&first_lower) || second_range.contains(&first_upper) {
            total += 1;
        }
    }

    total
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
            ("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8", 4)
        ];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
