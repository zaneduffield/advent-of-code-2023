use nom::{
    branch::alt,
    bytes::complete::*,
    character::complete::*,
    multi::*,
    sequence::{delimited, separated_pair, tuple},
    Parser,
};

pub struct Input {
    records: Vec<Record>,
}

pub struct Record {
    springs: Vec<Spring>,
    group_sizes: Vec<u8>,
}

pub enum Spring {
    Operational,
    Damaged,
    Unknown,
}

fn parse_spring(input: &str) -> nom::IResult<&str, Spring> {
    alt((
        char('.').map(|_| Spring::Operational),
        char('#').map(|_| Spring::Damaged),
        char('?').map(|_| Spring::Unknown),
    ))(input)
}

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    let (input, records) = separated_list0(
        line_ending,
        separated_pair(
            many0(parse_spring),
            char(' '),
            separated_list0(char(','), u8),
        )
        .map(|(springs, group_sizes)| Record {
            springs,
            group_sizes,
        }),
    )(input)?;
    Ok((input, Input { records }))
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

#[aoc(day12, part1)]
pub fn part_1(input: &Input) -> u32 {
    /*
    the idea here is to do a kind of DFS over the possible locations of the groups, pruning paths as they become inconsistent.
    Start with the leftmost group and find the iterate over the possible locations of it
    for each location toggle the required unknown positions (or skip if impossible) and start considering the next group with the altered state.
    if you get all the way to the end and find a valid state for the final group, then count it as a solution.
     */
    0
}

#[aoc(day12, part2)]
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
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
            "
        });
        assert_eq!(part_1(&input), 21);
        // assert_eq!(part_2(&input),);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2023/day12.txt"));
        // assert_eq!(part_1(&input), );
        // assert_eq!(part_2(&input),);
    }
}
