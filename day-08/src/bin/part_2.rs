type Tree = u32;

/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    // Create X Y Coordinate System
    let mut forest: Vec<Vec<Tree>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<Tree> = Vec::new();
        for c in line.chars() {
            let tree: Tree = c.to_string().parse().unwrap();
            row.push(tree);
        }
        forest.push(row);
    }

    // Iterate Inner Trees
    let mut best_score: i32 = 0;
    for y in 1..(forest.len() - 1) {
        for x in 1..(forest.len() - 1) {

            let tree = forest[y][x];

            let up;
            let down;
            let left;
            let right;

            // Check all trees to the left, right, up, down
            // Left
            let mut view = 0;
            for i in 1..(x + 1) {
                let other: Tree = forest[y][x - i];
                let distance: i32 = i as i32;
                view = distance;
                if other >= tree {
                    break;
                }
            }
            left = view;
            // Right
            let mut view = 0;
            for i in 1..(forest.len() - x) {
                let other: Tree = forest[y][x + i];
                let distance: i32 = i as i32;
                view = distance;
                if other >= tree {
                    break;
                }
            }
            right = view;
            // Up
            let mut view = 0;
            for i in 1..(y + 1) {
                let other: Tree = forest[y - i][x];
                let distance: i32 = i as i32;
                view = distance;
                if other >= tree {
                    break;
                }
            }
            up = view;
            // Down
            let mut view = 0;
            for i in 1..(forest.len() - y) {
                let other: Tree = forest[y + i][x];
                let distance: i32 = i as i32;
                view = distance;
                if other >= tree {
                    break;
                }
            }
            down = view;

            // Score
            let score: i32 = up * down * left * right;
            if score > best_score {
                best_score = score;
            }
        }
    }

    best_score
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
            ("30373
25512
65332
33549
35390", 8)
        ];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
