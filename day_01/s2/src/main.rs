use nom::{
    IResult, Parser,
    character::complete::{digit1, line_ending, one_of},
    combinator::{map, map_res, opt},
    multi::many1,
    sequence::{pair, terminated},
};

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<Data>> {
    // Parse one line like `L68`
    fn instruction(input: &str) -> IResult<&str, Data> {
        map(
            pair(
                map(one_of("LR"), |c| match c {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => unreachable!(),
                }),
                map_res(digit1, str::parse::<isize>),
            ),
            |(direction, clicks)| Data { direction, clicks },
        )
        .parse(input)
    }

    many1(terminated(instruction, opt(line_ending))).parse(input)
}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    direction: Direction,
    clicks: isize,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

// impl Data {
//     // Get all neighbours clockwise
//     fn get_neighbours(&self, coord: Coord) -> Vec<Option<(Coord, char)>> {
//         let neighbours_coords: Vec<Coord> =
//             Vec::from([(1, 0).into(), (0, 1).into(), (-1, 0).into(), (0, -1).into()]);
//
//         let neighbours = neighbours_coords
//             .iter()
//             .map(|nc| self.grid.get(&(coord + *nc)).map(|v| (coord + *nc, *v)))
//             .collect::<Vec<Option<(Coord, char)>>>();
//         neighbours
//     }
// }
//
// #[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
// enum Direction {
//     Right = 0,
//     Down = 1,
//     Left = 2,
//     Up = 3,
// }
//
// #[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd, Hash)]
// struct Coord {
//     x: isize,
//     y: isize,
// }
//
// impl From<(isize, isize)> for Coord {
//     fn from((x, y): (isize, isize)) -> Self {
//         Self { x, y }
//     }
// }
//
// impl Add<Coord> for Coord {
//     type Output = Coord;
//     fn add(self, rhs: Coord) -> Self::Output {
//         Self {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }

struct Dial {
    cursor: isize,
    zeros: usize,
}

impl Dial {
    fn new() -> Self {
        Self {
            cursor: 50,
            zeros: 0,
        }
    }

    fn right(&mut self, clicks: isize) {
        // Wrap around the 0-99 dial when moving clockwise, counting every time we hit 0
        let passes_zero = (self.cursor + clicks) / 100;

        self.cursor = (self.cursor + clicks).rem_euclid(100);
        self.zeros += passes_zero as usize;
    }

    fn left(&mut self, clicks: isize) {
        // Wrap around the 0-99 dial when moving counter-clockwise, counting every time we hit 0
        let passes_zero = if self.cursor == 0 {
            clicks / 100
        } else {
            (clicks + (100 - self.cursor)) / 100
        };

        self.cursor = (self.cursor - clicks).rem_euclid(100);
        self.zeros += passes_zero as usize;
    }
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();
    dbg!(&data);

    let mut dial = Dial::new();

    for instruction in data {
        match instruction.direction {
            Direction::Left => dial.left(instruction.clicks),
            Direction::Right => dial.right(instruction.clicks),
        }
    }

    // print_text_map(
    //     &data
    //         .grid
    //         .iter()
    //         .map(|c| (c.0.x as usize, c.0.y as usize, *c.1))
    //         .collect::<Vec<(usize, usize, char)>>(),
    //     data.length_x,
    //     data.length_y,
    // );

    dial.zeros
}

#[allow(dead_code)]
fn print_text_map(coordinates: &[(usize, usize, char)], width: usize, height: usize) {
    let mut grid = vec![vec!['.'; width]; height];

    // Place the points on the grid
    for &(x, y, v) in coordinates {
        if x < width && y < height {
            grid[y][x] = v; // Assuming the origin (0,0) is at the top-left corner
        }
    }

    // Print the grid row by row
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!(); // Newline at the end of each row
    }
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
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_counts_zero_passes_right() {
        let input = read_input(Some("R1000\n"));
        let answer = run(input);
        assert_eq!(answer, 10);
    }

    #[test]
    fn test_counts_zero_passes_left() {
        let input = read_input(Some("L250\n"));
        let answer = run(input);
        assert_eq!(answer, 3);
    }
}
