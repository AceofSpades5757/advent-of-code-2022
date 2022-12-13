use petgraph::algo::dijkstra;
use petgraph::prelude::DiGraphMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pos(usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Node {
    Start,
    End,
    Height(i32),
}

// Trait for chars to turn into numbers
// a -> 1, b -> 2, etc
trait CharToNum {
    fn to_num(&self) -> i32;
}

impl CharToNum for char {
    fn to_num(&self) -> i32 {
        let mut num = *self as i32 - 96;
        if num < 0 {
            num = 0;
        }
        num
    }
}

impl Node {
    pub fn height(&self) -> i32 {
        match self {
            Node::Start => 'a'.to_num(),
            Node::End => 'z'.to_num(),
            Node::Height(h) => *h,
        }
    }
}

impl Node {
    pub fn char(&self) -> char {
        match self {
            Node::Start => 'S',
            Node::End => 'E',
            Node::Height(h) => (b'a' + (*h as u8 - 1)) as char,
        }
    }
}

impl Node {
    pub fn can_move_to(&self, other: &Node) -> bool {
        // Other can be 1 unit higher, or any number of units lower
        let diff = self.height() - other.height();
        diff >= -1
    }
}

/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    let mut start: (i32, i32, char) = (-1, -1, ' ');
    let mut end: (i32, i32, char) = (-1, -1, ' ');
    let mut board: Vec<Vec<Node>> = Vec::new();
    for _ in 0..input.lines().count() {
        board.push(Vec::new());
    }

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = (x as i32, y as i32, c);
                    board[y].push(Node::Start);
                }
                'E' => {
                    end = (x as i32, y as i32, c);
                    board[y].push(Node::End)
                }
                'a'..='z' => board[y].push(Node::Height(c as i32 - 96)),
                _ => board[y].push(Node::Height(c.to_digit(10).unwrap() as i32)),
            }
        }
    }

    let mut edges: Vec<((i32, i32, char), (i32, i32, char))> = Vec::new();
    for (y, row) in board.iter().enumerate() {
        for (x, node) in row.iter().enumerate() {
            // Up
            if y > 0 {
                let up = board[y - 1][x];
                if node.can_move_to(&up) {
                    edges.push((
                        (x as i32, y as i32, node.char()),
                        (x as i32, y as i32 - 1, up.char()),
                    ));
                }
            }
            // Down
            if y < board.len() - 1 {
                let down = board[y + 1][x];
                if node.can_move_to(&down) {
                    edges.push((
                        (x as i32, y as i32, node.char()),
                        (x as i32, y as i32 + 1, down.char()),
                    ));
                }
            }
            // Left
            if x > 0 {
                let left = board[y][x - 1];
                if node.can_move_to(&left) {
                    edges.push((
                        (x as i32, y as i32, node.char()),
                        (x as i32 - 1, y as i32, left.char()),
                    ));
                }
            }
            // Right
            if x < row.len() - 1 {
                let right = board[y][x + 1];
                if node.can_move_to(&right) {
                    edges.push((
                        (x as i32, y as i32, node.char()),
                        (x as i32 + 1, y as i32, right.char()),
                    ));
                }
            }
        }
    }

    let graph = DiGraphMap::<_, ()>::from_edges(&edges);

    let result = dijkstra(&graph, start, Some(end), |_| 1);
    result[&end] as i32
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
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
            31,
        )];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
