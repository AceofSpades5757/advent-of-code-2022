type Tree = u32;

/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    let mut visible = 0;

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

    // Count Edges
    visible += (forest.len() - 1) * 4;

    // Iterate Inner Trees
    for y in 1..(forest.len() - 1) {
        for x in 1..(forest.len() - 1) {
            let tree = forest[y][x];
            // Check all trees to the left, right, up, down to see if any are the same height or bigger
            // Left
            let mut vis: bool = true;
            for i in 1..(x + 1) {
                let other: Tree = forest[y][x - i];
                if other >= tree {
                    vis = false;
                    break;
                }
            }
            if vis {
                visible += 1;
                continue;
            }
            // Right
            vis = true;
            for i in 1..(forest.len() - x) {
                let other: Tree = forest[y][x + i];
                if other >= tree {
                    vis = false;
                    break;
                }
            }
            if vis {
                visible += 1;
                continue;
            }
            // Up
            vis = true;
            for i in 1..(y + 1) {
                let other: Tree = forest[y - i][x];
                if other >= tree {
                    vis = false;
                    break;
                }
            }
            if vis {
                visible += 1;
                continue;
            }
            // Down
            vis = true;
            for i in 1..(forest.len() - y) {
                let other: Tree = forest[y + i][x];
                if other >= tree {
                    vis = false;
                    break;
                }
            }
            if vis {
                visible += 1;
                continue;
            }
        }
    }

    visible as i32
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
35390", 21)
        ];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
