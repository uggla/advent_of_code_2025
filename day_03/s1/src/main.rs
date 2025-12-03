use nom::{
    IResult, Parser,
    character::complete::{digit1, line_ending},
    combinator::{map, opt},
    multi::many1,
    sequence::terminated,
};

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Data {
    bank: Vec<u8>,
}

fn parse_line(input: &str) -> IResult<&str, Data> {
    map(digit1, |digits: &str| Data {
        bank: digits
            .chars()
            .map(|c| c.to_digit(10).expect("digit1 only yields digits") as u8)
            .collect(),
    })
    .parse(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Data>> {
    many1(terminated(parse_line, opt(line_ending))).parse(input)
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();

    let mut combined = Vec::new();
    for d in data.iter() {
        let digits = &d.bank;
        assert!(digits.len() >= 2, "expected at least two digits");

        // Find the first highest digit
        let mut suffix_max_after = vec![0u8; digits.len()];
        let mut current_max = 0u8;
        for i in (0..digits.len() - 1).rev() {
            if digits[i + 1] > current_max {
                current_max = digits[i + 1];
            }
            suffix_max_after[i] = current_max;
        }

        // Find the second digit that produce the highest number
        let mut best = 0u8;
        for i in 0..digits.len() - 1 {
            let second = suffix_max_after[i];
            let candidate = digits[i] * 10 + second;
            if candidate > best {
                best = candidate;
            }
        }

        combined.push(best);
    }

    // Sum the highest values for each bank.
    let total: usize = combined.iter().map(|&v| v as usize).sum();
    dbg!(&combined, &total);

    total
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
            987654321111111
            811111111111119
            234234234234278
            818181911112111
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 357);
    }
}
