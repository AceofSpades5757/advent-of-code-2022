use std::fmt;

type Coordinate = (i32, i32);

/// Solve the Puzzle
fn solve(input: &str) -> i32 {
    // Parse lists of coordinates
    let coordinate_groups = input
        .lines()
        .map(|line| {
            let mut coors = Vec::new();
            for coordinate_str in line.split(" -> ") {
                let pair = coordinate_str.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                let x = pair[0];
                let y = pair[1];
                coors.push((x, y));
            }
            coors
        })
        .collect::<Vec<Vec<(i32, i32)>>>();

    let mut map = Map::new(coordinate_groups);

    while let Ok(_) = map.next_sand() {}

    let answer = map.tiles.into_iter().flatten().filter(|t| *t == Tile::Sand).count();

    answer as i32
}

/// Grid Coordinate
///
/// Rock   - #
/// Air    - .
/// Source - +
/// Sand   - o
#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Rock,
    Air,
    Source,
    Sand,
}

#[derive(Debug)]
struct Map {
    // X Y Board
    tiles: Vec<Vec<Tile>>,
    x_min: i32,
}

impl Map {
    fn new(paths: Vec<Vec<Coordinate>>) -> Self {
        // Entrance
        let entrance = (500, 0);

        // Get min and max coordinate values.
        let mut x_min = i32::MAX;
        let mut x_max = i32::MIN;
        let mut y_max = i32::MIN;
        if entrance.0 < x_min {
            x_min = entrance.0;
        }
        if entrance.0 > x_max {
            x_max = entrance.0;
        }
        if entrance.1 > y_max {
            y_max = entrance.1;
        }
        for (x, y) in paths.clone().into_iter().flatten() {
            if x < x_min {
                x_min = x;
            }
            if x > x_max {
                x_max = x;
            }
            if y > y_max {
                y_max = y;
            }
        }

        // Rocks
        let mut path_coors: Vec<Coordinate> = Vec::new();
        for group in paths.iter() {
            for ((x_1, y_1), (x_2, y_2)) in group.clone().into_iter().zip(group.clone().into_iter().skip(1)) {
                let x_low: i32;
                let x_high: i32;
                let y_low: i32;
                let y_high: i32;
                if x_2 > x_1 || y_2 > y_1 {
                    x_low = x_1;
                    x_high = x_2;
                    y_low = y_1;
                    y_high = y_2;
                } else {
                    x_low = x_2;
                    x_high = x_1;
                    y_low = y_2;
                    y_high = y_1;
                }
                for x in x_low..=x_high {
                    for y in y_low..=y_high {
                        path_coors.push((x, y));
                    }
                }
            }
        }

        // Sand Coordinates
        let sand_coors: Vec<Coordinate> = Vec::new();

        // Tiles
        let mut map: Vec<Vec<Tile>> = Vec::new();
        for y in 0..=y_max {

            // New Row
            map.push(Vec::new());

            // Grid Coordinate
            //
            // Rock   - #
            // Air    - .
            // Source - +
            // Sand   - o
            for x in x_min..=x_max {
                // Source/Entrance
                if (x, y) == entrance {
                    map[y as usize].push(Tile::Source);
                    continue;
                }
                // Rock
                if path_coors.contains(&(x, y)) {
                    map[y as usize].push(Tile::Rock);
                    continue;
                }
                // Sand
                if sand_coors.contains(&(x, y)) {
                    map[y as usize].push(Tile::Sand);
                    continue;
                }
                // Air
                map[y as usize].push(Tile::Air);
            }
        }

        Self {
            tiles: map,
            x_min,
        }
    }
}

impl Map {
    /// Sand will either settle in the window of concern, or fall off into the void.
    /// There's also the chance of the source block being blocked.
    fn next_sand(&mut self) -> Result<(), MovementError> {
        let entrance = (500, 0);
        // Adjust for window of concern.
        let entrance = (entrance.0 - self.x_min, entrance.1);
        let sand = (entrance.0, entrance.1 + 1);

        // Source Blocked
        if self.tiles[sand.1 as usize][sand.0 as usize] == Tile::Sand {
            return Err(MovementError::SandFound);
        } else if self.tiles[sand.1 as usize][sand.0 as usize] == Tile::Rock {
            return Err(MovementError::RockFound);
        }

        // Move
        let mut moved_sand = move_sand(&sand, &self.tiles);
        let mut final_placement: Coordinate = sand;
        while let Ok(coor) = moved_sand {
            final_placement = coor;
            moved_sand = move_sand(&coor, &self.tiles);
        }

        if let Err(MovementError::OutofBounds) = moved_sand {
            return Err(MovementError::OutofBounds);
        }

        self.tiles[final_placement.1 as usize][final_placement.0 as usize] = Tile::Sand;
        Ok(())
    }
}

