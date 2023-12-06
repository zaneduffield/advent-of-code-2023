use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1, u16},
    multi::separated_list1,
    sequence::tuple,
    Parser,
};

pub struct Input {
    times: Vec<u16>,
    best_distances: Vec<u16>,
}

const START_SPEED_MM_PER_MS: u16 = 0;
const ACCEL_MM_PER_MS_2: u16 = 1;

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    tuple((
        tag("Time:"),
        space1,
        separated_list1(space1, u16),
        line_ending,
        tag("Distance:"),
        space1,
        separated_list1(space1, u16),
    ))
    .map(|(_, _, times, _, _, _, best_distances)| Input {
        times,
        best_distances,
    })
    .parse(input)
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

#[aoc(day6, part1)]
pub fn part_1(input: &Input) -> u32 {
    input
        .times
        .iter()
        .zip(&input.best_distances)
        .map(|(time, dist)| {
            (0..*time)
                .filter(|t| (START_SPEED_MM_PER_MS + *t * ACCEL_MM_PER_MS_2) * (time - t) > *dist)
                .count() as u32
        })
        .product()
}

#[aoc(day6, part2)]
pub fn part_2(input: &Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            Time:      7  15   30
            Distance:  9  40  200
            "
        });
        assert_eq!(part_1(&input), 288);
        // assert_eq!(part_2(&input),);
    }
}
