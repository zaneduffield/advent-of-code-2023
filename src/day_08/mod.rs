use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, multispace1},
    combinator::fail,
    multi::{many1, separated_list0},
    sequence::tuple,
    Parser,
};
use rustc_hash::FxHashMap;

pub struct Input {
    init: Id,
    goal: Id,
    instructions: Vec<Direction>,
    elements: Vec<(Id, Id)>,
}

pub enum Direction {
    Left,
    Right,
}

type Id = u16;

fn parse_dir(input: &str) -> nom::IResult<&str, Direction> {
    match input.chars().next() {
        Some('L') => Ok((&input[1..], Direction::Left)),
        Some('R') => Ok((&input[1..], Direction::Right)),
        _ => fail(input),
    }
}

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    let (input, instructions) = many1(parse_dir)(input)?;
    let (input, _) = multispace1(input)?;

    let (input, elements_str) = separated_list0(
        line_ending,
        tuple((alpha1, tag(" = ("), alpha1, tag(", "), alpha1, tag(")")))
            .map(|(from, _, left, _, right, _)| (from, left, right)),
    )(input)?;

    let mut id_by_name = FxHashMap::<&str, Id>::default();
    let mut elements = Vec::with_capacity(elements_str.len());
    let mut init = None;
    let mut goal = None;
    for (from, _, _) in &elements_str {
        let len = id_by_name.len() as u16;
        let id = id_by_name.entry(from).or_insert(len);
        if from.eq(&"AAA") {
            init = Some(*id);
        } else if from.eq(&"ZZZ") {
            goal = Some(*id);
        }
    }

    for (_, left, right) in &elements_str {
        let len = id_by_name.len() as u16;
        let id_left = *id_by_name.entry(left).or_insert(len);
        let len = id_by_name.len() as u16;
        let id_right = *id_by_name.entry(right).or_insert(len);
        elements.push((id_left, id_right));
    }

    Ok((
        input,
        Input {
            instructions,
            elements,
            init: init.expect("starting element not found"),
            goal: goal.expect("goal element not found"),
        },
    ))
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

#[aoc(day8, part1)]
pub fn part_1(input: &Input) -> u32 {
    let mut id = input.init;
    let mut steps = 0;
    while id != input.goal {
        let (left, right) = input.elements[id as usize];
        id = match input.instructions[steps % input.instructions.len()] {
            Direction::Left => left,
            Direction::Right => right,
        };
        steps += 1;
    }

    steps as u32
}

#[aoc(day8, part2)]
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
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
            "
        });
        assert_eq!(part_1(&input), 2);
        let input2 = input_generator(indoc! {
            "
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
            "
        });
        assert_eq!(part_1(&input2), 6);

        // assert_eq!(part_2(&input),);
    }
}