#[derive(Debug)]
enum MovementError {
    RockFound,
    SandFound,
    OutofBounds,
    AtRest,
}

fn move_sand(sand: &Coordinate, tiles: &Vec<Vec<Tile>>) -> Result<Coordinate, MovementError> {
    // Down
    let moved_sand = move_down(&sand, tiles);
    if let Err(MovementError::OutofBounds) = moved_sand {
        return Err(MovementError::OutofBounds);
    }

    if let Ok(coor) = moved_sand {
        return Ok(coor);
    } else {
        // Down Left
        let moved_sand = move_down_left(&sand, tiles);
        if let Err(MovementError::OutofBounds) = moved_sand {
            return Err(MovementError::OutofBounds);
        }

        if let Ok(coor) = moved_sand {
            return Ok(coor);
        } else {
            // Down Right
            let moved_sand = move_down_right(&sand, tiles);
            if let Err(MovementError::OutofBounds) = moved_sand {
                return Err(MovementError::OutofBounds);
            }

            if let Ok(coor) = moved_sand {
                return Ok(coor);
            } else {
                return Err(MovementError::AtRest);
            }
        }
    }
}

fn move_down(sand: &Coordinate, tiles: &Vec<Vec<Tile>>) -> Result<Coordinate, MovementError> {
    let new_pos = (sand.0, sand.1 + 1);

    if let Some(row) = tiles.get(new_pos.1 as usize) {
        if let Some(tile) = row.get(new_pos.0 as usize) {
            if *tile == Tile::Rock {
                return Err(MovementError::RockFound);
            } else if *tile == Tile::Sand {
                return Err(MovementError::SandFound);
            }
            return Ok(new_pos);
        } else {
            // Too far right
            return Err(MovementError::OutofBounds);
        }
    } else {
        // Too low
        return Err(MovementError::OutofBounds);
    }
}

fn move_down_left(sand: &Coordinate, tiles: &Vec<Vec<Tile>>) -> Result<Coordinate, MovementError> {
    let new_pos = (sand.0 - 1, sand.1 + 1);

    if let Some(row) = tiles.get(new_pos.1 as usize) {
        if let Some(tile) = row.get(new_pos.0 as usize) {
            if *tile == Tile::Rock {
                return Err(MovementError::RockFound);
            } else if *tile == Tile::Sand {
                return Err(MovementError::SandFound);
            }
            return Ok(new_pos);
        } else {
            // Too far right
            return Err(MovementError::OutofBounds);
        }
    } else {
        // Too low
        return Err(MovementError::OutofBounds);
    }
}

fn move_down_right(sand: &Coordinate, tiles: &Vec<Vec<Tile>>) -> Result<Coordinate, MovementError> {
    let new_pos = (sand.0 + 1, sand.1 + 1);

    if let Some(row) = tiles.get(new_pos.1 as usize) {
        if let Some(tile) = row.get(new_pos.0 as usize) {
            if *tile == Tile::Rock {
                return Err(MovementError::RockFound);
            } else if *tile == Tile::Sand {
                return Err(MovementError::SandFound);
            }
            return Ok(new_pos);
        } else {
            // Too far right
            return Err(MovementError::OutofBounds);
        }
    } else {
        // Too low
        return Err(MovementError::OutofBounds);
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();

        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[0].len() {
                let tile = &self.tiles[y][x];
                match tile {
                    Tile::Source => buffer.push_str("+"),
                    Tile::Rock => buffer.push_str("#"),
                    Tile::Sand => buffer.push_str("o"),
                    Tile::Air => buffer.push_str("."),
                }
            }
            buffer.push_str("\n");
        }

        write!(f, "{}", buffer)
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
            ("498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9", 24)
        ];
        for (input, expected) in tests {
            assert_eq!(solve(input), expected);
        }
    }
}
