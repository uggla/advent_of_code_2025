use nom::{
    IResult, Parser,
    character::complete::{char, digit1, line_ending, one_of, space1},
    combinator::{map, map_res, opt},
    multi::{many_till, many1, separated_list1},
    sequence::{delimited, preceded, terminated},
};
use std::collections::{HashSet, VecDeque};

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<Data>> {
    // Leading/trailing whitespace is trimmed to make the parser resilient to
    // test fixtures that start with a newline.
    let input = input.trim();

    many1(terminated(parse_line, opt(line_ending))).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Data> {
    let (input, lights) = parse_lights(input)?;
    let (input, (buttons, joltages)) = many_till(
        preceded(space1, parse_button),
        preceded(space1, parse_joltages),
    )
    .parse(input)?;

    Ok((
        input,
        Data {
            lights,
            buttons,
            joltages,
        },
    ))
}

fn parse_lights(input: &str) -> IResult<&str, Vec<bool>> {
    delimited(char('['), many1(map(one_of(".#"), |c| c == '#')), char(']')).parse(input)
}

fn parse_button(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        char('('),
        separated_list1(char(','), parse_usize),
        char(')'),
    )
    .parse(input)
}

fn parse_joltages(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        char('{'),
        separated_list1(char(','), parse_usize),
        char('}'),
    )
    .parse(input)
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse).parse(input)
}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();

    data.iter().map(min_presses).sum()
}

fn min_presses(data: &Data) -> usize {
    let target = lights_to_mask(&data.lights);
    let buttons = buttons_to_masks(&data.buttons);

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    visited.insert(0);
    queue.push_back((0, 0));

    while let Some((state, steps)) = queue.pop_front() {
        if state == target {
            return steps;
        }

        for &button in &buttons {
            // XOR state and button
            let next = state ^ button;
            if visited.insert(next) {
                queue.push_back((next, steps + 1));
            }
        }
    }

    panic!("No solution found for {:?}", data);
}

fn lights_to_mask(lights: &[bool]) -> usize {
    lights
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, on)| if *on { acc | (1 << idx) } else { acc })
}

fn buttons_to_masks(buttons: &[Vec<usize>]) -> Vec<usize> {
    buttons
        .iter()
        .map(|indices| indices.iter().fold(0, |acc, &idx| acc | (1 << idx)))
        .collect()
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
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 7);
    }
}
