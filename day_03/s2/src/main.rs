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

fn select_max_number(digits: &[u8], target_len: usize) -> Vec<u8> {
    assert!(
        digits.len() >= target_len,
        "expected at least {} digits",
        target_len
    );

    let mut to_remove = digits.len() - target_len;
    let mut stack: Vec<u8> = Vec::with_capacity(target_len);

    for &digit in digits {
        while to_remove > 0 && !stack.is_empty() && stack[stack.len() - 1] < digit {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(digit);
    }

    stack.truncate(target_len);
    stack
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();

    let mut combined = Vec::new();
    for d in data.iter() {
        let best_digits = select_max_number(&d.bank, 12);
        let best_number = best_digits
            .iter()
            .fold(0, |acc, &digit| acc * 10 + digit as usize);
        combined.push(best_number);
    }

    // Sum the highest values for each bank.
    let total = combined.iter().sum();
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
        assert_eq!(answer, 3_121_910_778_619);
    }

    #[test]
    fn test_select_max_number_basic() {
        let digits = vec![1, 2, 3, 1, 2];
        let result = select_max_number(&digits, 3);
        assert_eq!(result, vec![3, 1, 2]);
    }

    #[test]
    fn test_select_max_number_descending_keeps_prefix() {
        let digits = vec![9, 8, 7, 6];
        let result = select_max_number(&digits, 2);
        assert_eq!(result, vec![9, 8]);
    }
}
