fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

fn run(input: String) -> usize {
    let mut lines: Vec<&str> = input
        .lines()
        .filter(|line| !line.trim().is_empty()) // There is a fu..... trailing whitespace !
        .collect();
    assert!(!lines.is_empty(), "input should contain at least one line");

    let ops_line = lines.pop().expect("operations line missing");
    let ops: Vec<char> = ops_line
        .split_whitespace()
        .map(|s| {
            let ch = s.chars().next().expect("empty operator token");
            assert!(ch == '+' || ch == '*', "unsupported operator {}", ch);
            ch
        })
        .collect();

    let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let mut part2 = 0usize;
    let mut acc = 0usize;
    let mut op_idx = 0usize;

    for col_idx in 0..max_width {
        let mut col_chars = String::new();
        for line in &lines {
            let ch = line.chars().nth(col_idx).unwrap_or(' ');
            col_chars.push(ch);
        }

        dbg!(&col_chars);

        let is_blank_column = col_chars.chars().all(|c| c.is_whitespace());
        if is_blank_column {
            if acc != 0 {
                part2 += acc;
                acc = 0;
            }
            if op_idx + 1 < ops.len() {
                op_idx += 1;
            }
            continue;
        }

        let value_str = col_chars.trim();
        if value_str.is_empty() {
            continue;
        }

        let value = value_str.parse::<usize>().unwrap();
        dbg!(&value);
        match ops[op_idx] {
            '+' => acc += value,
            '*' => acc = if acc == 0 { value } else { acc * value },
            _ => unreachable!(),
        }
    }

    part2 += acc;

    dbg!(part2);

    part2
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
              * +   *   +
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 3263827);
    }
}
