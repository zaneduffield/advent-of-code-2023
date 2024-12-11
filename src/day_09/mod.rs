use itertools::Itertools;
use nom::{
    character::complete::{i64, space1},
    combinator::opt,
    sequence::preceded,
};

pub struct Input {
    solutions: (i64, i64),
}

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    let mut row = vec![];
    let mut next_row = vec![];
    let mut heads = vec![];

    let solutions = input
        .lines()
        .map(|mut line| {
            row.clear();
            while let Ok((next_line, num)) = parse_int(line) {
                line = next_line;
                row.push(num);
            }

            let mut tail_sum = 0;
            heads.clear();

            loop {
                tail_sum += row.last().unwrap_or(&0);
                heads.push(row[0]);

                next_row.clear();
                next_row.extend(row.iter().tuple_windows().map(|(a, b)| b - a));

                std::mem::swap(&mut row, &mut next_row);
                if row.iter().all(|i| *i == 0) {
                    break;
                }
            }

            let prev = heads.iter().rev().fold(0, |acc, next| next - acc);

            (prev, tail_sum)
        })
        .fold((0, 0), |acc, next| (acc.0 + next.0, acc.1 + next.1));

    Ok(("", Input { solutions }))
}

fn parse_int(input: &str) -> nom::IResult<&str, i64> {
    preceded(opt(space1), i64)(input)
}

pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

pub fn part_1(input: &Input) -> i64 {
    input.solutions.1
}

pub fn part_2(input: &Input) -> i64 {
    input.solutions.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
            "
        });
        assert_eq!(part_1(&input), 114);
        assert_eq!(part_2(&input), 2);
    }
}
