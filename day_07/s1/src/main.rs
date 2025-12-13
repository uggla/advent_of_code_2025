use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Add,
};

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
    fn get(&self, coord: &Coord) -> Option<&char> {
        self.data.get(coord)
    }

    fn set(&mut self, coord: &Coord, value: char) {
        let cell = self.data.get_mut(coord).unwrap();
        *cell = value;
    }

    fn is_splitter(&self, coord: &Coord) -> bool {
        self.get(coord).is_some_and(|c| "^".contains(*c))
    }

    // fn neighbors(&self, coord: &Coord) -> Vec<Coord> {
    //     let mut neighbors = Vec::new();
    //     // Clockwise
    //     // 0 -----> x
    //     // |
    //     // |
    //     // v
    //     // y
    //     let directions: [Coord; 8] = [
    //         (1, 0).into(),
    //         (1, 1).into(),
    //         (0, 1).into(),
    //         (-1, 1).into(),
    //         (-1, 0).into(),
    //         (-1, -1).into(),
    //         (0, -1).into(),
    //         (1, -1).into(),
    //     ];
    //     for direction in directions.iter() {
    //         let neighbor = *coord + *direction;
    //         if self.is_roll(&neighbor) {
    //             neighbors.push(neighbor);
    //         }
    //     }
    //     neighbors
    // }

    fn down(&self, coord: &Coord) -> Option<Coord> {
        let down = *coord + Coord::new(0, 1);
        self.get(&down)?;
        Some(down)
    }

    fn split(&self, coord: &Coord) -> VecDeque<Coord> {
        let mut split_positions: VecDeque<Coord> = VecDeque::new();
        if self.get(&(*coord + Coord::new(1, 0))).is_some() {
            split_positions.push_front(*coord + Coord::new(1, 0));
        }
        if self.get(&(*coord + Coord::new(-1, 0))).is_some() {
            split_positions.push_front(*coord + Coord::new(-1, 0));
        }

        split_positions
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
    let mut grid = Grid::from(input.as_str());
    grid.print();

    let mut beam_position = Coord::new(grid.start.x, grid.start.y);
    let mut seen: HashSet<Coord> = HashSet::new();
    let mut splits = 0;
    let mut beams = VecDeque::new();

    beams.push_back(beam_position);

    while !beams.is_empty() {
        beam_position = beams.pop_front().unwrap();
        beam_position = match grid.down(&beam_position) {
            Some(pos) => pos,
            None => continue,
        };

        if seen.contains(&beam_position) {
            continue;
        }

        if grid.is_splitter(&beam_position) {
            splits += 1;
            let mut new_pos = grid.split(&beam_position);
            new_pos.iter().for_each(|o| {
                grid.set(o, '|');
                seen.insert(*o);
            });
            beams.append(&mut new_pos);
        } else {
            grid.set(&beam_position, '|');
            seen.insert(beam_position);
            beams.push_back(beam_position);
        }
        println!("--------------------------------------------");
        grid.print();
        dbg!(splits);
    }

    splits
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
            .......S.......
            ...............
            .......^.......
            ...............
            ......^.^......
            ...............
            .....^.^.^.....
            ...............
            ....^.^...^....
            ...............
            ...^.^...^.^...
            ...............
            ..^...^.....^..
            ...............
            .^.^.^.^.^...^.
            ...............
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 21);
    }
}
