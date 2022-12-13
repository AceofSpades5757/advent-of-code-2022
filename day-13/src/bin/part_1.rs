use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Element {
    Vector(Vec<Element>),
    Number(i32),
}

impl FromStr for Element {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = serde_json::from_str::<serde_json::Value>(s).unwrap();
        if val.is_array() {
            let array = val.as_array().unwrap();
            let elements = array
                .into_iter()
                .map(|v| v.clone().into())
                .collect::<Vec<Element>>();
            Ok(Element::Vector(elements))
        } else if val.is_i64() {
            Ok(Element::Number(val.as_i64().unwrap() as i32))
        } else {
            Err(())
        }
    }
}

impl Into<Element> for i32 {
    fn into(self) -> Element {
        Element::Number(self)
    }
}

impl Into<Element> for &i32 {
    fn into(self) -> Element {
        Element::Number(*self)
    }
}

impl Into<Element> for serde_json::Value {
    fn into(self) -> Element {
        match self {
            serde_json::Value::Number(n) => Element::Number(n.as_i64().unwrap() as i32),
            serde_json::Value::Array(a) => {
                Element::Vector(a.into_iter().map(|v| v.into()).collect())
            }
            _ => panic!("Unexpected value"),
        }
    }
}

fn compare(left: &Element, right: &Element) -> Ordering {
    match (left, right) {
        // Both are numbers
        (Element::Number(a), Element::Number(b)) => a.cmp(&b),
        // Both are vectors
        (Element::Vector(a), Element::Vector(b)) => {
            // Go through all the elements, comparing them until we find a difference
            for (left, right) in a.iter().zip(b.iter()) {
                if let Ordering::Equal = compare(left, right) {
                    continue;
                } else {
                    return compare(left, right);
                }
            }

            // Ran out of elements
            a.len().cmp(&b.len())
        }
        // One is a number and the other is a vector
        // Convert the number to a vector and compare
        (Element::Number(a), Element::Vector(b)) => compare(
            &Element::Vector(vec![a.into()]),
            &Element::Vector(b.clone()),
        ),
        (Element::Vector(a), Element::Number(b)) => compare(
            &Element::Vector(a.clone()),
            &Element::Vector(vec![b.into()]),
        ),
    }
}

/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    let mut score = 0;
    for (index, (left_str, right_str)) in input
        .lines()
        .step_by(3)
        .zip(input.lines().skip(1).step_by(3))
        .enumerate()
    {
        let pair = (index + 1) as i32;
        let left = left_str.parse::<Element>().unwrap();
        let right = right_str.parse::<Element>().unwrap();
        let cmp = compare(&left, &right);

        if cmp == Ordering::Less {
            score += pair;
        }
    }

    score
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
    fn test_compare_numbers() {
        let tests = [
            (("1".to_string(), "1".to_string()), Ordering::Equal),
            (("1".to_string(), "2".to_string()), Ordering::Less),
            (("3".to_string(), "2".to_string()), Ordering::Greater),
        ];
        for ((left, right), expected) in tests {
            assert_eq!(
                compare(&left.parse().unwrap(), &right.parse().unwrap()),
                expected
            );
        }
    }

    #[test]
    fn test_compare_lists() {
        let tests = [
            (
                ("[1,1,3,1,1]".to_string(), "[1,1,5,1,1]".to_string()),
                Ordering::Less,
            ),
            (
                ("[[1],[2,3,4]]".to_string(), "[[1],4]".to_string()),
                Ordering::Less,
            ),
            (
                ("[[4,4],4,4]".to_string(), "[[4,4],4,4,4]".to_string()),
                Ordering::Less,
            ),
            (
                ("[7,7,7,7]".to_string(), "[7,7,7]".to_string()),
                Ordering::Greater,
            ),
            (("[]".to_string(), "[3]".to_string()), Ordering::Less),
            (
                ("[[[]]]".to_string(), "[[]]".to_string()),
                Ordering::Greater,
            ),
            (
                (
                    "[1,[2,[3,[4,[5,6,7]]]],8,9]".to_string(),
                    "[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string(),
                ),
                Ordering::Greater,
            ),
        ];
        for ((left, right), expected) in tests {
            assert_eq!(
                compare(&left.parse().unwrap(), &right.parse().unwrap()),
                expected
            );
        }
    }

    #[test]
    fn test_compute() {
        let tests = vec![(
            "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
            13,
        )];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
