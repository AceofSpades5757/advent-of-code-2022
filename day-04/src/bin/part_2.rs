use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::newline;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;

/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    let (_, pairs) = parse_file(input).unwrap();

    let mut tally = 0;
    for (elf_1, elf_2) in pairs.iter() {
        if elf_1.contains(&elf_2.start()) || elf_1.contains(&elf_2.end()) {
            tally += 1;
        } else if elf_2.contains(&elf_1.start()) || elf_2.contains(&elf_1.end()) {
            tally += 1;
        }
    }
    tally
}

fn parse_range(input: &str) -> IResult<&str, std::ops::RangeInclusive<u32>> {
    Ok(map_res(
        separated_pair(
            map_res(digit1, str::parse::<u32>),
            tag("-"),
            map_res(digit1, str::parse::<u32>),
        ),
        |(l, h)| Ok::<std::ops::RangeInclusive<u32>, String>(l..=h),
    )(input)?)
}

fn parse_pair(
    input: &str,
) -> IResult<&str, (std::ops::RangeInclusive<u32>, std::ops::RangeInclusive<u32>)> {
    Ok(separated_pair(parse_range, tag(","), parse_range)(input)?)
}

fn parse_line(
    input: &str,
) -> IResult<&str, (std::ops::RangeInclusive<u32>, std::ops::RangeInclusive<u32>)> {
    Ok(terminated(parse_pair, opt(newline))(input)?)
}

/// Get pairs from input file.
fn parse_file(
    input: &str,
) -> IResult<&str, Vec<(std::ops::RangeInclusive<u32>, std::ops::RangeInclusive<u32>)>> {
    let (input, pairs) = many1(parse_line)(input)?;

    Ok((input, pairs))
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
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
            4,
        )];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
