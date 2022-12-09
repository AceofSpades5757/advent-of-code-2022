use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

use parse_display::{Display, FromStr};

/// Rope knot with head and optional tail at x,y coordinates
#[derive(Debug, Clone)]
struct Knot {
    head: (i32, i32),
    tail: Rc<RefCell<Option<Knot>>>,
    history: HashSet<(i32, i32)>,
}

impl Knot {
    fn new() -> Self {
        let mut history = HashSet::new();
        history.insert((0, 0));
        Self {
            head: (0, 0),
            tail: Rc::new(RefCell::new(None)),
            history,
        }
    }
    fn add_tail(&mut self) {
        let mut last_tail = self.tail.borrow_mut();
        if last_tail.is_none() {
            *last_tail = Some(Knot::new());
        } else {
            last_tail.as_mut().unwrap().add_tail();
        }
    }
}

impl Knot {
    fn last(&self) -> Knot {
        match self.tail.borrow().as_ref() {
            Some(knot) => knot.last(),
            None => self.clone(),
        }
    }
}

/// Movement
impl Knot {
    /// Move the rope
    fn move_head(&mut self, movement: &Movement) {
        use Movement::*;

        match movement {
            Up(_) => {
                self.head.0 += 1;
                self.history.insert(self.head);
                self.move_tail();
            }
            Down(_) => {
                self.head.0 -= 1;
                self.history.insert(self.head);
                self.move_tail();
            }
            Right(_) => {
                self.head.1 += 1;
                self.history.insert(self.head);
                self.move_tail();
            }
            Left(_) => {
                self.head.1 -= 1;
                self.history.insert(self.head);
                self.move_tail();
            }
        }
    }
    /// Move the tail, which follows the head
    fn move_tail(&mut self) {
        if !self.tail.borrow().is_some() {
            return;
        }

        let tail: &mut Option<Knot> = &mut *self.tail.borrow_mut();
        if (self.head.0 - tail.clone().unwrap().head.0).abs() <= 1
            && (self.head.1 - tail.clone().unwrap().head.1).abs() <= 1
        {
            return;
        }
        if let Some(tail) = tail {
            // Diagonal
            if tail.head.0 != self.head.0 && tail.head.1 != self.head.1 {
                if tail.head.0 < self.head.0 {
                    tail.head.0 += 1;
                } else {
                    tail.head.0 -= 1;
                }
                if tail.head.1 < self.head.1 {
                    tail.head.1 += 1;
                } else {
                    tail.head.1 -= 1;
                }
            } else {
                // Not Diagonal
                if tail.head.0 == self.head.0 {
                    if tail.head.1 > self.head.1 {
                        tail.head.1 -= 1;
                    } else {
                        tail.head.1 += 1;
                    }
                } else if tail.head.1 == self.head.1 {
                    if tail.head.0 > self.head.0 {
                        tail.head.0 -= 1;
                    } else {
                        tail.head.0 += 1;
                    }
                }
            }
            tail.history.insert(tail.head);
            tail.move_tail();
        }
        //tail.as_mut().unwrap().move_tail();
    }
}

impl fmt::Display for Knot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        let mut max_x = 0;
        let mut max_y = 0;
        for (x, y) in &self.history {
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

        for x in (0..=max_x + 1).rev() {
            for y in 0..=max_y + 1 {
                if self.head == (x, y) {
                    buffer.push('H');
                } else if self.tail.borrow().is_some()
                    && self.tail.borrow().as_ref().unwrap().head == (x, y)
                {
                    buffer.push('T');
                } else if self.history.contains(&(x, y)) {
                    buffer.push('#');
                } else {
                    buffer.push('.');
                }
            }
            buffer.push('\n');
        }

        write!(f, "{}", buffer)
    }
}

/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    let knot_count: i32 = 9;

    let mut rope = Knot::new();
    //for _ in 0..(knot_count - 1) {
    // FIXME: But has the rope count off by one, but only when knot_count is > 2
    for _ in 0..knot_count {
        rope.add_tail();
    }

    for line in input.lines() {
        let movement = line.parse::<Movement>().unwrap();
        for _ in 0..(movement.distance()) {
            rope.move_head(&movement);
        }
    }

    println!("{}", rope.last());

    rope.last().history.len() as i32
}

fn main() {
    let input = include_str!("../../input.txt");
    let answer = solve(&input);
    println!("{}", answer);
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

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn simple_movement() {
        let input = "R 4";
        let answer = solve(&input);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_compute() {
        let tests = vec![
            (
                "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
                1,
            ),
            (
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
                36,
            ),
        ];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
