use nom::{
    bytes::complete::{tag, take_till, take_until},
    character::{
        complete::{char, line_ending, space1, u64},
        is_newline,
    },
    multi::{count, many1, separated_list0},
    sequence::{preceded, tuple},
    Parser,
};

pub struct Input {
    seeds: Vec<u64>,
    maps: Vec<Vec<Conversion>>,
}

pub struct Conversion {
    dest_start: u64,
    source_start: u64,
    len: u64,
}

pub fn parse_map(input: &str) -> nom::IResult<&str, Vec<Conversion>> {
    preceded(
        tuple((take_till(|c| matches!(c, '\n' | '\r')), line_ending)),
        separated_list0(
            line_ending,
            tuple((u64, char(' '), u64, char(' '), u64)).map(
                |(dest_start, _, source_start, _, len)| Conversion {
                    dest_start,
                    source_start,
                    len,
                },
            ),
        ),
    )(input)
}

pub fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    let (input, seeds) = preceded(tag("seeds: "), separated_list0(space1, u64))(input)?;
    let (input, maps) = preceded(
        many1(line_ending),
        separated_list0(many1(line_ending), parse_map),
    )(input)?;

    Ok((input, Input { seeds, maps }))
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let res = parse_input(input);
    let (remaining, out) = res.expect("failed to parse");
    assert!(remaining.trim().is_empty(), "didn't parse entire input");
    out
}

#[aoc(day5, part1)]
pub fn part_1(input: &Input) -> u64 {
    input
        .seeds
        .iter()
        .map(|&seed| {
            input.maps.iter().fold(seed, |val, map| {
                map.iter()
                    .filter_map(|conv| {
                        if (val >= conv.source_start) && (val < conv.source_start + conv.len) {
                            Some(conv.dest_start + val - conv.source_start)
                        } else {
                            None
                        }
                    })
                    .next()
                    .unwrap_or(val)
            })
        })
        .min()
        .expect("no mappings")
}

#[aoc(day5, part2)]
pub fn part_2(input: &Input) -> u64 {
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
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69
            humidity-to-location map:
            60 56 37
            56 93 4
            "
        });
        assert_eq!(part_1(&input), 35);
        // assert_eq!(part_2(&input),);
    }
}
