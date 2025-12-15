use itertools::Itertools;
use nom::{
    IResult, Parser,
    character::complete::{digit1, line_ending, multispace0, one_of},
    combinator::{map, map_res, opt},
    multi::separated_list1,
    sequence::{pair, preceded, terminated},
};

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

fn parse(input: &str) -> IResult<&str, Data> {
    let number = || preceded(multispace0, map_res(digit1, str::parse::<usize>));

    // Parse a single `x,y` coordinate pair into a `Points` instance.
    let point = map(
        pair(terminated(number(), one_of(",")), number()),
        |(x, y)| Points { x, y },
    );

    // Collect all lines into a single `Data` entry; allow a trailing newline.
    map(
        terminated(separated_list1(line_ending, point), opt(line_ending)),
        |points| Data { points },
    )
    .parse(input)
}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    points: Vec<Points>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Points {
    x: usize,
    y: usize,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Points,
    p2: Points,
    surface: usize,
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();

    let rectangles: Vec<Rectangle> = data
        .points
        .iter()
        .combinations(2)
        .map(|pair| {
            let p1 = pair[0].clone();
            let p2 = pair[1].clone();
            let surface = (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1);
            Rectangle { p1, p2, surface }
        })
        .collect();

    rectangles
        .iter()
        .max_by_key(|rect| rect.surface)
        .map(|rect| rect.surface)
        .unwrap_or(0)
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
            7,1
            11,1
            11,7
            9,7
            9,5
            2,5
        2,3
        7,3
        "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 50);
    }
}
