use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use parse_display::{Display, FromStr};

/// Rope knot with head and optional tail at x,y coordinates
/// Uses interior mutability to allow for recursive tail
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
}

/// Movement
impl Knot {
    /// Move the rope
    fn move_head(&mut self, movement: &Movement) {
        use Movement::*;

        match movement {
            Up(_) => {
                self.head.0 += 1;
                self.move_tail();
                self.history.insert(self.head);
            }
            Down(_) => {
                self.head.0 -= 1;
                self.move_tail();
                self.history.insert(self.head);
            }
            Right(_) => {
                self.head.1 += 1;
                self.move_tail();
                self.history.insert(self.head);
            }
            Left(_) => {
                self.head.1 -= 1;
                self.move_tail();
                self.history.insert(self.head);
            }
        }
    }
    /// Move the tail, which follows the head
    fn move_tail(&mut self) {
        if self.tail.borrow().is_some() {
            return;
        }

        /*
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
        } else { // Not Diagonal
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
        */

        // Do the same thing as above, in the comment
        let tail: &mut Option<Knot> = &mut *self.tail.borrow_mut();
        if let Some(tail) = tail {
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
        }
        //tail.move_tail();
        tail.as_mut().unwrap().move_tail();
    }
}

impl Knot {
    /// Print tail history in x,y grid
    fn print_tail_history(&self) {
        // get max x and max y of hashset
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

        // print grid
        //println!("{} {}|{} {}", self.head.0, self.head.1, self.tail.as_ref().unwrap().head.0, self.tail.as_ref().unwrap().head.1);
        for x in (0..=max_x + 1).rev() {
            for y in 0..=max_y + 1 {
                if self.head == (x, y) {
                    print!("H");
                //} else if self.tail.clone().as_ref().clone().borrow().is_some() && self.tail.as_ref().borrow().unwrap().head == (x, y) {
                } else if self.tail.borrow().is_some()
                    && self.tail.borrow().as_ref().unwrap().head == (x, y)
                {
                    print!("T");
                } else if self.history.contains(&(x, y)) {
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

/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    // Knot with 9 elements
    let mut knot = Knot::new();
    for _ in 0..9 {
        knot.add_tail();
    }
    //dbg!(&knot);

    //for line in input.lines() {
    for (index, line) in input.lines().enumerate() {
        let movement = line.parse::<Movement>().unwrap();
        //dbg!(&movement);
        for _ in 0..(movement.distance()) {
            //rope.print_tail_history();
            knot.move_head(&movement);
        }
        //rope.move_head(&movement);
        //rope.print_tail_history();
        if index == 2 {
            //panic!();
        }
    }

    // Get the last knot's history
    let last_history = knot.get_last_history();

    last_history.len() as i32
}

impl Knot {
    fn add_tail(&mut self) {
        let mut new_tail = Knot::new();
        new_tail.head = self.head;
        new_tail.tail = Rc::new(RefCell::new(Some(self.clone())));
        self.tail = Rc::new(RefCell::new(Some(new_tail)));
    }
}

impl Knot {
    fn get_last_history(&self) -> HashSet<(i32, i32)> {
        let mut last_tail: Option<Knot> = Some(self.clone());
        while let Some(tail) = last_tail.clone() {
            while let Some(tail) = tail.tail.borrow().clone() {
                last_tail = Some(tail);
            }
        }
        //dbg!(&last_tail);
        last_tail.unwrap().history
    }
}
