use std::ops::RangeInclusive;

use nom::{
    IResult, Parser,
    character::complete::{digit1, line_ending, one_of},
    combinator::{all_consuming, map, map_res, opt},
    multi::many1,
    sequence::separated_pair,
};

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

fn parse(input: &str) -> IResult<&str, Data> {
    fn num(input: &str) -> IResult<&str, usize> {
        map_res(digit1, str::parse).parse(input)
    }

    fn range(input: &str) -> IResult<&str, RangeInclusive<usize>> {
        map(separated_pair(num, one_of("-"), num), |(start, end)| {
            start..=end
        })
        .parse(input)
    }

    fn data(input: &str) -> IResult<&str, Data> {
        let (input, fresh_ingredients) =
            nom::multi::separated_list1(line_ending, range).parse(input)?;
        let (input, _) = many1(line_ending).parse(input)?; // consume the blank line between sections
        let (input, ingredients) = nom::multi::separated_list1(line_ending, num).parse(input)?;
        let (input, _) = opt(line_ending).parse(input)?;

        Ok((
            input,
            Data {
                fresh_ingredients,
                ingredients,
            },
        ))
    }

    all_consuming(data).parse(input)
}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    fresh_ingredients: Vec<RangeInclusive<usize>>,
    ingredients: Vec<usize>,
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();
    let mut fresh_ingredients = 0;
    dbg!(&data);

    for ingredient in data.ingredients.iter() {
        for range in data.fresh_ingredients.iter() {
            if range.contains(ingredient) {
                fresh_ingredients += 1;
                break;
            }
        }
    }

    dbg!(fresh_ingredients)
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
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 3);
    }
}
