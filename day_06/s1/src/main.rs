use nom::{IResult, error::ErrorKind};

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

fn parse(input: &str) -> IResult<&str, Data> {
    let mut lines: Vec<&str> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    if lines.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            ErrorKind::Eof,
        )));
    }

    let operations_line = lines.pop().unwrap();

    let mut numbers = Vec::new();
    for line in lines {
        for value in line.split_whitespace() {
            let parsed = value
                .parse::<usize>()
                .map_err(|_| nom::Err::Error(nom::error::Error::new(value, ErrorKind::Digit)))?;
            numbers.push(parsed);
        }
    }

    let mut operations = Vec::new();
    for op in operations_line.split_whitespace() {
        let mut chars = op.chars();
        let ch = chars
            .next()
            .ok_or_else(|| nom::Err::Error(nom::error::Error::new(op, ErrorKind::OneOf)))?;

        if chars.next().is_some() {
            return Err(nom::Err::Error(nom::error::Error::new(
                op,
                ErrorKind::OneOf,
            )));
        }

        operations.push(ch);
    }

    Ok((
        "",
        Data {
            numbers,
            operations,
        },
    ))
}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    numbers: Vec<usize>,
    operations: Vec<char>,
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();

    let columns = data.operations.len();
    assert!(columns > 0, "no operations parsed");
    assert!(
        data.numbers.len() % columns == 0,
        "numbers do not align with operations"
    );

    dbg!(&data);

    let rows = data.numbers.len() / columns;

    let mut total = 0usize;

    for (col_idx, op) in data.operations.iter().enumerate() {
        let col_iter = (0..rows).map(|row| data.numbers[row * columns + col_idx]);

        let col_value = match op {
            '*' => col_iter.product::<usize>(),
            '+' => col_iter.sum::<usize>(),
            other => panic!("unsupported operation {}", other),
        };

        total += col_value;
    }

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
            123 328  51 64
            45 64  387 23
            6 98  215 314
            *   +   *   +
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 4277556);
    }
}
