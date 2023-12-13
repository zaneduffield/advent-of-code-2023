use itertools::Itertools;
use nom::{bytes::complete::*, character::complete::*, multi::*, sequence::tuple, Parser};

pub struct Input {
    lines: Vec<Vec<i64>>,
}

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    let (input, lines) = separated_list0(line_ending, separated_list1(space1, i64))(input)?;
    Ok((input, Input { lines }))
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

fn seq_head_and_tail(seq: &[i64]) -> (i64, i64) {
    let mut rows = vec![seq.to_vec()];
    loop {
        rows.push(
            rows.last()
                .unwrap()
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect::<Vec<_>>(),
        );
        if rows.last().unwrap().iter().all(|i| *i == 0) {
            break;
        }
    }

    let next = rows.iter().flat_map(|row| row.last()).sum();
    let prev = rows.iter().rev().skip(1).fold(0, |acc, row| row[0] - acc);

    (prev, next)
}

#[aoc(day9, part1)]
pub fn part_1(input: &Input) -> i64 {
    input.lines.iter().map(|s| seq_head_and_tail(s).1).sum()
}

#[aoc(day9, part2)]
pub fn part_2(input: &Input) -> i64 {
    input.lines.iter().map(|s| seq_head_and_tail(s).0).sum()
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
