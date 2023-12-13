use itertools::Itertools;
use nom::{
    character::complete::{i64, line_ending, space1},
    multi::{separated_list0, separated_list1},
};

pub struct Input {
    line_row_diffs: Vec<Vec<Vec<i64>>>,
}

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    let (input, lines) = separated_list0(line_ending, separated_list1(space1, i64))(input)?;
    let line_row_diffs = lines.iter().map(|seq| line_row_diffs(seq)).collect();
    Ok((input, Input { line_row_diffs }))
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

fn line_row_diffs(seq: &[i64]) -> Vec<Vec<i64>> {
    // we could probably store all these row differences in one flat vector for performance
    // but that would be a pain and this is fast enough already
    let mut row_diffs = vec![seq.to_vec()];
    loop {
        row_diffs.push(
            row_diffs
                .last()
                .unwrap()
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect(),
        );
        if row_diffs.last().unwrap().iter().all(|i| *i == 0) {
            break;
        }
    }
    row_diffs
}

#[aoc(day9, part1)]
pub fn part_1(input: &Input) -> i64 {
    input
        .line_row_diffs
        .iter()
        .map(|row_diffs| row_diffs.iter().flat_map(|row| row.last()).sum::<i64>())
        .sum()
}

#[aoc(day9, part2)]
pub fn part_2(input: &Input) -> i64 {
    input
        .line_row_diffs
        .iter()
        .map(|row_diffs| {
            row_diffs
                .iter()
                .rev()
                .skip(1)
                .fold(0, |acc, row| row[0] - acc)
        })
        .sum()
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
