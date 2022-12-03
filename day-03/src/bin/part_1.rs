/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(|s| s.to_owned()).collect();

    let mut sum = 0;
    for rucksack in lines.into_iter() {

        let first_compartment = rucksack[0..rucksack.len() / 2].to_owned();
        let second_compartment = rucksack[rucksack.len() / 2..].to_owned();

        let mut letter = None;
        for c in first_compartment.chars() {
            if second_compartment.contains(c) {
                letter = Some(c);
                break;
            }
        }
        if letter.is_none() {
            panic!("No letter found");
        }
        sum += letter_to_number(letter.unwrap());
    }
    sum
}

fn letter_to_number(letter: char) -> i32 {
    match letter {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => panic!("Invalid letter"),
    }
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
            ("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw", 157)
        ];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
