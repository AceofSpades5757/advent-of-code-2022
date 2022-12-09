use std::collections::HashSet;

use parse_display::{Display, FromStr};

/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    let mut rope = Rope::new();

    for line in input.lines() {
        let movement = line.parse::<Movement>().unwrap();
        for _ in 0..(movement.distance()) {
            rope.move_head(&movement);
        }
    }

    rope.tail_history.len() as i32
}

/// Rope with head and tail at x,y coordinates
#[derive(Debug)]
struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
    tail_history: HashSet<(i32, i32)>,
}

impl Rope {
    fn new() -> Self {
        let mut tail_history = HashSet::new();
        tail_history.insert((0, 0));
        Self {
            head: (0, 0),
            tail: (0, 0),
            tail_history,
        }
    }
}

/// Movement
impl Rope {
    /// Move the rope
    fn move_head(&mut self, movement: &Movement) {
        use Movement::*;

        match movement {
            Up(_) => {
                self.head.0 += 1;
                self.move_tail();
                self.tail_history.insert(self.tail);
            }
            Down(_) => {
                self.head.0 -= 1;
                self.move_tail();
                self.tail_history.insert(self.tail);
            }
            Right(_) => {
                self.head.1 += 1;
                self.move_tail();
                self.tail_history.insert(self.tail);
            }
            Left(_) => {
                self.head.1 -= 1;
                self.move_tail();
                self.tail_history.insert(self.tail);
            }
        }
    }
    /// Move the tail, which follows the head
    fn move_tail(&mut self) {
        // If 1 space away, even diagonally, do nothing
        if (self.head.0 - self.tail.0).abs() <= 1 && (self.head.1 - self.tail.1).abs() <= 1 {
            return;
        }

        // Diagonal
        if self.tail.0 != self.head.0 && self.tail.1 != self.head.1 {
            if self.tail.0 < self.head.0 {
                self.tail.0 += 1;
            } else {
                self.tail.0 -= 1;
            }
            if self.tail.1 < self.head.1 {
                self.tail.1 += 1;
            } else {
                self.tail.1 -= 1;
            }
        } else {
            // Not Diagonal
            if self.tail.0 == self.head.0 {
                if self.tail.1 > self.head.1 {
                    self.tail.1 -= 1;
                } else {
                    self.tail.1 += 1;
                }
            } else if self.tail.1 == self.head.1 {
                if self.tail.0 > self.head.0 {
                    self.tail.0 -= 1;
                } else {
                    self.tail.0 += 1;
                }
            }
        }
    }
}

impl Rope {
    /// Print tail history in x,y grid
    fn print_tail_history(&self) {
        // get max x and max y of hashset
        let mut max_x = 0;
        let mut max_y = 0;
        for (x, y) in &self.tail_history {
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
        }
        if max_x > max_y {
            max_y = max_x;
        } else {
            max_x = max_y;
        }

        println!(
            "{} {}|{} {}",
            self.head.0, self.head.1, self.tail.0, self.tail.1
        );
        for x in (0..=max_x + 1).rev() {
            for y in 0..=max_y + 1 {
                if self.head == (x, y) {
                    print!("H");
                } else if self.tail == (x, y) {
                    print!("T");
                } else if self.tail_history.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

#[derive(Display, FromStr, Debug)]
pub enum Movement {
    #[display("U {0}")]
    Up(i32),
    #[display("D {0}")]
    Down(i32),
    #[display("L {0}")]
    Left(i32),
    #[display("R {0}")]
    Right(i32),
}

impl Movement {
    fn distance(&self) -> i32 {
        use Movement::*;
        match self {
            Up(distance) => *distance,
            Down(distance) => *distance,
            Right(distance) => *distance,
            Left(distance) => *distance,
        }
    }
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
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
            13,
        )];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
