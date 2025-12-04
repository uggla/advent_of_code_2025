use std::{collections::HashMap, ops::Add};

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy, Default)]
pub struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }
}

impl From<(isize, isize)> for Coord {
    fn from(value: (isize, isize)) -> Self {
        Coord::new(value.0, value.1)
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord::new(self.x + other.x, self.y + other.y)
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Grid {
    data: HashMap<Coord, char>,
    rows: usize,
    cols: usize,
    start: Coord,
    end: Coord,
}

impl Grid {
    fn set(&mut self, coord: &Coord, value: char) {
        let cell = self.data.get_mut(coord).unwrap();
        *cell = value;
    }

    fn in_grid(&self, coord: &Coord) -> Option<&char> {
        self.data.get(coord)
    }

    fn is_roll(&self, coord: &Coord) -> bool {
        self.in_grid(coord).is_some_and(|c| "@".contains(*c))
    }

    fn neighbors(&self, coord: &Coord) -> Vec<Coord> {
        let mut neighbors = Vec::new();
        // Clockwise
        // 0 -----> x
        // |
        // |
        // v
        // y
        let directions: [Coord; 8] = [
            (1, 0).into(),
            (1, 1).into(),
            (0, 1).into(),
            (-1, 1).into(),
            (-1, 0).into(),
            (-1, -1).into(),
            (0, -1).into(),
            (1, -1).into(),
        ];
        for direction in directions.iter() {
            let neighbor = *coord + *direction;
            if self.is_roll(&neighbor) {
                neighbors.push(neighbor);
            }
        }
        neighbors
    }

    pub fn print(&self) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                print!("{}", self.data[&Coord::new(x as isize, y as isize)]);
            }
            println!();
        }
    }
}

impl From<Vec<Vec<char>>> for Grid {
    fn from(value: Vec<Vec<char>>) -> Self {
        from_vec_of_vec(value)
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        from_vec_of_vec(value.lines().map(|l| l.chars().collect()).collect())
    }
}

fn from_vec_of_vec(value: Vec<Vec<char>>) -> Grid {
    let mut data = HashMap::new();
    let mut start = Coord::default();
    let mut end = Coord::default();
    // Y
    let rows = value.len();
    // X
    let cols = value[0].len();

    for (y, row) in value.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            data.insert(Coord::new(x as isize, y as isize), *c);
            if *c == 'S' {
                start = Coord::new(x as isize, y as isize);
            } else if *c == 'E' {
                end = Coord::new(x as isize, y as isize);
            }
        }
    }

    Grid {
        data,
        rows,
        cols,
        start,
        end,
    }
}

fn run(input: String) -> usize {
    let mut result = 0;
    let mut grid = Grid::from(input.as_str());
    grid.print();

    loop {
        let mut roll_removed = Vec::new();
        for y in 0..grid.rows {
            for x in 0..grid.cols {
                let current_coord = Coord::new(x as isize, y as isize);
                if grid.is_roll(&current_coord) && grid.neighbors(&current_coord).len() < 4 {
                    result += 1;
                    roll_removed.push(current_coord);
                }
            }
        }

        if roll_removed.is_empty() {
            break;
        }

        // Update the cells where roll is removed
        for cell in roll_removed.iter() {
            grid.set(cell, 'x');
        }

        println!("-------------------------");
        grid.print();
    }
    dbg!(result)
}

fn main() {
    let input = read_input(None);

    let answer = run(input);

    println!("Answer: {}", answer);
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use indoc::indoc;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_fake() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_run1() {
        let input = read_input(Some(indoc!(
            r"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 43);
    }
}
