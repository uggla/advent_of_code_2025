use nom::{
    IResult, Parser,
    character::complete::{char, digit1, one_of},
    combinator::{map, map_res, opt},
    multi::{many1, separated_list1},
    sequence::{pair, preceded, terminated},
};

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<Data>> {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, str::parse).parse(input)
    }

    fn range(input: &str) -> IResult<&str, Data> {
        map(pair(number, preceded(char('-'), number)), |(start, end)| {
            Data { start, end }
        })
        .parse(input)
    }

    // Accept commas and line endings as separators, including trailing ones.
    fn separator(input: &str) -> IResult<&str, ()> {
        map(many1(one_of(",\r\n")), |_| ()).parse(input)
    }

    terminated(separated_list1(separator, range), opt(separator)).parse(input)
}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    start: usize,
    end: usize,
}

impl Data {
    fn is_valid(&self) -> bool {
        self.start <= self.end
    }
}

#[allow(dead_code)]
fn has_even_digits(value: usize) -> bool {
    value.to_string().len().is_multiple_of(2)
}

fn collect_identical_patterns(range: &Data) -> Vec<usize> {
    let mut matches = Vec::new();

    for n in range.start..=range.end {
        let s = n.to_string();
        let pattern_size = s.len() / 2;

        for pattern_size in (1..=pattern_size).rev() {
            let pattern = s.split_at(pattern_size).0;
            let pattern_chunks: Vec<&str> = s.split(pattern).collect();
            // dbg!(&pattern_chunks, pattern_size, pattern);
            if pattern_chunks.iter().all(|o| o.is_empty()) {
                matches.push(n);
                break;
            }
        }
    }

    matches
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();
    dbg!(&data);

    // Super quick check to ensure ranges are valid
    if let Some(invalid) = data.iter().find(|d| !d.is_valid()) {
        panic!("Invalid range: {:?}", invalid);
    }

    let matches: Vec<usize> = data.iter().flat_map(collect_identical_patterns).collect();
    dbg!(&matches);

    matches.iter().sum()
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
            r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 4174379265);
    }
}
