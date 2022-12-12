use pathfinding::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Board {
    grid: Vec<Vec<Tile>>,
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pos(usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Start,
    End,
    Height(i32),
}

impl Tile {
    pub fn height(&self) -> i32 {
        match self {
            Tile::Start => 0,
            Tile::End => 0,
            Tile::Height(h) => *h,
        }
    }
}

/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    let mut curr: (i32, i32);
    let mut graph: Vec<Vec<Tile>> = Vec::new();
    for _ in 0..input.lines().count() {
        graph.push(Vec::new());
    }
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    curr = (i as i32, j as i32);
                    graph[i].push(Tile::Start);
                },
                'E' => graph[i].push(Tile::End),
                'a'..='z' => graph[i].push(Tile::Height(c as i32 - 96)),
                _ => graph[i].push(Tile::Height(c.to_digit(10).unwrap() as i32)),
            }
        }
    }
    dbg!(&graph);

    todo!()
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
            ("Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi", 31)
        ];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Successor {
    pub position: Pos,
    pub cost: i32,
}

impl Board {
    pub fn get_successors(&self, position: &Pos) -> Vec<Successor> {
        let mut successors = Vec::new();
        let (x, y) = (position.0, position.1);
        let height = self.grid[x][y].height();
        let mut add_successor = |x, y| {
            let cost = (self.grid[x][y].height() - height).abs();
            successors.push(Successor {
                position: Pos(x, y),
                cost,
            });
        };
        if x > 0 {
            add_successor(x - 1, y);
        }
        if x < self.grid.len() - 1 {
            add_successor(x + 1, y);
        }
        if y > 0 {
            add_successor(x, y - 1);
        }
        if y < self.grid[x].len() - 1 {
            add_successor(x, y + 1);
        }
        successors
    }
}
